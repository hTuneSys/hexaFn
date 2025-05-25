// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Pipeline Error Types
//!
//! Error definitions for 6F Lifecycle Flow pipeline operations.

use super::HexaError;
use thiserror::Error;

/// Comprehensive error type for 6F Lifecycle Flow phases
#[derive(Error, Debug, Clone)]
pub enum PipelineError {
    /// Feed phase errors - data ingestion failures
    #[error("Feed phase failed: {message}")]
    FeedFailed {
        message: String,
        source_info: Option<String>,
        correlation_id: Option<String>,
    },

    /// Filter phase errors - validation and gating failures
    #[error("Filter phase failed: {message}")]
    FilterFailed {
        message: String,
        predicate: String,
        correlation_id: Option<String>,
    },

    /// Format phase errors - transformation and formatting failures
    #[error("Format phase failed: {message}")]
    FormatFailed {
        message: String,
        input_type: String,
        output_type: String,
        correlation_id: Option<String>,
    },

    /// Function phase errors - execution and logic failures
    #[error("Function phase failed: {message}")]
    FunctionFailed {
        message: String,
        function_name: String,
        context: Option<String>,
        correlation_id: Option<String>,
    },

    /// Forward phase errors - output routing failures
    #[error("Forward phase failed: {message}")]
    ForwardFailed {
        message: String,
        target: String,
        retry_count: Option<u32>,
        correlation_id: Option<String>,
    },

    /// Feedback phase errors - observability and logging failures
    #[error("Feedback phase failed: {message}")]
    FeedbackFailed {
        message: String,
        observer_type: String,
        correlation_id: Option<String>,
    },

    /// Pipeline configuration errors
    #[error("Pipeline configuration error: {config}")]
    Configuration {
        config: String,
        reason: String,
        correlation_id: Option<String>,
    },

    /// Pipeline validation errors
    #[error("Pipeline validation error: {validation}")]
    Validation {
        validation: String,
        phase: Option<String>,
        correlation_id: Option<String>,
    },

    /// Pipeline timeout errors
    #[error("Pipeline timeout after {duration_ms}ms")]
    Timeout {
        duration_ms: u64,
        phase: Option<String>,
        correlation_id: Option<String>,
    },

    /// Generic phase error for extensibility
    #[error("Phase error in {phase}: {message}")]
    PhaseError {
        phase: String,
        message: String,
        error_code: String,
        correlation_id: Option<String>,
    },
}

impl PipelineError {
    /// Create a Feed phase error with basic context
    ///
    /// This constructor creates a simple Feed phase error for data ingestion failures.
    /// Use this for basic error cases where only an error message is needed.
    ///
    /// # Arguments
    ///
    /// * `message` - Human-readable error description
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_core::errors::PipelineError;
    ///
    /// let error = PipelineError::feed_error("Database connection failed");
    /// assert_eq!(error.phase_name(), "feed");
    /// assert_eq!(error.error_code(), "FEED_FAILED");
    /// ```
    ///
    /// # Use Cases
    ///
    /// - Database connection failures
    /// - File read errors
    /// - Network timeouts during data fetching
    /// - Invalid data source configurations
    pub fn feed_error(message: impl Into<String>) -> Self {
        Self::FeedFailed {
            message: message.into(),
            source_info: None,
            correlation_id: None,
        }
    }

    /// Create a Feed phase error with comprehensive context
    ///
    /// This constructor creates a detailed Feed phase error including source information
    /// and correlation ID for enhanced observability and debugging.
    ///
    /// # Arguments
    ///
    /// * `message` - Human-readable error description
    /// * `source_info` - Optional information about the data source (URL, file path, etc.)
    /// * `correlation_id` - Optional trace ID for distributed system tracking
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_core::errors::PipelineError;
    ///
    /// let error = PipelineError::feed_error_with_context(
    ///     "Connection timeout",
    ///     Some("https://api.example.com/data".to_string()),
    ///     Some("trace-123-456".to_string())
    /// );
    /// assert_eq!(error.correlation_id(), Some("trace-123-456"));
    /// ```
    ///
    /// # Use Cases
    ///
    /// - External API integration failures
    /// - Webhook ingestion errors with source tracking
    /// - Distributed system event processing with tracing
    /// - Multi-source data aggregation scenarios
    pub fn feed_error_with_context(
        message: impl Into<String>,
        source_info: Option<String>,
        correlation_id: Option<String>,
    ) -> Self {
        Self::FeedFailed {
            message: message.into(),
            source_info,
            correlation_id,
        }
    }

