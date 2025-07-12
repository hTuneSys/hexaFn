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
//!         Ok(true)
//!     }
//!     fn get_conditions(&self) -> Vec<Box<dyn TriggerCondition>> {
//!         vec![]
//!     }
//!     fn priority(&self) -> u32 { 1 }
//!     fn deactivate(&mut self) {}
//!     fn activate(&mut self) {}
//!     fn timeout(&self) -> Option<std::time::Duration> { None }
//! }
//!
//! let mut trigger = MyTrigger;
//! assert_eq!(trigger.id(), "my-trigger-1");
//! assert_eq!(trigger.name(), "Test Trigger");
//! assert!(trigger.is_active());
//! trigger.deactivate();
//! trigger.activate();
//! assert_eq!(trigger.priority(), 1);
//! assert_eq!(trigger.timeout(), None);
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
///     fn priority(&self) -> u32 { 0 }
///     fn deactivate(&mut self) {}
///     fn activate(&mut self) {}
///     fn timeout(&self) -> Option<std::time::Duration> { None }
/// }
/// let mut t = AlwaysActiveTrigger;
/// assert!(t.is_active());
/// t.deactivate();
/// t.activate();
/// assert_eq!(t.priority(), 0);
/// assert_eq!(t.timeout(), None);
/// assert_eq!(t.evaluate(&0u32 as &dyn Any).unwrap(), true);
/// ```
pub trait Trigger {
    /// Returns the unique identifier of the trigger.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_core::HexaError;
    /// use hexafn_trigger::Trigger;
    /// use hexafn_trigger::TriggerCondition;
    /// struct MyTrigger;
    /// impl Trigger for MyTrigger {
    ///     fn id(&self) -> String { "trigger-123".to_string() }
    ///     fn name(&self) -> String { "MyTrigger".to_string() }
    ///     fn is_active(&self) -> bool { true }
    ///     fn evaluate(&self, _: &dyn std::any::Any) -> Result<bool, Box<dyn HexaError>> { Ok(true) }
    ///     fn get_conditions(&self) -> Vec<Box<dyn TriggerCondition>> { vec![] }
    ///     fn priority(&self) -> u32 { 1 }
    ///     fn deactivate(&mut self) {}
    ///     fn activate(&mut self) {}
    ///     fn timeout(&self) -> Option<std::time::Duration> { None }
    /// }
    /// let mut t = MyTrigger;
    /// assert_eq!(t.id(), "trigger-123");
    /// assert_eq!(t.name(), "MyTrigger");
    /// assert!(t.is_active());
    /// t.deactivate();
    /// t.activate();
    /// assert_eq!(t.priority(), 1);
    /// assert_eq!(t.timeout(), None);
    /// assert_eq!(t.evaluate(&0u32 as &dyn std::any::Any).unwrap(), true);
    /// ```
    fn id(&self) -> String;

    /// Returns the name of the trigger.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_trigger::Trigger;
    /// struct NamedTrigger;
    /// impl Trigger for NamedTrigger {
    ///     fn id(&self) -> String { "named-1".to_string() }
    ///     fn name(&self) -> String { "NamedTrigger".to_string() }
    ///     fn is_active(&self) -> bool { true }
    ///     fn evaluate(&self, _: &dyn std::any::Any) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(true) }
    ///     fn get_conditions(&self) -> Vec<Box<dyn hexafn_trigger::TriggerCondition>> { vec![] }
    ///     fn priority(&self) -> u32 { 2 }
    ///     fn deactivate(&mut self) {}
    ///     fn activate(&mut self) {}
    ///     fn timeout(&self) -> Option<std::time::Duration> { None }
    /// }
    /// let mut t = NamedTrigger;
    /// assert_eq!(t.name(), "NamedTrigger");
    /// t.deactivate();
    /// t.activate();
    /// assert_eq!(t.priority(), 2);
    /// assert_eq!(t.timeout(), None);
    /// ```
    fn name(&self) -> String;

