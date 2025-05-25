// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Phase Execution Result
//! 
//! This module provides the PhaseResult enum for representing the outcome of phase execution.

/// Phase execution result
/// 
/// Represents the outcome of executing a single phase in the 6F Lifecycle Flow.
/// This enables proper error handling and flow control across the pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhaseResult {
    /// Phase completed successfully
    Success,
    /// Phase completed with warnings
    Warning(String),
    /// Phase failed with error
    Error(String),
    /// Phase was skipped
    Skipped(String),
}

impl PhaseResult {
    /// Check if the phase result indicates success
    ///
    /// Returns true for both Success and Warning states, as warnings
    /// don't prevent pipeline continuation.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::PhaseResult;
    ///
    /// assert!(PhaseResult::Success.is_success());
    /// assert!(PhaseResult::Warning("minor issue".to_string()).is_success());
    /// assert!(!PhaseResult::Error("failed".to_string()).is_success());
    /// assert!(!PhaseResult::Skipped("condition not met".to_string()).is_success());
    /// ```
    pub fn is_success(&self) -> bool {
        matches!(self, PhaseResult::Success | PhaseResult::Warning(_))
    }
    
    /// Check if the phase result indicates failure
    ///
    /// Returns true only for Error states, which should halt pipeline execution.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::PhaseResult;
    ///
    /// assert!(!PhaseResult::Success.is_failure());
    /// assert!(!PhaseResult::Warning("minor issue".to_string()).is_failure());
    /// assert!(PhaseResult::Error("failed".to_string()).is_failure());
    /// assert!(!PhaseResult::Skipped("condition not met".to_string()).is_failure());
    /// ```
    pub fn is_failure(&self) -> bool {
        matches!(self, PhaseResult::Error(_))
    }
    
    /// Check if the phase was skipped
    ///
    /// Returns true for Skipped states, which may or may not halt pipeline execution
    /// depending on the implementation strategy.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::PhaseResult;
    ///
    /// assert!(!PhaseResult::Success.is_skipped());
    /// assert!(!PhaseResult::Warning("minor issue".to_string()).is_skipped());
    /// assert!(!PhaseResult::Error("failed".to_string()).is_skipped());
    /// assert!(PhaseResult::Skipped("condition not met".to_string()).is_skipped());
    /// ```
    pub fn is_skipped(&self) -> bool {
        matches!(self, PhaseResult::Skipped(_))
    }
    
    /// Get the message associated with the result, if any
    ///
    /// Returns the message for Warning, Error, and Skipped states.
    /// Returns None for Success state.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::PhaseResult;
    ///
    /// assert_eq!(PhaseResult::Success.message(), None);
    /// assert_eq!(PhaseResult::Warning("issue".to_string()).message(), Some("issue"));
    /// assert_eq!(PhaseResult::Error("failed".to_string()).message(), Some("failed"));
    /// assert_eq!(PhaseResult::Skipped("skipped".to_string()).message(), Some("skipped"));
    /// ```
    pub fn message(&self) -> Option<&str> {
        match self {
            PhaseResult::Success => None,
            PhaseResult::Warning(msg) | PhaseResult::Error(msg) | PhaseResult::Skipped(msg) => Some(msg),
        }
    }
    
    /// Create a success result
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::PhaseResult;
    ///
    /// let result = PhaseResult::success();
    /// assert!(result.is_success());
    /// assert_eq!(result.message(), None);
    /// ```
    pub fn success() -> Self {
        PhaseResult::Success
    }
    
    /// Create a warning result with a message
    ///
    /// # Arguments
    ///
    /// * `message` - Warning message describing the issue
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::PhaseResult;
    ///
    /// let result = PhaseResult::warning("Data format deprecated");
    /// assert!(result.is_success());
    /// assert_eq!(result.message(), Some("Data format deprecated"));
    /// ```
    pub fn warning(message: impl Into<String>) -> Self {
        PhaseResult::Warning(message.into())
    }
    
