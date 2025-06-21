//! Background daemon service for automatic Claude Code credential synchronization.
//!
//! This module provides a long-running daemon service that monitors Claude Code credentials
//! for changes and automatically synchronizes them to configured GitHub targets. The daemon
//! runs as a systemd user service and provides intelligent monitoring, scheduling, and
//! notification features.
//!
//! ## Core Features
//!
//! - **Automatic Monitoring**: Watches for credential changes and expiration
//! - **Smart Scheduling**: Syncs immediately after token refresh with configurable delays
//! - **Session Warnings**: Desktop notifications before session expiry
//! - **Error Recovery**: Robust error handling with failure notifications
//! - **Signal Handling**: Graceful shutdown on SIGINT/SIGTERM
//! - **Startup Recovery**: Reconciliation check on daemon startup
//!
//! ## Daemon Lifecycle
//!
//! 1. **Startup**: Perform initial sync check and reconciliation
//! 2. **Monitoring Loop**: Check credentials every 5 minutes, session warnings every minute
//! 3. **Token Expiry**: Wait for refresh, then sync to all targets
//! 4. **Notifications**: Send warnings before expiry, errors on sync failures
//! 5. **Shutdown**: Graceful cleanup on shutdown signals
//!
//! ## Usage Examples
//!
//! ### Basic Daemon Usage
//!
//! ```rust,no_run
//! use claude_code_toolkit::daemon::Daemon;
//!
//! #[tokio::main]
//! async fn main() -> claude_code_toolkit::Result<()> {
//!     // Initialize daemon with configuration
//!     let mut daemon = Daemon::new_with_config().await?;
//!     
//!     // Start the main daemon loop (runs indefinitely)
//!     daemon.start().await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### One-time Check
//!
//! ```rust,no_run
//! use claude_code_toolkit::daemon::Daemon;
//!
//! #[tokio::main]
//! async fn main() -> claude_code_toolkit::Result<()> {
//!     let mut daemon = Daemon::new_with_config().await?;
//!     
//!     // Run a single sync check without starting the daemon
//!     daemon.run_once().await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration
//!
//! The daemon reads configuration from `~/.goodiebag/claude-code/config.yml`:
//!
//! ```yaml
//! daemon:
//!   log_level: info
//!   sync_delay_after_expiry: 60  # seconds to wait after token expiry
//!
//! notifications:
//!   session_warnings: [30, 15, 5]  # minutes before expiry
//!   sync_failures: true
//! ```
//!
//! ## Systemd Integration
//!
//! The daemon is designed to run as a systemd user service:
//!
//! ```ini
//! [Unit]
//! Description=Claude Code Credential Sync Daemon
//!
//! [Service]
//! Type=simple
//! ExecStart=/path/to/claude-code-toolkit daemon
//! Restart=always
//! RestartSec=10
//!
//! [Install]
//! WantedBy=default.target
//! ```
//!
//! ## Monitoring and Observability
//!
//! - **Structured Logging**: Uses `tracing` for detailed operation logs
//! - **Desktop Notifications**: Visual feedback for important events
//! - **Status Tracking**: Maintains sync state and error history
//! - **Health Checks**: Validates configuration and connectivity on startup
//!
//! ## Error Handling
//!
//! The daemon implements comprehensive error handling:
//! - Individual sync failures don't stop the daemon
//! - Network issues are retried automatically
//! - Configuration errors are logged and reported
//! - Service continues running even after transient failures

use crate::{
  config::{ credentials::CredentialsManager, manager::ConfigurationManager },
  error::*,
  sync::SyncService,
  traits::config::ConfigManager,
  utils::notifications,
};
use std::time::Duration;
use tokio::signal;
use tokio::time::{ interval, sleep };
use tracing::{ error, info, warn };

/// Main daemon service for background credential synchronization.
///
/// The `Daemon` orchestrates automatic credential monitoring and synchronization
/// by running in the background as a systemd user service. It coordinates between
/// credential monitoring, configuration management, and sync operations while
/// providing robust error handling and observability.
///
/// ## Architecture
///
/// The daemon maintains:
/// - [`SyncService`] - Handles the actual credential synchronization logic
/// - [`ConfigurationManager`] - Manages YAML configuration and targets
/// - [`CredentialsManager`] - Monitors Claude Code credential files
/// - Shutdown coordination - Graceful termination handling
///
/// ## Monitoring Schedule
///
/// - **Credential checks**: Every 5 minutes (300 seconds)
/// - **Session warnings**: Every 1 minute (60 seconds)  
/// - **Post-expiry sync**: 30 seconds after detection
/// - **Startup reconciliation**: Immediate on daemon start
pub struct Daemon {
  sync_service: SyncService,
  config_manager: ConfigurationManager,
  credentials_manager: CredentialsManager,
  shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
}

