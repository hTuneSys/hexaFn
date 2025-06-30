// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # HexaErrorSeverity Value Object
//!
//! This module defines the [`HexaErrorSeverity`] enum, which represents the severity level of errors
//! in the hexaFn framework. It is used for alerting, logging, and prioritization in all error contracts
//! and value objects (see [`crate::HexaError`]).
//!
//! ## Usage Example
//!
//! ```rust
//! use hexafn_core::HexaErrorSeverity;
//!
//! let sev = HexaErrorSeverity::Critical;
//! assert_eq!(sev.to_string(), "Critical");
//! ```
//!
//! ## DDD/Hexagonal Architecture Note
//!
//! This value object should be used in all error-related domain contracts and value objects to ensure
//! a consistent severity taxonomy throughout the system.

/// Enum representing the severity level for hexaFn errors.
///
/// Used for alerting, logging, and prioritization.
///
/// # Example
/// ```
/// use hexafn_core::HexaErrorSeverity;
/// let sev = HexaErrorSeverity::High;
/// assert_eq!(sev.to_string(), "High");
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
    /// Formats the severity as a string (e.g. "High").
    ///
    /// # Example
    /// ```
    /// use hexafn_core::HexaErrorSeverity;
    /// let sev = HexaErrorSeverity::Medium;
    /// assert_eq!(sev.to_string(), "Medium");
    /// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(HexaErrorSeverity::Low.to_string(), "Low");
        assert_eq!(HexaErrorSeverity::Medium.to_string(), "Medium");
        assert_eq!(HexaErrorSeverity::High.to_string(), "High");
        assert_eq!(HexaErrorSeverity::Critical.to_string(), "Critical");
    }

    #[test]
    fn test_equality() {
        let a = HexaErrorSeverity::High;
        let b = HexaErrorSeverity::High;
        let c = HexaErrorSeverity::Critical;
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_clone_and_copy() {
        let sev = HexaErrorSeverity::Medium;
        let copy = sev;
        let clone = sev;
        assert_eq!(sev, copy);
        assert_eq!(sev, clone);
    }

    #[test]
    fn test_debug_format() {
        let sev = HexaErrorSeverity::Critical;
        let debug_str = format!("{:?}", sev);
        assert_eq!(debug_str, "Critical");
    }
}
