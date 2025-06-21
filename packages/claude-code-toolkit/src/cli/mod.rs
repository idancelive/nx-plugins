//! Command-line interface components and argument parsing.
//!
//! This module provides the complete CLI structure for the Claude Code Toolkit,
//! including command definitions, argument parsing, and subcommand organization.
//! Built using the [`clap`] crate with derive macros for clean, maintainable
//! command definitions.
//!
//! ## Command Structure
//!
//! The CLI is organized into several main command groups:
//!
//! - **Status & Monitoring**: `status`, `timer` - Session information and real-time monitoring
//! - **Organization Management**: `org add/remove/list` - GitHub organization configuration
//! - **Repository Management**: `repo add/remove/list` - Individual repository setup
//! - **Synchronization**: `sync`, `sync force`, `sync status` - Credential sync operations
//! - **Service Management**: `service install/start/stop/restart` - Daemon lifecycle
//! - **Configuration**: `configure` - Interactive setup wizard
//!
//! ## Usage Examples
//!
//! ### Basic Status Commands
//!
//! ```bash
//! # Check current session and sync status
//! claude-code-toolkit status
//!
//! # Show real-time session timer
//! claude-code-toolkit timer
//! ```
//!
//! ### Organization and Repository Management
//!
//! ```bash
//! # Add GitHub organization for automatic sync
//! claude-code-toolkit org add my-organization
//!
//! # Add specific repository
//! claude-code-toolkit repo add owner/repository-name
//!
//! # List configured targets
//! claude-code-toolkit org list
//! claude-code-toolkit repo list
//! ```
//!
//! ### Credential Synchronization
//!
//! ```bash
//! # Smart sync (only if credentials changed)
//! claude-code-toolkit sync
//!
//! # Force sync to all targets
//! claude-code-toolkit sync force
//!
//! # Check sync status across all targets
//! claude-code-toolkit sync status
//!
//! # View recent sync logs
//! claude-code-toolkit sync logs --lines 100
//! ```
//!
//! ### Daemon Service Management
//!
//! ```bash
//! # Install and start background service
//! claude-code-toolkit service install
//!
//! # Control service lifecycle
//! claude-code-toolkit service start
//! claude-code-toolkit service stop
//! claude-code-toolkit service restart
//!
//! # Remove service (keeping config)
//! claude-code-toolkit service uninstall --keep-config
//! ```
//!
//! ## Command Documentation
//!
//! Each command and subcommand includes comprehensive help text accessible via:
//!
//! ```bash
//! claude-code-toolkit --help
//! claude-code-toolkit org --help
//! claude-code-toolkit sync --help
//! ```

pub mod commands;

use clap::{ Parser, Subcommand };

/// Main CLI structure for the Claude Code Toolkit.
///
/// This struct defines the root command and global options for the CLI.
/// All functionality is accessed through subcommands defined in [`Commands`].
#[derive(Parser)]
#[command(name = "claude-code-toolkit")]
#[command(about = "Claude Code Toolkit for credential sync and session monitoring")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
  /// The subcommand to execute
  #[command(subcommand)]
  pub command: Commands,
}

/// Available top-level commands for the Claude Code Toolkit.
///
/// These commands provide access to all major functionality including
/// session monitoring, credential synchronization, service management,
/// and configuration.
#[derive(Subcommand)]
pub enum Commands {
  /// Show Claude Code session and sync status
  ///
  /// Displays current session information including:
  /// - Session expiration time and remaining duration
  /// - Last successful sync timestamps
  /// - Configured organizations and repositories
  /// - Service status (running, stopped, etc.)
  Status,

  /// Show real-time session timer
  ///
  /// Displays a live countdown timer showing time remaining
  /// in the current Claude Code session. Updates every second
  /// until session expires.
  Timer,

  /// Run as daemon (used by systemd)
  ///
  /// Starts the background daemon service for automatic
  /// credential synchronization. This command is typically
  /// called by systemd and not run directly by users.
  Daemon,

  /// Organization management
  ///
  /// Commands for managing GitHub organizations that will
  /// receive automatic credential synchronization.
  #[command(subcommand)]
  Org(OrgCommands),

  /// Repository management
  ///
  /// Commands for managing individual GitHub repositories
  /// for credential synchronization.
  #[command(subcommand)]
  Repo(RepoCommands),

  /// Sync credentials to all configured targets (smart - only if changed)
  ///
  /// Performs credential synchronization with intelligent change detection.
  /// By default, only syncs if credentials have changed since last sync.
  /// Use subcommands for forced sync or status checking.
  Sync {
    #[command(subcommand)]
    command: Option<SyncCommands>,
  },

