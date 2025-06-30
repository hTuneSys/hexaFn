// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # HexaErrorKind Value Object
//!
//! This module defines the [`HexaErrorKind`] enum, which categorizes errors by their nature and origin
//! for consistent handling across all hexaFn modules. It is a value object in the DDD sense and is used
//! in all error contracts and error value objects (see [`crate::HexaError`]).
//!
//! ## Usage Example
//!
//! ```rust
//! use hexafn_core::HexaErrorKind;
//!
//! let kind = HexaErrorKind::Validation;
//! assert_eq!(kind.to_string(), "Validation");
//! ```
//!
//! ## DDD/Hexagonal Architecture Note
//!
//! This value object should be used in all error-related domain contracts and value objects to ensure
//! a consistent error taxonomy throughout the system. It is not intended for direct use in infrastructure
//! or application layers except via domain error types.

use std::fmt::{Debug, Display};

/// Enum representing the kind/category of an error in the hexaFn framework.
///
/// Each variant describes a distinct error category for consistent error handling and reporting.
///
/// # Example
/// ```
/// use hexafn_core::HexaErrorKind;
/// let kind = HexaErrorKind::NotFound;
/// assert_eq!(kind.to_string(), "NotFound");
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

impl Display for HexaErrorKind {
    /// Formats the error kind as a string (e.g. "Validation").
    ///
    /// # Example
    /// ```
    /// use hexafn_core::HexaErrorKind;
    /// let kind = HexaErrorKind::Validation;
    /// assert_eq!(kind.to_string(), "Validation");
    /// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(HexaErrorKind::NotFound.to_string(), "NotFound");
        assert_eq!(HexaErrorKind::Validation.to_string(), "Validation");
        assert_eq!(HexaErrorKind::Timeout.to_string(), "Timeout");
        assert_eq!(HexaErrorKind::Internal.to_string(), "Internal");
        assert_eq!(HexaErrorKind::External.to_string(), "External");
        assert_eq!(HexaErrorKind::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn test_equality() {
        let a = HexaErrorKind::Timeout;
        let b = HexaErrorKind::Timeout;
        let c = HexaErrorKind::Internal;
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_clone_and_copy() {
        let kind = HexaErrorKind::External;
        let copy = kind;
        let clone = kind;
        assert_eq!(kind, copy);
        assert_eq!(kind, clone);
    }

    #[test]
    fn test_debug_format() {
        let kind = HexaErrorKind::Validation;
        let debug_str = format!("{:?}", kind);
        assert_eq!(debug_str, "Validation");
    }
}
