// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # TriggerEvaluator Trait
//!
//! This module defines the [`TriggerEvaluator`] trait, which provides the contract for managing and evaluating triggers
//! in the hexaFn system. The trait enables registration, removal, listing, and evaluation of triggers against arbitrary
//! contexts, supporting the **Filter** phase of the 6F Lifecycle Flow.
//!
//! ## Responsibilities
//! - Register and unregister triggers at runtime
//! - List all or only active triggers
//! - Evaluate a trigger against a given context
//!
//! ## Example
//!
//! ```rust
//! use hexafn_trigger::domain::contracts::{Trigger, TriggerEvaluator, TriggerCondition};
//! use hexafn_core::HexaError;
//! use std::any::Any;
//!
//! struct AlwaysTrueCondition;
//! impl TriggerCondition for AlwaysTrueCondition {
//!     fn matches(&self, _context: &dyn Any) -> Result<bool, Box<dyn HexaError>> { Ok(true) }
//!     fn description(&self) -> String { "Always true".to_string() }
//!     fn get_priority(&self) -> u32 { 0 }
//! }
//!
//! struct SimpleTrigger;
//! impl Trigger for SimpleTrigger {
//!     fn id(&self) -> String { "simple".to_string() }
//!     fn name(&self) -> String { "Simple".to_string() }
//!     fn is_active(&self) -> bool { true }
//!     fn evaluate(&self, _context: &dyn Any) -> Result<bool, Box<dyn HexaError>> { Ok(true) }
//!     fn get_conditions(&self) -> Vec<Box<dyn TriggerCondition>> { vec![Box::new(AlwaysTrueCondition)] }
//! }
//!
//! struct InMemoryTriggerEvaluator {
//!     triggers: Vec<Box<dyn Trigger>>,
//! }
//! impl InMemoryTriggerEvaluator {
//!     fn new() -> Self { Self { triggers: Vec::new() } }
//! }
//! impl TriggerEvaluator for InMemoryTriggerEvaluator {
//!     fn evaluate(&self, trigger: &dyn Trigger, context: &dyn Any) -> Result<bool, Box<dyn HexaError>> {
//!         trigger.evaluate(context)
//!     }
//!     fn register_trigger(&mut self, trigger: Box<dyn Trigger>) -> Result<(), Box<dyn HexaError>> {
//!         self.triggers.push(trigger);
//!         Ok(())
//!     }
//!     fn unregister_trigger(&mut self, id: &str) -> Result<(), Box<dyn HexaError>> {
//!         self.triggers.retain(|t| t.id() != id);
//!         Ok(())
//!     }
//!     fn list_triggers(&self) -> Vec<&dyn Trigger> {
//!         self.triggers.iter().map(|t| t.as_ref()).collect()
//!     }
//!     fn get_active_triggers(&self) -> Vec<&dyn Trigger> {
//!         self.triggers.iter().filter(|t| t.is_active()).map(|t| t.as_ref()).collect()
//!     }
//! }
//!
//! let mut evaluator = InMemoryTriggerEvaluator::new();
//! evaluator.register_trigger(Box::new(SimpleTrigger)).unwrap();
//! let triggers = evaluator.list_triggers();
//! assert_eq!(triggers.len(), 1);
//! let result = evaluator.evaluate(triggers[0], &42u32 as &dyn Any);
//! assert_eq!(result.unwrap(), true);
//! ```

use super::trigger::Trigger;
use hexafn_core::HexaError;
use std::any::Any;

