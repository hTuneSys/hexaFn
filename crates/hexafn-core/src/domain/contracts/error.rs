// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! Error handling contracts and structured error management for the hexaFn system.
//!
//! This module provides the foundational error handling abstractions that ensure
//! consistent error classification, logging, and observability across all hexaFn modules
//! throughout the 6F Lifecycle Flow: Feed → Filter → Format → Function → Forward → Feedback
//!
//! # Architecture
//!
//! The error system follows hexagonal architecture and Domain-Driven Design principles:
//! - `HexaError` trait defines the core error contract for all hexaFn components
//! - `HexaErrorKind` categorizes errors by their nature and origin for proper handling
//! - `HexaErrorSeverity` provides prioritization levels for monitoring and alerting
//!
//! # Error Code Hierarchy
//!
//! All errors follow a hierarchical pattern: `<module>.<category>.<subcategory>`
//! This ensures consistent identification and traceability across the entire system.
//!
//! ## Module Prefixes
//! - `core.*` - Core pipeline and fundamental operations
//! - `trigger.*` - Trigger evaluation and execution
//! - `function.*` - Function runtime and execution
//! - `store.*` - Key-value storage and persistence
//! - `cast.*` - Pub-sub messaging and delivery
//! - `watch.*` - Observability, logging, and tracing
//! - `bridge.*` - External integrations and webhooks
//!
//! # Integration with 6F Lifecycle
//!
//! Errors can occur at any stage of the 6F Lifecycle Flow:
//! - **Feed**: Input validation, source connectivity issues
//! - **Filter**: Condition evaluation, rule validation failures
//! - **Format**: Data transformation, schema validation errors
//! - **Function**: Runtime errors, execution timeouts
//! - **Forward**: Delivery failures, storage write errors
//! - **Feedback**: Logging failures, monitoring export errors
//!
//! # Examples
//!
//! ## Basic Error Implementation
//!
//! ```rust
//! use hexafn_core::{HexaError, HexaErrorKind, HexaErrorSeverity};
//! use std::fmt::{Debug, Display};
//!
//! #[derive(Debug)]
//! struct TriggerNotFoundError {
//!     trigger_id: String,
//! }
//!
//! impl Display for TriggerNotFoundError {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         write!(f, "Trigger '{}' not found in registry", self.trigger_id)
//!     }
//! }
//!
//! impl HexaError for TriggerNotFoundError {
//!     fn error_code(&self) -> &str {
//!         "trigger.registry.not_found"
//!     }
//!
//!     fn error_message(&self) -> &str {
//!         "The requested trigger could not be found in the trigger registry"
//!     }
//!
//!     fn error_kind(&self) -> HexaErrorKind {
//!         HexaErrorKind::NotFound
//!     }
//!
//!     fn error_severity(&self) -> HexaErrorSeverity {
//!         HexaErrorSeverity::Medium
//!     }
//! }
//! ```
//!
//! ## Error Classification and Handling
//!
//! ```rust
//! use hexafn_core::{HexaErrorKind, HexaErrorSeverity};
//!
//! // Classify errors for appropriate response strategies
//! fn handle_error_by_kind(kind: HexaErrorKind) -> &'static str {
//!     match kind {
//!         HexaErrorKind::Validation | HexaErrorKind::NotFound => {
//!             "User-fixable error - return 4xx status"
//!         }
//!         HexaErrorKind::Internal | HexaErrorKind::External => {
//!             "System error - return 5xx status"
//!         }
//!         HexaErrorKind::Timeout => {
//!             "Retry-able error - implement exponential backoff"
//!         }
//!         HexaErrorKind::Unknown => {
//!             "Unknown error - log for investigation"
//!         }
//!     }
//! }
//!
//! // Prioritize errors for monitoring and alerting
//! fn get_alert_urgency(severity: HexaErrorSeverity) -> &'static str {
//!     match severity {
//!         HexaErrorSeverity::Critical => "Immediate page - system down",
//!         HexaErrorSeverity::High => "Alert within 5 minutes",
//!         HexaErrorSeverity::Medium => "Alert within 1 hour",
//!         HexaErrorSeverity::Low => "Log only - review during maintenance",
//!     }
//! }
//! ```
//!
//! ## Structured Logging Integration
//!
//! ```rust
//! use hexafn_core::HexaError;
//!
//! fn log_error_with_context(error: &dyn HexaError, context: &str) {
//!     let log_entry = error.to_log_entry();
//!     println!("[{}] {}", context, log_entry);
//!     // Output: [TriggerExecution] [trigger.registry.not_found] [NotFound Medium] The requested trigger could not be found
//! }
//! ```

use std::fmt::{Debug, Display};