    /// Indicates whether the trigger is currently active.
    ///
    /// Returns `true` if the trigger should be evaluated, or `false` if it is disabled.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_trigger::Trigger;
    /// struct InactiveTrigger { active: bool }
    /// impl Trigger for InactiveTrigger {
    ///     fn id(&self) -> String { "inactive-1".to_string() }
    ///     fn name(&self) -> String { "Inactive".to_string() }
    ///     fn is_active(&self) -> bool { self.active }
    ///     fn evaluate(&self, _: &dyn std::any::Any) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(self.active) }
    ///     fn get_conditions(&self) -> Vec<Box<dyn hexafn_trigger::TriggerCondition>> { vec![] }
    ///     fn priority(&self) -> u32 { 0 }
    ///     fn deactivate(&mut self) { self.active = false; }
    ///     fn activate(&mut self) { self.active = true; }
    ///     fn timeout(&self) -> Option<std::time::Duration> { None }
    /// }
    /// let mut t = InactiveTrigger { active: true };
    /// t.deactivate();
    /// assert!(!t.is_active());
    /// t.activate();
    /// assert!(t.is_active());
    /// ```
    fn is_active(&self) -> bool;

    /// Evaluates the trigger against the provided context.
    ///
    /// # Arguments
    /// * `context` - A reference to any context object (typically event data) to evaluate.
    ///
    /// # Returns
    /// * `Ok(true)` if the trigger should fire.
    /// * `Ok(false)` if the trigger should not fire.
    /// * `Err` if evaluation fails due to an error.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_trigger::Trigger;
    /// use hexafn_core::HexaError;
    /// use std::any::Any;
    /// struct AlwaysFire;
    /// impl Trigger for AlwaysFire {
    ///     fn id(&self) -> String { "always-fire".to_string() }
    ///     fn name(&self) -> String { "AlwaysFire".to_string() }
    ///     fn is_active(&self) -> bool { true }
    ///     fn evaluate(&self, _context: &dyn Any) -> Result<bool, Box<dyn HexaError>> { Ok(true) }
    ///     fn get_conditions(&self) -> Vec<Box<dyn hexafn_trigger::TriggerCondition>> { vec![] }
    ///     fn priority(&self) -> u32 { 3 }
    ///     fn deactivate(&mut self) {}
    ///     fn activate(&mut self) {}
    ///     fn timeout(&self) -> Option<std::time::Duration> { None }
    /// }
    /// let mut t = AlwaysFire;
    /// assert_eq!(t.evaluate(&42u32 as &dyn Any).unwrap(), true);
    /// t.deactivate();
    /// t.activate();
    /// assert_eq!(t.priority(), 3);
    /// assert_eq!(t.timeout(), None);
    /// ```
    fn evaluate(&self, context: &dyn std::any::Any) -> Result<bool, Box<dyn HexaError>>;

    /// Returns the list of conditions associated with this trigger.
    ///
    /// Each condition is evaluated as part of the trigger's logic.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_trigger::{Trigger, TriggerCondition};
    /// struct DummyCondition;
    /// impl TriggerCondition for DummyCondition {
    ///     fn matches(&self, _: &dyn std::any::Any) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(true) }
    ///     fn description(&self) -> String { "dummy".to_string() }
    ///     fn get_priority(&self) -> u32 { 0 }
    /// }
    /// struct MyTrigger;
    /// impl Trigger for MyTrigger {
    ///     fn id(&self) -> String { "my-trigger".to_string() }
    ///     fn name(&self) -> String { "MyTrigger".to_string() }
    ///     fn is_active(&self) -> bool { true }
    ///     fn evaluate(&self, _: &dyn std::any::Any) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(true) }
    ///     fn get_conditions(&self) -> Vec<Box<dyn TriggerCondition>> {
    ///         vec![Box::new(DummyCondition)]
    ///     }
    ///     fn priority(&self) -> u32 { 4 }
    ///     fn deactivate(&mut self) {}
    ///     fn activate(&mut self) {}
    ///     fn timeout(&self) -> Option<std::time::Duration> { None }
    /// }
    /// let mut t = MyTrigger;
    /// assert_eq!(t.get_conditions().len(), 1);
    /// t.deactivate();
    /// t.activate();
    /// assert_eq!(t.priority(), 4);
    /// assert_eq!(t.timeout(), None);
    /// ```
    fn get_conditions(&self) -> Vec<Box<dyn TriggerCondition>>;

