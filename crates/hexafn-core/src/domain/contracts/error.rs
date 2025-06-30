// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Error Handling Contracts (Core Module)
//!
//! This module provides the foundational error handling abstractions for the hexaFn system.
//! It ensures consistent error classification, logging, and observability across all modules
//! and the entire 6F Lifecycle Flow: Feed → Filter → Format → Function → Forward → Feedback.
//!
//! ## Architecture
//!
//! - The [`HexaError`] trait defines the core error contract for all hexaFn components.
//! - [`HexaErrorKind`] categorizes errors by their nature and origin for proper handling.
//! - [`HexaErrorSeverity`] provides prioritization levels for monitoring and alerting.
//!
//! ## Example
//!
//! ```rust
//! use hexafn_core::{HexaError, HexaErrorKind, HexaErrorSeverity};
//! use std::fmt::{Debug, Display};
//!
//! #[derive(Debug)]
//! struct MyError;
//!
//! impl Display for MyError {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         write!(f, "My error!")
//!     }
//! }
//!
//! impl HexaError for MyError {
//!     fn error_code(&self) -> &str { "core.example" }
//!     fn error_message(&self) -> &str { "Example error" }
//!     fn error_kind(&self) -> HexaErrorKind { HexaErrorKind::Internal }
//!     fn error_severity(&self) -> HexaErrorSeverity { HexaErrorSeverity::Low }
//! }
//! let err = MyError;
//! assert_eq!(err.error_code(), "core.example");
//! ```

use std::fmt::{Debug, Display};

use crate::{HexaErrorKind, HexaErrorSeverity};

/// Main trait for all structured errors in hexaFn.
///
/// Provides error code, message, kind, severity, source, and log formatting.
///
/// # Example
/// ```rust
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
    /// ```rust
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
    /// ```rust
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
    /// ```rust
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
    /// ```rust
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

    /// Structured log entry for this error (e.g. "(code) (Kind Severity) Message").
    ///
    /// # Example
    /// ```rust
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
    /// ```rust
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{HexaCoreError, HexaErrorKind, HexaErrorSeverity};
    use std::error::Error;
    use std::fmt::Display;

    /// Dummy error for testing HexaError trait implementation.
    #[derive(Debug)]
    struct DummyError;
    impl Display for DummyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Dummy error!")
        }
    }
    impl Error for DummyError {}

    /// Error with source for testing error chaining.
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

    /// Test Display and Debug for HexaErrorKind.
    #[test]
    fn test_kind_display_and_debug() {
        assert_eq!(format!("{}", HexaErrorKind::Validation), "Validation");
        assert_eq!(format!("{:?}", HexaErrorKind::Timeout), "Timeout");
    }
    /// Test Display and Debug for HexaErrorSeverity.
    #[test]
    fn test_severity_display_and_debug() {
        assert_eq!(format!("{}", HexaErrorSeverity::Critical), "Critical");
        assert_eq!(format!("{:?}", HexaErrorSeverity::Low), "Low");
    }
    /// Test basic trait implementation for HexaError.
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
    /// Test trait implementation with error source.
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
    /// Test log entry format for HexaError.
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
    /// Test uniqueness of enum variants for kind and severity.
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
    /// Test sorting by severity.
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
    /// Test HexaCoreError builder and trait implementation.
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
    /// Test Display for HexaCoreError.
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
    /// Test HexaCoreError with error source.
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
    /// Test HexaCoreError builder pattern.
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
