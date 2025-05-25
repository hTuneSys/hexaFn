// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Error Handling Module
//!
//! Provides unified error handling for the hexaFn ecosystem.
//! Implements structured error hierarchy for 6F Lifecycle Flow phases.

pub mod pipeline_error;

// Re-exports for convenience
pub use pipeline_error::{PhaseError, PipelineError};

/// Standard Result type for hexaFn core operations
pub type CoreResult<T> = Result<T, PipelineError>;

/// Core error trait for all hexaFn errors
pub trait HexaError: std::error::Error + Send + Sync + 'static {
    /// Error code for programmatic handling
    fn error_code(&self) -> &'static str;

    /// Human-readable error category
    fn error_category(&self) -> &'static str;

    /// Whether this error is recoverable
    fn is_recoverable(&self) -> bool {
        false
    }

    /// Correlation ID for tracing
    fn correlation_id(&self) -> Option<&str> {
        None
    }
}