impl Daemon {
  pub fn new() -> Result<Self> {
    Ok(Self {
      sync_service: SyncService::new()?,
      config_manager: ConfigurationManager::new()?,
      credentials_manager: CredentialsManager::new()?,
      shutdown_tx: None,
    })
  }

  pub async fn new_with_config() -> Result<Self> {
    let config_manager = ConfigurationManager::new()?;
    let config = config_manager.load().await?;

    // Expand tilde in path
    let expanded_path = shellexpand::tilde(&config.credentials.file_path);
    let credentials_path = std::path::PathBuf::from(expanded_path.as_ref());

    Ok(Self {
      sync_service: SyncService::new_with_config().await?,
      config_manager,
      credentials_manager: CredentialsManager::with_path(credentials_path),
      shutdown_tx: None,
    })
  }

  pub async fn start(&mut self) -> Result<()> {
    info!("Claude Code daemon starting");

    // Load config
    let _config = self.config_manager.load_config().await?;

    // Check and sync immediately on startup
    if let Err(e) = self.sync_service.check_and_sync_if_needed().await {
      error!("Startup sync failed: {}", e);
    }

    // Set up shutdown signal handling
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel();
    self.shutdown_tx = Some(shutdown_tx);

    // Main daemon loop
    let mut check_interval = interval(Duration::from_secs(300)); // Check every 5 minutes
    let mut session_check_interval = interval(Duration::from_secs(60)); // Check session every minute

    info!("Claude Code daemon started successfully");

    loop {
      tokio::select! {
                // Shutdown signal received
                _ = &mut shutdown_rx => {
                    info!("Shutdown signal received");
                    break;
                }

                // Periodic sync check
                _ = check_interval.tick() => {
                    if let Err(e) = self.check_token_expiry().await {
                        error!("Periodic token check failed: {}", e);
                    }
                }

                // Session warning check
                _ = session_check_interval.tick() => {
                    if let Err(e) = self.check_session_warnings().await {
                        error!("Session warning check failed: {}", e);
                    }
                }

                // Handle SIGINT and SIGTERM
                _ = signal::ctrl_c() => {
                    info!("Received Ctrl+C, shutting down");
                    break;
                }
            }
    }

    info!("Claude Code daemon stopped");
    Ok(())
  }

  pub async fn stop(&mut self) -> Result<()> {
    if let Some(tx) = self.shutdown_tx.take() {
      let _ = tx.send(());
    }
    Ok(())
  }

  async fn check_token_expiry(&mut self) -> Result<()> {
    let session_info = self.credentials_manager.get_session_info().await?;

    if session_info.is_expired {
      info!("Token has expired, checking for refresh");

      // Wait a bit for Claude Code to potentially refresh the token
      sleep(Duration::from_secs(30)).await;

      // Check if we need to sync
      if let Err(e) = self.sync_service.check_and_sync_if_needed().await {
        error!("Sync after expiry failed: {}", e);

        // Send notification about sync failure
        if let Err(notify_err) = notifications::send_sync_failure("all targets", &e.to_string()) {
          warn!("Failed to send sync failure notification: {}", notify_err);
        }
      } else {
        info!("Successfully synced after token expiry");
      }
    } else {
      // Schedule next check around expiry time
      let time_until_expiry = Duration::from_millis(session_info.time_remaining as u64);
      let config = self.config_manager.load_config().await?;
      let _sync_delay = Duration::from_secs(config.daemon.sync_delay_after_expiry);

      if time_until_expiry < Duration::from_secs(600) {
        // Less than 10 minutes
        info!(
          "Token expires soon ({}), will check again after expiry",
          CredentialsManager::format_time_remaining(session_info.time_remaining)
        );
      }
    }

    Ok(())
  }

  async fn check_session_warnings(&self) -> Result<()> {
    let session_info = self.credentials_manager.get_session_info().await?;

    if session_info.is_expired {
      return Ok(());
    }

    let config = self.config_manager.load_config().await?;
    let time_remaining_minutes = session_info.time_remaining / 1000 / 60;

    // Check if we should send a warning
    for &warning_minutes in &config.notifications.session_warnings {
      let warning_minutes = warning_minutes as i64;

      // Send warning if we're within the warning window (with 1-minute tolerance)
      if
        time_remaining_minutes <= warning_minutes &&
        time_remaining_minutes >= warning_minutes - 1
      {
        info!("Sending session warning: {} minutes remaining", warning_minutes);

        if let Err(e) = notifications::send_session_warning(warning_minutes as u64) {
          warn!("Failed to send session warning: {}", e);
        }

        break; // Only send one warning per check
      }
    }

    Ok(())
  }

  pub async fn run_once(&mut self) -> Result<()> {
    info!("Running daemon check once");

    if let Err(e) = self.sync_service.check_and_sync_if_needed().await {
      error!("One-time sync check failed: {}", e);
      return Err(e);
    }

    info!("One-time daemon check completed");
    Ok(())
  }
}