    /// Create a Filter phase error for validation and gating failures
    ///
    /// This constructor creates errors for the Filter phase when data fails
    /// validation rules, business constraints, or gating conditions.
    ///
    /// # Arguments
    ///
    /// * `message` - Human-readable error description
    /// * `predicate` - Name or description of the failed filter/predicate
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_core::errors::PipelineError;
    ///
    /// let error = PipelineError::filter_error(
    ///     "Age validation failed: must be >= 18",
    ///     "age_check"
    /// );
    /// assert_eq!(error.phase_name(), "filter");
    /// ```
    ///
    /// # Use Cases
    ///
    /// - Schema validation failures
    /// - Business rule violations
    /// - Data quality checks
    /// - Access control and authorization checks
    /// - Rate limiting and throttling
    pub fn filter_error(message: impl Into<String>, predicate: impl Into<String>) -> Self {
        Self::FilterFailed {
            message: message.into(),
            predicate: predicate.into(),
            correlation_id: None,
        }
    }

    /// Create a Format phase error for transformation failures
    ///
    /// This constructor creates errors for the Format phase when data transformation,
    /// serialization, or format conversion operations fail.
    ///
    /// # Arguments
    ///
    /// * `message` - Human-readable error description
    /// * `input_type` - Type or format of the input data
    /// * `output_type` - Expected type or format of the output data
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_core::errors::PipelineError;
    ///
    /// let error = PipelineError::format_error(
    ///     "JSON parsing failed: invalid syntax",
    ///     "application/json",
    ///     "UserProfile"
    /// );
    /// assert_eq!(error.error_code(), "FORMAT_FAILED");
    /// ```
    ///
    /// # Use Cases
    ///
    /// - JSON/XML parsing errors
    /// - Type conversion failures
    /// - Serialization/deserialization issues
    /// - Data structure mapping errors
    /// - Character encoding problems
    pub fn format_error(
        message: impl Into<String>,
        input_type: impl Into<String>,
        output_type: impl Into<String>,
    ) -> Self {
        Self::FormatFailed {
            message: message.into(),
            input_type: input_type.into(),
            output_type: output_type.into(),
            correlation_id: None,
        }
    }

    /// Create a Function phase error for execution failures
    ///
    /// This constructor creates errors for the Function phase when user-defined
    /// functions, business logic, or computational operations fail.
    ///
    /// # Arguments
    ///
    /// * `message` - Human-readable error description
    /// * `function_name` - Name or identifier of the failed function
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_core::errors::PipelineError;
    ///
    /// let error = PipelineError::function_error(
    ///     "Division by zero in calculation",
    ///     "calculate_average"
    /// );
    /// assert_eq!(error.phase_name(), "function");
    /// assert!(!error.is_recoverable()); // Function errors are not recoverable
    /// ```
    ///
    /// # Use Cases
    ///
    /// - Runtime exceptions in user functions
    /// - Mathematical computation errors
    /// - Business logic violations
    /// - Script execution failures (WASM, JS, Lua)
    /// - Algorithm processing errors
    pub fn function_error(message: impl Into<String>, function_name: impl Into<String>) -> Self {
        Self::FunctionFailed {
            message: message.into(),
            function_name: function_name.into(),
            context: None,
            correlation_id: None,
        }
    }

    /// Create a Forward phase error for output routing failures
    ///
    /// This constructor creates errors for the Forward phase when results
    /// cannot be delivered to their intended destinations.
    ///
    /// # Arguments
    ///
    /// * `message` - Human-readable error description
    /// * `target` - Destination or target identifier where delivery failed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_core::errors::PipelineError;
    ///
    /// let error = PipelineError::forward_error(
    ///     "Webhook delivery timeout",
    ///     "https://api.customer.com/webhook"
    /// );
    /// assert!(error.is_recoverable()); // Forward errors can be retried
    /// ```
    ///
    /// # Use Cases
    ///
    /// - Webhook delivery failures
    /// - Message queue publishing errors
    /// - Database write failures
    /// - File system write errors
    /// - External API call failures
    pub fn forward_error(message: impl Into<String>, target: impl Into<String>) -> Self {
        Self::ForwardFailed {
            message: message.into(),
            target: target.into(),
            retry_count: None,
            correlation_id: None,
        }
    }

