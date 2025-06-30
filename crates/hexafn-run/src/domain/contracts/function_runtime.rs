// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! FunctionRuntime trait for hexaFn
//!
//! This trait defines the contract for executing user-defined functions in different runtimes (DSL, WASM, JS, etc).
//! Implementations should provide logic for execution, initialization, shutdown, and runtime type identification.
//!
//! # Example
//!
//! ```
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
//!     fn execute(&self, context: FunctionContext) -> Result<ExecutionResult, String> {
//!         let mut outputs = HashMap::new();
//!         outputs.insert("result".to_string(), "ok".to_string());
//!         Ok(ExecutionResult::new(ExecutionStatus::Success, outputs, None, Duration::from_millis(1), 0))
//!     }
//!     fn get_runtime_type(&self) -> String { "dummy".to_string() }
//! }
//!
//! let runtime = DummyRuntime;
//! let ctx = FunctionContext::new();
//! let result = runtime.execute(ctx).unwrap();
//! assert!(result.is_success());
//! assert_eq!(runtime.get_runtime_type(), "dummy");
//! ```
//!
//! # Test
//!
//! See module tests for more examples.

use crate::{ExecutionResult, FunctionContext};
use dyn_clone::DynClone;

/// Trait for function execution runtimes (DSL, WASM, JS, etc).
///
/// # Example
/// ```
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
///     fn execute(&self, context: FunctionContext) -> Result<ExecutionResult, String> {
///         Ok(ExecutionResult::new(ExecutionStatus::Success, HashMap::new(), None, Duration::from_secs(0), 0))
///     }
/// }
/// let runtime = DummyRuntime;
/// let ctx = FunctionContext::new();
/// let result = runtime.execute(ctx).unwrap();
/// assert!(result.is_success());
/// ```
pub trait FunctionRuntime: Send + Sync + DynClone {
    /// Executes the function logic with the given context.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::FunctionRuntime;
    /// use hexafn_run::FunctionContext;
    /// use hexafn_run:: ExecutionResult;
    /// use hexafn_run::ExecutionStatus;
    /// use std::collections::HashMap;
    /// use std::time::Duration;
    ///
    /// #[derive(Clone)]
    /// struct DummyRuntime;
    /// impl FunctionRuntime for DummyRuntime {
    ///     fn execute(&self, context: FunctionContext) -> Result<ExecutionResult, String> {
    ///         Ok(ExecutionResult::new(ExecutionStatus::Success, HashMap::new(), None, Duration::from_secs(0), 0))
    ///     }
    /// }
    /// let runtime = DummyRuntime;
    /// let ctx = FunctionContext::new();
    /// let result = runtime.execute(ctx).unwrap();
    /// assert!(result.is_success());
    /// ```
    fn execute(&self, context: FunctionContext) -> Result<ExecutionResult, String>;

    /// Initializes the runtime (optional).
    ///
    /// # Example
    /// ```
    /// use hexafn_run::FunctionRuntime;
    /// use hexafn_run::FunctionContext;
    /// use hexafn_run::ExecutionResult;
    ///
    /// #[derive(Clone)]
    /// struct DummyRuntime;
    /// impl FunctionRuntime for DummyRuntime {
    ///     fn execute(&self, _context: FunctionContext) -> Result<ExecutionResult, String> { unimplemented!() }
    ///     fn init(&self) -> Result<(), String> { Ok(()) }
    /// }
    /// let runtime = DummyRuntime;
    /// assert!(runtime.init().is_ok());
    /// ```
    fn init(&self) -> Result<(), String> {
        Ok(())
    }

    /// Shuts down the runtime (optional).
    ///
    /// # Example
    /// ```
    /// use hexafn_run::FunctionRuntime;
    /// use hexafn_run::FunctionContext;
    /// use hexafn_run::ExecutionResult;
    ///
    /// #[derive(Clone)]
    /// struct DummyRuntime;
    /// impl FunctionRuntime for DummyRuntime {
    ///     fn execute(&self, _context: FunctionContext) -> Result<ExecutionResult, String> { unimplemented!() }
    ///     fn shutdown(&self) -> Result<(), String> { Ok(()) }
    /// }
    /// let runtime = DummyRuntime;
    /// assert!(runtime.shutdown().is_ok());
    /// ```
    fn shutdown(&self) -> Result<(), String> {
        Ok(())
    }

    /// Returns the runtime type as a string (e.g., "dsl", "wasm").
    ///
    /// # Example
    /// ```
    /// use hexafn_run::FunctionRuntime;
    /// use hexafn_run::FunctionContext;
    /// use hexafn_run::ExecutionResult;
    ///
    /// #[derive(Clone)]
    /// struct DummyRuntime;
    /// impl FunctionRuntime for DummyRuntime {
    ///     fn execute(&self, _context: FunctionContext) -> Result<ExecutionResult, String> { unimplemented!() }
    ///     fn get_runtime_type(&self) -> String { "dummy".to_string() }
    /// }
    /// let runtime = DummyRuntime;
    /// assert_eq!(runtime.get_runtime_type(), "dummy");
    /// ```
    fn get_runtime_type(&self) -> String {
        "dsl".to_string()
    }
}

dyn_clone::clone_trait_object!(FunctionRuntime);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ExecutionResult, ExecutionStatus, FunctionContext};
    use std::collections::HashMap;
    use std::time::Duration;

    #[derive(Clone)]
    struct DummyRuntime;
    impl FunctionRuntime for DummyRuntime {
        fn execute(&self, _context: FunctionContext) -> Result<ExecutionResult, String> {
            let mut outputs = HashMap::new();
            outputs.insert("result".to_string(), "ok".to_string());
            Ok(ExecutionResult::new(
                ExecutionStatus::Success,
                outputs,
                None,
                Duration::from_millis(1),
                0,
            ))
        }
        fn get_runtime_type(&self) -> String {
            "dummy".to_string()
        }
    }

    #[test]
    fn test_execute_success() {
        let runtime = DummyRuntime;
        let ctx = FunctionContext::new();
        let result = runtime.execute(ctx).unwrap();
        assert!(result.is_success());
        assert_eq!(result.get_output("result"), Some(&"ok".to_string()));
    }

    #[test]
    fn test_runtime_type() {
        let runtime = DummyRuntime;
        assert_eq!(runtime.get_runtime_type(), "dummy");
    }

    #[test]
    fn test_init_and_shutdown() {
        let runtime = DummyRuntime;
        assert!(runtime.init().is_ok());
        assert!(runtime.shutdown().is_ok());
    }
}
