// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Domain Value Objects Module
//! 
//! This module contains all value objects for the hexafn-trigger domain.
//! Value objects represent immutable business concepts with validation and
//! type safety, following Domain-Driven Design principles.
//!
//! ## Value Objects Overview
//!
//! - **TriggerConfig**: Complete trigger configuration with builder pattern
//! - **TriggerCondition**: Complex conditional logic with composition support
//! - **TriggerName**: Validated naming with business rules
//! - **TriggerId**: UUID-based unique identification
//! - **TriggerState**: Complete state machine management
//!
//! ## Design Principles
//!
//! All value objects in this module follow these principles:
//! - **Immutability**: Value objects cannot be modified after creation
//! - **Value Equality**: Equality based on value, not identity
//! - **Validation**: All inputs validated at construction time
//! - **Self-Validation**: Objects can validate their own consistency
//! - **Serialization**: Full serde support for persistence and transport
//!
//! ## Usage Examples
//!
//! ```rust
//! use hexafn_trigger::domain::value_objects::*;
//!
//! // Create trigger configuration
//! let config = TriggerConfig::new(
//!     TriggerName::new("user_signup_trigger")?,
//!     TriggerCondition::timer("5s")?
//! )?
//! .with_description("Trigger for user signup events")
//! .with_metadata("team", "backend")
//! .build()?;
//!
//! // Create trigger state
//! let state = TriggerState::new(StateType::Inactive)
//!     .transition_to(StateType::Active)?;
//!
//! // Complex condition composition
//! let condition = TriggerCondition::timer("10s")?
//!     .and(TriggerCondition::event("user.created")?);
//! ```

pub mod trigger_config;
pub mod trigger_condition;
pub mod trigger_name;
pub mod trigger_id;
pub mod trigger_state;

// Re-export all public types for convenient access
pub use trigger_config::TriggerConfig;

pub use trigger_condition::{
    TriggerCondition, 
    TimerExpression, 
    EventPattern, 
    LogicalExpression,
    LogicalOperator
};

pub use trigger_name::TriggerName;

pub use trigger_id::TriggerId;

pub use trigger_state::{
    TriggerState, 
    StateType, 
    StateTransitionError
};

/// Module version for compatibility tracking
pub const VALUE_OBJECTS_VERSION: &str = "0.1.0";

/// Supported trigger condition types for external API documentation
pub const SUPPORTED_CONDITION_TYPES: &[&str] = &[
    "always", 
    "never", 
    "timer", 
    "event", 
    "expression", 
    "composite"
];

/// Supported trigger state types for external API documentation
pub const SUPPORTED_STATE_TYPES: &[&str] = &[
    "inactive",
    "active", 
    "executing",
    "success",
    "failed",
    "suspended",
    "archived"
];

/// Maximum allowed values for various constraints
pub mod limits {
    /// Maximum trigger name length
    pub const MAX_TRIGGER_NAME_LENGTH: usize = 64;
    
    /// Maximum trigger description length
    pub const MAX_DESCRIPTION_LENGTH: usize = 1000;
    
    /// Maximum metadata key length
    pub const MAX_METADATA_KEY_LENGTH: usize = 100;
    
    /// Maximum metadata value length
    pub const MAX_METADATA_VALUE_LENGTH: usize = 1000;
    
    /// Maximum number of metadata entries per trigger
    pub const MAX_METADATA_ENTRIES: usize = 50;
    
    /// Maximum timer duration in seconds (30 days)
    pub const MAX_TIMER_DURATION_SECONDS: u64 = 30 * 24 * 60 * 60;
    
    /// Minimum timer duration in seconds
    pub const MIN_TIMER_DURATION_SECONDS: u64 = 1;
    
    /// Maximum event pattern length
    pub const MAX_EVENT_PATTERN_LENGTH: usize = 255;
    
    /// Maximum logical expression length
    pub const MAX_LOGICAL_EXPRESSION_LENGTH: usize = 1000;
    
    /// Maximum consecutive failures before auto-suspension
    pub const DEFAULT_MAX_FAILURES: u64 = 10;
}

/// Common validation patterns used across value objects
pub mod patterns {
    use regex::Regex;
    
    /// Regex pattern for valid trigger names
    pub fn trigger_name_pattern() -> &'static Regex {
        static PATTERN: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
        PATTERN.get_or_init(|| {
            Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_-]*$")
                .expect("Invalid trigger name regex pattern")
        })
    }
    
    /// Regex pattern for timer duration strings
    pub fn timer_duration_pattern() -> &'static Regex {
        static PATTERN: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
        PATTERN.get_or_init(|| {
            Regex::new(r"^(\d+)(s|m|h|d)$")
                .expect("Invalid timer duration regex pattern")
        })
    }
    
    /// UUID pattern validation
    pub fn uuid_pattern() -> &'static Regex {
        static PATTERN: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
        PATTERN.get_or_init(|| {
            Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$")
                .expect("Invalid UUID regex pattern")
        })
    }
}

