// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! ExecutionStatus for hexaFn
//!
//! Represents the result of a function execution, such as success, failure, timeout, or cancellation.
//!
//! # Example
//!
//! ```
//! use hexafn_run::ExecutionStatus;
//!
//! let status = ExecutionStatus::Success;
//! assert_eq!(status, ExecutionStatus::Success);
//! ```
//!
//! # Test
//!
//! See module tests for more examples.

/// Represents the result of a function execution.
///
/// # Example
/// ```
/// use hexafn_run::ExecutionStatus;
/// let status = ExecutionStatus::Failure;
/// assert_eq!(status, ExecutionStatus::Failure);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionStatus {
    /// The execution completed successfully
    Success,
    /// The execution failed due to an error
    Failure,
    /// The execution timed out
    Timeout,
    /// The execution was cancelled by the user
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_variant() {
        let status = ExecutionStatus::Success;
        assert_eq!(status, ExecutionStatus::Success);
    }

    #[test]
    fn test_failure_variant() {
        let status = ExecutionStatus::Failure;
        assert_eq!(status, ExecutionStatus::Failure);
    }

    #[test]
    fn test_timeout_variant() {
        let status = ExecutionStatus::Timeout;
        assert_eq!(status, ExecutionStatus::Timeout);
    }

    #[test]
    fn test_cancelled_variant() {
        let status = ExecutionStatus::Cancelled;
        assert_eq!(status, ExecutionStatus::Cancelled);
    }
}
