// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! FunctionRegistry for hexaFn
//!
//! This registry provides a thread-safe way to register and retrieve function runtimes by name.
//! It is used to manage multiple implementations of the `FunctionRuntime` trait.
//!
//! # Example
//!
//! ```
//! use hexafn_run::FunctionRegistry;
//! use hexafn_run::FunctionRuntime;
//! use hexafn_run::FunctionContext;
//! use hexafn_run::ExecutionResult;
//! use hexafn_run::ExecutionStatus;
//! use std::collections::HashMap;
//! use std::time::Duration;
//!
//! #[derive(Clone)]
//! struct DummyRuntime;
//! impl FunctionRuntime for DummyRuntime {
//!     fn execute(&self, _context: FunctionContext) -> Result<ExecutionResult, String> {
//!         Ok(ExecutionResult::new(ExecutionStatus::Success, HashMap::new(), None, Duration::from_millis(1), 0))
//!     }
//! }
//!
//! let registry = FunctionRegistry::new();
//! registry.register("dummy", Box::new(DummyRuntime));
//! assert!(registry.get("dummy").is_some());
//! ```
//!
//! # Test
//!
//! See module tests for more examples.

use crate::FunctionRuntime;
use dyn_clone::clone_box;
use std::collections::HashMap;
use std::sync::RwLock;

/// Thread-safe registry for function runtimes.
///
/// Allows registering and retrieving `FunctionRuntime` implementations by name.
///
/// # Example
/// ```
/// use hexafn_run::FunctionRegistry;
/// use hexafn_run::FunctionRuntime;
/// use hexafn_run::FunctionContext;
/// use hexafn_run::ExecutionResult;
/// use hexafn_run::ExecutionStatus;
/// use std::collections::HashMap;
/// use std::time::Duration;
///
/// #[derive(Clone)]
/// struct DummyRuntime;
/// impl FunctionRuntime for DummyRuntime {
///     fn execute(&self, _context: FunctionContext) -> Result<ExecutionResult, String> {
///         Ok(ExecutionResult::new(ExecutionStatus::Success, HashMap::new(), None, Duration::from_secs(0), 0))
///     }
/// }
/// let registry = FunctionRegistry::new();
/// registry.register("dummy", Box::new(DummyRuntime));
/// assert!(registry.get("dummy").is_some());
/// ```
pub struct FunctionRegistry {
    /// A thread-safe registry for function runtimes.
    map: RwLock<HashMap<String, Box<dyn FunctionRuntime>>>,
}

impl FunctionRegistry {
    /// Creates a new, empty registry.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::FunctionRegistry;
    /// let registry = FunctionRegistry::new();
    /// assert!(registry.get("any").is_none());
    /// ```
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }
    /// Registers a function runtime under the given name.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::FunctionRegistry;
    /// use hexafn_run::FunctionRuntime;
    /// use hexafn_run::FunctionContext;
    /// use hexafn_run::ExecutionResult;
    ///
    /// #[derive(Clone)]
    /// struct DummyRuntime;
    /// impl FunctionRuntime for DummyRuntime {
    ///     fn execute(&self, _context: FunctionContext) -> Result<ExecutionResult, String> { Err("not implemented".into()) }
    /// }
    /// let registry = FunctionRegistry::new();
    /// registry.register("dummy", Box::new(DummyRuntime));
    /// assert!(registry.get("dummy").is_some());
    /// ```
    pub fn register(&self, name: &str, function: Box<dyn FunctionRuntime>) {
        self.map.write().unwrap().insert(name.to_string(), function);
    }
    /// Retrieves a function runtime by name, if it exists.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::FunctionRegistry;
    /// use hexafn_run::FunctionRuntime;
    /// use hexafn_run::FunctionContext;
    /// use hexafn_run::ExecutionResult;
    ///
    /// #[derive(Clone)]
    /// struct DummyRuntime;
    /// impl FunctionRuntime for DummyRuntime {
    ///     fn execute(&self, _context: FunctionContext) -> Result<ExecutionResult, String> { Err("not implemented".into()) }
    /// }
    /// let registry = FunctionRegistry::new();
    /// registry.register("dummy", Box::new(DummyRuntime));
    /// assert!(registry.get("dummy").is_some());
    /// assert!(registry.get("unknown").is_none());
    /// ```
    pub fn get(&self, name: &str) -> Option<Box<dyn FunctionRuntime>> {
        self.map
            .read()
            .unwrap()
            .get(name)
            .map(|runtime| clone_box(&**runtime))
    }
}

//TODO add comment
impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ExecutionResult, ExecutionStatus, FunctionContext, FunctionRuntime};
    use std::collections::HashMap;
    use std::time::Duration;

    #[derive(Clone)]
    struct DummyRuntime;
    impl FunctionRuntime for DummyRuntime {
        fn execute(&self, _context: FunctionContext) -> Result<ExecutionResult, String> {
            Ok(ExecutionResult::new(
                ExecutionStatus::Success,
                HashMap::new(),
                None,
                Duration::from_millis(1),
                0,
            ))
        }
    }

    #[test]
    fn test_register_and_get() {
        let registry = FunctionRegistry::new();
        registry.register("dummy", Box::new(DummyRuntime));
        assert!(registry.get("dummy").is_some());
        assert!(registry.get("unknown").is_none());
    }

    #[test]
    fn test_execute_via_registry() {
        let registry = FunctionRegistry::new();
        registry.register("dummy", Box::new(DummyRuntime));
        let runtime = registry.get("dummy").unwrap();
        let ctx = FunctionContext::new();
        let result = runtime.execute(ctx).unwrap();
        assert!(result.is_success());
    }
}
