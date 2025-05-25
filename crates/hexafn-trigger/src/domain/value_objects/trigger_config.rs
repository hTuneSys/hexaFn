// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Trigger Configuration Value Object
//! 
//! Represents the configuration settings for a trigger, ensuring
//! all configuration values are valid and type-safe.

use crate::domain::value_objects::{TriggerName, TriggerCondition};
use hexafn_core::types::{Timestamp, ValidationError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for trigger creation and management
///
/// # Examples
///
/// ```rust
/// use hexafn_trigger::domain::value_objects::{TriggerConfig, TriggerCondition};
///
/// // Timer-based trigger
/// let config = TriggerConfig::new(
///     "heartbeat",
///     TriggerCondition::Timer("5s".to_string())
/// )?;
///
/// // Event-based trigger with metadata
/// let mut config = TriggerConfig::new(
///     "user-signup",
///     TriggerCondition::Event("user.created".to_string())
/// )?;
/// config = config.with_metadata("source", "webhook");
/// config = config.with_description("Trigger on user registration");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriggerConfig {
    /// Human-readable name for the trigger
    name: TriggerName,
    
    /// Condition that determines when trigger executes
    condition: TriggerCondition,
    
    /// Optional description of trigger purpose
    description: Option<String>,
    
    /// Key-value metadata for trigger context
    metadata: HashMap<String, String>,
    
    /// Whether trigger is enabled by default
    enabled: bool,
    
    /// Maximum number of times trigger can execute
    max_executions: Option<u64>,
    
    /// Timeout for trigger execution
    timeout_seconds: Option<u64>,
    
    /// Configuration creation timestamp
    created_at: Timestamp,
}

impl TriggerConfig {
    /// Create a new trigger configuration
    ///
    /// # Arguments
    ///
    /// * `name` - Human-readable trigger name
    /// * `condition` - Execution condition
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` if:
    /// - Name is invalid (empty, too long, invalid characters)
    /// - Condition is invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::{TriggerConfig, TriggerCondition};
    ///
    /// let config = TriggerConfig::new(
    ///     "daily-backup",
    ///     TriggerCondition::Timer("24h".to_string())
    /// )?;
    /// ```
    pub fn new<N: Into<String>>(
        name: N,
        condition: TriggerCondition,
    ) -> Result<Self, ValidationError> {
        let name = TriggerName::new(name)?;
        condition.validate()?;
        
        Ok(Self {
            name,
            condition,
            description: None,
            metadata: HashMap::new(),
            enabled: true,
            max_executions: None,
            timeout_seconds: Some(30), // Default 30 second timeout
            created_at: Timestamp::now(),
        })
    }
    
    /// Add a description to the trigger configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// let config = TriggerConfig::new("test", condition)?
    ///     .with_description("Test trigger for development");
    /// ```
    pub fn with_description<D: Into<String>>(mut self, description: D) -> Self {
        self.description = Some(description.into());
        self
    }
    
    /// Add metadata key-value pair
    ///
    /// # Examples
    ///
    /// ```rust
    /// let config = TriggerConfig::new("test", condition)?
    ///     .with_metadata("environment", "production")
    ///     .with_metadata("team", "backend");
    /// ```
    pub fn with_metadata<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
    
    /// Set whether trigger is enabled
    ///
    /// # Examples
    ///
    /// ```rust
    /// let config = TriggerConfig::new("test", condition)?
    ///     .with_enabled(false); // Disabled by default
    /// ```
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    
    /// Set maximum number of executions
    ///
    /// # Examples
    ///
    /// ```rust
    /// let config = TriggerConfig::new("test", condition)?
    ///     .with_max_executions(100); // Execute at most 100 times
    /// ```
    pub fn with_max_executions(mut self, max_executions: u64) -> Self {
        self.max_executions = Some(max_executions);
        self
    }
    
    /// Set execution timeout in seconds
    ///
    /// # Examples
    ///
    /// ```rust
    /// let config = TriggerConfig::new("test", condition)?
    ///     .with_timeout_seconds(60); // 1 minute timeout
    /// ```
    pub fn with_timeout_seconds(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = Some(timeout_seconds);
        self
    }
    
    /// Get trigger name
    pub fn name(&self) -> &TriggerName {
        &self.name
    }
    
    /// Get trigger condition
    pub fn condition(&self) -> &TriggerCondition {
        &self.condition
    }
    