/// Trait for managing and evaluating triggers in the system.
///
/// `TriggerEvaluator` provides methods to register, remove, list, and evaluate triggers.
/// It acts as a domain service for orchestrating trigger logic in the **Filter** phase of the pipeline.
///
/// # Example
///
/// ```rust
/// use hexafn_trigger::domain::contracts::{Trigger, TriggerEvaluator};
/// # use hexafn_core::HexaError;
/// # use std::any::Any;
/// struct DummyTrigger;
/// impl Trigger for DummyTrigger {
///     fn id(&self) -> String { "dummy".to_string() }
///     fn name(&self) -> String { "Dummy".to_string() }
///     fn is_active(&self) -> bool { true }
///     fn evaluate(&self, _context: &dyn Any) -> Result<bool, Box<dyn HexaError>> { Ok(true) }
///     fn get_conditions(&self) -> Vec<Box<dyn hexafn_trigger::domain::contracts::TriggerCondition>> { vec![] }
/// }
/// struct DummyEvaluator { triggers: Vec<Box<dyn Trigger>> }
/// impl TriggerEvaluator for DummyEvaluator {
///     fn evaluate(&self, trigger: &dyn Trigger, context: &dyn Any) -> Result<bool, Box<dyn HexaError>> {
///         trigger.evaluate(context)
///     }
///     fn register_trigger(&mut self, trigger: Box<dyn Trigger>) -> Result<(), Box<dyn HexaError>> {
///         self.triggers.push(trigger); Ok(())
///     }
///     fn unregister_trigger(&mut self, id: &str) -> Result<(), Box<dyn HexaError>> {
///         self.triggers.retain(|t| t.id() != id); Ok(())
///     }
///     fn list_triggers(&self) -> Vec<&dyn Trigger> {
///         self.triggers.iter().map(|t| t.as_ref()).collect()
///     }
///     fn get_active_triggers(&self) -> Vec<&dyn Trigger> {
///         self.triggers.iter().filter(|t| t.is_active()).map(|t| t.as_ref()).collect()
///     }
/// }
/// ```
pub trait TriggerEvaluator {
    /// Evaluates a trigger against the provided context.
    ///
    /// # Arguments
    /// * `trigger` - The trigger to evaluate.
    /// * `context` - The context (typically event data) to evaluate against.
    ///
    /// # Returns
    /// * `Ok(true)` if the trigger should fire.
    /// * `Ok(false)` if the trigger should not fire.
    /// * `Err` if evaluation fails due to an error.
    ///
    /// # Example
    ///
    /// ```
    /// use hexafn_trigger::domain::contracts::{Trigger, TriggerEvaluator, TriggerCondition};
    /// use hexafn_core::HexaError;
    /// use std::any::Any;
    ///
    /// struct TrueCondition;
    /// impl TriggerCondition for TrueCondition {
    ///     fn matches(&self, _: &dyn Any) -> Result<bool, Box<dyn HexaError>> { Ok(true) }
    ///     fn description(&self) -> String { "Always true".to_string() }
    ///     fn get_priority(&self) -> u32 { 0 }
    /// }
    ///
    /// struct MyTrigger;
    /// impl Trigger for MyTrigger {
    ///     fn id(&self) -> String { "t".to_string() }
    ///     fn name(&self) -> String { "T".to_string() }
    ///     fn is_active(&self) -> bool { true }
    ///     fn evaluate(&self, context: &dyn Any) -> Result<bool, Box<dyn HexaError>> {
    ///         for cond in self.get_conditions() {
    ///             if !cond.matches(context)? {
    ///                 return Ok(false);
    ///             }
    ///         }
    ///         Ok(true)
    ///     }
    ///     fn get_conditions(&self) -> Vec<Box<dyn TriggerCondition>> {
    ///         vec![Box::new(TrueCondition)]
    ///     }
    /// }
    ///
    /// struct DummyEvaluator { triggers: Vec<Box<dyn Trigger>> }
    /// impl TriggerEvaluator for DummyEvaluator {
    ///     fn evaluate(&self, trigger: &dyn Trigger, context: &dyn Any) -> Result<bool, Box<dyn HexaError>> {
    ///         trigger.evaluate(context)
    ///     }
    ///     fn register_trigger(&mut self, trigger: Box<dyn Trigger>) -> Result<(), Box<dyn HexaError>> {
    ///         self.triggers.push(trigger); Ok(())
    ///     }
    ///     fn unregister_trigger(&mut self, id: &str) -> Result<(), Box<dyn HexaError>> {
    ///         self.triggers.retain(|t| t.id() != id); Ok(())
    ///     }
    ///     fn list_triggers(&self) -> Vec<&dyn Trigger> {
    ///         self.triggers.iter().map(|t| t.as_ref()).collect()
    ///     }
    ///     fn get_active_triggers(&self) -> Vec<&dyn Trigger> {
    ///         self.triggers.iter().filter(|t| t.is_active()).map(|t| t.as_ref()).collect()
    ///     }
    /// }
    ///
    /// let mut evaluator = DummyEvaluator { triggers: vec![Box::new(MyTrigger)] };
    /// let triggers = evaluator.list_triggers();
    /// let ctx = 42u32;
    /// assert_eq!(evaluator.evaluate(triggers[0], &ctx as &dyn Any).unwrap(), true);
    /// ```
    fn evaluate(&self, trigger: &dyn Trigger, context: &dyn Any) -> Result<bool, Box<dyn HexaError>>;

