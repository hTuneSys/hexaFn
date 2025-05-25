// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # hexaFn Core
//!
//! Core architecture and shared components for the hexaFn ecosystem.
//! Provides the foundation for the 6F Lifecycle Flow and Hexagonal Architecture.

pub mod errors;
pub mod phases;
pub mod types;

// Re-exports for convenience
pub use errors::{HexaError, PipelineError};
pub use phases::{FEED, FEEDBACK, FILTER, FORMAT, FORWARD, FUNCTION};
pub use types::{CorrelationId, EventId, Timestamp, TraceId};
