//! External service provider integrations for credential synchronization.
//!
//! This module contains provider implementations for various external services
//! like GitHub, following the Repository Pattern and Dependency Injection principles.
//! Providers handle secure credential synchronization, API interactions, and
//! service-specific configuration management.
//!
//! ## Supported Providers
//!
//! - [`github`] - GitHub API integration for repository and organization sync
//! - [`registry`] - Provider registry management and factory patterns
//!
//! ## Provider Architecture
//!
//! The provider system uses several design patterns:
//!
//! - **Factory Pattern**: [`ProviderFactory`] creates providers dynamically
//! - **Repository Pattern**: Providers implement [`SecretProvider`] trait
//! - **Dependency Injection**: Providers receive configuration via constructor
//!
//! ## Usage Examples
//!
//! ### Using the Provider Factory
//!
//! ```rust,no_run
//! use claude_code_toolkit::providers::ProviderFactory;
//! use std::collections::HashMap;
//!
//! #[tokio::main]
//! async fn main() -> claude_code_toolkit::Result<()> {
//!     let factory = ProviderFactory::new();
//!     
//!     // List available providers
//!     let providers = factory.available_providers();
//!     println!("Available providers: {:?}", providers);
//!     
//!     // Create a GitHub provider
//!     let mut config = HashMap::new();
//!     config.insert("token".to_string(), "your-github-token".to_string());
//!     config.insert("org".to_string(), "your-org".to_string());
//!     
//!     let provider = factory.create("github", &config)?;
//!     // Use provider for operations...
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Direct Provider Usage
//!
//! ```rust,no_run
//! use claude_code_toolkit::providers::github::GitHubProvider;
//! use std::collections::HashMap;
//!
//! #[tokio::main]
//! async fn main() -> claude_code_toolkit::Result<()> {
//!     let mut config = HashMap::new();
//!     config.insert("token".to_string(), "your-token".to_string());
//!     
//!     let provider = GitHubProvider::new(config)?;
//!     
//!     // Use provider for operations...
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration Requirements
//!
//! Each provider has specific configuration requirements:
//!
//! ### GitHub Provider
//! - **Required**: `token` (GitHub personal access token)
//! - **Optional**: `org` (default organization), `base_url` (GitHub Enterprise URL)
//!
//! ## Security Considerations
//!
//! - All API tokens are stored securely and never logged
//! - Network communications use HTTPS with certificate validation
//! - Sensitive configuration is validated before use
//! - Rate limiting and retry logic prevent API abuse

pub mod github;
pub mod registry;

use crate::error::Result;
use crate::traits::SecretProvider;
use std::collections::HashMap;

/// Provider factory following Factory Pattern
pub struct ProviderFactory {
  creators: HashMap<String, Box<dyn ProviderCreator>>,
}

impl ProviderFactory {
  pub fn new() -> Self {
    let mut factory = Self {
      creators: HashMap::new(),
    };

    // Register built-in providers
    factory.register("github", Box::new(github::GitHubProviderCreator));

    factory
  }

  pub fn register(&mut self, name: &str, creator: Box<dyn ProviderCreator>) {
    self.creators.insert(name.to_string(), creator);
  }

  pub fn create(
    &self,
    name: &str,
    config: &HashMap<String, String>
  ) -> Result<Box<dyn SecretProvider>> {
    let creator = self.creators
      .get(name)
      .ok_or_else(|| {
        crate::error::ClaudeCodeError::Generic(format!("Unknown provider: {}", name))
      })?;

    creator.create(config)
  }

  pub fn available_providers(&self) -> Vec<&str> {
    self.creators
      .keys()
      .map(|s| s.as_str())
      .collect()
  }
}

impl Default for ProviderFactory {
  fn default() -> Self {
    Self::new()
  }
}

/// Provider creator trait for Factory Pattern
pub trait ProviderCreator: Send + Sync {
  fn create(&self, config: &HashMap<String, String>) -> Result<Box<dyn SecretProvider>>;
  fn provider_type(&self) -> &str;
  fn required_config(&self) -> Vec<&str>;
  fn optional_config(&self) -> Vec<&str> {
    Vec::new()
  }
}

/// Base provider implementation with common functionality
pub struct BaseProvider {
  pub name: String,
  pub config: HashMap<String, String>,
}

impl BaseProvider {
  pub fn new(name: &str, config: HashMap<String, String>) -> Self {
    Self {
      name: name.to_string(),
      config,
    }
  }

  pub fn get_config(&self, key: &str) -> Option<&String> {
    self.config.get(key)
  }

  pub fn require_config(&self, key: &str) -> Result<&String> {
    self
      .get_config(key)
      .ok_or_else(|| {
        crate::error::ClaudeCodeError::Generic(format!("Missing required config: {}", key))
      })
  }
}
