// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Types Module
//!
//! Shared domain types and identifier management for hexaFn ecosystem.
//! Provides consistent type definitions across all modules.

pub mod identifiers;

// Re-exports for convenience
pub use identifiers::{CorrelationId, EventId, Timestamp, TraceId};

/// Common result type alias for type operations
pub type TypeResult<T> = Result<T, TypeError>;

/// Type validation error
#[derive(thiserror::Error, Debug, Clone)]
pub enum TypeError {
    #[error("Invalid identifier format: {value}")]
    InvalidFormat { value: String },

    #[error("Identifier too long: {length} (max: {max})")]
    TooLong { length: usize, max: usize },

    #[error("Identifier too short: {length} (min: {min})")]
    TooShort { length: usize, min: usize },

    #[error("Invalid timestamp: {reason}")]
    InvalidTimestamp { reason: String },
}