    /// Returns the priority of the trigger (higher value means higher priority).
    ///
    /// Triggers with higher priority are evaluated first when multiple triggers are present.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_trigger::Trigger;
    /// use hexafn_trigger::TriggerCondition;
    /// use hexafn_core::HexaError;
    /// struct PriorityTrigger;
    /// impl Trigger for PriorityTrigger {
    ///     fn id(&self) -> String { "".to_string() }
    ///     fn name(&self) -> String { "".to_string() }
    ///     fn is_active(&self) -> bool { true }
    ///     fn evaluate(&self, _: &dyn std::any::Any) -> Result<bool, Box<dyn HexaError>> { Ok(true) }
    ///     fn get_conditions(&self) -> Vec<Box<dyn TriggerCondition>> { vec![] }
    ///     fn priority(&self) -> u32 { 42 }
    ///     fn deactivate(&mut self) {}
    ///     fn activate(&mut self) {}
    ///     fn timeout(&self) -> Option<std::time::Duration> { None }
    /// }
    /// let t = PriorityTrigger;
    /// assert_eq!(t.priority(), 42);
    /// ```
    fn priority(&self) -> u32;

    /// Deactivates the trigger (sets it to inactive state).
    ///
    /// After calling this method, `is_active()` should return false and the trigger will not be evaluated.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_trigger::Trigger;
    /// struct DeactivatableTrigger { active: bool }
    /// impl Trigger for DeactivatableTrigger {
    ///     fn id(&self) -> String { "".to_string() }
    ///     fn name(&self) -> String { "".to_string() }
    ///     fn is_active(&self) -> bool { self.active }
    ///     fn evaluate(&self, _: &dyn std::any::Any) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(self.active) }
    ///     fn get_conditions(&self) -> Vec<Box<dyn hexafn_trigger::TriggerCondition>> { vec![] }
    ///     fn priority(&self) -> u32 { 0 }
    ///     fn deactivate(&mut self) { self.active = false; }
    ///     fn activate(&mut self) { self.active = true; }
    ///     fn timeout(&self) -> Option<std::time::Duration> { None }
    /// }
    /// let mut t = DeactivatableTrigger { active: true };
    /// t.deactivate();
    /// assert!(!t.is_active());
    /// ```
    fn deactivate(&mut self);

    /// Activates the trigger (sets it to active state).
    ///
    /// After calling this method, `is_active()` should return true and the trigger will be eligible for evaluation.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_trigger::Trigger;
    /// struct ActivatableTrigger { active: bool }
    /// impl Trigger for ActivatableTrigger {
    ///     fn id(&self) -> String { "".to_string() }
    ///     fn name(&self) -> String { "".to_string() }
    ///     fn is_active(&self) -> bool { self.active }
    ///     fn evaluate(&self, _: &dyn std::any::Any) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(self.active) }
    ///     fn get_conditions(&self) -> Vec<Box<dyn hexafn_trigger::TriggerCondition>> { vec![] }
    ///     fn priority(&self) -> u32 { 0 }
    ///     fn deactivate(&mut self) { self.active = false; }
    ///     fn activate(&mut self) { self.active = true; }
    ///     fn timeout(&self) -> Option<std::time::Duration> { None }
    /// }
    /// let mut t = ActivatableTrigger { active: false };
    /// t.activate();
    /// assert!(t.is_active());
    /// ```
    fn activate(&mut self);