    /// Create a Feedback phase error for observability failures
    ///
    /// This constructor creates errors for the Feedback phase when logging,
    /// monitoring, or observability operations fail.
    ///
    /// # Arguments
    ///
    /// * `message` - Human-readable error description
    /// * `observer_type` - Type or name of the observer that failed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_core::errors::PipelineError;
    ///
    /// let error = PipelineError::feedback_error(
    ///     "Failed to write to log file",
    ///     "file_logger"
    /// );
    /// assert!(error.is_recoverable()); // Feedback errors are often recoverable
    /// ```
    ///
    /// # Use Cases
    ///
    /// - Log file write failures
    /// - Metrics collection errors
    /// - Tracing system failures
    /// - Audit trail recording issues
    /// - Monitoring system connectivity problems
    pub fn feedback_error(message: impl Into<String>, observer_type: impl Into<String>) -> Self {
        Self::FeedbackFailed {
            message: message.into(),
            observer_type: observer_type.into(),
            correlation_id: None,
        }
    }

    /// Add correlation ID to any existing error for enhanced tracing
    ///
    /// This method enables distributed tracing by attaching a correlation ID
    /// to any pipeline error after creation. This is essential for tracking
    /// errors across service boundaries in the 6F Lifecycle Flow.
    ///
    /// # Arguments
    ///
    /// * `correlation_id` - Unique identifier for tracing this error across systems
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_core::errors::PipelineError;
    ///
    /// let error = PipelineError::feed_error("Connection failed")
    ///     .with_correlation_id("req-abc-123");
    ///
    /// assert_eq!(error.correlation_id(), Some("req-abc-123"));
    /// ```
    ///
    /// # Use Cases
    ///
    /// - Distributed system error tracking
    /// - Cross-service request correlation
    /// - Debugging pipeline flows
    /// - Audit trail maintenance
    /// - Performance monitoring integration
    ///
    /// # Integration with HexaWatch
    ///
    /// This correlation ID integrates seamlessly with the HexaWatch observability
    /// module for comprehensive system monitoring and debugging.
    pub fn with_correlation_id(mut self, correlation_id: impl Into<String>) -> Self {
        let id = Some(correlation_id.into());
        match &mut self {
            Self::FeedFailed {
                correlation_id: cid,
                ..
            } => *cid = id,
            Self::FilterFailed {
                correlation_id: cid,
                ..
            } => *cid = id,
            Self::FormatFailed {
                correlation_id: cid,
                ..
            } => *cid = id,
            Self::FunctionFailed {
                correlation_id: cid,
                ..
            } => *cid = id,
            Self::ForwardFailed {
                correlation_id: cid,
                ..
            } => *cid = id,
            Self::FeedbackFailed {
                correlation_id: cid,
                ..
            } => *cid = id,
            Self::Configuration {
                correlation_id: cid,
                ..
            } => *cid = id,
            Self::Validation {
                correlation_id: cid,
                ..
            } => *cid = id,
            Self::Timeout {
                correlation_id: cid,
                ..
            } => *cid = id,
            Self::PhaseError {
                correlation_id: cid,
                ..
            } => *cid = id,
        }
        self
    }