/// Reserved trigger names that cannot be used
pub mod reserved {
    /// System reserved names
    pub const SYSTEM_RESERVED: &[&str] = &[
        "system", "admin", "root", "default", "config"
    ];
    
    /// hexaFn reserved names
    pub const HEXAFN_RESERVED: &[&str] = &[
        "hexafn", "hexa", "trigger", "condition", "pipeline"
    ];
    
    /// 6F Lifecycle reserved names
    pub const LIFECYCLE_RESERVED: &[&str] = &[
        "feed", "filter", "format", "function", "forward", "feedback"
    ];
    
    /// Module reserved names
    pub const MODULE_RESERVED: &[&str] = &[
        "store", "cast", "run", "watch", "bridge", "core"
    ];
    
    /// Temporal reserved names
    pub const TEMPORAL_RESERVED: &[&str] = &[
        "timer", "schedule", "cron", "interval", "delay"
    ];
    
    /// Event reserved names
    pub const EVENT_RESERVED: &[&str] = &[
        "event", "emit", "publish", "subscribe", "topic"
    ];
    
    /// Get all reserved names as a single slice
    pub fn all_reserved() -> Vec<&'static str> {
        let mut reserved = Vec::new();
        reserved.extend_from_slice(SYSTEM_RESERVED);
        reserved.extend_from_slice(HEXAFN_RESERVED);
        reserved.extend_from_slice(LIFECYCLE_RESERVED);
        reserved.extend_from_slice(MODULE_RESERVED);
        reserved.extend_from_slice(TEMPORAL_RESERVED);
        reserved.extend_from_slice(EVENT_RESERVED);
        reserved
    }
    
    /// Check if a name is reserved
    pub fn is_reserved(name: &str) -> bool {
        let lowercase_name = name.to_lowercase();
        all_reserved().iter().any(|&reserved| reserved == lowercase_name)
    }
}

/// Utility functions for working with value objects
pub mod utils {
    use super::*;
    use hexafn_core::types::ValidationError;
    
    /// Create a basic trigger configuration for testing
    pub fn create_test_config(name: &str) -> Result<TriggerConfig, ValidationError> {
        TriggerConfig::new(
            TriggerName::new(name)?,
            TriggerCondition::timer("5s")?
        )
    }
    
    /// Create a trigger with timer condition
    pub fn create_timer_config(
        name: &str, 
        duration: &str
    ) -> Result<TriggerConfig, ValidationError> {
        TriggerConfig::new(
            TriggerName::new(name)?,
            TriggerCondition::timer(duration)?
        )
    }
    
    /// Create a trigger with event condition
    pub fn create_event_config(
        name: &str, 
        event_pattern: &str
    ) -> Result<TriggerConfig, ValidationError> {
        TriggerConfig::new(
            TriggerName::new(name)?,
            TriggerCondition::event(event_pattern)?
        )
    }
    
    /// Validate a trigger name without creating the object
    pub fn validate_trigger_name(name: &str) -> Result<(), ValidationError> {
        TriggerName::new(name).map(|_| ())
    }
    
    /// Parse duration string to seconds
    pub fn parse_duration_to_seconds(duration: &str) -> Result<u64, ValidationError> {
        let timer_expr = TimerExpression::new(duration)?;
        Ok(timer_expr.duration()?.as_secs())
    }
    
    /// Check if a state transition is valid
    pub fn is_valid_state_transition(from: StateType, to: StateType) -> bool {
        from.can_transition_to(to)
    }
    
    /// Get all possible transitions from a given state
    pub fn get_possible_transitions(from: StateType) -> Vec<StateType> {
        use StateType::*;
        
        match from {
            Inactive => vec![Active, Archived],
            Active => vec![Executing, Suspended, Inactive, Archived],
            Executing => vec![Success, Failed, Suspended],
            Success => vec![Active, Suspended, Archived],
            Failed => vec![Active, Suspended, Archived], 
            Suspended => vec![Active, Inactive, Archived],
            Archived => vec![], // Terminal state
        }
    }
}

/// Common error messages for consistent user experience
pub mod errors {
    /// Trigger name validation error messages
    pub mod trigger_name {
        pub const EMPTY_NAME: &str = "Trigger name cannot be empty";
        pub const TOO_LONG: &str = "Trigger name exceeds maximum length";
        pub const INVALID_CHARS: &str = "Trigger name contains invalid characters";
        pub const INVALID_START: &str = "Trigger name must start with letter or underscore";
        pub const RESERVED_NAME: &str = "Trigger name is reserved and cannot be used";
    }
    
    /// Trigger condition validation error messages
    pub mod trigger_condition {
        pub const INVALID_TIMER: &str = "Invalid timer duration format";
        pub const INVALID_EVENT: &str = "Invalid event pattern";
        pub const INVALID_EXPRESSION: &str = "Invalid logical expression";
        pub const EMPTY_CONDITION: &str = "Trigger condition cannot be empty";
    }
    
