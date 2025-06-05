// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Trigger Trait
//!
//! This module defines the [`Trigger`] trait, which represents a trigger in the hexaFn system.
//! A trigger encapsulates the logic for evaluating a set of conditions against an input context
//! and determines whether an action should be fired. Triggers are central to the **Filter** phase
//! of the 6F Lifecycle Flow and are designed to be composable, testable, and extensible.
//!
//! ## Example
//!
//! ```rust
//! use hexafn_trigger::Trigger;
//! use hexafn_trigger::TriggerCondition;
//! use hexafn_core::HexaError;
//! use std::any::Any;
//!
//! struct MyTrigger;
//!
//! impl Trigger for MyTrigger {
//!     fn id(&self) -> String { "my-trigger-1".to_string() }
//!     fn name(&self) -> String { "Test Trigger".to_string() }
//!     fn is_active(&self) -> bool { true }
//!     fn evaluate(&self, context: &dyn Any) -> Result<bool, Box<dyn HexaError>> {
//!         // Example: always fire
//!         Ok(true)
//!     }
//!     fn get_conditions(&self) -> Vec<Box<dyn TriggerCondition>> {
//!         vec![]
//!     }
//! }
//!
//! let trigger = MyTrigger;
//! let result = trigger.evaluate(&42u32 as &dyn Any);
//! assert_eq!(result.unwrap(), true);
//! ```

use super::trigger_condition::TriggerCondition;
use hexafn_core::HexaError;

/// Trait representing a trigger in the system.
///
/// A `Trigger` is responsible for evaluating a set of [`TriggerCondition`]s against a given context.
/// If all conditions are satisfied, the trigger is considered to have fired. Triggers are used to
/// orchestrate event-driven flows and can be enabled or disabled dynamically.
///
/// # Example
///
/// ```rust
/// use hexafn_trigger::{Trigger, TriggerCondition};
/// use hexafn_core::HexaError;
/// use std::any::Any;
///
/// struct AlwaysActiveTrigger;
///
/// impl Trigger for AlwaysActiveTrigger {
///     fn id(&self) -> String { "always-active".to_string() }
///     fn name(&self) -> String { "Always Active".to_string() }
///     fn is_active(&self) -> bool { true }
///     fn evaluate(&self, _context: &dyn Any) -> Result<bool, Box<dyn HexaError>> { Ok(true) }
///     fn get_conditions(&self) -> Vec<Box<dyn TriggerCondition>> { vec![] }
/// }
/// ```
pub trait Trigger {
    /// Returns the unique identifier of the trigger.
    ///
    /// # Example
    ///
    /// ```rust
    /// use hexafn_core::HexaError;
    /// use hexafn_trigger::Trigger;
    /// use hexafn_trigger::TriggerCondition;
    ///
    /// struct MyTrigger;
    /// impl Trigger for MyTrigger {
    ///     fn id(&self) -> String { "trigger-123".to_string() }
    ///     # fn name(&self) -> String { "".to_string() }
    ///     # fn is_active(&self) -> bool { true }
    ///     # fn evaluate(&self, _: &dyn std::any::Any) -> Result<bool, Box<dyn HexaError>> { Ok(true) }
    ///     # fn get_conditions(&self) -> Vec<Box<dyn TriggerCondition>> { vec![] }
    /// }
    /// let t = MyTrigger;
    /// assert_eq!(t.id(), "trigger-123");
    /// ```
    fn id(&self) -> String;

    /// Returns the name of the trigger.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexafn_trigger::Trigger;
    /// struct NamedTrigger;
    /// impl Trigger for NamedTrigger {
    ///     fn id(&self) -> String { "".to_string() }
    ///     fn name(&self) -> String { "MyTrigger".to_string() }
    ///     # fn is_active(&self) -> bool { true }
    ///     # fn evaluate(&self, _: &dyn std::any::Any) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(true) }
    ///     # fn get_conditions(&self) -> Vec<Box<dyn hexafn_trigger::TriggerCondition>> { vec![] }
    /// }
    /// let t = NamedTrigger;
    /// assert_eq!(t.name(), "MyTrigger");
    /// ```
    fn name(&self) -> String;

    /// Indicates whether the trigger is currently active.
    ///
    /// Returns `true` if the trigger should be evaluated, or `false` if it is disabled.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexafn_trigger::Trigger;
    /// struct InactiveTrigger;
    /// impl Trigger for InactiveTrigger {
    ///     fn id(&self) -> String { "".to_string() }
    ///     fn name(&self) -> String { "".to_string() }
    ///     fn is_active(&self) -> bool { false }
    ///     # fn evaluate(&self, _: &dyn std::any::Any) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(false) }
    ///     # fn get_conditions(&self) -> Vec<Box<dyn hexafn_trigger::TriggerCondition>> { vec![] }
    /// }
    /// let t = InactiveTrigger;
    /// assert!(!t.is_active());
    /// ```
    fn is_active(&self) -> bool;

