// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! ExecutionResult for hexaFn
//!
//! Represents the result of a function execution, including status, outputs, error, duration, and memory usage.
//!
//! # Example
//!
//! ```
//! use hexafn_run::ExecutionResult;
//! use hexafn_run::ExecutionStatus;
//! use std::collections::HashMap;
//! use std::time::Duration;
//!
//! let mut outputs = HashMap::new();
//! outputs.insert("result".to_string(), "42".to_string());
//! let result = ExecutionResult::new(ExecutionStatus::Success, outputs, None, Duration::from_millis(10), 1024);
//! assert!(result.is_success());
//! assert_eq!(result.get_output("result"), Some(&"42".to_string()));
//! ```
//!
//! # Test
//!
//! See module tests for more examples.

use crate::ExecutionStatus;
use std::collections::HashMap;

/// Represents the result of a function execution.
///
/// # Example
/// ```
/// use hexafn_run::ExecutionResult;
/// use hexafn_run::ExecutionStatus;
/// use std::collections::HashMap;
/// use std::time::Duration;
/// let result = ExecutionResult::new(ExecutionStatus::Failure, HashMap::new(), Some("error".to_string()), Duration::from_secs(1), 0);
/// assert!(!result.is_success());
/// ```
pub struct ExecutionResult {
    /// The status of the execution
    pub status: ExecutionStatus,
    /// Outputs from the execution
    pub outputs: HashMap<String, String>,
    /// Optional error if the execution failed
    pub error: Option<String>,
    /// Duration of the execution
    pub duration: std::time::Duration,
    /// Memory used during execution in bytes
    pub memory_used: u64,
}

impl ExecutionResult {
    /// Creates a new `ExecutionResult` with the given status and outputs.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::ExecutionResult;
    /// use hexafn_run::ExecutionStatus;
    /// use std::collections::HashMap;
    /// use std::time::Duration;
    /// let result = ExecutionResult::new(ExecutionStatus::Success, HashMap::new(), None, Duration::from_secs(0), 0);
    /// assert_eq!(result.status, ExecutionStatus::Success);
    /// ```
    pub fn new(
        status: ExecutionStatus,
        outputs: HashMap<String, String>,
        error: Option<String>,
        duration: std::time::Duration,
        memory_used: u64,
    ) -> Self {
        Self {
            status,
            outputs,
            error,
            duration,
            memory_used,
        }
    }

    /// Checks if the execution was successful.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::ExecutionResult;
    /// use hexafn_run::ExecutionStatus;
    /// use std::collections::HashMap;
    /// use std::time::Duration;
    /// let result = ExecutionResult::new(ExecutionStatus::Success, HashMap::new(), None, Duration::from_secs(0), 0);
    /// assert!(result.is_success());
    /// ```
    pub fn is_success(&self) -> bool {
        self.status == ExecutionStatus::Success
    }

    /// Gets an output by key.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::ExecutionResult;
    /// use hexafn_run::ExecutionStatus;
    /// use std::collections::HashMap;
    /// use std::time::Duration;
    /// let mut outputs = HashMap::new();
    /// outputs.insert("foo".to_string(), "bar".to_string());
    /// let result = ExecutionResult::new(ExecutionStatus::Success, outputs, None, Duration::from_secs(0), 0);
    /// assert_eq!(result.get_output("foo"), Some(&"bar".to_string()));
    /// ```
    pub fn get_output(&self, key: &str) -> Option<&String> {
        self.outputs.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ExecutionStatus;
    use std::collections::HashMap;
    use std::time::Duration;

    #[test]
    fn test_new_success() {
        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), "42".to_string());
        let result = ExecutionResult::new(
            ExecutionStatus::Success,
            outputs.clone(),
            None,
            Duration::from_millis(10),
            1024,
        );
        assert_eq!(result.status, ExecutionStatus::Success);
        assert!(result.is_success());
        assert_eq!(result.get_output("result"), Some(&"42".to_string()));
        assert_eq!(result.outputs, outputs);
        assert_eq!(result.error, None);
        assert_eq!(result.duration, Duration::from_millis(10));
        assert_eq!(result.memory_used, 1024);
    }

    #[test]
    fn test_new_failure() {
        let result = ExecutionResult::new(
            ExecutionStatus::Failure,
            HashMap::new(),
            Some("fail".to_string()),
            Duration::from_secs(1),
            0,
        );
        assert_eq!(result.status, ExecutionStatus::Failure);
        assert!(!result.is_success());
        assert_eq!(result.error, Some("fail".to_string()));
    }

    #[test]
    fn test_get_output_none() {
        let result = ExecutionResult::new(
            ExecutionStatus::Success,
            HashMap::new(),
            None,
            Duration::from_secs(0),
            0,
        );
        assert_eq!(result.get_output("missing"), None);
    }
}