    /// Create an error result with a message
    ///
    /// # Arguments
    ///
    /// * `message` - Error message describing the failure
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::PhaseResult;
    ///
    /// let result = PhaseResult::error("Validation failed");
    /// assert!(result.is_failure());
    /// assert_eq!(result.message(), Some("Validation failed"));
    /// ```
    pub fn error(message: impl Into<String>) -> Self {
        PhaseResult::Error(message.into())
    }
    
    /// Create a skipped result with a reason
    ///
    /// # Arguments
    ///
    /// * `reason` - Reason why the phase was skipped
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::PhaseResult;
    ///
    /// let result = PhaseResult::skipped("Condition not met");
    /// assert!(result.is_skipped());
    /// assert_eq!(result.message(), Some("Condition not met"));
    /// ```
    pub fn skipped(reason: impl Into<String>) -> Self {
        PhaseResult::Skipped(reason.into())
    }
    
    /// Map the result to another type while preserving the result state
    ///
    /// This is useful for transforming success values while keeping error states intact.
    ///
    /// # Arguments
    ///
    /// * `f` - Function to apply to success values
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::PhaseResult;
    ///
    /// let success = PhaseResult::success();
    /// let mapped = success.map(|_| "transformed");
    /// assert_eq!(mapped, PhaseResult::success());
    ///
    /// let error = PhaseResult::error("failed");
    /// let mapped = error.map(|_| "transformed");
    /// assert_eq!(mapped, PhaseResult::error("failed"));
    /// ```
    pub fn map<F>(self, _f: F) -> Self
    where
        F: FnOnce(()),
    {
        self
    }
    
    /// Combine this result with another result using AND logic
    ///
    /// Returns the first error/skip encountered, or success if both succeed.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::PhaseResult;
    ///
    /// let success1 = PhaseResult::success();
    /// let success2 = PhaseResult::success();
    /// assert_eq!(success1.and(success2), PhaseResult::success());
    ///
    /// let success = PhaseResult::success();
    /// let error = PhaseResult::error("failed");
    /// assert_eq!(success.and(error), PhaseResult::error("failed"));
    /// ```
    pub fn and(self, other: PhaseResult) -> PhaseResult {
        match self {
            PhaseResult::Success => other,
            PhaseResult::Warning(msg1) => match other {
                PhaseResult::Success => PhaseResult::Warning(msg1),
                PhaseResult::Warning(msg2) => PhaseResult::Warning(format!("{}, {}", msg1, msg2)),
                other => other, // Error or Skipped takes precedence
            },
            _ => self, // Error or Skipped
        }
    }
    
    /// Combine this result with another result using OR logic
    ///
    /// Returns the first success encountered, or the last error if both fail.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::PhaseResult;
    ///
    /// let error1 = PhaseResult::error("failed1");
    /// let error2 = PhaseResult::error("failed2");
    /// assert_eq!(error1.or(error2), PhaseResult::error("failed2"));
    ///
    /// let error = PhaseResult::error("failed");
    /// let success = PhaseResult::success();
    /// assert_eq!(error.or(success), PhaseResult::success());
    /// ```
    pub fn or(self, other: PhaseResult) -> PhaseResult {
        match self {
            PhaseResult::Success | PhaseResult::Warning(_) => self,
            _ => other, // Use other if self is Error or Skipped
        }
    }
    
    /// Check if pipeline should continue after this result
    ///
    /// Returns true for Success, Warning, and Skipped states.
    /// Returns false for Error states.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::PhaseResult;
    ///
    /// assert!(PhaseResult::success().should_continue());
    /// assert!(PhaseResult::warning("issue").should_continue());
    /// assert!(PhaseResult::skipped("condition").should_continue());
    /// assert!(!PhaseResult::error("failed").should_continue());
    /// ```
    pub fn should_continue(&self) -> bool {
        !self.is_failure()
    }
    
    /// Get the severity level of this result
    ///
    /// Returns:
    /// - 0 for Success
    /// - 1 for Warning  
    /// - 2 for Skipped
    /// - 3 for Error
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::PhaseResult;
    ///
    /// assert_eq!(PhaseResult::success().severity_level(), 0);
    /// assert_eq!(PhaseResult::warning("issue").severity_level(), 1);
    /// assert_eq!(PhaseResult::skipped("condition").severity_level(), 2);
    /// assert_eq!(PhaseResult::error("failed").severity_level(), 3);
    /// ```
    pub fn severity_level(&self) -> u8 {
        match self {
            PhaseResult::Success => 0,
            PhaseResult::Warning(_) => 1,
            PhaseResult::Skipped(_) => 2,
            PhaseResult::Error(_) => 3,
        }
    }
}