/// Represents the category of errors that can occur in the hexaFn system.
///
/// This enum categorizes errors by their nature and origin, allowing for
/// consistent error handling and appropriate response strategies across
/// all modules in the 6F Lifecycle Flow.
///
/// # Examples
///
/// ```
/// use hexafn_core::{HexaErrorKind};
///
/// let kind = HexaErrorKind::Validation;
/// println!("Error kind: {}", kind); // Prints: Error kind: Validation
///
/// // Check if error is user-fixable
/// match kind {
///     HexaErrorKind::Validation | HexaErrorKind::NotFound => {
///         println!("User can potentially fix this error");
///     }
///     HexaErrorKind::Internal | HexaErrorKind::External => {
///         println!("System-level error, user cannot fix");
///     }
///     _ => println!("Other error type"),
/// }
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HexaErrorKind {
    /// Resource or entity was not found
    ///
    /// Typically occurs in Feed or Forward phases when:
    /// - Trigger not found in registry
    /// - Function not available in runtime
    /// - Storage key does not exist
    NotFound,

    /// Input validation failed
    ///
    /// Commonly occurs in Filter and Format phases when:
    /// - Invalid event payload structure
    /// - Schema validation failure
    /// - Business rule violation
    Validation,

    /// Operation exceeded time limit
    ///
    /// Can occur in any phase, especially:
    /// - Function execution timeout
    /// - Network request timeout
    /// - Database operation timeout
    Timeout,

    /// Internal system error
    ///
    /// Represents errors within hexaFn components:
    /// - Memory allocation failure
    /// - Thread pool exhaustion
    /// - Internal state corruption
    Internal,

    /// External system error
    ///
    /// Errors from external dependencies:
    /// - Database connection failure
    /// - Third-party API error
    /// - Network infrastructure issues
    External,

    /// Unclassified or unexpected error
    ///
    /// Fallback for errors that don't fit other categories:
    /// - Parsing errors from unexpected formats
    /// - Runtime errors in dynamic contexts
    /// - Legacy error types during migration
    Unknown,
}

impl Display for HexaErrorKind {
    /// Formats the error kind for display purposes.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::HexaErrorKind;
    ///
    /// assert_eq!(format!("{}", HexaErrorKind::NotFound), "NotFound");
    /// assert_eq!(format!("{}", HexaErrorKind::Validation), "Validation");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HexaErrorKind::NotFound => write!(f, "NotFound"),
            HexaErrorKind::Validation => write!(f, "Validation"),
            HexaErrorKind::Timeout => write!(f, "Timeout"),
            HexaErrorKind::Internal => write!(f, "Internal"),
            HexaErrorKind::External => write!(f, "External"),
            HexaErrorKind::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Represents the severity level of errors in the hexaFn system.
///
/// Severity levels help determine appropriate response strategies,
/// logging levels, and alerting thresholds. Higher severity errors
/// require immediate attention and may trigger automated responses.
///
/// # Examples
///
/// ```
/// use hexafn_core::HexaErrorSeverity;
///
/// let severity = HexaErrorSeverity::Critical;
/// println!("Severity: {}", severity); // Prints: Severity: Critical
///
/// // Determine response strategy based on severity
/// match severity {
///     HexaErrorSeverity::Critical => {
///         println!("Immediate action required - system may be compromised");
///     }
///     HexaErrorSeverity::High => {
///         println!("Urgent attention needed - functionality affected");
///     }
///     HexaErrorSeverity::Medium => {
///         println!("Should be addressed soon - minor impact");
///     }
///     HexaErrorSeverity::Low => {
///         println!("Can be addressed during maintenance window");
///     }
/// }
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HexaErrorSeverity {
    /// System-threatening error requiring immediate attention
    ///
    /// Examples:
    /// - Complete system failure
    /// - Data corruption detected
    /// - Security breach
    /// - Core service unavailable
    Critical,

    /// Significant error affecting functionality
    ///
    /// Examples:
    /// - Pipeline execution failure
    /// - Database connection lost
    /// - Authentication service down
    /// - Memory exhaustion warning
    High,

    /// Moderate error with limited impact
    ///
    /// Examples:
    /// - Single trigger execution failed
    /// - Non-critical service degraded
    /// - Configuration warning
    /// - Performance threshold exceeded
    Medium,

    /// Minor error with minimal impact
    ///
    /// Examples:
    /// - Optional feature unavailable
    /// - Cosmetic validation warning
    /// - Debug information missing
    /// - Non-essential service slow
    Low,
}

impl Display for HexaErrorSeverity {
    /// Formats the error severity for display purposes.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::HexaErrorSeverity;
    ///
    /// assert_eq!(format!("{}", HexaErrorSeverity::Critical), "Critical");
    /// assert_eq!(format!("{}", HexaErrorSeverity::High), "High");
    /// assert_eq!(format!("{}", HexaErrorSeverity::Medium), "Medium");
    /// assert_eq!(format!("{}", HexaErrorSeverity::Low), "Low");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HexaErrorSeverity::Critical => write!(f, "Critical"),
            HexaErrorSeverity::High => write!(f, "High"),
            HexaErrorSeverity::Medium => write!(f, "Medium"),
            HexaErrorSeverity::Low => write!(f, "Low"),
        }
    }
}