    /// Registers a new trigger in the evaluator.
    ///
    /// # Arguments
    /// * `trigger` - The trigger to register.
    ///
    /// # Returns
    /// * `Ok(())` if registration is successful.
    /// * `Err` if registration fails.
    ///
    /// # Example
    ///
    /// ```
    /// use hexafn_trigger::domain::contracts::{Trigger, TriggerEvaluator};
    /// # use hexafn_core::HexaError;
    /// struct MyTrigger;
    /// impl Trigger for MyTrigger {
    ///     fn id(&self) -> String { "t".to_string() }
    ///     fn name(&self) -> String { "T".to_string() }
    ///     fn is_active(&self) -> bool { true }
    ///     fn evaluate(&self, _: &dyn std::any::Any) -> Result<bool, Box<dyn HexaError>> { Ok(true) }
    ///     fn get_conditions(&self) -> Vec<Box<dyn hexafn_trigger::domain::contracts::TriggerCondition>> { vec![] }
    /// }
    /// struct DummyEvaluator { triggers: Vec<Box<dyn Trigger>> }
    /// impl TriggerEvaluator for DummyEvaluator {
    ///     fn evaluate(&self, _: &dyn Trigger, _: &dyn std::any::Any) -> Result<bool, Box<dyn HexaError>> { Ok(true) }
    ///     fn register_trigger(&mut self, trigger: Box<dyn Trigger>) -> Result<(), Box<dyn HexaError>> {
    ///         self.triggers.push(trigger); Ok(())
    ///     }
    ///     fn unregister_trigger(&mut self, _: &str) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    ///     fn list_triggers(&self) -> Vec<&dyn Trigger> { self.triggers.iter().map(|t| t.as_ref()).collect() }
    ///     fn get_active_triggers(&self) -> Vec<&dyn Trigger> { self.triggers.iter().map(|t| t.as_ref()).collect() }
    /// }
    /// let mut evaluator = DummyEvaluator { triggers: vec![] };
    /// assert!(evaluator.register_trigger(Box::new(MyTrigger)).is_ok());
    /// ```
    fn register_trigger(&mut self, trigger: Box<dyn Trigger>) -> Result<(), Box<dyn HexaError>>;

    /// Unregisters a trigger by its unique identifier.
    ///
    /// # Arguments
    /// * `id` - The unique identifier of the trigger to remove.
    ///
    /// # Returns
    /// * `Ok(())` if removal is successful.
    /// * `Err` if the trigger is not found or removal fails.
    ///
    /// # Example
    ///
    /// ```
    /// use hexafn_trigger::domain::contracts::{Trigger, TriggerEvaluator};
    /// # use hexafn_core::HexaError;
    /// struct MyTrigger;
    /// impl Trigger for MyTrigger {
    ///     fn id(&self) -> String { "remove-me".to_string() }
    ///     fn name(&self) -> String { "".to_string() }
    ///     fn is_active(&self) -> bool { true }
    ///     fn evaluate(&self, _: &dyn std::any::Any) -> Result<bool, Box<dyn HexaError>> { Ok(true) }
    ///     fn get_conditions(&self) -> Vec<Box<dyn hexafn_trigger::domain::contracts::TriggerCondition>> { vec![] }
    /// }
    /// struct DummyEvaluator { triggers: Vec<Box<dyn Trigger>> }
    /// impl TriggerEvaluator for DummyEvaluator {
    ///     fn evaluate(&self, _: &dyn Trigger, _: &dyn std::any::Any) -> Result<bool, Box<dyn HexaError>> { Ok(true) }
    ///     fn register_trigger(&mut self, trigger: Box<dyn Trigger>) -> Result<(), Box<dyn HexaError>> {
    ///         self.triggers.push(trigger); Ok(())
    ///     }
    ///     fn unregister_trigger(&mut self, id: &str) -> Result<(), Box<dyn HexaError>> {
    ///         self.triggers.retain(|t| t.id() != id); Ok(())
    ///     }
    ///     fn list_triggers(&self) -> Vec<&dyn Trigger> { self.triggers.iter().map(|t| t.as_ref()).collect() }
    ///     fn get_active_triggers(&self) -> Vec<&dyn Trigger> { self.triggers.iter().map(|t| t.as_ref()).collect() }
    /// }
    /// let mut evaluator = DummyEvaluator { triggers: vec![Box::new(MyTrigger)] };
    /// assert!(evaluator.unregister_trigger("remove-me").is_ok());
    /// ```
    fn unregister_trigger(&mut self, id: &str) -> Result<(), Box<dyn HexaError>>;