    /// Evaluates the trigger against the provided context.
    ///
    /// # Arguments
    ///
    /// * `context` - A reference to any context object (typically event data) to evaluate.
    ///
    /// # Returns
    ///
    /// * `Ok(true)` if the trigger should fire.
    /// * `Ok(false)` if the trigger should not fire.
    /// * `Err` if evaluation fails due to an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexafn_trigger::Trigger;
    /// # use hexafn_core::HexaError;
    /// # use std::any::Any;
    /// struct AlwaysFire;
    /// impl Trigger for AlwaysFire {
    ///     fn id(&self) -> String { "".to_string() }
    ///     fn name(&self) -> String { "".to_string() }
    ///     fn is_active(&self) -> bool { true }
    ///     fn evaluate(&self, _context: &dyn Any) -> Result<bool, Box<dyn HexaError>> { Ok(true) }
    ///     fn get_conditions(&self) -> Vec<Box<dyn hexafn_trigger::TriggerCondition>> { vec![] }
    /// }
    /// let t = AlwaysFire;
    /// assert_eq!(t.evaluate(&42u32 as &dyn Any).unwrap(), true);
    /// ```
    fn evaluate(&self, context: &dyn std::any::Any) -> Result<bool, Box<dyn HexaError>>;

    /// Returns the list of conditions associated with this trigger.
    ///
    /// Each condition is evaluated as part of the trigger's logic.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexafn_trigger::{Trigger, TriggerCondition};
    /// struct DummyCondition;
    /// impl TriggerCondition for DummyCondition {
    ///     fn matches(&self, _: &dyn std::any::Any) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(true) }
    ///     fn description(&self) -> String { "dummy".to_string() }
    ///     fn get_priority(&self) -> u32 { 0 }
    /// }
    /// struct MyTrigger;
    /// impl Trigger for MyTrigger {
    ///     fn id(&self) -> String { "".to_string() }
    ///     fn name(&self) -> String { "".to_string() }
    ///     fn is_active(&self) -> bool { true }
    ///     fn evaluate(&self, _: &dyn std::any::Any) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(true) }
    ///     fn get_conditions(&self) -> Vec<Box<dyn TriggerCondition>> {
    ///         vec![Box::new(DummyCondition)]
    ///     }
    /// }
    /// let t = MyTrigger;
    /// assert_eq!(t.get_conditions().len(), 1);
    /// ```
    fn get_conditions(&self) -> Vec<Box<dyn TriggerCondition>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use hexafn_core::HexaError;
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

    struct TestTrigger;

    impl Trigger for TestTrigger {
        fn id(&self) -> String {
            "test-trigger".to_string()
        }
        fn name(&self) -> String {
            "Test Trigger".to_string()
        }
        fn is_active(&self) -> bool {
            true
        }
        fn evaluate(&self, context: &dyn Any) -> Result<bool, Box<dyn HexaError>> {
            // Fires only if all conditions match
            for cond in self.get_conditions() {
                if !cond.matches(context)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        fn get_conditions(&self) -> Vec<Box<dyn TriggerCondition>> {
            vec![Box::new(AlwaysTrueCondition)]
        }
    }

    #[test]
    fn test_trigger_id_and_name() {
        let trigger = TestTrigger;
        assert_eq!(trigger.id(), "test-trigger");
        assert_eq!(trigger.name(), "Test Trigger");
    }

    #[test]
    fn test_trigger_is_active() {
        let trigger = TestTrigger;
        assert!(trigger.is_active());
    }

    #[test]
    fn test_trigger_evaluate_true() {
        let trigger = TestTrigger;
        let context = 123u32;
        let result = trigger.evaluate(&context as &dyn Any);
        assert!(result.unwrap());
    }

    #[test]
    fn test_trigger_get_conditions() {
        let trigger = TestTrigger;
        let conditions = trigger.get_conditions();
        assert_eq!(conditions.len(), 1);
        assert_eq!(conditions[0].description(), "Always true");
    }

    struct InactiveTrigger;

    impl Trigger for InactiveTrigger {
        fn id(&self) -> String {
            "inactive".to_string()
        }
        fn name(&self) -> String {
            "Inactive".to_string()
        }
        fn is_active(&self) -> bool {
            false
        }
        fn evaluate(&self, _context: &dyn Any) -> Result<bool, Box<dyn HexaError>> {
            Ok(false)
        }
        fn get_conditions(&self) -> Vec<Box<dyn TriggerCondition>> {
            vec![]
        }
    }

    #[test]
    fn test_inactive_trigger() {
        let trigger = InactiveTrigger;
        assert!(!trigger.is_active());
        let context = ();
        assert!(!trigger.evaluate(&context as &dyn Any).unwrap());
    }
}
