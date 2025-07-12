// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # CompoundType Enum
//!
//! CompoundType specifies how multiple trigger conditions (TriggerCondition) are logically combined.
//! It enables flexible rule definitions using logical operators like AND, OR, and NOT.
//!
//! ## Example
//! ```rust
//! use hexafn_trigger::domain::value_objects::CompoundType;
//!
//! let t_and = CompoundType::And;
//! let t_or = CompoundType::Or;
//! let t_not = CompoundType::Not;
//! assert_eq!(format!("{:?}", t_and), "And");
//! ```

/// Logical combination types for trigger conditions.
///
/// - `And`: Fires if all conditions are met.
/// - `Or`: Fires if at least one condition is met.
/// - `Not`: Fires if the specified condition is not met.
///
/// ## Example
/// ```rust
/// use hexafn_trigger::domain::value_objects::CompoundType;
/// let c = CompoundType::And;
/// match c {
///     CompoundType::And => println!("AND"),
///     CompoundType::Or => println!("OR"),
///     CompoundType::Not => println!("NOT"),
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CompoundType {
    /// Fires if all conditions are met.
    And,
    /// Fires if at least one condition is met.
    Or,
    /// Fires if the specified condition is not met.
    Not,
}

use std::fmt;

impl fmt::Display for CompoundType {
    /// Formats the CompoundType as a string.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::CompoundType;
    /// let c = CompoundType::Or;
    /// assert_eq!(format!("{}", c), "Or");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompoundType::And => write!(f, "And"),
            CompoundType::Or => write!(f, "Or"),
            CompoundType::Not => write!(f, "Not"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compound_type_debug() {
        assert_eq!(format!("{:?}", CompoundType::And), "And");
        assert_eq!(format!("{:?}", CompoundType::Or), "Or");
        assert_eq!(format!("{:?}", CompoundType::Not), "Not");
    }

    #[test]
    fn test_compound_type_match() {
        let c = CompoundType::Or;
        let result = match c {
            CompoundType::And => "AND",
            CompoundType::Or => "OR",
            CompoundType::Not => "NOT",
        };
        assert_eq!(result, "OR");
    }

    #[test]
    fn test_compound_type_display() {
        assert_eq!(format!("{}", CompoundType::And), "And");
        assert_eq!(format!("{}", CompoundType::Or), "Or");
        assert_eq!(format!("{}", CompoundType::Not), "Not");
    }
}
