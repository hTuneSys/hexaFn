// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # PipelineStage Trait (Domain Contract)
//!
//! This module defines the [`PipelineStage`] trait, the contract for all pipeline stages in the 6F Lifecycle Flow.
//! Each stage implements this trait to participate in the pipeline execution, using [`PipelineStageType`]
//! and [`PipelineContext`] for type and context.
//!
//! ## Usage Example
//!
//! ```rust,no_run
//! use hexafn_core::{PipelineStage, PipelineStageType, PipelineContext};
//! use hexafn_core::HexaError;
//! use std::fmt::{Display, Formatter};
//! #[derive(Debug)]
//! struct MyError;
//! impl Display for MyError {
//!     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "Stage error") }
//! }
//! impl HexaError for MyError {
//!     fn error_code(&self) -> &str { "STAGE_ERROR" }
//!     fn error_message(&self) -> &str { "Stage error" }
//!     fn error_kind(&self) -> hexafn_core::HexaErrorKind { hexafn_core::HexaErrorKind::Internal }
//!     fn error_severity(&self) -> hexafn_core::HexaErrorSeverity { hexafn_core::HexaErrorSeverity::Medium }
//! }
//! struct FeedStage;
//! #[async_trait::async_trait]
//! impl PipelineStage for FeedStage {
//!     fn stage_type(&self) -> PipelineStageType { PipelineStageType::Feed }
//!     fn get_order(&self) -> u32 { 1 }
//!     async fn execute(&self, context: &mut PipelineContext) -> Result<(), Box<dyn HexaError>> {
//!         context.set("foo".to_string(), serde_json::json!(42));
//!         Ok(())
//!     }
//!     fn validate(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
//! }
//! ```

use crate::{HexaError, PipelineContext, PipelineStageType};
use async_trait::async_trait;

#[async_trait]
pub trait PipelineStage: Send + Sync {
    /// Get the type of this pipeline stage.
    ///
    /// Identifies which phase of the 6F Lifecycle Flow this stage represents.
    /// Used for pipeline validation and stage ordering.
    ///
    /// # Returns
    ///
    /// The `PipelineStageType` enum value for this stage
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use hexafn_core::{PipelineStage, PipelineStageType};
    /// # use hexafn_core::HexaError;
    /// # use std::fmt::{Debug, Display, Formatter};
    /// # struct FeedStage;
    /// # #[derive(Debug)]
    /// # struct MyError;
    /// #
    /// # impl Display for MyError {
    /// #     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    /// #         write!(f, "Test error occurred")
    /// #     }
    /// # }
    /// #
    /// # impl HexaError for MyError {
    /// #     fn error_code(&self) -> &str { "TEST" }
    /// #     fn error_message(&self) -> &str { "Test error" }
    /// #     fn error_kind(&self) -> hexafn_core::HexaErrorKind {
    /// #         hexafn_core::HexaErrorKind::Unknown
    /// #     }
    /// #     fn error_severity(&self) -> hexafn_core::HexaErrorSeverity {
    /// #         hexafn_core::HexaErrorSeverity::Low
    /// #     }
    /// # }
    /// # #[async_trait::async_trait] impl PipelineStage for FeedStage {
    /// #     async fn execute(&self, context: &mut hexafn_core::PipelineContext) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    /// #     fn get_order(&self) -> u32 { 1 } fn validate(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    ///
    /// fn stage_type(&self) -> PipelineStageType {
    ///     PipelineStageType::Feed
    /// }
    /// # }
    ///
    /// fn check_stage_type(stage: &dyn PipelineStage) {
    ///     match stage.stage_type() {
    ///         PipelineStageType::Feed => println!("This is a feed stage"),
    ///         PipelineStageType::Function => println!("This is a function stage"),
    ///         _ => println!("Other stage type"),
    ///     }
    /// }
    /// ```
    fn stage_type(&self) -> PipelineStageType;

