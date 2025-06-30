// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! Pipeline execution contracts and context management.
//!
//! This module provides the core abstractions for the 6F Lifecycle Flow pipeline execution:
//! Feed → Filter → Format → Function → Forward → Feedback
//!
//! # Architecture
//!
//! The pipeline system follows hexagonal architecture principles:
//! - `Pipeline` trait defines the core pipeline execution contract
//! - `PipelineStage` trait represents individual 6F lifecycle stages
//! - `PipelineContext` provides shared state across pipeline execution
//!
//! # Examples
//!
//! ```rust
//! use hexafn_core::PipelineContext;
//! use serde_json::json;
//!
//! let mut context = PipelineContext::new();
//! context.set("user_id".to_string(), json!("12345"));
//! context.set("action".to_string(), json!("login"));
//!
//! assert_eq!(context.get("user_id"), Some(&json!("12345")));
//! ```

use crate::HexaError;
use async_trait::async_trait;

/// Core pipeline execution contract for the 6F Lifecycle Flow.
///
/// This trait defines the interface for executing complete data pipelines
/// following the hexaFn 6F lifecycle: Feed → Filter → Format → Function → Forward → Feedback.
///
/// # Type Parameters
///
/// - `E`: Error type that implements `HexaError` for consistent error handling
///
/// # Architecture
///
/// Pipelines follow hexagonal architecture principles:
/// - Domain logic is isolated from infrastructure concerns
/// - Stages are composable and independently testable
/// - Error handling is consistent across all implementations
///
/// # Examples
///
/// ```rust,no_run
/// use hexafn_core::{Pipeline, PipelineStage, HexaError};
/// use hexafn_core::PipelineStageType;
/// use hexafn_core::PipelineContext;
/// use async_trait::async_trait;
/// use std::fmt::{Debug, Display, Formatter};
///
/// #[derive(Debug)]
/// struct MyError;
///
/// impl Display for MyError {
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///         write!(f, "Test error occurred")
///     }
/// }
///
/// impl HexaError for MyError {
///     fn error_code(&self) -> &str { "TEST_ERROR" }
///     fn error_message(&self) -> &str { "Test error" }
///     fn error_kind(&self) -> hexafn_core::HexaErrorKind {
///         hexafn_core::HexaErrorKind::Unknown
///     }
///     fn error_severity(&self) -> hexafn_core::HexaErrorSeverity {
///         hexafn_core::HexaErrorSeverity::Low
///     }
/// }
///
/// #[derive(Default)]
/// struct MyPipeline {
///     stages: Vec<Box<dyn PipelineStage>>,
/// }
///
/// #[async_trait]
/// impl Pipeline for MyPipeline {
///     type Input = String;
///     type Output = String;
///     async fn execute(&self, input: Self::Input) -> Result<Self::Output, Box<dyn HexaError>> {
///         Ok(input)
///     }
///     fn add_stage(&mut self, stage: Box<dyn PipelineStage>) -> Result<(), Box<dyn HexaError>> {
///         self.stages.push(stage);
///         Ok(())
///     }
///     fn get_stages(&self) -> &Vec<Box<dyn PipelineStage>> {
///         &self.stages
///     }
///     fn build(self) -> Result<Self, Box<dyn HexaError>> {
///         Ok(self)
///     }
///     fn validate(&self) -> Result<(), Box<dyn HexaError>> {
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait Pipeline: Send + Sync {
    /// Input data type for pipeline execution
    type Input;
    /// Output data type produced by pipeline
    type Output;

    /// Execute the complete pipeline with given input.
    ///
    /// Processes input data through all configured pipeline stages in the correct order,
    /// following the 6F Lifecycle Flow sequence.
    ///
    /// # Arguments
    ///
    /// * `input` - The input data to process through the pipeline
    ///
    /// # Returns
    ///
    /// * `Ok(Self::Output)` - Successfully processed output data
    /// * `Err(E)` - Pipeline execution error (stage failure, validation error, etc.)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use hexafn_core::{Pipeline, HexaError};
    /// use async_trait::async_trait;
    /// use std::fmt::{Debug, Display, Formatter};
    ///
    /// #[derive(Debug)]
    /// struct MyError;
    ///
    /// impl Display for MyError {
    ///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    ///         write!(f, "Test error occurred")
    ///     }
    /// }
    ///
    /// impl HexaError for MyError {
    ///     fn error_code(&self) -> &str { "TEST" }
    ///     fn error_message(&self) -> &str { "Test error" }
    ///     fn error_kind(&self) -> hexafn_core::HexaErrorKind {
    ///         hexafn_core::HexaErrorKind::Unknown
    ///     }
    ///     fn error_severity(&self) -> hexafn_core::HexaErrorSeverity {
    ///         hexafn_core::HexaErrorSeverity::Low
    ///     }
    /// }
    ///
    /// #[derive(Default)]
    /// struct MyPipeline;
    ///
    /// #[async_trait]
    /// impl Pipeline for MyPipeline {
    ///     type Input = String;
    ///     type Output = String;
    ///     async fn execute(&self, input: Self::Input) -> Result<Self::Output, Box<dyn HexaError>> {
    ///         Ok(input)
    ///     }
    ///     fn add_stage(&mut self, _stage: Box<dyn hexafn_core::PipelineStage>) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    ///     fn get_stages(&self) -> &Vec<Box<dyn hexafn_core::PipelineStage>> { todo!() }
    ///     fn build(self) -> Result<Self, Box<dyn HexaError>> { Ok(self) }
    ///     fn validate(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    /// }
    /// ```
    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Box<dyn HexaError>>;

    /// Add a stage to the pipeline.
    ///
    /// Stages are executed in the order they implement `get_order()`.
    /// Each stage type (Feed, Filter, Format, Function, Forward, Feedback)
    /// should appear exactly once in a valid pipeline.
    ///
    /// # Arguments
    ///
    /// * `stage` - Pipeline stage implementation to add
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Stage added successfully
    /// * `Err(E)` - Stage validation failed or pipeline is invalid
    fn add_stage(&mut self, stage: Box<dyn crate::PipelineStage>)
    -> Result<(), Box<dyn HexaError>>;

    /// Get all pipeline stages.
    ///
    /// Returns a reference to all configured pipeline stages in execution order.
    /// Useful for pipeline inspection, debugging, and validation.
    fn get_stages(&self) -> &Vec<Box<dyn crate::PipelineStage>>;

    /// Build and validate the pipeline.
    ///
    /// Performs final pipeline construction and validation before execution.
    /// Ensures all required stages are present and properly configured.
    fn build(self) -> Result<Self, Box<dyn HexaError>>
    where
        Self: Sized;

    /// Validate pipeline configuration.
    ///
    /// Checks that the pipeline is properly configured:
    /// - All required stage types are present
    /// - Stages are in correct execution order
    /// - Stage configurations are valid
    /// - Pipeline follows 6F Lifecycle Flow requirements
    fn validate(&self) -> Result<(), Box<dyn HexaError>>;
}

