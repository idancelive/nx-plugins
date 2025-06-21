//! Credential synchronization service for automated Claude Code token management.
//!
//! This module provides intelligent credential synchronization between Claude Code
//! and external services like GitHub. It implements smart change detection, multi-target
//! sync capabilities, and state tracking to ensure efficient and reliable credential
//! distribution.
//!
//! ## Core Functionality
//!
//! - **Intelligent Sync**: Only syncs when credentials change or targets are missing secrets
//! - **Multi-Target Support**: Sync to multiple GitHub organizations and repositories
//! - **State Tracking**: Maintains sync history and prevents unnecessary operations
//! - **Error Handling**: Robust error reporting and partial failure recovery
//! - **Configuration-Driven**: Uses YAML configuration for flexible target management
//!
//! ## Architecture
//!
//! The sync service follows the Service Pattern and coordinates between:
//! - [`CredentialsManager`] - Reads Claude Code credentials from file system
//! - [`ConfigurationManager`] - Manages sync targets and settings
//! - [`ProviderRegistry`] - Handles external service integrations (GitHub)
//! - State tracking system - Prevents duplicate syncs and tracks status
//!
//! ## Usage Examples
//!
//! ### Basic Synchronization
//!
//! ```rust,no_run
//! use claude_code_toolkit::sync::SyncService;
//!
//! #[tokio::main]
//! async fn main() -> claude_code_toolkit::Result<()> {
//!     let mut sync_service = SyncService::new_with_config().await?;
//!     
//!     // Perform smart sync (only if needed)
//!     sync_service.check_and_sync_if_needed().await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Force Synchronization
//!
//! ```rust,no_run
//! use claude_code_toolkit::sync::SyncService;
//!
//! #[tokio::main]
//! async fn main() -> claude_code_toolkit::Result<()> {
//!     let mut sync_service = SyncService::new_with_config().await?;
//!     
//!     // Force sync regardless of change detection
//!     let result = sync_service.force_sync().await?;
//!     println!("Synced to {} targets, {} failed", result.succeeded, result.failed);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Check Sync Status
//!
//! ```rust,no_run
//! use claude_code_toolkit::sync::SyncService;
//!
//! #[tokio::main]
//! async fn main() -> claude_code_toolkit::Result<()> {
//!     let sync_service = SyncService::new_with_config().await?;
//!     
//!     // Check if sync is needed
//!     if sync_service.is_sync_needed().await? {
//!         println!("Sync required - credentials changed or secrets missing");
//!     }
//!     
//!     // Get target status
//!     let status = sync_service.get_sync_status().await?;
//!     for (target, available) in status {
//!         println!("Target {}: {}", target, if available { "✓" } else { "✗" });
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Smart Sync Logic
//!
//! The sync service implements intelligent change detection:
//!
//! 1. **Token Change Detection**: Compares current access token with last known state
//! 2. **Secret Validation**: Checks if required secrets exist in target repositories/organizations
//! 3. **State Persistence**: Saves sync state to `~/.goodiebag/sync-state.json`
//! 4. **Incremental Updates**: Only syncs when changes are detected
//!
//! ## Configuration Integration
//!
//! The service reads sync targets from YAML configuration:
//!
//! ```yaml
//! github:
//!   organizations:
//!     - name: my-org
//!       secret_name: CLAUDE_CODE_TOKEN
//!   repositories:
//!     - repo: owner/repository
//!       secret_name: CUSTOM_TOKEN_NAME
//! ```
//!
//! ## Error Handling
//!
//! The sync service provides comprehensive error handling:
//! - Individual target failures don't stop the entire sync process
//! - Detailed error reporting for debugging
//! - Graceful degradation when external services are unavailable
//! - State consistency maintained even during partial failures

use crate::config::{ credentials::CredentialsManager, manager::ConfigurationManager };
use crate::error::Result;
use crate::providers::registry::ProviderRegistry;
use crate::traits::config::ConfigManager;
use crate::traits::{ Credentials, SecretManager, SecretMapping, SyncResult };
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{ error, info, warn };

/// High-level synchronization service
pub struct SyncService {
  credentials_manager: CredentialsManager,
  config_manager: ConfigurationManager,
  provider_registry: ProviderRegistry,
}

impl SyncService {
  pub fn new() -> Result<Self> {
    Ok(Self {
      credentials_manager: CredentialsManager::new()?,
      config_manager: ConfigurationManager::with_yaml_provider()?,
      provider_registry: ProviderRegistry::new(),
    })
  }