    /// Get the phase name where the error occurred
    ///
    /// Returns the 6F Lifecycle Flow phase name where this error originated.
    /// This enables phase-specific error handling and debugging strategies.
    ///
    /// # Returns
    ///
    /// A static string representing the 6F phase name:
    /// - `"feed"` - Data ingestion phase
    /// - `"filter"` - Validation and gating phase  
    /// - `"format"` - Transformation phase
    /// - `"function"` - Execution phase
    /// - `"forward"` - Output routing phase
    /// - `"feedback"` - Observability phase
    /// - `"configuration"` - System configuration issues
    /// - `"validation"` - Pipeline validation issues
    /// - `"timeout"` - Timeout-related issues
    /// - `"phase_error"` - Generic phase issues
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_core::errors::PipelineError;
    ///
    /// let feed_error = PipelineError::feed_error("Database unavailable");
    /// assert_eq!(feed_error.phase_name(), "feed");
    ///
    /// let filter_error = PipelineError::filter_error("Invalid data", "schema_check");
    /// assert_eq!(filter_error.phase_name(), "filter");
    /// ```
    ///
    /// # Use Cases
    ///
    /// - Phase-specific error recovery strategies
    /// - Error metrics aggregation by phase
    /// - Debugging pipeline flow issues
    /// - Custom error handling per phase
    pub fn phase_name(&self) -> &'static str {
        match self {
            Self::FeedFailed { .. } => crate::phases::FEED,
            Self::FilterFailed { .. } => crate::phases::FILTER,
            Self::FormatFailed { .. } => crate::phases::FORMAT,
            Self::FunctionFailed { .. } => crate::phases::FUNCTION,
            Self::ForwardFailed { .. } => crate::phases::FORWARD,
            Self::FeedbackFailed { .. } => crate::phases::FEEDBACK,
            Self::Configuration { .. } => "configuration",
            Self::Validation { .. } => "validation",
            Self::Timeout { .. } => "timeout",
            Self::PhaseError { .. } => "phase_error",
        }
    }

    /// Check if error is recoverable for retry logic
    ///
    /// Determines whether this error represents a recoverable condition that
    /// can benefit from retry strategies. This is crucial for building resilient
    /// pipeline systems that can handle transient failures.
    ///
    /// # Returns
    ///
    /// `true` if the error is recoverable and retry might succeed, `false` otherwise.
    ///
    /// # Recoverable Errors
    ///
    /// - **Forward failures**: Network issues, temporary service unavailability
    /// - **Feedback failures**: Logging system temporary issues
    /// - **Timeouts**: Can often be resolved with retry and backoff
    ///
    /// # Non-Recoverable Errors
    ///
    /// - **Feed failures**: Usually indicate configuration or access issues
    /// - **Filter failures**: Data validation issues that won't change on retry
    /// - **Format failures**: Type conversion or parsing issues
    /// - **Function failures**: Logic errors or runtime exceptions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_core::errors::PipelineError;
    ///
    /// // Recoverable errors
    /// let forward_error = PipelineError::forward_error("Network timeout", "api.example.com");
    /// assert!(forward_error.is_recoverable());
    ///
    /// let feedback_error = PipelineError::feedback_error("Log write failed", "file_logger");
    /// assert!(feedback_error.is_recoverable());
    ///
    /// // Non-recoverable errors
    /// let function_error = PipelineError::function_error("Division by zero", "calculator");
    /// assert!(!function_error.is_recoverable());
    ///
    /// let filter_error = PipelineError::filter_error("Invalid schema", "json_validator");
    /// assert!(!filter_error.is_recoverable());
    /// ```
    ///
    /// # Use Cases
    ///
    /// - Automatic retry logic implementation
    /// - Circuit breaker pattern decisions
    /// - Error handling strategy selection
    /// - Alerting and monitoring decisions
    /// - SLA and reliability calculations
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::ForwardFailed { .. } | Self::FeedbackFailed { .. } | Self::Timeout { .. }
        )
    }
}

