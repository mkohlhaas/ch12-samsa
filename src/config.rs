//! Configuration management demonstrating Result and Option patterns
//!
//! This module shows how to use Result and Option types for:
//! - Robust configuration loading with error handling
//! - Builder pattern with validation
//! - Configuration composition and fallbacks
//! - Parse, don't validate principle

use crate::error::{Result, SamsaError};
use std::num::NonZeroUsize;
use std::path::Path;
use std::time::Duration;

// ============ //
// Samsa Config //
// ============ //

/// Complete Samsa service configuration
#[derive(Debug, Clone)]
pub struct SamsaConfig {
    pub broker_config: BrokerConfig,
    pub log_level: String,
    pub storage_path: String,
}

impl SamsaConfig {
    /// Load configuration from a file with fallback chain
    pub fn load() -> Result<Self> {
        // Try multiple configuration locations in order
        Self::load_from_file("samsa.conf")
            .or_else(|_| Self::load_from_file("/etc/samsa/samsa.conf"))
            .or_else(|_| Self::load_from_file("/usr/local/etc/samsa.conf"))
            .or_else(|_| {
                eprintln!("Warning: Using default configuration");
                Ok(Self::default())
            })
    }

    /// Load configuration from a specific file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|_| SamsaError::config("Configuration file not found"))?;

        Self::parse_config(&content)
    }

    /// Parse configuration from text content
    fn parse_config(content: &str) -> Result<Self> {
        // Collect broker settings, then create via the builder
        let mut port: Option<u16> = None;
        let mut max_connections: Option<usize> = None;
        let mut buffer_size: Option<usize> = None;
        let mut connection_timeout: Option<Duration> = None;
        let mut enable_metrics: Option<bool> = None;

        let mut config = Self::default();

        for line in content.lines() {
            let line = line.trim();

            // skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                match key.trim() {
                    // Broker Config
                    "broker_port" => {
                        port = Some(
                            value
                                .trim()
                                .parse()
                                .map_err(|_| SamsaError::config("Invalid port"))?,
                        )
                    }
                    "buffer_size" => {
                        buffer_size = Some(
                            value
                                .trim()
                                .parse()
                                .map_err(|_| SamsaError::config("Invalid buffer_size"))?,
                        )
                    }
                    "connection_timeout" => {
                        let timeout: u64 = value
                            .trim()
                            .parse()
                            .map_err(|_| SamsaError::config("Invalid connection_timeout"))?;
                        connection_timeout = Some(Duration::from_secs(timeout));
                    }
                    "enable_metrics" => {
                        enable_metrics = Some(
                            value
                                .trim()
                                .parse()
                                .map_err(|_| SamsaError::config("Invalid enable_metrics"))?,
                        )
                    }
                    "max_connections" => {
                        max_connections = Some(
                            value
                                .trim()
                                .parse()
                                .map_err(|_| SamsaError::config("Invalid max_connections"))?,
                        )
                    }
                    // Log Level
                    "log_level" => {
                        config.log_level = value.trim().to_string();
                    }
                    // Storage Path
                    "storage_path" => {
                        config.storage_path = value.trim().to_string();
                    }
                    _ => {} // Ignore unknown keys
                }
            }
        }

        let mut broker_builder = BrokerConfigBuilder::new();
        if let Some(port) = port {
            broker_builder = broker_builder.port(port);
        }
        if let Some(max) = max_connections {
            broker_builder = broker_builder.max_connections(max);
        }
        if let Some(size) = buffer_size {
            broker_builder = broker_builder.buffer_size(size);
        }
        if let Some(timeout) = connection_timeout {
            broker_builder = broker_builder.connection_timeout(timeout);
        }
        if let Some(enable) = enable_metrics {
            broker_builder = broker_builder.enable_metrics(enable);
        }
        config.broker_config = broker_builder.build()?;

        config.validate()?;
        Ok(config)
    }

    /// Validate configuration
    fn validate(&self) -> Result<()> {
        // BrokerConfig is always valid because it can only be created
        // through BrokerConfigBuilder::build(), which validates it.
        if self.log_level.is_empty() {
            return Err(SamsaError::config("log_level cannot be empty"));
        }

        Ok(())
    }
}

impl Default for SamsaConfig {
    fn default() -> Self {
        Self {
            broker_config: BrokerConfig::builder()
                .build()
                .expect("default broker config is valid"),
            log_level: "info".to_string(),
            storage_path: "memory://".to_string(),
        }
    }
}