  /// Service management
  ///
  /// Commands for managing the background daemon service,
  /// including installation, lifecycle control, and removal.
  #[command(subcommand)]
  Service(ServiceCommands),

  /// Interactive configuration wizard
  ///
  /// Launches an interactive setup process to configure
  /// GitHub integration, sync settings, and service options.
  Configure,
}

/// GitHub organization management commands.
///
/// These commands configure which GitHub organizations will receive
/// automatic credential synchronization from the Claude Code session.
#[derive(Subcommand)]
pub enum OrgCommands {
  /// Add a GitHub organization for credential sync
  ///
  /// Adds a GitHub organization to the list of sync targets.
  /// All repositories in this organization will receive
  /// credential updates when changes are detected.
  Add {
    /// GitHub organization name (e.g., "my-company")
    name: String,
  },

  /// Remove a GitHub organization
  ///
  /// Removes an organization from the sync target list.
  /// Credentials will no longer be synchronized to this organization.
  Remove {
    /// GitHub organization name to remove
    name: String,
  },

  /// List configured organizations
  ///
  /// Shows all GitHub organizations currently configured
  /// for credential synchronization, including last sync status.
  List,
}

/// GitHub repository management commands.
///
/// These commands configure individual repositories for credential sync,
/// providing more granular control than organization-wide sync.
#[derive(Subcommand)]
pub enum RepoCommands {
  /// Add a GitHub repository for credential sync (format: owner/repo)
  ///
  /// Adds a specific repository to the sync target list.
  /// This allows selective syncing without configuring entire organizations.
  Add {
    /// Repository in format "owner/repository-name" (e.g., "user/my-repo")
    repo: String,
  },

  /// Remove a GitHub repository
  ///
  /// Removes a repository from the sync target list.
  /// Credentials will no longer be synchronized to this repository.
  Remove {
    /// Repository in format "owner/repository-name" to remove
    repo: String,
  },

  /// List configured repositories
  ///
  /// Shows all repositories currently configured for credential
  /// synchronization, including last sync status and timestamps.
  List,
}

/// Credential synchronization commands.
///
/// These commands control when and how credentials are synchronized
/// to configured GitHub targets, with options for forced sync and monitoring.
#[derive(Subcommand)]
pub enum SyncCommands {
  /// Force sync credentials to all targets (ignores change detection)
  ///
  /// Performs immediate synchronization to all configured organizations
  /// and repositories, bypassing change detection. Useful for testing
  /// or when manual sync is required.
  Force,

  /// Show detailed sync status for all targets
  ///
  /// Displays comprehensive sync status including:
  /// - Last successful sync timestamps for each target
  /// - Failed sync attempts and error details
  /// - Credential change detection status
  /// - Next scheduled sync times
  Status,

  /// Show daemon sync logs
  ///
  /// Displays recent log entries from the background daemon,
  /// including sync operations, errors, and system events.
  Logs {
    /// Number of log lines to display (default: 50)
    #[arg(short, long, default_value = "50")]
    lines: usize,
  },
}

/// Background service management commands.
///
/// These commands control the lifecycle of the background daemon service
/// that handles automatic credential synchronization.
#[derive(Subcommand)]
pub enum ServiceCommands {
  /// Install and start the sync daemon
  ///
  /// Installs the systemd service and starts automatic background synchronization.
  /// Requires Linux or WSL with systemd enabled. Creates necessary configuration
  /// files and sets up auto-start on system boot.
  ///
  /// **Platform Requirements**: Linux or WSL with systemd enabled
  Install,

  /// Stop and uninstall the sync daemon
  ///
  /// Stops the running service and removes it from the system.
  /// Optionally preserves configuration files for future reinstallation.
  Uninstall {
    /// Keep configuration files when uninstalling
    #[arg(long)]
    keep_config: bool,
  },

  /// Start the sync daemon
  ///
  /// Starts the background service if it's installed but not running.
  /// The service will begin monitoring for credential changes and
  /// performing automatic synchronization.
  Start,

  /// Stop the sync daemon
  ///
  /// Stops the running background service while leaving it installed.
  /// Automatic synchronization will cease until the service is restarted.
  Stop,

  /// Restart the sync daemon
  ///
  /// Stops and then starts the background service, useful for
  /// applying configuration changes or recovering from errors.
  Restart,

  /// Enable daemon auto-start on system boot
  ///
  /// Configures the service to start automatically when the system boots.
  /// Ensures continuous credential synchronization across reboots.
  Enable,

  /// Disable daemon auto-start
  ///
  /// Prevents the service from starting automatically on system boot.
  /// The service can still be started manually when needed.
  Disable,
}
