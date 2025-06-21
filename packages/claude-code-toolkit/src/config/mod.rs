//! Configuration management system for Claude Code credentials and settings.
//!
//! This module provides a comprehensive configuration management system following
//! the Repository Pattern. It handles secure credential storage, YAML-based
//! configuration files, and runtime session information.
//!
//! ## Core Components
//!
//! - [`CredentialsManager`] - Secure credential storage and retrieval
//! - [`ConfigurationManager`] - Main configuration orchestrator
//! - [`YamlConfigProvider`] - YAML file-based configuration provider
//!
//! ## Usage Examples
//!
//! ### Basic Configuration Setup
//!
//! ```rust,no_run
//! use claude_code_toolkit::config::{ConfigurationManager, CredentialsManager};
//! use claude_code_toolkit::traits::config::ConfigManager;
//!
//! #[tokio::main]
//! async fn main() -> claude_code_toolkit::Result<()> {
//!     // Initialize configuration manager
//!     let config_manager = ConfigurationManager::new()?;
//!     
//!     // Load configuration
//!     let config = config_manager.load().await?;
//!     println!("Loaded configuration with {} repositories", config.github.repositories.len());
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Credential Management
//!
//! ```rust,no_run
//! use claude_code_toolkit::config::CredentialsManager;
//!
//! #[tokio::main]
//! async fn main() -> claude_code_toolkit::Result<()> {
//!     let creds_manager = CredentialsManager::new()?;
//!     
//!     // Check if credentials exist
//!     let session_info = creds_manager.get_session_info().await?;
//!     println!("Session expires in: {} ms", session_info.time_remaining);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Security Features
//!
//! - Platform-specific secure credential storage
//! - Automatic permission setting on configuration files (600)
//! - Credential validation and expiration checking
//! - Secure credential file format with encryption support
//!
//! ## Configuration Structure
//!
//! The configuration system supports:
//! - GitHub organization and repository settings
//! - Sync intervals and retry policies
//! - Notification preferences
//! - Daemon service configuration
//! - Custom provider settings

pub mod credentials;
pub mod manager;

pub use credentials::CredentialsManager;
pub use manager::{ ConfigurationManager, YamlConfigProvider };