/// # Pipeline Trait Unit Tests
///
/// These tests validate the behavior of a minimal Pipeline implementation.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::PipelineContext;
    use crate::PipelineStage;
    use crate::PipelineStageType;
    use async_trait::async_trait;
    use std::fmt::{Debug, Display, Formatter};

    #[derive(Debug)]
    struct DummyError;

    impl Display for DummyError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Dummy error")
        }
    }
    impl HexaError for DummyError {
        fn error_code(&self) -> &str {
            "DUMMY"
        }
        fn error_message(&self) -> &str {
            "Dummy error"
        }
        fn error_kind(&self) -> crate::HexaErrorKind {
            crate::HexaErrorKind::Unknown
        }
        fn error_severity(&self) -> crate::HexaErrorSeverity {
            crate::HexaErrorSeverity::Low
        }
    }

    #[derive(Default)]
    struct DummyStage;

    #[async_trait]
    impl PipelineStage for DummyStage {
        fn stage_type(&self) -> PipelineStageType {
            PipelineStageType::Feed
        }
        fn get_order(&self) -> u32 {
            1
        }
        async fn execute(&self, _context: &mut PipelineContext) -> Result<(), Box<dyn HexaError>> {
            Ok(())
        }
        fn validate(&self) -> Result<(), Box<dyn HexaError>> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct DummyPipeline {
        stages: Vec<Box<dyn PipelineStage>>,
    }

    #[async_trait]
    impl Pipeline for DummyPipeline {
        type Input = String;
        type Output = String;
        async fn execute(&self, input: Self::Input) -> Result<Self::Output, Box<dyn HexaError>> {
            Ok(format!("executed: {input}"))
        }
        fn add_stage(&mut self, stage: Box<dyn PipelineStage>) -> Result<(), Box<dyn HexaError>> {
            self.stages.push(stage);
            Ok(())
        }
        fn get_stages(&self) -> &Vec<Box<dyn PipelineStage>> {
            &self.stages
        }
        fn build(self) -> Result<Self, Box<dyn HexaError>> {
            Ok(self)
        }
        fn validate(&self) -> Result<(), Box<dyn HexaError>> {
            if self.stages.is_empty() {
                Err(Box::new(DummyError))
            } else {
                Ok(())
            }
        }
    }

    #[tokio::test]
    async fn pipeline_execute_returns_expected_output() {
        let pipeline = DummyPipeline::default();
        let result = pipeline.execute("test".to_string()).await.unwrap();
        assert_eq!(result, "executed: test");
    }

    #[test]
    fn pipeline_add_stage_and_get_stages() {
        let mut pipeline = DummyPipeline::default();
        assert_eq!(pipeline.get_stages().len(), 0);
        pipeline.add_stage(Box::new(DummyStage)).unwrap();
        assert_eq!(pipeline.get_stages().len(), 1);
        assert_eq!(
            pipeline.get_stages()[0].stage_type(),
            PipelineStageType::Feed
        );
    }

    #[test]
    fn pipeline_build_returns_self() {
        let pipeline = DummyPipeline::default();
        let built = pipeline.build().unwrap();
        assert_eq!(built.get_stages().len(), 0);
    }

    #[test]
    fn pipeline_validate_fails_if_no_stages() {
        let pipeline = DummyPipeline::default();
        let result = pipeline.validate();
        assert!(result.is_err());
    }

    #[test]
    fn pipeline_validate_succeeds_with_stage() {
        let mut pipeline = DummyPipeline::default();
        pipeline.add_stage(Box::new(DummyStage)).unwrap();
        let result = pipeline.validate();
        assert!(result.is_ok());
    }
}