    /// Returns the timeout duration for trigger evaluation, if any.
    ///
    /// If a timeout is set, trigger evaluation must complete within this duration. Otherwise, it may be aborted or marked as failed.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_trigger::Trigger;
    /// struct TimeoutTrigger;
    /// impl Trigger for TimeoutTrigger {
    ///     fn id(&self) -> String { "".to_string() }
    ///     fn name(&self) -> String { "".to_string() }
    ///     fn is_active(&self) -> bool { true }
    ///     fn evaluate(&self, _: &dyn std::any::Any) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(true) }
    ///     fn get_conditions(&self) -> Vec<Box<dyn hexafn_trigger::TriggerCondition>> { vec![] }
    ///     fn priority(&self) -> u32 { 0 }
    ///     fn deactivate(&mut self) {}
    ///     fn activate(&mut self) {}
    ///     fn timeout(&self) -> Option<std::time::Duration> { Some(std::time::Duration::from_secs(5)) }
    /// }
    /// let t = TimeoutTrigger;
    /// assert_eq!(t.timeout(), Some(std::time::Duration::from_secs(5)));
    /// ```
    fn timeout(&self) -> Option<std::time::Duration>;
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
        fn priority(&self) -> u32 {
            10
        }
        fn deactivate(&mut self) {}
        fn activate(&mut self) {}
        fn timeout(&self) -> Option<std::time::Duration> {
            None
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
        fn priority(&self) -> u32 {
            0
        }
        fn deactivate(&mut self) {}
        fn activate(&mut self) {}
        fn timeout(&self) -> Option<std::time::Duration> {
            None
        }
    }

    #[test]
    fn test_inactive_trigger() {
        let trigger = InactiveTrigger;
        assert!(!trigger.is_active());
        let context = ();
        assert!(!trigger.evaluate(&context as &dyn Any).unwrap());
    }

    #[test]
    fn test_trigger_priority() {
        struct PriorityTrigger;
        impl Trigger for PriorityTrigger {
            fn id(&self) -> String {
                "".to_string()
            }
            fn name(&self) -> String {
                "".to_string()
            }
            fn is_active(&self) -> bool {
                true
            }
            fn evaluate(&self, _: &dyn Any) -> Result<bool, Box<dyn HexaError>> {
                Ok(true)
            }
            fn get_conditions(&self) -> Vec<Box<dyn TriggerCondition>> {
                vec![]
            }
            fn priority(&self) -> u32 {
                99
            }
            fn deactivate(&mut self) {}
            fn activate(&mut self) {}
            fn timeout(&self) -> Option<std::time::Duration> {
                None
            }
        }
        let t = PriorityTrigger;
        assert_eq!(t.priority(), 99);
    }

    #[test]
    fn test_trigger_deactivate_activate() {
        struct StateTrigger {
            active: bool,
        }
        impl Trigger for StateTrigger {
            fn id(&self) -> String {
                "".to_string()
            }
            fn name(&self) -> String {
                "".to_string()
            }
            fn is_active(&self) -> bool {
                self.active
            }
            fn evaluate(&self, _: &dyn Any) -> Result<bool, Box<dyn HexaError>> {
                Ok(self.active)
            }
            fn get_conditions(&self) -> Vec<Box<dyn TriggerCondition>> {
                vec![]
            }
            fn priority(&self) -> u32 {
                0
            }
            fn deactivate(&mut self) {
                self.active = false;
            }
            fn activate(&mut self) {
                self.active = true;
            }
            fn timeout(&self) -> Option<std::time::Duration> {
                None
            }
        }
        let mut t = StateTrigger { active: true };
        t.deactivate();
        assert!(!t.is_active());
        t.activate();
        assert!(t.is_active());
    }

    #[test]
    fn test_trigger_timeout() {
        struct TimeoutTrigger;
        impl Trigger for TimeoutTrigger {
            fn id(&self) -> String {
                "".to_string()
            }
            fn name(&self) -> String {
                "".to_string()
            }
            fn is_active(&self) -> bool {
                true
            }
            fn evaluate(&self, _: &dyn Any) -> Result<bool, Box<dyn HexaError>> {
                Ok(true)
            }
            fn get_conditions(&self) -> Vec<Box<dyn TriggerCondition>> {
                vec![]
            }
            fn priority(&self) -> u32 {
                0
            }
            fn deactivate(&mut self) {}
            fn activate(&mut self) {}
            fn timeout(&self) -> Option<std::time::Duration> {
                Some(std::time::Duration::from_secs(3))
            }
        }
        let t = TimeoutTrigger;
        assert_eq!(t.timeout(), Some(std::time::Duration::from_secs(3)));
    }
}