impl std::fmt::Display for PhaseResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhaseResult::Success => write!(f, "Success"),
            PhaseResult::Warning(msg) => write!(f, "Warning: {}", msg),
            PhaseResult::Error(msg) => write!(f, "Error: {}", msg),
            PhaseResult::Skipped(msg) => write!(f, "Skipped: {}", msg),
        }
    }
}

impl From<Result<(), String>> for PhaseResult {
    fn from(result: Result<(), String>) -> Self {
        match result {
            Ok(()) => PhaseResult::Success,
            Err(msg) => PhaseResult::Error(msg),
        }
    }
}

impl From<PhaseResult> for Result<(), String> {
    fn from(result: PhaseResult) -> Self {
        match result {
            PhaseResult::Success => Ok(()),
            PhaseResult::Warning(_) => Ok(()),
            PhaseResult::Error(msg) => Err(msg),
            PhaseResult::Skipped(msg) => Err(format!("Skipped: {}", msg)),
        }
    }
}

/// Macro for creating phase results with context
///
/// # Examples
///
/// ```
/// use hexafn_core::phases_result;
///
/// let success = phases_result!(success);
/// let warning = phases_result!(warning, "Data format deprecated");
/// let error = phases_result!(error, "Validation failed: {}", "invalid input");
/// let skipped = phases_result!(skipped, "Condition not met");
/// ```
#[macro_export]
macro_rules! phases_result {
    (success) => {
        $crate::phases::PhaseResult::success()
    };
    (warning, $msg:expr) => {
        $crate::phases::PhaseResult::warning($msg)
    };
    (warning, $fmt:expr, $($arg:tt)*) => {
        $crate::phases::PhaseResult::warning(format!($fmt, $($arg)*))
    };
    (error, $msg:expr) => {
        $crate::phases::PhaseResult::error($msg)
    };
    (error, $fmt:expr, $($arg:tt)*) => {
        $crate::phases::PhaseResult::error(format!($fmt, $($arg)*))
    };
    (skipped, $msg:expr) => {
        $crate::phases::PhaseResult::skipped($msg)
    };
    (skipped, $fmt:expr, $($arg:tt)*) => {
        $crate::phases::PhaseResult::skipped(format!($fmt, $($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_phase_result_creation() {
        let success = PhaseResult::success();
        assert!(success.is_success());
        assert!(!success.is_failure());
        assert!(!success.is_skipped());
        assert_eq!(success.message(), None);
        
        let warning = PhaseResult::warning("test warning");
        assert!(warning.is_success());
        assert!(!warning.is_failure());
        assert!(!warning.is_skipped());
        assert_eq!(warning.message(), Some("test warning"));
        
        let error = PhaseResult::error("test error");
        assert!(!error.is_success());
        assert!(error.is_failure());
        assert!(!error.is_skipped());
        assert_eq!(error.message(), Some("test error"));
        
        let skipped = PhaseResult::skipped("test skip");
        assert!(!skipped.is_success());
        assert!(!skipped.is_failure());
        assert!(skipped.is_skipped());
        assert_eq!(skipped.message(), Some("test skip"));
    }
    
    #[test]
    fn test_phase_result_display() {
        assert_eq!(format!("{}", PhaseResult::Success), "Success");
        assert_eq!(format!("{}", PhaseResult::Warning("warn".to_string())), "Warning: warn");
        assert_eq!(format!("{}", PhaseResult::Error("err".to_string())), "Error: err");
        assert_eq!(format!("{}", PhaseResult::Skipped("skip".to_string())), "Skipped: skip");
    }
    
    #[test]
    fn test_phase_result_and_logic() {
        let success1 = PhaseResult::success();
        let success2 = PhaseResult::success();
        assert_eq!(success1.and(success2), PhaseResult::success());
        
        let success = PhaseResult::success();
        let warning = PhaseResult::warning("warn");
        assert_eq!(success.and(warning), PhaseResult::warning("warn"));
        
        let warning1 = PhaseResult::warning("warn1");
        let warning2 = PhaseResult::warning("warn2");
        assert_eq!(warning1.and(warning2), PhaseResult::warning("warn1, warn2"));
        
        let success = PhaseResult::success();
        let error = PhaseResult::error("err");
        assert_eq!(success.and(error), PhaseResult::error("err"));
        
        let error = PhaseResult::error("err");
        let success = PhaseResult::success();
        assert_eq!(error.and(success), PhaseResult::error("err"));
    }
    
    #[test]
    fn test_phase_result_or_logic() {
        let error1 = PhaseResult::error("err1");
        let error2 = PhaseResult::error("err2");
        assert_eq!(error1.or(error2), PhaseResult::error("err2"));
        
        let error = PhaseResult::error("err");
        let success = PhaseResult::success();
        assert_eq!(error.or(success), PhaseResult::success());
        
        let success = PhaseResult::success();
        let error = PhaseResult::error("err");
        assert_eq!(success.or(error), PhaseResult::success());
        
        let warning = PhaseResult::warning("warn");
        let error = PhaseResult::error("err");
        assert_eq!(warning.or(error), PhaseResult::warning("warn"));
    }
    
    #[test]
    fn test_should_continue() {
        assert!(PhaseResult::success().should_continue());
        assert!(PhaseResult::warning("warn").should_continue());
        assert!(PhaseResult::skipped("skip").should_continue());
        assert!(!PhaseResult::error("err").should_continue());
    }
    
    #[test]
    fn test_severity_level() {
        assert_eq!(PhaseResult::success().severity_level(), 0);
        assert_eq!(PhaseResult::warning("warn").severity_level(), 1);
        assert_eq!(PhaseResult::skipped("skip").severity_level(), 2);
        assert_eq!(PhaseResult::error("err").severity_level(), 3);
    }
    
    #[test]
    fn test_result_conversion() {
        // From Result to PhaseResult
        let ok_result: Result<(), String> = Ok(());
        let phase_result = PhaseResult::from(ok_result);
        assert_eq!(phase_result, PhaseResult::Success);
        
        let err_result: Result<(), String> = Err("error".to_string());
        let phase_result = PhaseResult::from(err_result);
        assert_eq!(phase_result, PhaseResult::Error("error".to_string()));
        
        // From PhaseResult to Result
        let success = PhaseResult::success();
        let result: Result<(), String> = success.into();
        assert!(result.is_ok());
        
        let warning = PhaseResult::warning("warn");
        let result: Result<(), String> = warning.into();
        assert!(result.is_ok());
        
        let error = PhaseResult::error("err");
        let result: Result<(), String> = error.into();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "err");
        
        let skipped = PhaseResult::skipped("skip");
        let result: Result<(), String> = skipped.into();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Skipped: skip");
    }
    
    #[test]
    fn test_phases_result_macro() {
        let success = phases_result!(success);
        assert_eq!(success, PhaseResult::Success);
        
        let warning = phases_result!(warning, "test warning");
        assert_eq!(warning, PhaseResult::Warning("test warning".to_string()));
        
        let formatted_warning = phases_result!(warning, "formatted {}", "warning");
        assert_eq!(formatted_warning, PhaseResult::Warning("formatted warning".to_string()));
        
        let error = phases_result!(error, "test error");
        assert_eq!(error, PhaseResult::Error("test error".to_string()));
        
        let formatted_error = phases_result!(error, "error code: {}", 404);
        assert_eq!(formatted_error, PhaseResult::Error("error code: 404".to_string()));
        
        let skipped = phases_result!(skipped, "test skip");
        assert_eq!(skipped, PhaseResult::Skipped("test skip".to_string()));
        
        let formatted_skip = phases_result!(skipped, "skipped due to {}", "condition");
        assert_eq!(formatted_skip, PhaseResult::Skipped("skipped due to condition".to_string()));
    }
}