    /// Get the execution order of this stage.
    ///
    /// Defines when this stage executes relative to other stages.
    /// Lower numbers execute first. Standard order follows 6F sequence:
    /// 1. Feed, 2. Filter, 3. Format, 4. Function, 5. Forward, 6. Feedback
    ///
    /// # Returns
    ///
    /// Numeric execution order (1-based)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use hexafn_core::PipelineStage;
    /// # use hexafn_core::HexaError;
    /// # use std::fmt::{Debug, Display, Formatter};
    /// # struct MyStage;
    /// # #[derive(Debug)]
    /// # struct MyError;
    /// #
    /// # impl Display for MyError {
    /// #     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    /// #         write!(f, "Test error occurred")
    /// #     }
    /// # }
    /// #
    /// # impl HexaError for MyError {
    /// #     fn error_code(&self) -> &str { "TEST" }
    /// #     fn error_message(&self) -> &str { "Test error" }
    /// #     fn error_kind(&self) -> hexafn_core::HexaErrorKind {
    /// #         hexafn_core::HexaErrorKind::Unknown
    /// #     }
    /// #     fn error_severity(&self) -> hexafn_core::HexaErrorSeverity {
    /// #         hexafn_core::HexaErrorSeverity::Low
    /// #     }
    /// # }
    /// # #[async_trait::async_trait] impl PipelineStage for MyStage {
    /// #     fn stage_type(&self) -> hexafn_core::PipelineStageType { hexafn_core::PipelineStageType::Feed }
    /// #     async fn execute(&self, context: &mut hexafn_core::PipelineContext) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    /// #     fn validate(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    ///
    /// fn get_order(&self) -> u32 {
    ///     1  // Execute first in pipeline
    /// }
    /// # }
    ///
    /// fn sort_stages_by_order(mut stages: Vec<Box<dyn PipelineStage>>)
    ///     -> Vec<Box<dyn PipelineStage>> {
    ///     stages.sort_by_key(|stage| stage.get_order());
    ///     stages
    /// }
    /// ```
    fn get_order(&self) -> u32;

    /// Execute this stage with the given context.
    ///
    /// Performs the stage-specific processing as part of the pipeline execution.
    /// Modifies the `PipelineContext` to pass data to subsequent stages.
    ///
    /// # Arguments
    ///
    /// * `context` - Mutable reference to shared pipeline context
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Stage executed successfully
    /// * `Err(E)` - Stage execution failed
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use hexafn_core::{PipelineStage, PipelineContext};
    /// # use hexafn_core::HexaError;
    /// # use std::fmt::{Debug, Display, Formatter};
    /// # struct FilterStage;
    /// # #[derive(Debug)]
    /// # struct MyError;
    /// #
    /// # impl Display for MyError {
    /// #     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    /// #         write!(f, "Filter error occurred")
    /// #     }
    /// # }
    /// #
    /// # impl HexaError for MyError {
    /// #     fn error_code(&self) -> &str { "FILTER_ERROR" }
    /// #     fn error_message(&self) -> &str { "Filter error" }
    /// #     fn error_kind(&self) -> hexafn_core::HexaErrorKind {
    /// #         hexafn_core::HexaErrorKind::Validation
    /// #     }
    /// #     fn error_severity(&self) -> hexafn_core::HexaErrorSeverity {
    /// #         hexafn_core::HexaErrorSeverity::High
    /// #     }
    /// # }
    /// # #[async_trait::async_trait] impl PipelineStage for FilterStage {
    /// #     fn stage_type(&self) -> hexafn_core::PipelineStageType { hexafn_core::PipelineStageType::Filter }
    /// #     fn get_order(&self) -> u32 { 2 } fn validate(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    ///
    /// async fn execute(&self, context: &mut PipelineContext) -> Result<(), Box<dyn HexaError>> {
    ///     // Get input from previous stage
    ///     let user_id = context.get("user_id")
    ///         .and_then(|v| v.as_str())
    ///         .ok_or(Box::new(MyError) as Box<dyn HexaError>)?;
    ///     
    ///     // Apply filter logic
    ///     if user_id.is_empty() {
    ///         return Err(Box::new(MyError));  // Filter failed
    ///     }
    ///     
    ///     // Set result for next stage
    ///     context.set("filter_passed".to_string(),
    ///                serde_json::json!(true));
    ///     
    ///     Ok(())
    /// }
    /// # }
    /// ```
    async fn execute(&self, context: &mut PipelineContext) -> Result<(), Box<dyn HexaError>>;

