//! Error types and handling for the Claude Code Toolkit.
//!
//! This module defines comprehensive error types using the [`thiserror`] crate
//! for consistent error handling throughout the toolkit. All errors implement
//! standard traits and provide detailed context for debugging and user feedback.

use thiserror::Error;

/// Main error type for the Claude Code Toolkit.
///
/// This enum covers all possible error conditions that can occur during
/// toolkit operations, from file I/O and network errors to configuration
/// and validation failures. Each variant provides specific context about
/// the error condition.
///
/// # Examples
///
/// ```rust
/// use claude_code_toolkit::{ClaudeCodeError, Result};
///
/// fn validate_repo_format(repo: &str) -> Result<()> {
///     if !repo.contains('/') {
///         return Err(ClaudeCodeError::InvalidRepoFormat {
///             repo: repo.to_string(),
///         });
///     }
///     Ok(())
/// }
/// ```
#[derive(Error, Debug)]
pub enum ClaudeCodeError {
  /// File system I/O operation failed
  #[error("IO error: {0}")] 
  Io(#[from] std::io::Error),

  /// JSON serialization or deserialization failed
  #[error("JSON error: {0}")] 
  Json(#[from] serde_json::Error),

  /// YAML parsing or generation failed
  #[error("YAML error: {0}")] 
  Yaml(#[from] serde_yaml::Error),

  /// External process execution failed
  #[error("Process execution error: {0}")] 
  Process(String),

  /// HTTP request or response error
  #[error("HTTP error: {0}")] 
  Http(#[from] reqwest::Error),

  /// Configuration file or settings validation failed
  #[error("Configuration error: {0}")] 
  InvalidConfig(String),

  /// External provider (GitHub, etc.) integration error
  #[error("Provider error: {0}")] 
  Provider(String),

  /// Input validation or constraint violation
  #[error("Validation error: {0}")] 
  Validation(String),

  /// Initial setup or configuration wizard error
  #[error("Setup error: {0}")] 
  Setup(String),

  /// Claude Code credentials file not found
  #[error("Credentials not found at {path}")] 
  CredentialsNotFound {
    /// File system path where credentials were expected
    path: String,
  },

  /// Credentials file format is invalid or corrupted
  #[error("Invalid credentials format: {0}")] 
  InvalidCredentials(String),

  /// Date/time parsing failed
  #[error("Time parsing error: {0}")] 
  Time(#[from] chrono::ParseError),

  /// Desktop notification system error
  #[error("Notification error: {0}")] 
  Notification(String),

  /// Systemd service management error (Linux only)
  #[error("Systemd error: {0}")] 
  Systemd(String),

  /// Daemon service is not currently running
  #[error("Daemon not running")]
  DaemonNotRunning,

  /// Daemon service is already running
  #[error("Daemon already running")]
  DaemonAlreadyRunning,

  /// Access denied to specified resource
  #[error("Access denied to {target_type}: {name}")] 
  AccessDenied {
    /// Type of target (organization, repository, etc.)
    target_type: String,
    /// Name of the target that access was denied to
    name: String,
  },

  /// Specified target (org, repo) not found
  #[error("Target not found: {target_type} '{name}'")] 
  TargetNotFound {
    /// Type of target that was not found
    target_type: String,
    /// Name of the target that was not found
    name: String,
  },

  /// Repository name format is invalid (should be "owner/repo")
  #[error("Invalid repository format: {repo} (expected 'owner/repository')")] 
  InvalidRepoFormat {
    /// The invalid repository string provided
    repo: String,
  },

  /// Catch-all for other error conditions
  #[error("Generic error: {0}")] 
  Generic(String),
}

/// Convenient type alias for Results using [`ClaudeCodeError`].
///
/// This type alias simplifies function signatures throughout the codebase
/// by providing a default error type for all operations.
///
/// # Examples
///
/// ```rust
/// use claude_code_toolkit::Result;
///
/// fn may_fail() -> Result<String> {
///     Ok("success".to_string())
/// }
/// ```
pub type Result<T> = std::result::Result<T, ClaudeCodeError>;

impl From<String> for ClaudeCodeError {
  fn from(s: String) -> Self {
    ClaudeCodeError::Generic(s)
  }
}

impl From<&str> for ClaudeCodeError {
  fn from(s: &str) -> Self {
    ClaudeCodeError::Generic(s.to_string())
  }
}