  pub async fn new_with_config() -> Result<Self> {
    let config_manager = ConfigurationManager::with_yaml_provider()?;
    let config = config_manager.load().await?;

    // Expand tilde in path
    let expanded_path = shellexpand::tilde(&config.credentials.file_path);
    let credentials_path = PathBuf::from(expanded_path.as_ref());

    Ok(Self {
      credentials_manager: CredentialsManager::with_path(credentials_path),
      config_manager,
      provider_registry: ProviderRegistry::new(),
    })
  }

  /// Initialize providers from configuration
  pub async fn initialize(&mut self) -> Result<()> {
    let config = self.config_manager.load().await?;

    // Initialize GitHub provider if we have GitHub targets
    if !config.github.organizations.is_empty() || !config.github.repositories.is_empty() {
      let github_config = HashMap::new(); // GitHub provider uses gh CLI, no config needed
      match self.provider_registry.initialize_provider("github", github_config).await {
        Ok(()) => info!("Initialized GitHub provider"),
        Err(e) => warn!("Failed to initialize GitHub provider: {}", e),
      }
    }

    Ok(())
  }

  /// Convert Claude credentials to our generic format
  async fn get_credentials(&self) -> Result<Credentials> {
    let claude_creds = self.credentials_manager.read_credentials().await?;
    let config = self.config_manager.load().await?;

    // Convert credential structure to generic map
    let cred_value = serde_json::to_value(&claude_creds)?;
    let oauth_obj = cred_value
      .get(&config.credentials.json_path)
      .and_then(|v| v.as_object())
      .ok_or_else(|| {
        crate::error::ClaudeCodeError::InvalidCredentials(
          format!("Could not find '{}' in credentials file", config.credentials.json_path)
        )
      })?;

    // Build credential data dynamically based on configured mappings
    let mut credential_data = HashMap::new();
    for field_name in config.credentials.field_mappings.keys() {
      if let Some(value) = oauth_obj.get(field_name) {
        // Convert JSON value to string
        let string_value = match value {
          serde_json::Value::String(s) => s.clone(),
          serde_json::Value::Number(n) => n.to_string(),
          serde_json::Value::Bool(b) => b.to_string(),
          _ => value.to_string(),
        };
        credential_data.insert(field_name.clone(), string_value);
      }
    }

    // For backward compatibility, try to extract common fields if they exist
    let access_token = credential_data
      .get("access_token")
      .or_else(|| credential_data.get("accessToken"))
      .cloned()
      .unwrap_or_default();

    let refresh_token = credential_data
      .get("refresh_token")
      .or_else(|| credential_data.get("refreshToken"))
      .cloned();

    let expires_at = credential_data
      .get("expires_at")
      .or_else(|| credential_data.get("expiresAt"))
      .and_then(|s| s.parse::<i64>().ok());

    // Store all discovered fields in metadata for generic access
    let metadata = credential_data;

    Ok(Credentials {
      access_token,
      refresh_token,
      expires_at,
      metadata,
    })
  }

  /// Create secret mapping from configuration
  async fn get_secret_mapping(&self) -> Result<SecretMapping> {
    let config = self.config_manager.load().await?;

    let mut mapping = SecretMapping::new("claude");

    // Use the field mappings from credentials configuration
    for (field, secret_name) in &config.credentials.field_mappings {
      mapping.add_mapping(field, secret_name);
    }

    Ok(mapping)
  }

  /// Get targets from configuration
  async fn get_targets_from_config(&self) -> Result<Vec<crate::traits::Target>> {
    let config = self.config_manager.load().await?;
    let mut targets = Vec::new();

    // Add organizations
    for org in &config.github.organizations {
      targets.push(crate::traits::Target {
        provider: "github".to_string(),
        target_type: "organization".to_string(),
        name: org.name.clone(),
        config: HashMap::new(),
      });
    }

    // Add repositories
    for repo in &config.github.repositories {
      targets.push(crate::traits::Target {
        provider: "github".to_string(),
        target_type: "repository".to_string(),
        name: repo.repo.clone(),
        config: HashMap::new(),
      });
    }

    info!("Found {} targets for sync", targets.len());
    Ok(targets)
  }

