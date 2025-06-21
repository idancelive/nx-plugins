//! # Claude Code Toolkit
//!
//! A comprehensive Rust toolkit for managing Claude Code credentials, session monitoring,
//! and GitHub integration. This crate provides both a command-line interface and a library
//! for programmatic access to Claude Code functionality.
//!
//! ## Features
//!
//! - **Credential Management**: Secure storage and synchronization of Claude Code credentials
//! - **Session Monitoring**: Real-time tracking of Claude Code sessions with timer functionality
//! - **GitHub Integration**: Seamless sync with GitHub repositories and organizations
//! - **Daemon Mode**: Background service for automatic credential synchronization
//! - **Cross-Platform**: Primary support for Linux, with WSL support for Windows users
//! - **Systemd Integration**: Native systemd service support on Linux (optional feature)
//! - **Desktop Notifications**: Optional desktop notification support
//!
//! ## Quick Start
//!
//! ### Command Line Usage
//!
//! ```bash
//! # Install the daemon service
//! claude-code-toolkit service install
//!
//! # Add a GitHub organization for sync
//! claude-code-toolkit org add my-org
//!
//! # Add a specific repository
//! claude-code-toolkit repo add owner/repo
//!
//! # Sync credentials to all configured targets
//! claude-code-toolkit sync
//!
//! # Check status
//! claude-code-toolkit status
//! ```
//!
//! ### Library Usage
//!
//! ```rust,no_run
//! use claude_code_toolkit::{
//!     config::manager::ConfigurationManager,
//!     sync::SyncService,
//!     Result,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Initialize configuration
//!     let config_manager = ConfigurationManager::new()?;
//!     
//!     // Set up sync service
//!     let mut sync_service = SyncService::new_with_config().await?;
//!     
//!     // Sync credentials
//!     sync_service.sync_all().await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Architecture
//!
//! The toolkit is organized into several key modules:
//!
//! - [`config`] - Configuration management and credential storage
//! - [`daemon`] - Background service functionality
//! - [`providers`] - Integration with external services (GitHub, etc.)
//! - [`sync`] - Credential synchronization logic
//! - [`cli`] - Command-line interface components
//! - [`utils`] - Utility functions and helpers
//!
//! ## Configuration
//!
//! The toolkit uses YAML-based configuration files stored in platform-appropriate
//! directories. Configuration includes:
//!
//! - GitHub API tokens and repository settings
//! - Sync intervals and retry policies
//! - Notification preferences
//! - Daemon service settings
//!
//! ## Security
//!
//! - Credentials are stored securely using platform-specific secure storage
//! - All API communications use HTTPS
//! - Sensitive data is never logged or exposed in error messages
//! - Configuration files have restrictive permissions
//!
//! ## Feature Flags
//!
//! - `systemd` (default): Enables systemd service integration on Linux
//! - `notifications` (default): Enables desktop notification support
//!
//! ## Platform Support
//!
//! | Platform | CLI | Sync | Daemon | Notes |
//! |----------|-----|------|--------|-------|
//! | 🐧 Linux | ✅ | ✅ | ✅ | Complete systemd integration |
//! | 🪟 Windows (WSL) | ✅ | ✅ | ✅ | Requires WSL with systemd enabled |
//! | 🪟 Windows (Native) | ⚠️ | ❌ | ❌ | CLI only - credentials in system store |
//! |  macOS | ⚠️ | ❌ | ❌ | CLI only - credentials in Keychain |
//!
//! **Feature Details**:
//! - **✅ Full**: Complete functionality as designed
//! - **⚠️ Partial**: CLI commands work, but credential reading fails
//! - **❌ No**: Feature unavailable due to platform limitations
//!
//! **Support Breakdown**:
//! - **CLI Full**: All commands including org/repo management, configuration
//! - **CLI Partial**: Commands work but `status`, `sync`, `timer` fail (no credential access)
//! - **Sync**: Requires file-based credential storage at `~/.claude/.credentials.json`
//! - **Daemon**: Requires systemd for background service functionality
//!
//! **Important Notes**: 
//! - This toolkit expects Claude Code credentials to be stored as **files**, not in system keychains
//! - **Expected credential location**: `~/.claude/.credentials.json`
//! - **Linux and WSL** store credentials as files at this location, making them compatible
//! - **macOS and native Windows** store credentials in Keychain/Credential Manager respectively
//! - On incompatible platforms, credential reading will fail and sync operations won't work

pub mod cli;
pub mod config;
pub mod daemon;
pub mod error;
pub mod providers;
pub mod sync;
pub mod traits;
pub mod types;
pub mod utils;

pub use error::{ ClaudeCodeError, Result };
pub use traits::*;