    /// Returns a list of all registered triggers.
    ///
    /// # Returns
    /// * `Vec<&dyn Trigger>` - All triggers currently registered.
    ///
    /// # Example
    ///
    /// ```
    /// use hexafn_trigger::domain::contracts::{Trigger, TriggerEvaluator};
    /// struct MyTrigger;
    /// impl Trigger for MyTrigger {
    ///     fn id(&self) -> String { "t".to_string() }
    ///     fn name(&self) -> String { "".to_string() }
    ///     fn is_active(&self) -> bool { true }
    ///     fn evaluate(&self, _: &dyn std::any::Any) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(true) }
    ///     fn get_conditions(&self) -> Vec<Box<dyn hexafn_trigger::domain::contracts::TriggerCondition>> { vec![] }
    /// }
    /// struct DummyEvaluator { triggers: Vec<Box<dyn Trigger>> }
    /// impl TriggerEvaluator for DummyEvaluator {
    ///     fn evaluate(&self, _: &dyn Trigger, _: &dyn std::any::Any) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(true) }
    ///     fn register_trigger(&mut self, trigger: Box<dyn Trigger>) -> Result<(), Box<dyn hexafn_core::HexaError>> {
    ///         self.triggers.push(trigger); Ok(())
    ///     }
    ///     fn unregister_trigger(&mut self, _: &str) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    ///     fn list_triggers(&self) -> Vec<&dyn Trigger> { self.triggers.iter().map(|t| t.as_ref()).collect() }
    ///     fn get_active_triggers(&self) -> Vec<&dyn Trigger> { self.triggers.iter().map(|t| t.as_ref()).collect() }
    /// }
    /// let evaluator = DummyEvaluator { triggers: vec![Box::new(MyTrigger)] };
    /// let triggers = evaluator.list_triggers();
    /// assert_eq!(triggers.len(), 1);
    /// ```
    fn list_triggers(&self) -> Vec<&dyn Trigger>;

    /// Returns a list of all active triggers.
    ///
    /// # Returns
    /// * `Vec<&dyn Trigger>` - Only triggers that are currently active.
    ///
    /// # Example
    ///
    /// ```
    /// use hexafn_trigger::domain::contracts::{Trigger, TriggerEvaluator};
    /// struct ActiveTrigger;
    /// impl Trigger for ActiveTrigger {
    ///     fn id(&self) -> String { "active".to_string() }
    ///     fn name(&self) -> String { "".to_string() }
    ///     fn is_active(&self) -> bool { true }
    ///     fn evaluate(&self, _: &dyn std::any::Any) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(true) }
    ///     fn get_conditions(&self) -> Vec<Box<dyn hexafn_trigger::domain::contracts::TriggerCondition>> { vec![] }
    /// }
    /// struct InactiveTrigger;
    /// impl Trigger for InactiveTrigger {
    ///     fn id(&self) -> String { "inactive".to_string() }
    ///     fn name(&self) -> String { "".to_string() }
    ///     fn is_active(&self) -> bool { false }
    ///     fn evaluate(&self, _: &dyn std::any::Any) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(false) }
    ///     fn get_conditions(&self) -> Vec<Box<dyn hexafn_trigger::domain::contracts::TriggerCondition>> { vec![] }
    /// }
    /// struct DummyEvaluator { triggers: Vec<Box<dyn Trigger>> }
    /// impl TriggerEvaluator for DummyEvaluator {
    ///     fn evaluate(&self, _: &dyn Trigger, _: &dyn std::any::Any) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(true) }
    ///     fn register_trigger(&mut self, trigger: Box<dyn Trigger>) -> Result<(), Box<dyn hexafn_core::HexaError>> {
    ///         self.triggers.push(trigger); Ok(())
    ///     }
    ///     fn unregister_trigger(&mut self, _: &str) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    ///     fn list_triggers(&self) -> Vec<&dyn Trigger> { self.triggers.iter().map(|t| t.as_ref()).collect() }
    ///     fn get_active_triggers(&self) -> Vec<&dyn Trigger> {
    ///         self.triggers.iter().filter(|t| t.is_active()).map(|t| t.as_ref()).collect()
    ///     }
    /// }
    /// let evaluator = DummyEvaluator {
    ///     triggers: vec![Box::new(ActiveTrigger), Box::new(InactiveTrigger)]
    /// };
    /// let active = evaluator.get_active_triggers();
    /// assert_eq!(active.len(), 1);
    /// assert_eq!(active[0].id(), "active");
    /// ```
    fn get_active_triggers(&self) -> Vec<&dyn Trigger>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;