    /// Validate this stage configuration.
    ///
    /// Checks that the stage is properly configured and ready for execution.
    /// Called during pipeline validation before execution begins.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Stage configuration is valid
    /// * `Err(E)` - Stage configuration is invalid
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use hexafn_core::PipelineStage;
    /// # use hexafn_core::HexaError;
    /// # use std::fmt::{Debug, Display, Formatter};
    /// # struct DatabaseStage { connection_string: String }
    /// # #[derive(Debug)]
    /// # struct MyError;
    /// #
    /// # impl Display for MyError {
    /// #     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    /// #         write!(f, "Validation error occurred")
    /// #     }
    /// # }
    /// #
    /// # impl HexaError for MyError {
    /// #     fn error_code(&self) -> &str { "VALIDATION_ERROR" }
    /// #     fn error_message(&self) -> &str { "Validation error" }
    /// #     fn error_kind(&self) -> hexafn_core::HexaErrorKind {
    /// #         hexafn_core::HexaErrorKind::Validation
    /// #     }
    /// #     fn error_severity(&self) -> hexafn_core::HexaErrorSeverity {
    /// #         hexafn_core::HexaErrorSeverity::High
    /// #     }
    /// # }
    /// # #[async_trait::async_trait] impl PipelineStage for DatabaseStage {
    /// #     fn stage_type(&self) -> hexafn_core::PipelineStageType { hexafn_core::PipelineStageType::Forward }
    /// #     fn get_order(&self) -> u32 { 5 }
    /// #     async fn execute(&self, context: &mut hexafn_core::PipelineContext) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    ///
    /// fn validate(&self) -> Result<(), Box<dyn HexaError>> {
    ///     if self.connection_string.is_empty() {
    ///         return Err(Box::new(MyError));  // Missing required configuration
    ///     }
    ///     
    ///     if !self.connection_string.starts_with("postgresql://") {
    ///         return Err(Box::new(MyError));  // Invalid connection format
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// # }
    /// ```
    fn validate(&self) -> Result<(), Box<dyn HexaError>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HexaError;
    use crate::PipelineContext;
    use crate::PipelineStageType;
    use serde_json::json;
    use std::fmt::{Display, Formatter};

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
            crate::HexaErrorKind::Internal
        }
        fn error_severity(&self) -> crate::HexaErrorSeverity {
            crate::HexaErrorSeverity::Low
        }
    }

    struct TestStage;
    #[async_trait::async_trait]
    impl PipelineStage for TestStage {
        fn stage_type(&self) -> PipelineStageType {
            PipelineStageType::Format
        }
        fn get_order(&self) -> u32 {
            3
        }
        async fn execute(&self, context: &mut PipelineContext) -> Result<(), Box<dyn HexaError>> {
            context.set("test_key".to_string(), json!("test_val"));
            Ok(())
        }
        fn validate(&self) -> Result<(), Box<dyn HexaError>> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_stage_methods() {
        let stage = TestStage;
        assert_eq!(stage.stage_type(), PipelineStageType::Format);
        assert_eq!(stage.get_order(), 3);
        let mut ctx = PipelineContext::new();
        stage.execute(&mut ctx).await.unwrap();
        assert_eq!(ctx.get("test_key"), Some(&json!("test_val")));
        assert!(stage.validate().is_ok());
    }

    struct InvalidStage;
    #[async_trait::async_trait]
    impl PipelineStage for InvalidStage {
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
            Err(Box::new(DummyError))
        }
    }

    #[tokio::test]
    async fn test_invalid_stage_validate() {
        let stage = InvalidStage;
        let result = stage.validate();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.error_code(), "DUMMY");
    }
}
