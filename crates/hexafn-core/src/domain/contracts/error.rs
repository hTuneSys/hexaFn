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
//! - `bridge.*` - External integrations and webhooks
//! - `trigger.*` - Trigger evaluation and execution
//! - `run.*` - Run runtime and execution
//! - `store.*` - Key-value storage and persistence
//! - `cast.*` - Pub-sub messaging and delivery
//! - `watch.*` - Observability, logging, and tracing
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

/// Error kind/category for all hexaFn errors.
///
/// Categorizes errors by their nature and origin for consistent handling.
///
/// # Example
/// ```
/// use hexafn_core::HexaErrorKind;
/// let kind = HexaErrorKind::Validation;
/// assert_eq!(kind.to_string(), "Validation");
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HexaErrorKind {
    /// Resource or entity was not found (e.g. missing trigger, function, or key)
    NotFound,
    /// Input validation failed (e.g. schema, business rule)
    Validation,
    /// Operation exceeded time limit (timeout)
    Timeout,
    /// Internal system error (e.g. state corruption, panic)
    Internal,
    /// External system error (e.g. DB, network, 3rd party)
    External,
    /// Unclassified or unexpected error
    Unknown,
}

impl std::fmt::Display for HexaErrorKind {
    /// Display as string (e.g. "Validation")
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HexaErrorKind::NotFound => "NotFound",
                HexaErrorKind::Validation => "Validation",
                HexaErrorKind::Timeout => "Timeout",
                HexaErrorKind::Internal => "Internal",
                HexaErrorKind::External => "External",
                HexaErrorKind::Unknown => "Unknown",
            }
        )
    }
}

/// Severity level for hexaFn errors.
///
/// Used for alerting, logging, and prioritization.
///
/// # Example
/// ```
/// use hexafn_core::HexaErrorSeverity;
/// let sev = HexaErrorSeverity::Critical;
/// assert_eq!(sev.to_string(), "Critical");
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HexaErrorSeverity {
    /// Minor error, minimal impact (e.g. cosmetic, debug)
    Low,
    /// Moderate error, limited impact (e.g. single trigger fail)
    Medium,
    /// Significant error, affects functionality (e.g. pipeline fail)
    High,
    /// System-threatening, immediate attention (e.g. data loss)
    Critical,
}

impl std::fmt::Display for HexaErrorSeverity {
    /// Display as string (e.g. "High")
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HexaErrorSeverity::Low => "Low",
                HexaErrorSeverity::Medium => "Medium",
                HexaErrorSeverity::High => "High",
                HexaErrorSeverity::Critical => "Critical",
            }
        )
    }
}