    struct TestCondition;
    impl super::super::trigger_condition::TriggerCondition for TestCondition {
        fn matches(&self, context: &dyn Any) -> Result<bool, Box<dyn HexaError>> {
            Ok(context.is::<u32>())
        }
        fn description(&self) -> String {
            "Matches if context is u32".to_string()
        }
        fn get_priority(&self) -> u32 {
            1
        }
    }

    struct TestTrigger;
    impl Trigger for TestTrigger {
        fn id(&self) -> String { "test".to_string() }
        fn name(&self) -> String { "Test".to_string() }
        fn is_active(&self) -> bool { true }
        fn evaluate(&self, context: &dyn Any) -> Result<bool, Box<dyn HexaError>> {
            for cond in self.get_conditions() {
                if !cond.matches(context)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        fn get_conditions(&self) -> Vec<Box<dyn super::super::trigger_condition::TriggerCondition>> {
            vec![Box::new(TestCondition)]
        }
    }

    struct DummyEvaluator {
        triggers: Vec<Box<dyn Trigger>>,
    }

    impl DummyEvaluator {
        fn new() -> Self {
            Self { triggers: Vec::new() }
        }
    }

    impl TriggerEvaluator for DummyEvaluator {
        fn evaluate(&self, trigger: &dyn Trigger, context: &dyn Any) -> Result<bool, Box<dyn HexaError>> {
            trigger.evaluate(context)
        }
        fn register_trigger(&mut self, trigger: Box<dyn Trigger>) -> Result<(), Box<dyn HexaError>> {
            self.triggers.push(trigger);
            Ok(())
        }
        fn unregister_trigger(&mut self, id: &str) -> Result<(), Box<dyn HexaError>> {
            self.triggers.retain(|t| t.id() != id);
            Ok(())
        }
        fn list_triggers(&self) -> Vec<&dyn Trigger> {
            self.triggers.iter().map(|t| t.as_ref()).collect()
        }
        fn get_active_triggers(&self) -> Vec<&dyn Trigger> {
            self.triggers.iter().filter(|t| t.is_active()).map(|t| t.as_ref()).collect()
        }
    }

    #[test]
    fn test_register_and_list_triggers() {
        let mut evaluator = DummyEvaluator::new();
        assert_eq!(evaluator.list_triggers().len(), 0);

        evaluator.register_trigger(Box::new(TestTrigger)).unwrap();
        let triggers = evaluator.list_triggers();
        assert_eq!(triggers.len(), 1);
        assert_eq!(triggers[0].id(), "test");
    }

    #[test]
    fn test_unregister_trigger() {
        let mut evaluator = DummyEvaluator::new();
        evaluator.register_trigger(Box::new(TestTrigger)).unwrap();
        assert_eq!(evaluator.list_triggers().len(), 1);

        evaluator.unregister_trigger("test").unwrap();
        assert_eq!(evaluator.list_triggers().len(), 0);
    }

    #[test]
    fn test_get_active_triggers() {
        struct InactiveTrigger;
        impl Trigger for InactiveTrigger {
            fn id(&self) -> String { "inactive".to_string() }
            fn name(&self) -> String { "Inactive".to_string() }
            fn is_active(&self) -> bool { false }
            fn evaluate(&self, _: &dyn Any) -> Result<bool, Box<dyn HexaError>> { Ok(false) }
            fn get_conditions(&self) -> Vec<Box<dyn super::super::trigger_condition::TriggerCondition>> { vec![] }
        }

        let mut evaluator = DummyEvaluator::new();
        evaluator.register_trigger(Box::new(TestTrigger)).unwrap();
        evaluator.register_trigger(Box::new(InactiveTrigger)).unwrap();

        let active = evaluator.get_active_triggers();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].id(), "test");
    }

    #[test]
    fn test_evaluate_trigger_success() {
        let evaluator = DummyEvaluator {
            triggers: vec![Box::new(TestTrigger)],
        };
        let triggers = evaluator.list_triggers();
        let ctx = 42u32;
        let result = evaluator.evaluate(triggers[0], &ctx as &dyn Any);
        assert!(result.unwrap());
    }

    #[test]
    fn test_evaluate_trigger_fail_on_condition() {
        let evaluator = DummyEvaluator {
            triggers: vec![Box::new(TestTrigger)],
        };
        let triggers = evaluator.list_triggers();
        let ctx = "not a u32";
        let result = evaluator.evaluate(triggers[0], &ctx as &dyn Any);
        assert!(!result.unwrap());
    }
}