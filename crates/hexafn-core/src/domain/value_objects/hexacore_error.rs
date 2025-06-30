// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # HexaCoreError Value Object
//!
//! This module defines the [`HexaCoreError`] struct, a reusable, structured error type for all hexaFn modules.
//! It implements the [`HexaError`] trait and uses value objects
//! [`HexaErrorKind`] and [`HexaErrorSeverity`].
//!
//! ## Usage Example
//!
//! ```rust
//! use hexafn_core::{HexaCoreError, HexaError, HexaErrorKind, HexaErrorSeverity};
//! let err = HexaCoreError::new("core.example.invalid")
//!     .with_message("Invalid operation")
//!     .with_kind(HexaErrorKind::Validation)
//!     .with_severity(HexaErrorSeverity::Medium);
//! assert_eq!(err.error_code(), "core.example.invalid");
//! assert_eq!(err.error_kind(), HexaErrorKind::Validation);
//! assert_eq!(err.error_severity(), HexaErrorSeverity::Medium);
//! assert_eq!(err.error_message(), "Invalid operation");
//! ```

use crate::HexaError;
use crate::HexaErrorKind;
use crate::HexaErrorSeverity;

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
    /// The error code, a static string identifier for the error.
    code: &'static str,
    /// The error message, a human-readable description of the error.
    message: String,
    /// The kind of error, represented by a `HexaErrorKind` value object.
    kind: HexaErrorKind,
    /// The severity of the error, represented by a `HexaErrorSeverity` value object.
    severity: HexaErrorSeverity,
    /// An optional source error, which can be any type that implements the `std::error::Error` trait.
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
    use crate::HexaError;
    use crate::HexaErrorKind;
    use crate::HexaErrorSeverity;
    use std::fmt::{Display, Formatter, Result as FmtResult};

    #[test]
    fn test_builder_and_trait_methods() {
        let err = HexaCoreError::new("core.test")
            .with_message("msg")
            .with_kind(HexaErrorKind::Timeout)
            .with_severity(HexaErrorSeverity::High);
        assert_eq!(err.error_code(), "core.test");
        assert_eq!(err.error_message(), "msg");
        assert_eq!(err.error_kind(), HexaErrorKind::Timeout);
        assert_eq!(err.error_severity(), HexaErrorSeverity::High);
    }

    #[test]
    fn test_with_source_and_trait() {
        #[derive(Debug)]
        struct DummySrc;
        impl Display for DummySrc {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, "DummySrc")
            }
        }
        impl std::error::Error for DummySrc {}
        let src = DummySrc;
        let err = HexaCoreError::new("core.src").with_source(Box::new(src));
        assert!(HexaError::source(&err).is_some());
        assert_eq!(format!("{}", HexaError::source(&err).unwrap()), "DummySrc");
    }

    #[test]
    fn test_display_format() {
        let err = HexaCoreError::new("core.display")
            .with_message("Display test")
            .with_kind(HexaErrorKind::Internal)
            .with_severity(HexaErrorSeverity::Critical);
        let s = format!("{err}");
        assert!(s.contains("Display test"));
        assert!(s.contains("core.display"));
        assert!(s.contains("Internal"));
        assert!(s.contains("Critical"));
    }

    #[test]
    fn test_default_values() {
        let err = HexaCoreError::new("core.default");
        assert_eq!(err.error_code(), "core.default");
        assert_eq!(err.error_kind(), HexaErrorKind::Unknown);
        assert_eq!(err.error_severity(), HexaErrorSeverity::Low);
        assert_eq!(err.error_message(), "");
        assert!(err.source().is_none());
    }
}