    /// Trigger state validation error messages
    pub mod trigger_state {
        pub const INVALID_TRANSITION: &str = "Invalid state transition";
        pub const MAX_FAILURES_EXCEEDED: &str = "Maximum failure count exceeded";
        pub const EXECUTION_COUNT_MISMATCH: &str = "Execution count inconsistency";
        pub const INVALID_STATE_DATA: &str = "Invalid state data";
    }
}

/// Type aliases for commonly used combinations
pub type TriggerResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub type ConfigResult = Result<TriggerConfig, ValidationError>;
pub type StateResult = Result<TriggerState, StateTransitionError>;
pub type ConditionResult = Result<TriggerCondition, ValidationError>;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_module_constants() {
        assert_eq!(VALUE_OBJECTS_VERSION, "0.1.0");
        assert!(SUPPORTED_CONDITION_TYPES.contains(&"timer"));
        assert!(SUPPORTED_STATE_TYPES.contains(&"active"));
    }
    
    #[test]
    fn test_limits() {
        assert_eq!(limits::MAX_TRIGGER_NAME_LENGTH, 64);
        assert_eq!(limits::MIN_TIMER_DURATION_SECONDS, 1);
        assert_eq!(limits::MAX_TIMER_DURATION_SECONDS, 30 * 24 * 60 * 60);
    }
    
    #[test]
    fn test_reserved_names() {
        assert!(reserved::is_reserved("system"));
        assert!(reserved::is_reserved("HEXAFN"));
        assert!(reserved::is_reserved("Feed"));
        assert!(!reserved::is_reserved("my_trigger"));
    }
    
    #[test]
    fn test_pattern_validation() {
        let name_pattern = patterns::trigger_name_pattern();
        assert!(name_pattern.is_match("valid_trigger"));
        assert!(name_pattern.is_match("_internal"));
        assert!(!name_pattern.is_match("123invalid"));
        assert!(!name_pattern.is_match("invalid@name"));
        
        let duration_pattern = patterns::timer_duration_pattern();
        assert!(duration_pattern.is_match("5s"));
        assert!(duration_pattern.is_match("10m"));
        assert!(!duration_pattern.is_match("invalid"));
    }
    
    #[test]
    fn test_utility_functions() {
        // Test config creation
        let config = utils::create_test_config("test_trigger");
        assert!(config.is_ok());
        
        // Test timer config
        let timer_config = utils::create_timer_config("timer_trigger", "10s");
        assert!(timer_config.is_ok());
        
        // Test event config
        let event_config = utils::create_event_config("event_trigger", "user.created");
        assert!(event_config.is_ok());
        
        // Test name validation
        assert!(utils::validate_trigger_name("valid_name").is_ok());
        assert!(utils::validate_trigger_name("").is_err());
        
        // Test duration parsing
        let seconds = utils::parse_duration_to_seconds("5s");
        assert_eq!(seconds.unwrap(), 5);
        
        // Test state transitions
        assert!(utils::is_valid_state_transition(StateType::Inactive, StateType::Active));
        assert!(!utils::is_valid_state_transition(StateType::Archived, StateType::Active));
        
        // Test possible transitions
        let transitions = utils::get_possible_transitions(StateType::Active);
        assert!(transitions.contains(&StateType::Executing));
        assert!(transitions.contains(&StateType::Suspended));
    }
    
    #[test]
    fn test_complete_value_object_integration() {
        // Test complete integration of all value objects
        let trigger_id = TriggerId::new();
        let trigger_name = TriggerName::new("integration_test_trigger").unwrap();
        let condition = TriggerCondition::timer("30s").unwrap();
        let config = TriggerConfig::new(trigger_name, condition).unwrap();
        let state = TriggerState::new(StateType::Inactive);
        
        // Verify all objects are created correctly
        assert!(!trigger_id.is_nil());
        assert_eq!(config.name().value(), "integration_test_trigger");
        assert!(config.condition().is_timer());
        assert_eq!(state.current_state(), StateType::Inactive);
        
        // Test state transitions
        let active_state = state.transition_to(StateType::Active).unwrap();
        assert_eq!(active_state.current_state(), StateType::Active);
        assert!(active_state.can_execute());
        
        // Test condition evaluation (conceptual)
        assert_eq!(config.condition().to_string(), "Timer(30s)");
        
        // Test serialization (all objects implement Serialize)
        let config_json = serde_json::to_string(&config);
        assert!(config_json.is_ok());
        
        let state_json = serde_json::to_string(&active_state);
        assert!(state_json.is_ok());
    }
    
    #[test]
    fn test_error_consistency() {
        // Test that error messages are consistent
        assert!(!errors::trigger_name::EMPTY_NAME.is_empty());
        assert!(!errors::trigger_condition::INVALID_TIMER.is_empty());
        assert!(!errors::trigger_state::INVALID_TRANSITION.is_empty());
    }
}