    /// Get trigger description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    
    /// Get trigger metadata
    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }
    
    /// Check if trigger is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Get maximum executions limit
    pub fn max_executions(&self) -> Option<u64> {
        self.max_executions
    }
    
    /// Get timeout in seconds
    pub fn timeout_seconds(&self) -> Option<u64> {
        self.timeout_seconds
    }
    
    /// Get creation timestamp
    pub fn created_at(&self) -> &Timestamp {
        &self.created_at
    }
    
    /// Validate entire configuration
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` if any configuration is invalid
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Validate name
        self.name.validate()?;
        
        // Validate condition
        self.condition.validate()?;
        
        // Validate timeout
        if let Some(timeout) = self.timeout_seconds {
            if timeout == 0 {
                return Err(ValidationError::InvalidValue {
                    field: "timeout_seconds".to_string(),
                    value: timeout.to_string(),
                    reason: "Timeout must be greater than 0".to_string(),
                });
            }
            if timeout > 3600 { // Max 1 hour
                return Err(ValidationError::InvalidValue {
                    field: "timeout_seconds".to_string(),
                    value: timeout.to_string(),
                    reason: "Timeout cannot exceed 3600 seconds (1 hour)".to_string(),
                });
            }
        }
        
        // Validate max executions
        if let Some(max_exec) = self.max_executions {
            if max_exec == 0 {
                return Err(ValidationError::InvalidValue {
                    field: "max_executions".to_string(),
                    value: max_exec.to_string(),
                    reason: "Max executions must be greater than 0".to_string(),
                });
            }
        }
        
        // Validate metadata
        for (key, value) in &self.metadata {
            if key.is_empty() {
                return Err(ValidationError::EmptyValue {
                    field: "metadata_key".to_string(),
                });
            }
            if key.len() > 100 {
                return Err(ValidationError::TooLong {
                    field: "metadata_key".to_string(),
                    length: key.len(),
                    max: 100,
                });
            }
            if value.len() > 1000 {
                return Err(ValidationError::TooLong {
                    field: "metadata_value".to_string(),
                    length: value.len(),
                    max: 1000,
                });
            }
        }
        
        Ok(())
    }
}

impl std::fmt::Display for TriggerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TriggerConfig[{}]({})", self.name, self.condition)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::TriggerCondition;
    
    #[test]
    fn test_trigger_config_creation() {
        let config = TriggerConfig::new(
            "test-trigger",
            TriggerCondition::Timer("5s".to_string())
        ).unwrap();
        
        assert_eq!(config.name().value(), "test-trigger");
        assert!(config.is_enabled());
        assert_eq!(config.timeout_seconds(), Some(30));
        assert!(config.description().is_none());
        assert!(config.metadata().is_empty());
    }
    
    #[test]
    fn test_trigger_config_with_fluent_interface() {
        let config = TriggerConfig::new(
            "test-trigger",
            TriggerCondition::Timer("5s".to_string())
        ).unwrap()
            .with_description("Test trigger for development")
            .with_metadata("env", "test")
            .with_metadata("team", "backend")
            .with_enabled(false)
            .with_max_executions(100)
            .with_timeout_seconds(60);
        
        assert_eq!(config.description(), Some("Test trigger for development"));
        assert_eq!(config.metadata().get("env"), Some(&"test".to_string()));
        assert_eq!(config.metadata().get("team"), Some(&"backend".to_string()));
        assert!(!config.is_enabled());
        assert_eq!(config.max_executions(), Some(100));
        assert_eq!(config.timeout_seconds(), Some(60));
    }
    
    #[test]
    fn test_trigger_config_validation() {
        let config = TriggerConfig::new(
            "valid-trigger",
            TriggerCondition::Timer("5s".to_string())
        ).unwrap();
        
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_trigger_config_invalid_timeout() {
        let config = TriggerConfig::new(
            "test-trigger",
            TriggerCondition::Timer("5s".to_string())
        ).unwrap()
            .with_timeout_seconds(0);
        
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_trigger_config_timeout_too_long() {
        let config = TriggerConfig::new(
            "test-trigger",
            TriggerCondition::Timer("5s".to_string())
        ).unwrap()
            .with_timeout_seconds(3601); // Over 1 hour
        
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_trigger_config_display() {
        let config = TriggerConfig::new(
            "test-trigger",
            TriggerCondition::Timer("5s".to_string())
        ).unwrap();
        
        let display_str = format!("{}", config);
        assert!(display_str.contains("test-trigger"));
        assert!(display_str.contains("Timer"));
    }
}