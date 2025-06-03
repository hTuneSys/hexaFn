// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # TriggerCondition Trait
//!
//! This module defines the [`TriggerCondition`] trait, which represents a single condition
//! that must be satisfied for a [`Trigger`](super::trigger::Trigger) to fire in the hexaFn system.
//!
//! ## Purpose
//!
//! - Encapsulates reusable, composable logic for evaluating whether a trigger should activate
//! - Supports prioritization and human-readable descriptions for introspection and debugging
//! - Enables flexible, type-erased context evaluation using `&dyn Any`
//!
//! ## Usage
//!
//! Implement this trait to define custom trigger conditions. Each condition can be prioritized
//! and described, and is evaluated against an arbitrary context (typically event data).
//!
//! ## Example
//!
//! ```rust
//! use hexafn_trigger::TriggerCondition;
//! use hexafn_core::HexaError;
//! use std::any::Any;
//!
//! struct IsStringCondition;
//!
//! impl TriggerCondition for IsStringCondition {
//!     fn matches(&self, context: &dyn Any) -> Result<bool, Box<dyn HexaError>> {
//!         Ok(context.is::<String>())
//!     }
//!     fn description(&self) -> String { "Matches if context is a String".to_string() }
//!     fn get_priority(&self) -> u32 { 10 }
//! }
//!
//! let cond = IsStringCondition;
//! let ctx = "hello".to_string();
//! assert_eq!(cond.matches(&ctx as &dyn Any).unwrap(), true);
//! ```

use hexafn_core::HexaError;
use std::any::Any;

/// Trait representing a condition for a trigger.
///
/// Implement this trait to define custom logic that determines whether a trigger should fire
/// based on the provided context. Conditions can be prioritized and described for introspection.
///
/// # Example
///
/// ```rust
/// use hexafn_trigger::TriggerCondition;
/// use hexafn_core::HexaError;
/// use std::any::Any;
///
/// struct AlwaysTrueCondition;
///
/// impl TriggerCondition for AlwaysTrueCondition {
///     fn matches(&self, _context: &dyn Any) -> Result<bool, Box<dyn HexaError>> {
///         Ok(true)
///     }
///     fn description(&self) -> String {
///         "Always true".to_string()
///     }
///     fn get_priority(&self) -> u32 {
///         0
///     }
/// }
/// ```
pub trait TriggerCondition {
    /// Checks if the condition matches the given context.
    ///
    /// # Arguments
    ///
    /// * `context` - A reference to any context object (typically event data) to evaluate.
    ///
    /// # Returns
    ///
    /// * `Ok(true)` if the condition matches and the trigger should fire.
    /// * `Ok(false)` if the condition does not match.
    /// * `Err` if evaluation fails due to an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use hexafn_trigger::TriggerCondition;
    /// use hexafn_core::HexaError;
    /// use std::any::Any;
    ///
    /// struct TrueIfString;
    ///
    /// impl TriggerCondition for TrueIfString {
    ///     fn matches(&self, context: &dyn Any) -> Result<bool, Box<dyn HexaError>> {
    ///         Ok(context.is::<String>())
    ///     }
    ///     fn description(&self) -> String { "Matches if context is String".to_string() }
    ///     fn get_priority(&self) -> u32 { 1 }
    /// }
    ///
    /// let cond = TrueIfString;
    /// let ctx = "hello".to_string();
    /// assert_eq!(cond.matches(&ctx as &dyn Any).unwrap(), true);
    /// ```
    fn matches(&self, context: &dyn Any) -> Result<bool, Box<dyn HexaError>>;

    /// Returns a human-readable description of the condition.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crate::hexafn_trigger::TriggerCondition;
    /// use hexafn_core::HexaError;
    /// use std::any::Any;
    ///
    /// struct MyCondition;
    /// impl TriggerCondition for MyCondition {
    ///     fn matches(&self, _context: &dyn Any) -> Result<bool, Box<dyn HexaError>> { Ok(true) }
    ///     fn description(&self) -> String { "My custom condition".to_string() }
    ///     fn get_priority(&self) -> u32 { 0 }
    /// }
    ///
    /// let cond = MyCondition;
    /// assert_eq!(cond.description(), "My custom condition");
    /// ```
    fn description(&self) -> String;

    /// Returns the priority of this condition (lower is higher priority).
    ///
    /// # Example
    ///
    /// ```rust
    /// use hexafn_trigger::TriggerCondition;
    /// use hexafn_core::HexaError;
    /// use std::any::Any;
    ///
    /// struct PriorityCondition;
    /// impl TriggerCondition for PriorityCondition {
    ///     fn matches(&self, _context: &dyn Any) -> Result<bool, Box<dyn HexaError>> { Ok(false) }
    ///     fn description(&self) -> String { "Priority condition".to_string() }
    ///     fn get_priority(&self) -> u32 { 42 }
    /// }
    ///
    /// let cond = PriorityCondition;
    /// assert_eq!(cond.get_priority(), 42);
    /// ```
    fn get_priority(&self) -> u32;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;

    struct AlwaysTrueCondition;

    impl TriggerCondition for AlwaysTrueCondition {
        fn matches(&self, _context: &dyn Any) -> Result<bool, Box<dyn HexaError>> {
            Ok(true)
        }
        fn description(&self) -> String {
            "Always true".to_string()
        }
        fn get_priority(&self) -> u32 {
            0
        }
    }

    struct StringOnlyCondition;

    impl TriggerCondition for StringOnlyCondition {
        fn matches(&self, context: &dyn Any) -> Result<bool, Box<dyn HexaError>> {
            Ok(context.is::<String>())
        }
        fn description(&self) -> String {
            "Matches if context is String".to_string()
        }
        fn get_priority(&self) -> u32 {
            5
        }
    }

    #[test]
    fn test_always_true_condition_matches() {
        let cond = AlwaysTrueCondition;
        let dummy = 42u32;
        assert!(cond.matches(&dummy as &dyn Any).unwrap());
    }

    #[test]
    fn test_always_true_condition_description() {
        let cond = AlwaysTrueCondition;
        assert_eq!(cond.description(), "Always true");
    }

    #[test]
    fn test_always_true_condition_priority() {
        let cond = AlwaysTrueCondition;
        assert_eq!(cond.get_priority(), 0);
    }

    #[test]
    fn test_string_only_condition_matches_string() {
        let cond = StringOnlyCondition;
        let ctx = "hello".to_string();
        assert!(cond.matches(&ctx as &dyn Any).unwrap());
    }

    #[test]
    fn test_string_only_condition_matches_non_string() {
        let cond = StringOnlyCondition;
        let ctx = 123u32;
        assert!(!cond.matches(&ctx as &dyn Any).unwrap());
    }

    #[test]
    fn test_string_only_condition_description() {
        let cond = StringOnlyCondition;
        assert_eq!(cond.description(), "Matches if context is String");
    }

    #[test]
    fn test_string_only_condition_priority() {
        let cond = StringOnlyCondition;
        assert_eq!(cond.get_priority(), 5);
    }
}