/// Main trait for all structured errors in hexaFn.
///
/// Provides error code, message, kind, severity, source, and log formatting.
///
/// # Example
/// ```
/// use hexafn_core::{HexaError, HexaErrorKind, HexaErrorSeverity};
/// use std::fmt::{Debug, Display};
/// #[derive(Debug)]
/// struct MyError;
/// impl Display for MyError {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "My error!")
///     }
/// }
/// impl HexaError for MyError {
///     fn error_code(&self) -> &str { "core.example" }
///     fn error_message(&self) -> &str { "Example error" }
///     fn error_kind(&self) -> HexaErrorKind { HexaErrorKind::Internal }
///     fn error_severity(&self) -> HexaErrorSeverity { HexaErrorSeverity::Low }
/// }
/// let err = MyError;
/// assert_eq!(err.error_code(), "core.example");
/// ```
pub trait HexaError: Debug + Display + Send + Sync + 'static {
    /// Unique error code for this error (e.g. "trigger.registry.not_found").
    ///
    /// # Example
    /// ```
    /// use hexafn_core::{HexaError, HexaErrorKind, HexaErrorSeverity};
    /// #[derive(Debug)]
    /// struct MyError;
    /// impl std::fmt::Display for MyError {
    ///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "err") }
    /// }
    /// impl HexaError for MyError {
    ///     fn error_code(&self) -> &str { "core.example.code" }
    ///     fn error_message(&self) -> &str { "msg" }
    ///     fn error_kind(&self) -> HexaErrorKind { HexaErrorKind::Internal }
    ///     fn error_severity(&self) -> HexaErrorSeverity { HexaErrorSeverity::Low }
    /// }
    /// let err = MyError;
    /// assert_eq!(err.error_code(), "core.example.code");
    /// ```
    fn error_code(&self) -> &str;

    /// Human-readable error message for this error.
    ///
    /// # Example
    /// ```
    /// use hexafn_core::{HexaError, HexaErrorKind, HexaErrorSeverity};
    /// #[derive(Debug)]
    /// struct MyError;
    /// impl std::fmt::Display for MyError {
    ///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "err") }
    /// }
    /// impl HexaError for MyError {
    ///     fn error_code(&self) -> &str { "core.example.code" }
    ///     fn error_message(&self) -> &str { "A user-friendly message" }
    ///     fn error_kind(&self) -> HexaErrorKind { HexaErrorKind::Internal }
    ///     fn error_severity(&self) -> HexaErrorSeverity { HexaErrorSeverity::Low }
    /// }
    /// let err = MyError;
    /// assert_eq!(err.error_message(), "A user-friendly message");
    /// ```
    fn error_message(&self) -> &str;

    /// Error kind/category for this error (see [`HexaErrorKind`]).
    ///
    /// # Example
    /// ```
    /// use hexafn_core::{HexaError, HexaErrorKind, HexaErrorSeverity};
    /// #[derive(Debug)]
    /// struct MyError;
    /// impl std::fmt::Display for MyError {
    ///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "err") }
    /// }
    /// impl HexaError for MyError {
    ///     fn error_code(&self) -> &str { "core.example.code" }
    ///     fn error_message(&self) -> &str { "msg" }
    ///     fn error_kind(&self) -> HexaErrorKind { HexaErrorKind::Validation }
    ///     fn error_severity(&self) -> HexaErrorSeverity { HexaErrorSeverity::Low }
    /// }
    /// let err = MyError;
    /// assert_eq!(err.error_kind(), HexaErrorKind::Validation);
    /// ```
    fn error_kind(&self) -> HexaErrorKind;

    /// Error severity/priority for this error (see [`HexaErrorSeverity`]).
    ///
    /// # Example
    /// ```
    /// use hexafn_core::{HexaError, HexaErrorKind, HexaErrorSeverity};
    /// #[derive(Debug)]
    /// struct MyError;
    /// impl std::fmt::Display for MyError {
    ///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "err") }
    /// }
    /// impl HexaError for MyError {
    ///     fn error_code(&self) -> &str { "core.example.code" }
    ///     fn error_message(&self) -> &str { "msg" }
    ///     fn error_kind(&self) -> HexaErrorKind { HexaErrorKind::Unknown }
    ///     fn error_severity(&self) -> HexaErrorSeverity { HexaErrorSeverity::High }
    /// }
    /// let err = MyError;
    /// assert_eq!(err.error_severity(), HexaErrorSeverity::High);
    /// ```
    fn error_severity(&self) -> HexaErrorSeverity;

    /// Structured log entry for this error (e.g. "[code] [Kind Severity] Message").
    ///
    /// # Example
    /// ```
    /// use hexafn_core::{HexaError, HexaErrorKind, HexaErrorSeverity};
    /// #[derive(Debug)]
    /// struct MyError;
    /// impl std::fmt::Display for MyError {
    ///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "err") }
    /// }
    /// impl HexaError for MyError {
    ///     fn error_code(&self) -> &str { "core.example.code" }
    ///     fn error_message(&self) -> &str { "msg" }
    ///     fn error_kind(&self) -> HexaErrorKind { HexaErrorKind::Unknown }
    ///     fn error_severity(&self) -> HexaErrorSeverity { HexaErrorSeverity::Medium }
    /// }
    /// let err = MyError;
    /// let log = err.to_log_entry();
    /// assert!(log.contains("core.example.code"));
    /// assert!(log.contains("Unknown"));
    /// assert!(log.contains("Medium"));
    /// assert!(log.contains("msg"));
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

    /// Underlying source error for chaining (if any).
    ///
    /// # Example
    /// ```
    /// use hexafn_core::{HexaError, HexaErrorKind, HexaErrorSeverity};
    /// use std::fmt::{Debug, Display};
    /// use std::error::Error;
    /// #[derive(Debug)]
    /// struct SrcErr;
    /// impl Display for SrcErr { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "src") } }
    /// impl Error for SrcErr {}
    /// #[derive(Debug)]
    /// struct MyErr { src: Option<Box<dyn Error + Send + Sync + 'static>> }
    /// impl Display for MyErr { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "myerr") } }
    /// impl HexaError for MyErr {
    ///     fn error_code(&self) -> &str { "core.with_source" }
    ///     fn error_message(&self) -> &str { "Has source" }
    ///     fn error_kind(&self) -> HexaErrorKind { HexaErrorKind::Internal }
    ///     fn error_severity(&self) -> HexaErrorSeverity { HexaErrorSeverity::Medium }
    ///     fn source(&self) -> Option<&(dyn Error + Send + Sync + 'static)> { self.src.as_deref() }
    /// }
    /// let src = SrcErr;
    /// let err = MyErr { src: Some(Box::new(src)) };
    /// assert!(HexaError::source(&err).is_some());
    /// ```
    fn source(&self) -> Option<&(dyn std::error::Error + Send + Sync + 'static)> {
        None
    }
}