impl HexaError for PipelineError {
    fn error_code(&self) -> &'static str {
        match self {
            Self::FeedFailed { .. } => "FEED_FAILED",
            Self::FilterFailed { .. } => "FILTER_FAILED",
            Self::FormatFailed { .. } => "FORMAT_FAILED",
            Self::FunctionFailed { .. } => "FUNCTION_FAILED",
            Self::ForwardFailed { .. } => "FORWARD_FAILED",
            Self::FeedbackFailed { .. } => "FEEDBACK_FAILED",
            Self::Configuration { .. } => "CONFIGURATION_ERROR",
            Self::Validation { .. } => "VALIDATION_ERROR",
            Self::Timeout { .. } => "TIMEOUT_ERROR",
            Self::PhaseError { .. } => "PHASE_ERROR",
        }
    }

    fn error_category(&self) -> &'static str {
        match self {
            Self::FeedFailed { .. }
            | Self::FilterFailed { .. }
            | Self::FormatFailed { .. }
            | Self::FunctionFailed { .. }
            | Self::ForwardFailed { .. }
            | Self::FeedbackFailed { .. } => "pipeline_execution",
            Self::Configuration { .. } => "configuration",
            Self::Validation { .. } => "validation",
            Self::Timeout { .. } => "timeout",
            Self::PhaseError { .. } => "phase_specific",
        }
    }

    fn is_recoverable(&self) -> bool {
        PipelineError::is_recoverable(self)
    }

    fn correlation_id(&self) -> Option<&str> {
        match self {
            Self::FeedFailed { correlation_id, .. }
            | Self::FilterFailed { correlation_id, .. }
            | Self::FormatFailed { correlation_id, .. }
            | Self::FunctionFailed { correlation_id, .. }
            | Self::ForwardFailed { correlation_id, .. }
            | Self::FeedbackFailed { correlation_id, .. }
            | Self::Configuration { correlation_id, .. }
            | Self::Validation { correlation_id, .. }
            | Self::Timeout { correlation_id, .. }
            | Self::PhaseError { correlation_id, .. } => correlation_id.as_deref(),
        }
    }
}

/// Specific phase error for extensible error handling
///
/// This struct provides a flexible way to create custom phase-specific errors
/// that don't fit into the standard 6F Lifecycle Flow categories. It enables
/// extensibility while maintaining consistency with the error handling system.
///
/// # Examples
///
/// ```rust
/// use hexafn_core::errors::PhaseError;
///
/// let custom_error = PhaseError::new(
///     "custom_validation",
///     "Field 'email' format is invalid",
///     "VALIDATION_001"
/// ).with_correlation_id("trace-123");
///
/// assert_eq!(custom_error.phase, "custom_validation");
/// assert_eq!(custom_error.error_code(), "PHASE_ERROR");
/// ```
#[derive(Error, Debug, Clone)]
#[error("Phase {phase} error: {message}")]
pub struct PhaseError {
    pub phase: String,
    pub message: String,
    pub error_code: String,
    pub correlation_id: Option<String>,
}

impl PhaseError {
    /// Create a new phase-specific error
    ///
    /// This constructor creates a custom phase error with detailed context
    /// for scenarios that don't fit standard 6F Lifecycle Flow categories.
    ///
    /// # Arguments
    ///
    /// * `phase` - Name of the phase where the error occurred
    /// * `message` - Human-readable error description
    /// * `error_code` - Machine-readable error identifier
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_core::errors::PhaseError;
    ///
    /// let error = PhaseError::new(
    ///     "data_cleanup",
    ///     "Temporary files could not be removed",
    ///     "CLEANUP_001"
    /// );
    ///
    /// assert_eq!(error.phase, "data_cleanup");
    /// assert_eq!(error.message, "Temporary files could not be removed");
    /// ```
    pub fn new(
        phase: impl Into<String>,
        message: impl Into<String>,
        error_code: impl Into<String>,
    ) -> Self {
        Self {
            phase: phase.into(),
            message: message.into(),
            error_code: error_code.into(),
            correlation_id: None,
        }
    }

    /// Add correlation ID for tracing
    ///
    /// Associates a correlation ID with this phase error to enable
    /// distributed tracing and cross-system error tracking.
    ///
    /// # Arguments
    ///
    /// * `correlation_id` - Unique identifier for tracing
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_core::errors::PhaseError;
    ///
    /// let error = PhaseError::new("auth", "Token expired", "AUTH_001")
    ///     .with_correlation_id("request-456");
    ///
    /// assert_eq!(error.correlation_id(), Some("request-456"));
    /// ```
    pub fn with_correlation_id(mut self, correlation_id: impl Into<String>) -> Self {
        self.correlation_id = Some(correlation_id.into());
        self
    }
}