// =============  //
// Broker Config  //
// =============  //

/// Broker-specific configuration
///
/// A `BrokerConfig` can only be created through [BrokerConfigBuilder],
/// which validates the configuration before a value comes into existence.
#[derive(Debug, Clone)]
pub struct BrokerConfig {
    buffer_size: NonZeroUsize,
    connection_timeout: Duration,
    enable_metrics: bool,
    max_connections: usize,
    port: u16,
}

impl BrokerConfig {
    /// Create a new builder for BrokerConfig
    pub fn builder() -> BrokerConfigBuilder {
        BrokerConfigBuilder::new()
    }

    /// Buffer size for the broker
    pub fn buffer_size(&self) -> NonZeroUsize {
        self.buffer_size
    }

    /// Connection timeout for the broker
    pub fn connection_timeout(&self) -> Duration {
        self.connection_timeout
    }

    /// Whether metrics are enabled
    pub fn enable_metrics(&self) -> bool {
        self.enable_metrics
    }

    /// Maximum number of connections
    pub fn max_connections(&self) -> usize {
        self.max_connections
    }

    /// Port the broker listens on
    pub fn port(&self) -> u16 {
        self.port
    }
}

// ===================== //
// Broker Config Builder //
// ===================== //

/// Builder for BrokerConfig demonstrating the builder pattern with validation
///
/// Configurations are validated in `build()` so an invalid `BrokerConfig`
/// can never be created.
///
/// Enforcement of "builder-only" creation!
#[derive(Default)]
pub struct BrokerConfigBuilder {
    port: Option<u16>,
    max_connections: Option<usize>,
    buffer_size: Option<usize>,
    connection_timeout: Option<Duration>,
    enable_metrics: bool,
}

impl BrokerConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = Some(max);
        self
    }

    pub fn buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = Some(size);
        self
    }

    pub fn connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = Some(timeout);
        self
    }

    pub fn enable_metrics(mut self, enable: bool) -> Self {
        self.enable_metrics = enable;
        self
    }

    /// Build the config, validating all fields
    pub fn build(self) -> Result<BrokerConfig> {
        let buffer_size = NonZeroUsize::new(self.buffer_size.unwrap_or(4096))
            .ok_or_else(|| SamsaError::config("Buffer size must be positive"))?;

        let config = BrokerConfig {
            port: self.port.unwrap_or(8080),
            max_connections: self.max_connections.unwrap_or(1000),
            buffer_size,
            connection_timeout: self.connection_timeout.unwrap_or(Duration::from_secs(30)),
            enable_metrics: self.enable_metrics,
        };

        // Validation

        if config.port == 0 {
            return Err(SamsaError::config("Port cannot be zero"));
        }

        if config.port < 1024 {
            return Err(SamsaError::config(
                "Port cannot be in reserved range (0-1023)",
            ));
        }

        if config.max_connections == 0 {
            return Err(SamsaError::config("Max connections must be positive"));
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = SamsaConfig::default();
        assert_eq!(config.broker_config.port(), 8080);
        assert_eq!(config.broker_config.max_connections(), 1000);
    }

    #[test]
    fn test_config_builder() {
        let config = BrokerConfig::builder()
            .port(9000)
            .max_connections(500)
            .enable_metrics(true)
            .build()
            .unwrap();

        assert_eq!(config.port(), 9000);
        assert_eq!(config.max_connections(), 500);
        assert!(config.enable_metrics());
    }

    #[test]
    fn test_validation() {
        // Validation happens at build time; invalid values are rejected
        let result = BrokerConfig::builder().port(0).build();
        assert!(result.is_err());

        let result = BrokerConfig::builder().port(500).build(); // Reserved range
        assert!(result.is_err());

        // Valid configuration succeeds
        let result = BrokerConfig::builder().port(8080).build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_validation() {
        // A valid port builds successfully
        let valid = BrokerConfigBuilder::new().port(8080).build();
        assert!(valid.is_ok());

        // A reserved port fails at build time
        let result = BrokerConfigBuilder::new().port(80).build();
        assert!(result.is_err());

        // Zero max connections fails at build time
        let result = BrokerConfigBuilder::new().max_connections(0).build();
        assert!(result.is_err());
    }
}