  /// Perform complete credential synchronization
  pub async fn sync_all(&mut self) -> Result<SyncResult> {
    info!("Starting credential synchronization");

    // Initialize providers
    self.initialize().await?;

    // Get credentials and mapping
    let credentials = self.get_credentials().await?;
    let mapping = self.get_secret_mapping().await?;

    // Get targets from configuration
    let targets = self.get_targets_from_config().await?;

    // Perform sync
    let result = self.provider_registry.sync_credentials_to_targets(
      &credentials,
      &mapping,
      &targets
    ).await?;

    info!("Sync completed: {} succeeded, {} failed", result.succeeded, result.failed);

    if !result.errors.is_empty() {
      for error in &result.errors {
        error!("Sync error: {}", error);
      }
    }

    // Save sync state if successful
    if result.succeeded > 0 && result.failed == 0 {
      self.save_sync_state(&credentials).await?;
    }

    Ok(result)
  }

  /// Force synchronization (ignore token change checks)
  pub async fn force_sync(&mut self) -> Result<SyncResult> {
    info!("Force syncing credentials");
    self.sync_all().await
  }

  /// Check if sync is needed and perform sync if required
  pub async fn check_and_sync_if_needed(&mut self) -> Result<()> {
    if self.is_sync_needed().await? {
      let result = self.sync_all().await?;
      if result.failed > 0 {
        warn!("Sync completed with {} failures", result.failed);
      } else {
        info!("Sync completed successfully: {} targets", result.succeeded);
      }
    } else {
      info!("Credentials are already up to date, no sync needed");
    }
    Ok(())
  }

  /// Check if sync is needed (token has changed or secrets are missing)
  pub async fn is_sync_needed(&self) -> Result<bool> {
    let credentials = self.get_credentials().await?;
    let mapping = self.get_secret_mapping().await?;
    let secrets = mapping.to_secrets(&credentials);

    // First check if credentials have changed
    let state_path = std::path::Path
      ::new(&std::env::var("HOME").unwrap_or_default())
      .join(".goodiebag")
      .join("sync-state.json");

    let token_changed = if let Ok(state_data) = std::fs::read_to_string(&state_path) {
      if let Ok(last_state) = serde_json::from_str::<crate::types::SyncState>(&state_data) {
        last_state.last_token != credentials.access_token
      } else {
        true // Invalid state file
      }
    } else {
      true // No state file
    };

    if token_changed {
      info!("Access token has changed, sync needed");
      return Ok(true);
    }

    // Check if all required secrets exist in GitHub
    let targets = self.get_targets_from_config().await?;
    for target in &targets {
      for secret in &secrets {
        let args = match target.target_type.as_str() {
          "repository" => vec!["secret", "list", "--repo", &target.name],
          "organization" => vec!["secret", "list", "--org", &target.name],
          _ => {
            continue;
          }
        };

        let check_result = std::process::Command::new("gh").args(&args).output();

        match check_result {
          Ok(output) if output.status.success() => {
            let secret_list = String::from_utf8_lossy(&output.stdout);
            if !secret_list.contains(&secret.name) {
              info!(
                "Secret {} missing from {} {}, sync needed",
                secret.name,
                target.target_type,
                target.name
              );
              return Ok(true);
            }
          }
          _ => {
            warn!(
              "Could not check secrets for {} {}, assuming sync needed",
              target.target_type,
              target.name
            );
            return Ok(true);
          }
        }
      }
    }

    info!("Credentials unchanged and all secrets present, no sync needed");
    Ok(false)
  }

  /// Save sync state after successful sync
  async fn save_sync_state(&self, credentials: &Credentials) -> Result<()> {
    let state_dir = std::path::Path
      ::new(&std::env::var("HOME").unwrap_or_default())
      .join(".goodiebag");

    // Create directory if it doesn't exist
    std::fs
      ::create_dir_all(&state_dir)
      .map_err(|e| {
        crate::error::ClaudeCodeError::Generic(format!("Failed to create state directory: {}", e))
      })?;

    let state_path = state_dir.join("sync-state.json");
    let sync_state = crate::types::SyncState {
      last_sync: chrono::Utc::now().timestamp(),
      last_token: credentials.access_token.clone(),
      targets: Vec::new(), // TODO: Add target status tracking
    };

    let state_json = serde_json::to_string_pretty(&sync_state)?;
    std::fs
      ::write(&state_path, state_json)
      .map_err(|e| {
        crate::error::ClaudeCodeError::Generic(format!("Failed to save sync state: {}", e))
      })?;

    info!("Saved sync state to {:?}", state_path);
    Ok(())
  }

  /// Get sync status
  pub async fn get_sync_status(&self) -> Result<HashMap<String, bool>> {
    self.provider_registry.validate_targets().await
  }
}