impl HexaError for PhaseError {
    fn error_code(&self) -> &'static str {
        "PHASE_ERROR"
    }

    fn error_category(&self) -> &'static str {
        "phase_specific"
    }

    fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_error_creation() {
        let error = PipelineError::feed_error("Data source unavailable");
        assert_eq!(error.phase_name(), "feed");
        assert_eq!(error.error_code(), "FEED_FAILED");
        assert_eq!(error.error_category(), "pipeline_execution");
    }

    #[test]
    fn test_error_with_correlation_id() {
        let error = PipelineError::filter_error("Invalid input", "size_check")
            .with_correlation_id("trace-123");

        assert_eq!(error.correlation_id(), Some("trace-123"));
        assert_eq!(error.phase_name(), "filter");
    }

    #[test]
    fn test_recoverable_errors() {
        assert!(PipelineError::forward_error("Network timeout", "webhook").is_recoverable());
        assert!(PipelineError::feedback_error("Log write failed", "file_logger").is_recoverable());
        assert!(!PipelineError::function_error("Runtime panic", "user_fn").is_recoverable());
    }

    #[test]
    fn test_phase_error() {
        let phase_error = PhaseError::new("custom_phase", "Custom error", "CUSTOM_001")
            .with_correlation_id("trace-456");

        assert_eq!(phase_error.phase, "custom_phase");
        assert_eq!(phase_error.error_code(), "PHASE_ERROR");
        assert_eq!(phase_error.correlation_id(), Some("trace-456"));
    }

    #[test]
    fn test_feed_error_with_context() {
        let error = PipelineError::feed_error_with_context(
            "API connection failed",
            Some("https://api.example.com/v1/data".to_string()),
            Some("trace-789".to_string()),
        );

        assert_eq!(error.phase_name(), "feed");
        assert_eq!(error.correlation_id(), Some("trace-789"));
        assert_eq!(error.error_category(), "pipeline_execution");
    }

    #[test]
    fn test_error_recovery_classification() {
        // Test all recoverable error types
        let recoverable_errors = vec![
            PipelineError::forward_error("Network issue", "api.example.com"),
            PipelineError::feedback_error("Log unavailable", "syslog"),
            PipelineError::Timeout {
                duration_ms: 5000,
                phase: Some("function".to_string()),
                correlation_id: None,
            },
        ];

        for error in recoverable_errors {
            assert!(
                error.is_recoverable(),
                "Error should be recoverable: {:?}",
                error
            );
        }

        // Test all non-recoverable error types
        let non_recoverable_errors = vec![
            PipelineError::feed_error("Invalid credentials"),
            PipelineError::filter_error("Schema violation", "json_schema"),
            PipelineError::format_error("Parse error", "json", "struct"),
            PipelineError::function_error("Runtime panic", "user_function"),
        ];

        for error in non_recoverable_errors {
            assert!(
                !error.is_recoverable(),
                "Error should not be recoverable: {:?}",
                error
            );
        }
    }

    #[test]
    fn test_error_phase_mapping() {
        let test_cases = vec![
            (PipelineError::feed_error("test"), "feed"),
            (PipelineError::filter_error("test", "predicate"), "filter"),
            (
                PipelineError::format_error("test", "input", "output"),
                "format",
            ),
            (PipelineError::function_error("test", "func"), "function"),
            (PipelineError::forward_error("test", "target"), "forward"),
            (
                PipelineError::feedback_error("test", "observer"),
                "feedback",
            ),
        ];

        for (error, expected_phase) in test_cases {
            assert_eq!(error.phase_name(), expected_phase);
        }
    }

    #[test]
    fn test_correlation_id_propagation() {
        let correlation_id = "test-correlation-123";

        let errors = vec![
            PipelineError::feed_error("test").with_correlation_id(correlation_id),
            PipelineError::filter_error("test", "pred").with_correlation_id(correlation_id),
            PipelineError::format_error("test", "in", "out").with_correlation_id(correlation_id),
            PipelineError::function_error("test", "func").with_correlation_id(correlation_id),
            PipelineError::forward_error("test", "target").with_correlation_id(correlation_id),
            PipelineError::feedback_error("test", "obs").with_correlation_id(correlation_id),
        ];

        for error in errors {
            assert_eq!(error.correlation_id(), Some(correlation_id));
        }
    }
}