/// Core error trait for the hexaFn system following Domain-Driven Design principles.
///
/// This trait defines the contract that all errors must implement across the entire
/// hexaFn ecosystem. It provides a consistent interface for error handling, logging,
/// and monitoring throughout the 6F Lifecycle Flow.
///
/// # Design Principles
///
/// - **Structured Error Information**: All errors must provide categorized metadata
/// - **Consistent Logging**: Standardized log format across all modules
/// - **Observability**: Integration with HexaWatch tracing system
/// - **Domain Boundaries**: Maintains error context across hexagonal architecture layers
///
/// # Error Code Format
///
/// Error codes follow the hierarchical pattern: `<module>.<category>.<subcategory>`
/// This format provides clear organization and traceability across the system.
///
/// # Examples
///
/// ## Basic Implementation
///
/// ```
/// use hexafn_core::{HexaError, HexaErrorKind, HexaErrorSeverity};
/// use std::fmt::{Debug, Display};
///
/// #[derive(Debug)]
/// struct TriggerNotFoundError {
///     trigger_id: String,
/// }
///
/// impl Display for TriggerNotFoundError {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "Trigger '{}' not found in registry", self.trigger_id)
///     }
/// }
///
/// impl HexaError for TriggerNotFoundError {
///     fn error_code(&self) -> &str {
///         "trigger.registry.not_found"
///     }
///
///     fn error_message(&self) -> &str {
///         "The requested trigger could not be found in the trigger registry"
///     }
///
///     fn error_kind(&self) -> HexaErrorKind {
///         HexaErrorKind::NotFound
///     }
///
///     fn error_severity(&self) -> HexaErrorSeverity {
///         HexaErrorSeverity::Medium
///     }
/// }
///
/// // Usage
/// let error = TriggerNotFoundError {
///     trigger_id: "user-login-trigger".to_string(),
/// };
///
/// println!("Error code: {}", error.error_code());
/// println!("Log entry: {}", error.to_log_entry());
/// ```
///
/// ## Error Code Examples by Module
///
/// ### Core Module
/// - `core.pipeline.initialization_failed`
/// - `core.event.parsing_error`
/// - `core.lifecycle.phase_transition_failed`
///
/// ### Trigger Module
/// - `trigger.registry.not_found`
/// - `trigger.evaluation.condition_failed`
/// - `trigger.execution.timeout`
///
/// ### Function Module  
/// - `function.runtime.wasm_error`
/// - `function.execution.timeout`
/// - `function.validation.schema_mismatch`
///
/// ### Store Module
/// - `store.persistence.write_failed`
/// - `store.validation.key_invalid`
/// - `store.backend.connection_lost`
///
/// ### Cast Module
/// - `cast.subscription.invalid_filter`
/// - `cast.delivery.retry_exhausted`
/// - `cast.topic.not_found`
///
/// ### Watch Module
/// - `watch.tracing.span_creation_failed`
/// - `watch.metrics.export_error`
/// - `watch.logging.format_error`
///
/// ### Bridge Module
/// - `bridge.webhook.payload_invalid`
/// - `bridge.integration.auth_failed`
/// - `bridge.normalization.format_unsupported`
///
/// ## Integration with Result Types
///
/// ```
/// use hexafn_core::{HexaError, HexaErrorKind, HexaErrorSeverity};
/// use std::fmt::{Debug, Display};
///
/// #[derive(Debug)]
/// struct ValidationError {
///     field: String,
///     message: String,
/// }
///
/// impl Display for ValidationError {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "Validation failed for field '{}': {}", self.field, self.message)
///     }
/// }
///
/// impl HexaError for ValidationError {
///     fn error_code(&self) -> &str { "core.validation.field_invalid" }
///     fn error_message(&self) -> &str { &self.message }
///     fn error_kind(&self) -> HexaErrorKind { HexaErrorKind::Validation }
///     fn error_severity(&self) -> HexaErrorSeverity { HexaErrorSeverity::High }
/// }
///
/// // Function returning structured error
/// fn validate_trigger_config(config: &str) -> Result<(), Box<dyn HexaError>> {
///     if config.is_empty() {
///         return Err(Box::new(ValidationError {
///             field: "config".to_string(),
///             message: "Configuration cannot be empty".to_string(),
///         }));
///     }
///     Ok(())
/// }
/// ```
pub trait HexaError: Debug + Display + Send + Sync + 'static {
    /// Returns a unique hierarchical error code for programmatic identification.
    ///
    /// Error codes should follow the pattern: `<module>.<category>.<subcategory>`
    /// and remain stable across versions for API compatibility.
    ///
    /// # Format Rules
    ///
    /// - **Module**: Lowercase module name (core, trigger, function, store, cast, watch, bridge)
    /// - **Category**: Functional area within the module (registry, execution, validation, etc.)
    /// - **Subcategory**: Specific error type (not_found, timeout, invalid, etc.)
    ///
    /// # Examples
    ///
    /// ```text
    /// "trigger.registry.not_found"
    /// "function.execution.timeout"
    /// "store.persistence.write_failed"
    /// "cast.subscription.invalid_filter"
    /// "watch.tracing.span_creation_failed"
    /// "bridge.webhook.payload_invalid"
    /// "core.pipeline.initialization_failed"
    /// ```
    fn error_code(&self) -> &str;

    /// Returns a human-readable error message for end users.
    ///
    /// Messages should be clear, actionable, and free of technical jargon.
    /// Avoid exposing internal implementation details or sensitive information.
    ///
    /// # Examples
    ///
    /// ```text
    /// "The requested trigger could not be found"
    /// "Function execution timed out after 30 seconds"
    /// "Invalid JSON format in event payload"
    /// ```
    fn error_message(&self) -> &str;

    /// Returns the category of this error for classification purposes.
    ///
    /// Used by error handling middleware to determine appropriate
    /// response strategies and HTTP status codes.
    fn error_kind(&self) -> HexaErrorKind;

    /// Returns the severity level of this error for prioritization.
    ///
    /// Used by logging and monitoring systems to determine alert
    /// thresholds and escalation procedures.
    fn error_severity(&self) -> HexaErrorSeverity;

    /// Generates a structured log entry for this error.
    ///
    /// This method provides a standardized format for logging errors
    /// across all hexaFn modules, ensuring consistent observability
    /// and easier debugging.
    ///
    /// # Format
    ///
    /// The log entry follows the pattern:
    /// `[ERROR_CODE] [KIND SEVERITY] MESSAGE`
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::{HexaError, HexaErrorKind, HexaErrorSeverity};
    /// use std::fmt::{Debug, Display};
    ///
    /// #[derive(Debug)]
    /// struct TestError;
    ///
    /// impl Display for TestError {
    ///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ///         write!(f, "Test error occurred")
    ///     }
    /// }
    ///
    /// impl HexaError for TestError {
    ///     fn error_code(&self) -> &str { "core.test.general_error" }
    ///     fn error_message(&self) -> &str { "A test error occurred" }
    ///     fn error_kind(&self) -> HexaErrorKind { HexaErrorKind::Internal }
    ///     fn error_severity(&self) -> HexaErrorSeverity { HexaErrorSeverity::Low }
    /// }
    ///
    /// let error = TestError;
    /// let log_entry = error.to_log_entry();
    /// assert_eq!(log_entry, "[core.test.general_error] [Internal Low] A test error occurred");
    /// ```
    fn to_log_entry(&self) -> String {
        format!(
            "[{}] [{} {}] {}",
            self.error_code(),
            self.error_kind(),
            self.error_severity(),
            self.error_message(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test helper struct for HexaError trait testing
    #[derive(Debug)]
    struct TestError {
        code: String,
        message: String,
        kind: HexaErrorKind,
        severity: HexaErrorSeverity,
    }

    impl Display for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl HexaError for TestError {
        fn error_code(&self) -> &str {
            &self.code
        }

        fn error_message(&self) -> &str {
            &self.message
        }

        fn error_kind(&self) -> HexaErrorKind {
            self.kind
        }

        fn error_severity(&self) -> HexaErrorSeverity {
            self.severity
        }
    }

    mod hexa_error_kind_tests {
        use super::*;

        #[test]
        fn test_error_kind_display() {
            assert_eq!(format!("{}", HexaErrorKind::NotFound), "NotFound");
            assert_eq!(format!("{}", HexaErrorKind::Validation), "Validation");
            assert_eq!(format!("{}", HexaErrorKind::Timeout), "Timeout");
            assert_eq!(format!("{}", HexaErrorKind::Internal), "Internal");
            assert_eq!(format!("{}", HexaErrorKind::External), "External");
            assert_eq!(format!("{}", HexaErrorKind::Unknown), "Unknown");
        }

        #[test]
        fn test_error_kind_debug() {
            assert_eq!(format!("{:?}", HexaErrorKind::NotFound), "NotFound");
            assert_eq!(format!("{:?}", HexaErrorKind::Validation), "Validation");
        }

        #[test]
        fn test_error_kind_clone() {
            let original = HexaErrorKind::Timeout;
            let cloned = original;
            assert_eq!(original, cloned);
        }

        #[test]
        fn test_error_kind_copy() {
            let original = HexaErrorKind::Internal;
            let copied = original;
            assert_eq!(original, copied);
        }

        #[test]
        fn test_error_kind_equality() {
            assert_eq!(HexaErrorKind::NotFound, HexaErrorKind::NotFound);
            assert_ne!(HexaErrorKind::NotFound, HexaErrorKind::Validation);
            assert_ne!(HexaErrorKind::Timeout, HexaErrorKind::External);
        }

        #[test]
        fn test_error_kind_all_variants() {
            // Ensure all variants can be created and are distinct
            let variants = [
                HexaErrorKind::NotFound,
                HexaErrorKind::Validation,
                HexaErrorKind::Timeout,
                HexaErrorKind::Internal,
                HexaErrorKind::External,
                HexaErrorKind::Unknown,
            ];

            // Check that all variants are different
            for (i, variant1) in variants.iter().enumerate() {
                for (j, variant2) in variants.iter().enumerate() {
                    if i != j {
                        assert_ne!(variant1, variant2);
                    }
                }
            }
        }
    }

    mod hexa_error_severity_tests {
        use super::*;

        #[test]
        fn test_error_severity_display() {
            assert_eq!(format!("{}", HexaErrorSeverity::Critical), "Critical");
            assert_eq!(format!("{}", HexaErrorSeverity::High), "High");
            assert_eq!(format!("{}", HexaErrorSeverity::Medium), "Medium");
            assert_eq!(format!("{}", HexaErrorSeverity::Low), "Low");
        }

        #[test]
        fn test_error_severity_debug() {
            assert_eq!(format!("{:?}", HexaErrorSeverity::Critical), "Critical");
            assert_eq!(format!("{:?}", HexaErrorSeverity::High), "High");
        }

        #[test]
        fn test_error_severity_clone() {
            let original = HexaErrorSeverity::Medium;
            let cloned = original;
            assert_eq!(original, cloned);
        }

        #[test]
        fn test_error_severity_copy() {
            let original = HexaErrorSeverity::Low;
            let copied = original;
            assert_eq!(original, copied);
        }

        #[test]
        fn test_error_severity_equality() {
            assert_eq!(HexaErrorSeverity::Critical, HexaErrorSeverity::Critical);
            assert_ne!(HexaErrorSeverity::Critical, HexaErrorSeverity::High);
            assert_ne!(HexaErrorSeverity::Medium, HexaErrorSeverity::Low);
        }

        #[test]
        fn test_error_severity_all_variants() {
            // Ensure all variants can be created and are distinct
            let variants = [
                HexaErrorSeverity::Critical,
                HexaErrorSeverity::High,
                HexaErrorSeverity::Medium,
                HexaErrorSeverity::Low,
            ];

            // Check that all variants are different
            for (i, variant1) in variants.iter().enumerate() {
                for (j, variant2) in variants.iter().enumerate() {
                    if i != j {
                        assert_ne!(variant1, variant2);
                    }
                }
            }
        }
    }

    mod hexa_error_trait_tests {
        use super::*;

        #[test]
        fn test_error_trait_implementation() {
            let error = TestError {
                code: "core.test.sample_error".to_string(),
                message: "Test error message".to_string(),
                kind: HexaErrorKind::Validation,
                severity: HexaErrorSeverity::High,
            };

            assert_eq!(error.error_code(), "core.test.sample_error");
            assert_eq!(error.error_message(), "Test error message");
            assert_eq!(error.error_kind(), HexaErrorKind::Validation);
            assert_eq!(error.error_severity(), HexaErrorSeverity::High);
        }

        #[test]
        fn test_to_log_entry_format() {
            let error = TestError {
                code: "trigger.execution.failed".to_string(),
                message: "Trigger execution failed".to_string(),
                kind: HexaErrorKind::Internal,
                severity: HexaErrorSeverity::Critical,
            };

            let log_entry = error.to_log_entry();
            assert_eq!(
                log_entry,
                "[trigger.execution.failed] [Internal Critical] Trigger execution failed"
            );
        }

        #[test]
        fn test_to_log_entry_with_different_severities() {
            let test_cases = [
                (HexaErrorSeverity::Critical, "Critical"),
                (HexaErrorSeverity::High, "High"),
                (HexaErrorSeverity::Medium, "Medium"),
                (HexaErrorSeverity::Low, "Low"),
            ];

            for (severity, expected_severity_str) in test_cases {
                let error = TestError {
                    code: "core.test.severity_test".to_string(),
                    message: "Test message".to_string(),
                    kind: HexaErrorKind::Unknown,
                    severity,
                };

                let log_entry = error.to_log_entry();
                assert!(log_entry.contains(expected_severity_str));
                assert!(log_entry.contains("core.test.severity_test"));
                assert!(log_entry.contains("Test message"));
            }
        }

        #[test]
        fn test_to_log_entry_with_different_kinds() {
            let test_cases = [
                (HexaErrorKind::NotFound, "NotFound"),
                (HexaErrorKind::Validation, "Validation"),
                (HexaErrorKind::Timeout, "Timeout"),
                (HexaErrorKind::Internal, "Internal"),
                (HexaErrorKind::External, "External"),
                (HexaErrorKind::Unknown, "Unknown"),
            ];

            for (kind, expected_kind_str) in test_cases {
                let error = TestError {
                    code: "core.test.kind_test".to_string(),
                    message: "Test message".to_string(),
                    kind,
                    severity: HexaErrorSeverity::Medium,
                };

                let log_entry = error.to_log_entry();
                assert!(log_entry.contains(expected_kind_str));
                assert!(log_entry.contains("core.test.kind_test"));
                assert!(log_entry.contains("Test message"));
            }
        }

        #[test]
        fn test_error_trait_object_compatibility() {
            let error: Box<dyn HexaError> = Box::new(TestError {
                code: "core.test.boxed_error".to_string(),
                message: "Boxed error message".to_string(),
                kind: HexaErrorKind::External,
                severity: HexaErrorSeverity::Low,
            });

            assert_eq!(error.error_code(), "core.test.boxed_error");
            assert_eq!(error.error_message(), "Boxed error message");
            assert_eq!(error.error_kind(), HexaErrorKind::External);
            assert_eq!(error.error_severity(), HexaErrorSeverity::Low);

            let log_entry = error.to_log_entry();
            assert_eq!(
                log_entry,
                "[core.test.boxed_error] [External Low] Boxed error message"
            );
        }

        #[test]
        fn test_error_send_sync_static() {
            // This test ensures the trait object is Send + Sync + 'static
            fn assert_send_sync_static<T: Send + Sync + 'static>(_: T) {}

            let error = TestError {
                code: "core.test.thread_safe".to_string(),
                message: "Thread safe error".to_string(),
                kind: HexaErrorKind::Internal,
                severity: HexaErrorSeverity::Medium,
            };

            assert_send_sync_static(error);
        }

        #[test]
        fn test_error_debug_display() {
            let error = TestError {
                code: "core.test.debug_test".to_string(),
                message: "Debug test message".to_string(),
                kind: HexaErrorKind::Validation,
                severity: HexaErrorSeverity::High,
            };

            // Test Debug implementation
            let debug_output = format!("{:?}", error);
            assert!(debug_output.contains("TestError"));
            assert!(debug_output.contains("core.test.debug_test"));

            // Test Display implementation
            let display_output = format!("{}", error);
            assert_eq!(display_output, "Debug test message");
        }

        #[test]
        fn test_hierarchical_error_code_format() {
            let test_cases = [
                // Core module errors
                (
                    "core.pipeline.initialization_failed",
                    "Core pipeline initialization",
                ),
                ("core.event.parsing_error", "Core event parsing"),
                (
                    "core.lifecycle.phase_transition_failed",
                    "Core lifecycle phase transition",
                ),
                // Trigger module errors
                ("trigger.registry.not_found", "Trigger registry lookup"),
                (
                    "trigger.evaluation.condition_failed",
                    "Trigger condition evaluation",
                ),
                ("trigger.execution.timeout", "Trigger execution timeout"),
                // Function module errors
                ("function.runtime.wasm_error", "Function WASM runtime"),
                ("function.execution.timeout", "Function execution timeout"),
                (
                    "function.validation.schema_mismatch",
                    "Function schema validation",
                ),
                // Store module errors
                ("store.persistence.write_failed", "Store persistence write"),
                ("store.validation.key_invalid", "Store key validation"),
                ("store.backend.connection_lost", "Store backend connection"),
                // Cast module errors
                (
                    "cast.subscription.invalid_filter",
                    "Cast subscription filter",
                ),
                ("cast.delivery.retry_exhausted", "Cast delivery retry"),
                ("cast.topic.not_found", "Cast topic lookup"),
                // Watch module errors
                (
                    "watch.tracing.span_creation_failed",
                    "Watch tracing span creation",
                ),
                ("watch.metrics.export_error", "Watch metrics export"),
                ("watch.logging.format_error", "Watch logging format"),
                // Bridge module errors
                ("bridge.webhook.payload_invalid", "Bridge webhook payload"),
                ("bridge.integration.auth_failed", "Bridge integration auth"),
                (
                    "bridge.normalization.format_unsupported",
                    "Bridge normalization format",
                ),
            ];

            for (error_code, description) in test_cases {
                let error = TestError {
                    code: error_code.to_string(),
                    message: description.to_string(),
                    kind: HexaErrorKind::Unknown,
                    severity: HexaErrorSeverity::Medium,
                };

                // Verify the error code follows the hierarchical format
                let parts: Vec<&str> = error.error_code().split('.').collect();
                assert_eq!(
                    parts.len(),
                    3,
                    "Error code {} should have exactly 3 parts",
                    error_code
                );

                // Verify module part is valid
                let valid_modules = [
                    "core", "trigger", "function", "store", "cast", "watch", "bridge",
                ];
                assert!(
                    valid_modules.contains(&parts[0]),
                    "Invalid module: {}",
                    parts[0]
                );

                // Verify format consistency
                assert!(!parts[1].is_empty(), "Category should not be empty");
                assert!(!parts[2].is_empty(), "Subcategory should not be empty");

                // Verify log entry includes the hierarchical code
                let log_entry = error.to_log_entry();
                assert!(log_entry.contains(error_code));
            }
        }

        #[test]
        fn test_real_world_error_scenarios_with_hierarchical_codes() {
            // Scenario 1: Trigger not found
            let trigger_error = TestError {
                code: "trigger.registry.not_found".to_string(),
                message: "Trigger 'user-login' not found in registry".to_string(),
                kind: HexaErrorKind::NotFound,
                severity: HexaErrorSeverity::Medium,
            };

            assert_eq!(
                trigger_error.to_log_entry(),
                "[trigger.registry.not_found] [NotFound Medium] Trigger 'user-login' not found in registry"
            );

            // Scenario 2: Function execution timeout
            let timeout_error = TestError {
                code: "function.execution.timeout".to_string(),
                message: "Function execution exceeded 30 second limit".to_string(),
                kind: HexaErrorKind::Timeout,
                severity: HexaErrorSeverity::High,
            };

            assert_eq!(
                timeout_error.to_log_entry(),
                "[function.execution.timeout] [Timeout High] Function execution exceeded 30 second limit"
            );

            // Scenario 3: Validation failure
            let validation_error = TestError {
                code: "core.validation.schema_mismatch".to_string(),
                message: "Event payload does not match expected schema".to_string(),
                kind: HexaErrorKind::Validation,
                severity: HexaErrorSeverity::High,
            };

            assert_eq!(
                validation_error.to_log_entry(),
                "[core.validation.schema_mismatch] [Validation High] Event payload does not match expected schema"
            );

            // Scenario 4: Critical system failure
            let critical_error = TestError {
                code: "core.pipeline.system_failure".to_string(),
                message: "Core pipeline engine has stopped responding".to_string(),
                kind: HexaErrorKind::Internal,
                severity: HexaErrorSeverity::Critical,
            };

            assert_eq!(
                critical_error.to_log_entry(),
                "[core.pipeline.system_failure] [Internal Critical] Core pipeline engine has stopped responding"
            );

            // Scenario 5: Store write failure
            let store_error = TestError {
                code: "store.persistence.write_failed".to_string(),
                message: "Failed to write data to persistent storage".to_string(),
                kind: HexaErrorKind::External,
                severity: HexaErrorSeverity::High,
            };

            assert_eq!(
                store_error.to_log_entry(),
                "[store.persistence.write_failed] [External High] Failed to write data to persistent storage"
            );

            // Scenario 6: Cast delivery failure
            let cast_error = TestError {
                code: "cast.delivery.retry_exhausted".to_string(),
                message: "Message delivery failed after maximum retry attempts".to_string(),
                kind: HexaErrorKind::External,
                severity: HexaErrorSeverity::Medium,
            };

            assert_eq!(
                cast_error.to_log_entry(),
                "[cast.delivery.retry_exhausted] [External Medium] Message delivery failed after maximum retry attempts"
            );
        }

        #[test]
        fn test_default_trait_method_log_entry() {
            let error: Box<dyn HexaError> = Box::new(TestError {
                code: "test.default.log".into(),
                message: "From default trait".into(),
                kind: HexaErrorKind::Unknown,
                severity: HexaErrorSeverity::Low,
            });

            let log = error.to_log_entry();
            assert!(log.contains("test.default.log"));
        }
    }

    mod integration_tests {
        use super::*;

        #[test]
        fn test_error_classification_by_kind() {
            let errors = [
                (HexaErrorKind::NotFound, "Resource errors"),
                (HexaErrorKind::Validation, "Input errors"),
                (HexaErrorKind::Timeout, "Performance errors"),
                (HexaErrorKind::Internal, "System errors"),
                (HexaErrorKind::External, "Dependency errors"),
                (HexaErrorKind::Unknown, "Unclassified errors"),
            ];

            for (kind, category) in errors {
                let error = TestError {
                    code: "core.test.classification".to_string(),
                    message: category.to_string(),
                    kind,
                    severity: HexaErrorSeverity::Medium,
                };

                // Verify error can be classified correctly
                match error.error_kind() {
                    HexaErrorKind::NotFound => assert!(category.contains("Resource")),
                    HexaErrorKind::Validation => assert!(category.contains("Input")),
                    HexaErrorKind::Timeout => assert!(category.contains("Performance")),
                    HexaErrorKind::Internal => assert!(category.contains("System")),
                    HexaErrorKind::External => assert!(category.contains("Dependency")),
                    HexaErrorKind::Unknown => assert!(category.contains("Unclassified")),
                }
            }
        }

        #[test]
        fn test_error_prioritization_by_severity() {
            let mut errors = vec![
                TestError {
                    code: "core.test.low_priority".to_string(),
                    message: "Low priority message".to_string(),
                    kind: HexaErrorKind::Unknown,
                    severity: HexaErrorSeverity::Low,
                },
                TestError {
                    code: "core.test.critical_issue".to_string(),
                    message: "Critical issue message".to_string(),
                    kind: HexaErrorKind::Internal,
                    severity: HexaErrorSeverity::Critical,
                },
                TestError {
                    code: "core.test.medium_concern".to_string(),
                    message: "Medium concern message".to_string(),
                    kind: HexaErrorKind::Validation,
                    severity: HexaErrorSeverity::Medium,
                },
                TestError {
                    code: "core.test.high_priority".to_string(),
                    message: "High priority message".to_string(),
                    kind: HexaErrorKind::Timeout,
                    severity: HexaErrorSeverity::High,
                },
            ];

            // Sort by severity (this would be done by a monitoring system)
            errors.sort_by(|a, b| {
                use HexaErrorSeverity::*;
                let order = |s: &HexaErrorSeverity| match s {
                    Critical => 0,
                    High => 1,
                    Medium => 2,
                    Low => 3,
                };
                order(&a.severity).cmp(&order(&b.severity))
            });

            // Verify sorting order
            assert_eq!(errors[0].error_severity(), HexaErrorSeverity::Critical);
            assert_eq!(errors[1].error_severity(), HexaErrorSeverity::High);
            assert_eq!(errors[2].error_severity(), HexaErrorSeverity::Medium);
            assert_eq!(errors[3].error_severity(), HexaErrorSeverity::Low);
        }

        #[test]
        fn test_hierarchical_error_code_parsing() {
            let error_codes = [
                "core.pipeline.initialization_failed",
                "trigger.registry.not_found",
                "function.runtime.wasm_error",
                "store.persistence.write_failed",
                "cast.subscription.invalid_filter",
                "watch.tracing.span_creation_failed",
                "bridge.webhook.payload_invalid",
            ];

            for error_code in error_codes {
                let parts: Vec<&str> = error_code.split('.').collect();
                assert_eq!(parts.len(), 3, "Error code should have exactly 3 parts");

                let module = parts[0];
                let category = parts[1];
                let subcategory = parts[2];

                // Verify module is valid
                let valid_modules = [
                    "core", "trigger", "function", "store", "cast", "watch", "bridge",
                ];
                assert!(
                    valid_modules.contains(&module),
                    "Invalid module: {}",
                    module
                );

                // Verify category and subcategory are not empty
                assert!(!category.is_empty(), "Category should not be empty");
                assert!(!subcategory.is_empty(), "Subcategory should not be empty");

                // Verify parts follow naming conventions (lowercase with underscores)
                assert!(
                    category.chars().all(|c| c.is_lowercase() || c == '_'),
                    "Category should be lowercase with underscores: {}",
                    category
                );
                assert!(
                    subcategory.chars().all(|c| c.is_lowercase() || c == '_'),
                    "Subcategory should be lowercase with underscores: {}",
                    subcategory
                );
            }
        }
    }
}