/// A reusable, structured error type for all hexaFn modules.
///
/// `HexaCoreError` implements the [`HexaError`] trait and can be used as a standard error type
/// across all modules. It supports error code, message, kind, severity, and optional source error.
///
/// # Example
/// ```rust
/// use hexafn_core::{HexaCoreError, HexaError, HexaErrorKind, HexaErrorSeverity};
/// let err = HexaCoreError::new("core.example.invalid")
///     .with_message("Invalid operation")
///     .with_kind(HexaErrorKind::Validation)
///     .with_severity(HexaErrorSeverity::Medium);
/// assert_eq!(err.error_code(), "core.example.invalid");
/// assert_eq!(err.error_kind(), HexaErrorKind::Validation);
/// assert_eq!(err.error_severity(), HexaErrorSeverity::Medium);
/// assert_eq!(err.error_message(), "Invalid operation");
/// ```
#[derive(Debug)]
pub struct HexaCoreError {
    code: &'static str,
    message: String,
    kind: HexaErrorKind,
    severity: HexaErrorSeverity,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl HexaCoreError {
    /// Create a new `HexaCoreError` with a code. Use builder methods to set message, kind, severity, and source.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_core::{HexaError,HexaCoreError, HexaErrorKind, HexaErrorSeverity};
    /// let err = HexaCoreError::new("core.test.invalid")
    ///     .with_message("Test error")
    ///     .with_kind(HexaErrorKind::Validation)
    ///     .with_severity(HexaErrorSeverity::Low);
    /// assert_eq!(err.error_code(), "core.test.invalid");
    /// assert_eq!(err.error_message(), "Test error");
    /// assert_eq!(err.error_kind(), HexaErrorKind::Validation);
    /// assert_eq!(err.error_severity(), HexaErrorSeverity::Low);
    /// ```
    pub fn new(code: &'static str) -> Self {
        Self {
            code,
            message: String::new(),
            kind: HexaErrorKind::Unknown,
            severity: HexaErrorSeverity::Low,
            source: None,
        }
    }
    /// Set the error message (builder pattern).
    ///
    /// # Example
    /// ```rust
    /// use hexafn_core::{HexaCoreError,HexaError, HexaErrorKind, HexaErrorSeverity};
    /// let err = HexaCoreError::new("core.example.msg").with_message("A message");
    /// assert_eq!(err.error_message(), "A message");
    /// ```
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }
    /// Set the error kind (builder pattern).
    ///
    /// # Example
    /// ```rust
    /// use hexafn_core::{HexaCoreError,HexaError, HexaErrorKind, HexaErrorSeverity};
    /// let err = HexaCoreError::new("core.example.kind").with_kind(HexaErrorKind::Timeout);
    /// assert_eq!(err.error_kind(), HexaErrorKind::Timeout);
    /// ```
    pub fn with_kind(mut self, kind: HexaErrorKind) -> Self {
        self.kind = kind;
        self
    }
    /// Set the error severity (builder pattern).
    ///
    /// # Example
    /// ```rust
    /// use hexafn_core::{HexaCoreError,HexaError, HexaErrorKind, HexaErrorSeverity};
    /// let err = HexaCoreError::new("core.example.sev").with_severity(HexaErrorSeverity::Critical);
    /// assert_eq!(err.error_severity(), HexaErrorSeverity::Critical);
    /// ```
    pub fn with_severity(mut self, severity: HexaErrorSeverity) -> Self {
        self.severity = severity;
        self
    }
    /// Set the source error (builder pattern).
    ///
    /// # Example
    /// ```rust
    /// use hexafn_core::{HexaCoreError,HexaError, HexaErrorKind, HexaErrorSeverity};
    /// use std::fmt::{Display, Formatter, Result as FmtResult};
    /// #[derive(Debug)]
    /// struct DummySrc;
    /// impl Display for DummySrc {
    ///     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "DummySrc") }
    /// }
    /// impl std::error::Error for DummySrc {}
    /// let src = DummySrc;
    /// let err = HexaCoreError::new("core.example.src")
    ///     .with_source(Box::new(src));
    /// assert!(HexaError::source(&err).is_some());
    /// assert_eq!(format!("{}", HexaError::source(&err).unwrap()), "DummySrc");
    /// ```
    pub fn with_source(
        mut self,
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    ) -> Self {
        self.source = Some(source);
        self
    }
}

