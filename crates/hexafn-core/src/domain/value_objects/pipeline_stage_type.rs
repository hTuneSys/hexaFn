// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # PipelineStageType Enum
//!
//! This module defines the [`PipelineStageType`] enum, which represents the type of each stage in the 6F Lifecycle Flow
//! for the hexaFn pipeline. Each variant corresponds to a distinct phase in the pipeline execution, supporting
//! DDD and Hexagonal Architecture principles.
//!
//! ## Usage Example
//!
//! ```rust
//! use hexafn_core::PipelineStageType;
//!
//! let stage_type = PipelineStageType::Feed;
//! assert_eq!(stage_type, PipelineStageType::Feed);
//! ```
//!
//! ## Serialization Example
//!
//! ```rust
//! use hexafn_core::PipelineStageType;
//! use serde_json;
//!
//! let stage = PipelineStageType::Function;
//! let json = serde_json::to_string(&stage).unwrap();
//! let deserialized: PipelineStageType = serde_json::from_str(&json).unwrap();
//! assert_eq!(stage, deserialized);
//! ```

use serde::{Deserialize, Serialize};

/// Represents the type of pipeline stage in the 6F Lifecycle Flow.
///
/// Each stage type corresponds to a specific phase in the hexaFn pipeline execution:
/// - **Feed**: Ingest data from external sources (events, APIs, queues)
/// - **Filter**: Apply pre-condition checks and gating logic
/// - **Format**: Normalize, transform, and validate data
/// - **Function**: Execute business logic with user-defined behavior
/// - **Forward**: Route results to KV stores, topics, or external services
/// - **Feedback**: Log, trace, trigger, or audit pipeline execution
///
/// # Example
/// ```
/// use hexafn_core::PipelineStageType;
/// let stage_type = PipelineStageType::Feed;
/// assert_eq!(stage_type, PipelineStageType::Feed);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PipelineStageType {
    /// Feed stage: Ingest data from external sources
    Feed,
    /// Filter stage: Apply pre-condition checks and gating
    Filter,
    /// Format stage: Normalize, transform, and validate data
    Format,
    /// Function stage: Execute business logic
    Function,
    /// Forward stage: Route results to destinations
    Forward,
    /// Feedback stage: Log, trace, and audit execution
    Feedback,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_variant_order() {
        assert_eq!(PipelineStageType::Feed as u8, 0);
        assert_eq!(PipelineStageType::Filter as u8, 1);
        assert_eq!(PipelineStageType::Format as u8, 2);
        assert_eq!(PipelineStageType::Function as u8, 3);
        assert_eq!(PipelineStageType::Forward as u8, 4);
        assert_eq!(PipelineStageType::Feedback as u8, 5);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let stage = PipelineStageType::Function;
        let json = serde_json::to_string(&stage).unwrap();
        let de: PipelineStageType = serde_json::from_str(&json).unwrap();
        assert_eq!(stage, de);
    }

    #[test]
    fn test_debug_and_clone() {
        let stage = PipelineStageType::Forward;
        let debug_str = format!("{stage:?}");
        assert_eq!(debug_str, "Forward");
        let clone = stage;
        assert_eq!(stage, clone);
    }
}
