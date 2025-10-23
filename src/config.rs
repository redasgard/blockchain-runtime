//! Configuration types for blockchain runtime

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::constants::*;
use crate::security::SecurityConfig;
use crate::types::NetworkMode;

/// Runtime configuration with security features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub timeout_seconds: u64,
    pub memory_limit_mb: u64,
    pub network_mode: NetworkMode,
    pub enable_monitoring: bool,
    pub blockchain_config: HashMap<String, serde_json::Value>,
    /// Security configuration
    pub security_config: SecurityConfig,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: DEFAULT_TIMEOUT_SECONDS,
            memory_limit_mb: DEFAULT_MEMORY_LIMIT_MB,
            network_mode: NetworkMode::Local,
            enable_monitoring: true,
            blockchain_config: HashMap::new(),
            security_config: SecurityConfig::default(),
        }
    }
}

impl RuntimeConfig {
    /// Create a new runtime configuration
    pub fn new(
        timeout_seconds: u64,
        memory_limit_mb: u64,
        network_mode: NetworkMode,
    ) -> Self {
        Self {
            timeout_seconds,
            memory_limit_mb,
            network_mode,
            enable_monitoring: true,
            blockchain_config: HashMap::new(),
            security_config: SecurityConfig::default(),
        }
    }

    /// Create a configuration for local development
    pub fn local_development() -> Self {
        Self {
            timeout_seconds: DEFAULT_TIMEOUT_SECONDS,
            memory_limit_mb: DEFAULT_MEMORY_LIMIT_MB,
            network_mode: NetworkMode::Local,
            enable_monitoring: true,
            blockchain_config: HashMap::new(),
            security_config: SecurityConfig::permissive(),
        }
    }

    /// Create a configuration for production
    pub fn production() -> Self {
        Self {
            timeout_seconds: DEFAULT_TIMEOUT_SECONDS,
            memory_limit_mb: DEFAULT_MEMORY_LIMIT_MB,
            network_mode: NetworkMode::MainnetFork,
            enable_monitoring: true,
            blockchain_config: HashMap::new(),
            security_config: SecurityConfig::strict(),
        }
    }

    /// Create a configuration for testing
    pub fn testing() -> Self {
        Self {
            timeout_seconds: 60, // Shorter timeout for tests
            memory_limit_mb: 256, // Lower memory limit for tests
            network_mode: NetworkMode::Local,
            enable_monitoring: false,
            blockchain_config: HashMap::new(),
            security_config: SecurityConfig::permissive(),
        }
    }

    /// Set security configuration
    pub fn with_security_config(mut self, security_config: SecurityConfig) -> Self {
        self.security_config = security_config;
        self
    }

    /// Set blockchain-specific configuration
    pub fn with_blockchain_config(mut self, key: String, value: serde_json::Value) -> Self {
        self.blockchain_config.insert(key, value);
        self
    }

    /// Enable or disable monitoring
    pub fn with_monitoring(mut self, enabled: bool) -> Self {
        self.enable_monitoring = enabled;
        self
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.timeout_seconds == 0 {
            return Err("Timeout cannot be zero".to_string());
        }

        if self.memory_limit_mb == 0 {
            return Err("Memory limit cannot be zero".to_string());
        }

        // Validate security configuration
        if self.security_config.max_call_depth == 0 {
            return Err("Maximum call depth cannot be zero".to_string());
        }

        if self.security_config.max_external_calls == 0 {
            return Err("Maximum external calls cannot be zero".to_string());
        }

        if self.security_config.max_gas_limit == 0 {
            return Err("Maximum gas limit cannot be zero".to_string());
        }

        if self.security_config.max_memory_bytes == 0 {
            return Err("Maximum memory bytes cannot be zero".to_string());
        }

        Ok(())
    }

    /// Get a human-readable description of the configuration
    pub fn describe(&self) -> String {
        format!(
            "RuntimeConfig: timeout={}s, memory={}MB, network={:?}, monitoring={}, security={}",
            self.timeout_seconds,
            self.memory_limit_mb,
            self.network_mode,
            self.enable_monitoring,
            if self.security_config.sandbox_enabled { "strict" } else { "permissive" }
        )
    }

    /// Check if this is a development configuration
    pub fn is_development(&self) -> bool {
        matches!(self.network_mode, NetworkMode::Local) && 
        !self.security_config.sandbox_enabled
    }

    /// Check if this is a production configuration
    pub fn is_production(&self) -> bool {
        matches!(self.network_mode, NetworkMode::MainnetFork) && 
        self.security_config.sandbox_enabled
    }

    /// Check if this is a test configuration
    pub fn is_test(&self) -> bool {
        self.timeout_seconds <= 60 && 
        self.memory_limit_mb <= 256 && 
        !self.enable_monitoring
    }
}

/// Builder pattern for runtime configuration
pub struct RuntimeConfigBuilder {
    config: RuntimeConfig,
}

impl RuntimeConfigBuilder {
    /// Create a new builder with default values
    pub fn new() -> Self {
        Self {
            config: RuntimeConfig::default(),
        }
    }

    /// Set timeout in seconds
    pub fn timeout_seconds(mut self, seconds: u64) -> Self {
        self.config.timeout_seconds = seconds;
        self
    }

    /// Set memory limit in MB
    pub fn memory_limit_mb(mut self, mb: u64) -> Self {
        self.config.memory_limit_mb = mb;
        self
    }

    /// Set network mode
    pub fn network_mode(mut self, mode: NetworkMode) -> Self {
        self.config.network_mode = mode;
        self
    }

    /// Enable or disable monitoring
    pub fn monitoring(mut self, enabled: bool) -> Self {
        self.config.enable_monitoring = enabled;
        self
    }

    /// Set security configuration
    pub fn security_config(mut self, config: SecurityConfig) -> Self {
        self.config.security_config = config;
        self
    }

    /// Add blockchain configuration
    pub fn blockchain_config(mut self, key: String, value: serde_json::Value) -> Self {
        self.config.blockchain_config.insert(key, value);
        self
    }

    /// Build the final configuration
    pub fn build(self) -> Result<RuntimeConfig, String> {
        self.config.validate()?;
        Ok(self.config)
    }
}

impl Default for RuntimeConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