impl std::error::Error for HexaCoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Downcast Option<&(dyn Error + Send + Sync + 'static)> to Option<&(dyn Error + 'static)>
        self.source
            .as_ref()
            .map(|e| &**e as &(dyn std::error::Error + 'static))
    }
}

impl HexaError for HexaCoreError {
    fn error_code(&self) -> &str {
        self.code
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
    fn source(&self) -> Option<&(dyn std::error::Error + Send + Sync + 'static)> {
        self.source.as_deref()
    }
}

impl std::fmt::Display for HexaCoreError {
    /// Display the error as a string.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_core::{HexaCoreError, HexaErrorKind, HexaErrorSeverity};
    /// let err = HexaCoreError::new("core.display.test")
    ///     .with_message("Display test")
    ///     .with_kind(HexaErrorKind::Internal)
    ///     .with_severity(HexaErrorSeverity::High);
    /// assert!(format!("{}", err).contains("Display test"));
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] [{} {}] {}",
            self.code, self.kind, self.severity, self.message
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fmt::Display;

    #[derive(Debug)]
    struct DummyError;
    impl Display for DummyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Dummy error!")
        }
    }
    impl Error for DummyError {}

    #[derive(Debug)]
    struct ErrorWithSource {
        source: Option<Box<dyn Error + Send + Sync + 'static>>,
    }
    impl Display for ErrorWithSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ErrorWithSource occurred")
        }
    }
    impl HexaError for ErrorWithSource {
        fn error_code(&self) -> &str {
            "core.test.with_source"
        }
        fn error_message(&self) -> &str {
            "Error with a source"
        }
        fn error_kind(&self) -> HexaErrorKind {
            HexaErrorKind::Internal
        }
        fn error_severity(&self) -> HexaErrorSeverity {
            HexaErrorSeverity::High
        }
        fn source(&self) -> Option<&(dyn Error + Send + Sync + 'static)> {
            self.source.as_deref()
        }
    }

    #[test]
    fn test_kind_display_and_debug() {
        assert_eq!(format!("{}", HexaErrorKind::Validation), "Validation");
        assert_eq!(format!("{:?}", HexaErrorKind::Timeout), "Timeout");
    }
    #[test]
    fn test_severity_display_and_debug() {
        assert_eq!(format!("{}", HexaErrorSeverity::Critical), "Critical");
        assert_eq!(format!("{:?}", HexaErrorSeverity::Low), "Low");
    }
    #[test]
    fn test_trait_basic() {
        #[derive(Debug)]
        struct E;
        impl Display for E {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "E")
            }
        }
        impl HexaError for E {
            fn error_code(&self) -> &str {
                "core.test"
            }
            fn error_message(&self) -> &str {
                "msg"
            }
            fn error_kind(&self) -> HexaErrorKind {
                HexaErrorKind::Unknown
            }
            fn error_severity(&self) -> HexaErrorSeverity {
                HexaErrorSeverity::Low
            }
        }
        let e = E;
        assert_eq!(e.error_code(), "core.test");
        assert_eq!(e.error_message(), "msg");
        assert_eq!(e.error_kind(), HexaErrorKind::Unknown);
        assert_eq!(e.error_severity(), HexaErrorSeverity::Low);
        assert!(HexaError::source(&e).is_none());
        assert!(e.to_log_entry().contains("core.test"));
    }
    #[test]
    fn test_trait_with_source() {
        let src = DummyError;
        let err = ErrorWithSource {
            source: Some(Box::new(src)),
        };
        let trait_obj: &dyn HexaError = &err;
        assert!(trait_obj.source().is_some());
        assert_eq!(format!("{}", trait_obj.source().unwrap()), "Dummy error!");
    }
    #[test]
    fn test_to_log_entry_format() {
        #[derive(Debug)]
        struct E;
        impl Display for E {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "E")
            }
        }
        impl HexaError for E {
            fn error_code(&self) -> &str {
                "core.test.log"
            }
            fn error_message(&self) -> &str {
                "Log msg"
            }
            fn error_kind(&self) -> HexaErrorKind {
                HexaErrorKind::Validation
            }
            fn error_severity(&self) -> HexaErrorSeverity {
                HexaErrorSeverity::Medium
            }
        }
        let e = E;
        let log = e.to_log_entry();
        assert!(log.contains("core.test.log"));
        assert!(log.contains("Validation"));
        assert!(log.contains("Medium"));
        assert!(log.contains("Log msg"));
    }
    #[test]
    fn test_enum_variants_unique() {
        let kinds = [
            HexaErrorKind::NotFound,
            HexaErrorKind::Validation,
            HexaErrorKind::Timeout,
            HexaErrorKind::Internal,
            HexaErrorKind::External,
            HexaErrorKind::Unknown,
        ];
        for (i, a) in kinds.iter().enumerate() {
            for (j, b) in kinds.iter().enumerate() {
                if i != j {
                    assert_ne!(a, b);
                }
            }
        }
        let sevs = [
            HexaErrorSeverity::Low,
            HexaErrorSeverity::Medium,
            HexaErrorSeverity::High,
            HexaErrorSeverity::Critical,
        ];
        for (i, a) in sevs.iter().enumerate() {
            for (j, b) in sevs.iter().enumerate() {
                if i != j {
                    assert_ne!(a, b);
                }
            }
        }
    }
    #[test]
    fn test_sort_by_severity() {
        let mut v = vec![
            HexaErrorSeverity::Critical,
            HexaErrorSeverity::Low,
            HexaErrorSeverity::High,
            HexaErrorSeverity::Medium,
        ];
        v.sort_by_key(|s| match s {
            HexaErrorSeverity::Low => 0,
            HexaErrorSeverity::Medium => 1,
            HexaErrorSeverity::High => 2,
            HexaErrorSeverity::Critical => 3,
        });
        assert_eq!(
            v,
            [
                HexaErrorSeverity::Low,
                HexaErrorSeverity::Medium,
                HexaErrorSeverity::High,
                HexaErrorSeverity::Critical,
            ]
        );
    }
    #[test]
    fn test_hexacoreerror_new_and_trait() {
        let err = HexaCoreError::new("core.test.hexacore")
            .with_message("HexaCoreError test")
            .with_kind(HexaErrorKind::Internal)
            .with_severity(HexaErrorSeverity::High);
        assert_eq!(err.error_code(), "core.test.hexacore");
        assert_eq!(err.error_message(), "HexaCoreError test");
        assert_eq!(err.error_kind(), HexaErrorKind::Internal);
        assert_eq!(err.error_severity(), HexaErrorSeverity::High);
        assert!(HexaError::source(&err).is_none());
        let log = err.to_log_entry();
        assert!(log.contains("core.test.hexacore"));
        assert!(log.contains("HexaCoreError test"));
    }
    #[test]
    fn test_hexacoreerror_display() {
        let err = HexaCoreError::new("core.display.test")
            .with_message("Display test")
            .with_kind(HexaErrorKind::Validation)
            .with_severity(HexaErrorSeverity::Low);
        let s = format!("{}", err);
        assert!(s.contains("Display test"));
        assert!(s.contains("core.display.test"));
    }
    #[test]
    fn test_hexacoreerror_with_source() {
        #[derive(Debug)]
        struct DummySrc;
        impl Display for DummySrc {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "DummySrc")
            }
        }
        impl Error for DummySrc {}
        let src = DummySrc;
        let err = HexaCoreError::new("core.test.with_source")
            .with_message("With source")
            .with_kind(HexaErrorKind::External)
            .with_severity(HexaErrorSeverity::Medium)
            .with_source(Box::new(src));
        assert!(HexaError::source(&err).is_some());
        assert_eq!(format!("{}", HexaError::source(&err).unwrap()), "DummySrc");
    }
    #[test]
    fn test_hexacoreerror_builder_pattern() {
        let err = HexaCoreError::new("core.builder.test")
            .with_message("Builder pattern error")
            .with_kind(HexaErrorKind::Timeout)
            .with_severity(HexaErrorSeverity::Critical);
        assert_eq!(err.error_code(), "core.builder.test");
        assert_eq!(err.error_message(), "Builder pattern error");
        assert_eq!(err.error_kind(), HexaErrorKind::Timeout);
        assert_eq!(err.error_severity(), HexaErrorSeverity::Critical);
    }
}
