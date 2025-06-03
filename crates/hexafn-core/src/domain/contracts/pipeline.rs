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
//! use hexafn_core::{PipelineContext, PipelineStageType};
//! use serde_json::json;
//!
//! let mut context = PipelineContext::new();
//! context.set("user_id".to_string(), json!("12345"));
//! context.set("action".to_string(), json!("login"));
//!
//! assert_eq!(context.get("user_id"), Some(&json!("12345")));
//! ```

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::domain::contracts::HexaError;

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
/// # Examples
///
/// ```rust
/// use hexafn_core::PipelineStageType;
/// 
/// let stage_type = PipelineStageType::Feed;
/// assert_eq!(stage_type, PipelineStageType::Feed);
///
/// // Stage types are ordered by execution sequence
/// let feed_order = PipelineStageType::Feed as u8;
/// let filter_order = PipelineStageType::Filter as u8;
/// assert!(feed_order < filter_order);
/// ```
///
/// # Serialization
///
/// Pipeline stage types are serializable for configuration persistence:
///
/// ```rust
/// use hexafn_core::PipelineStageType;
/// use serde_json;
/// 
/// let stage = PipelineStageType::Function;
/// let json = serde_json::to_string(&stage).unwrap();
/// let deserialized: PipelineStageType = serde_json::from_str(&json).unwrap();
/// assert_eq!(stage, deserialized);
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
/// use hexafn_core::{Pipeline, PipelineStage};
/// use hexafn_core::domain::contracts::HexaError;
/// use std::fmt::{Debug, Display, Formatter};
/// 
/// # struct MyPipeline;
/// #
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
/// #     fn error_code(&self) -> &str { "TEST_ERROR" }
/// #     fn error_message(&self) -> &str { "Test error" }
/// #     fn error_kind(&self) -> hexafn_core::HexaErrorKind { 
/// #         hexafn_core::HexaErrorKind::Unknown 
/// #     }
/// #     fn error_severity(&self) -> hexafn_core::HexaErrorSeverity { 
/// #         hexafn_core::HexaErrorSeverity::Low 
/// #     }
/// # }
/// # #[async_trait::async_trait]
/// # impl Pipeline for MyPipeline {
/// #     type Input = String;
/// #     type Output = String;
/// #     async fn execute(&self, input: Self::Input) -> Result<Self::Output, Box<dyn HexaError>> { Ok(input) }
/// #     fn add_stage(&mut self, stage: Box<dyn PipelineStage>) -> Result<(), Box<dyn HexaError>> { Ok(()) }
/// #     fn get_stages(&self) -> &Vec<Box<dyn PipelineStage>> { todo!() }
/// #     fn build(self) -> Result<Self, Box<dyn HexaError>> { Ok(self) }
/// #     fn validate(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
/// # }
///
/// async fn example_pipeline_usage() -> Result<(), Box<dyn HexaError>> {
///     let mut pipeline = MyPipeline;
///     
///     // Build and validate pipeline
///     let pipeline = pipeline.build()?;
///     
///     // Execute with input data
///     let result = pipeline.execute("input_data".to_string()).await?;
///     
///     Ok(())
/// }
/// ```
///
/// # Error Handling
///
/// All pipeline operations return `Result<T, E>` where `E` implements `HexaError`:
///
/// ```rust,no_run
/// # use hexafn_core::domain::contracts::HexaError;
/// # use std::fmt::{Debug, Display, Formatter};
/// #
/// # #[derive(Debug)]
/// # struct MyError;
/// # 
/// # impl Display for MyError {
/// #     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
/// #         write!(f, "Pipeline error occurred")
/// #     }
/// # }
/// #
/// # impl HexaError for MyError {
/// #     fn error_code(&self) -> &str { "PIPELINE_ERROR" }
/// #     fn error_message(&self) -> &str { "Pipeline error" }
/// #     fn error_kind(&self) -> hexafn_core::HexaErrorKind { 
/// #         hexafn_core::HexaErrorKind::Unknown 
/// #     }
/// #     fn error_severity(&self) -> hexafn_core::HexaErrorSeverity { 
/// #         hexafn_core::HexaErrorSeverity::Low 
/// #     }
/// # }
/// 
/// fn handle_pipeline_error(error: MyError) {
///     eprintln!("Pipeline failed with code: {}", error.error_code());
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
    /// # use hexafn_core::Pipeline;
    /// # use hexafn_core::domain::contracts::HexaError;
    /// # use std::fmt::{Debug, Display, Formatter};
    /// # struct MyPipeline;
    /// #
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
    /// # #[async_trait::async_trait]
    /// # impl Pipeline for MyPipeline {
    /// #     type Input = String; type Output = String;
    /// #     async fn execute(&self, input: Self::Input) -> Result<Self::Output, Box<dyn HexaError>> { Ok(input) }
    /// #     fn add_stage(&mut self, stage: Box<dyn hexafn_core::PipelineStage>) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    /// #     fn get_stages(&self) -> &Vec<Box<dyn hexafn_core::PipelineStage>> { todo!() }
    /// #     fn build(self) -> Result<Self, Box<dyn HexaError>> { Ok(self) }
    /// #     fn validate(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    /// # }
    ///
    /// async fn execute_data_pipeline() -> Result<(), Box<dyn HexaError>> {
    ///     let pipeline = MyPipeline;
    ///     let input = "user_data".to_string();
    ///     
    ///     let output = pipeline.execute(input).await?;
    ///     println!("Pipeline result: {}", output);
    ///     
    ///     Ok(())
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
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use hexafn_core::{Pipeline, PipelineStage};
    /// # use hexafn_core::domain::contracts::HexaError;
    /// # use std::fmt::{Debug, Display, Formatter};
    /// # struct MyPipeline; 
    /// # #[derive(Debug)]
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
    /// #     fn get_order(&self) -> u32 { 1 }
    /// #     async fn execute(&self, context: &mut hexafn_core::PipelineContext) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    /// #     fn validate(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    /// # }
    /// # #[async_trait::async_trait] impl Pipeline for MyPipeline {
    /// #     type Input = String; type Output = String;
    /// #     async fn execute(&self, input: Self::Input) -> Result<Self::Output, Box<dyn HexaError>> { Ok(input) }
    /// #     fn add_stage(&mut self, stage: Box<dyn PipelineStage>) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    /// #     fn get_stages(&self) -> &Vec<Box<dyn PipelineStage>> { todo!() }
    /// #     fn build(self) -> Result<Self, Box<dyn HexaError>> { Ok(self) }
    /// #     fn validate(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    /// # }
    /// 
    /// fn configure_pipeline() -> Result<(), Box<dyn HexaError>> {
    ///     let mut pipeline = MyPipeline;
    ///     let feed_stage = Box::new(MyStage);
    ///     
    ///     pipeline.add_stage(feed_stage)?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    fn add_stage(&mut self, stage: Box<dyn PipelineStage>) -> Result<(), Box<dyn HexaError>>;

    /// Get all pipeline stages.
    ///
    /// Returns a reference to all configured pipeline stages in execution order.
    /// Useful for pipeline inspection, debugging, and validation.
    ///
    /// # Returns
    ///
    /// Reference to vector of all pipeline stages
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use hexafn_core::{Pipeline, PipelineStageType};
    /// # use hexafn_core::domain::contracts::HexaError;
    /// # use std::fmt::{Debug, Display, Formatter};
    /// # struct MyPipeline; 
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
    /// # #[async_trait::async_trait] impl Pipeline for MyPipeline {
    /// #     type Input = String; type Output = String;
    /// #     async fn execute(&self, input: Self::Input) -> Result<Self::Output, Box<dyn HexaError>> { Ok(input) }
    /// #     fn add_stage(&mut self, stage: Box<dyn hexafn_core::PipelineStage>) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    /// #     fn get_stages(&self) -> &Vec<Box<dyn hexafn_core::PipelineStage>> { todo!() }
    /// #     fn build(self) -> Result<Self, Box<dyn HexaError>> { Ok(self) }
    /// #     fn validate(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    /// # }
    /// 
    /// fn inspect_pipeline_stages(pipeline: &MyPipeline) {
    ///     let stages = pipeline.get_stages();
    ///     println!("Pipeline has {} stages", stages.len());
    ///     
    ///     for stage in stages {
    ///         println!("Stage type: {:?}, Order: {}", 
    ///                  stage.stage_type(), 
    ///                  stage.get_order());
    ///     }
    /// }
    /// ```
    fn get_stages(&self) -> &Vec<Box<dyn PipelineStage>>;

    /// Build and validate the pipeline.
    ///
    /// Performs final pipeline construction and validation before execution.
    /// Ensures all required stages are present and properly configured.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - Valid, executable pipeline
    /// * `Err(E)` - Pipeline validation failed
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use hexafn_core::Pipeline;
    /// # use hexafn_core::domain::contracts::HexaError;
    /// # use std::fmt::{Debug, Display, Formatter};
    /// # struct MyPipeline; 
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
    /// # #[async_trait::async_trait] impl Pipeline for MyPipeline {
    /// #     type Input = String; type Output = String;
    /// #     async fn execute(&self, input: Self::Input) -> Result<Self::Output, Box<dyn HexaError>> { Ok(input) }
    /// #     fn add_stage(&mut self, stage: Box<dyn hexafn_core::PipelineStage>) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    /// #     fn get_stages(&self) -> &Vec<Box<dyn hexafn_core::PipelineStage>> { todo!() }
    /// #     fn build(self) -> Result<Self, Box<dyn HexaError>> { Ok(self) }
    /// #     fn validate(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    /// # }
    ///
    /// fn create_validated_pipeline() -> Result<MyPipeline, Box<dyn HexaError>> {
    ///     let mut pipeline = MyPipeline;
    ///     
    ///     // Add stages...
    ///     
    ///     // Build and validate
    ///     let validated_pipeline = pipeline.build()?;
    ///     
    ///     Ok(validated_pipeline)
    /// }
    /// ```
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
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Pipeline configuration is valid
    /// * `Err(E)` - Pipeline configuration is invalid
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use hexafn_core::Pipeline;
    /// # use hexafn_core::domain::contracts::HexaError;
    /// # use std::fmt::{Debug, Display, Formatter};
    /// # struct MyPipeline; 
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
    /// # #[async_trait::async_trait] impl Pipeline for MyPipeline {
    /// #     type Input = String; type Output = String;
    /// #     async fn execute(&self, input: Self::Input) -> Result<Self::Output, Box<dyn HexaError>> { Ok(input) }
    /// #     fn add_stage(&mut self, stage: Box<dyn hexafn_core::PipelineStage>) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    /// #     fn get_stages(&self) -> &Vec<Box<dyn hexafn_core::PipelineStage>> { todo!() }
    /// #     fn build(self) -> Result<Self, Box<dyn HexaError>> { Ok(self) }
    /// #     fn validate(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    /// # }
    ///
    /// fn check_pipeline_validity(pipeline: &MyPipeline) -> Result<(), Box<dyn HexaError>> {
    ///     match pipeline.validate() {
    ///         Ok(()) => {
    ///             println!("Pipeline configuration is valid");
    ///             Ok(())
    ///         }
    ///         Err(e) => {
    ///             eprintln!("Pipeline validation failed: {}", e.error_code());
    ///             Err(e)
    ///         }
    ///     }
    /// }
    /// ```
    fn validate(&self) -> Result<(), Box<dyn HexaError>>;
}

/// Individual stage in the 6F Lifecycle Flow pipeline.
///
/// Each pipeline stage represents one phase of the hexaFn execution flow:
/// Feed → Filter → Format → Function → Forward → Feedback
///
/// # Type Parameters
///
/// - `E`: Error type that implements `HexaError` for consistent error handling
///
/// # Architecture
///
/// Pipeline stages follow the single responsibility principle:
/// - Each stage handles one specific concern
/// - Stages communicate through `PipelineContext`
/// - Execution order is determined by `get_order()`
/// - Stages are independently testable and composable
///
/// # Examples
///
/// ```rust,no_run
/// use hexafn_core::{
///     PipelineStage, PipelineStageType, PipelineContext
/// };
/// use hexafn_core::domain::contracts::HexaError;
/// use std::fmt::{Debug, Display, Formatter};
/// 
/// # #[derive(Debug)]
/// # struct MyError;
/// # 
/// # impl Display for MyError {
/// #     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
/// #         write!(f, "Stage error occurred")
/// #     }
/// # }
/// #
/// # impl HexaError for MyError { 
/// #     fn error_code(&self) -> &str { "STAGE_ERROR" }
/// #     fn error_message(&self) -> &str { "Stage error" }
/// #     fn error_kind(&self) -> hexafn_core::HexaErrorKind { 
/// #         hexafn_core::HexaErrorKind::Internal 
/// #     }
/// #     fn error_severity(&self) -> hexafn_core::HexaErrorSeverity { 
/// #         hexafn_core::HexaErrorSeverity::Medium 
/// #     }
/// # }
/// 
/// struct FeedStage {
///     source_name: String,
/// }
/// 
/// #[async_trait::async_trait]
/// impl PipelineStage for FeedStage {
///     fn stage_type(&self) -> PipelineStageType {
///         PipelineStageType::Feed
///     }
///     
///     fn get_order(&self) -> u32 {
///         1  // Feed is the first stage
///     }
///
///     async fn execute(&self, context: &mut PipelineContext) -> Result<(), Box<dyn HexaError>> {
///         // Implement feed logic
///         context.set("source".to_string(), 
///                    serde_json::json!(&self.source_name));
///         Ok(())
///     }
///     
///     fn validate(&self) -> Result<(), Box<dyn HexaError>> {
///         if self.source_name.is_empty() {
///             return Err(Box::new(MyError));  // Invalid configuration
///         }
///         Ok(())
///     }
/// }
/// ```
///
/// # Stage Execution Order
///
/// Stages must implement `get_order()` to define execution sequence:
///
/// ```rust,no_run
/// # use hexafn_core::{PipelineStage, PipelineStageType};
/// # use hexafn_core::domain::contracts::HexaError;
/// # use std::fmt::{Debug, Display, Formatter};
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
/// # struct MyStage;
/// # #[async_trait::async_trait] impl PipelineStage for MyStage {
/// #     fn stage_type(&self) -> PipelineStageType { PipelineStageType::Feed }
/// #     async fn execute(&self, context: &mut hexafn_core::PipelineContext) -> Result<(), Box<dyn HexaError>> { Ok(()) }
/// #     fn validate(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
/// 
/// fn get_order(&self) -> u32 {
///     match self.stage_type() {
///         PipelineStageType::Feed => 1,
///         PipelineStageType::Filter => 2,
///         PipelineStageType::Format => 3,
///         PipelineStageType::Function => 4,
///         PipelineStageType::Forward => 5,
///         PipelineStageType::Feedback => 6,
///     }
/// }
/// # }
/// ```
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
    /// # use hexafn_core::domain::contracts::HexaError;
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
    /// # use hexafn_core::domain::contracts::HexaError;
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
    /// # use hexafn_core::domain::contracts::HexaError;
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
    /// # use hexafn_core::domain::contracts::HexaError;
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

/// Shared context for pipeline execution.
///
/// Provides a data store for passing information between pipeline stages.
/// Each stage can read data set by previous stages and set data for subsequent stages.
///
/// # Architecture
///
/// The context follows these principles:
/// - **Immutable keys**: Once set, keys maintain their identity
/// - **Mutable values**: Values can be updated by subsequent stages
/// - **JSON serialization**: All values are JSON-serializable for flexibility
/// - **Type safety**: Access methods handle type conversion safely
///
/// # Data Flow
///
/// ```text
/// Feed Stage    -> sets "input_data"
/// Filter Stage  -> reads "input_data", sets "filter_result" 
/// Format Stage  -> reads "filter_result", sets "formatted_data"
/// Function Stage-> reads "formatted_data", sets "function_output"
/// Forward Stage -> reads "function_output", sets "forward_status"
/// Feedback Stage-> reads all previous data for audit logging
/// ```
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use hexafn_core::PipelineContext;
/// use serde_json::json;
/// 
/// let mut context = PipelineContext::new();
/// 
/// // Set various data types
/// context.set("user_id".to_string(), json!("12345"));
/// context.set("timestamp".to_string(), json!(1642781234));
/// context.set("is_valid".to_string(), json!(true));
/// context.set("metadata".to_string(), json!({
///     "source": "webhook",
///     "version": "1.0"
/// }));
/// 
/// // Retrieve and use data
/// assert_eq!(context.get("user_id"), Some(&json!("12345")));
/// assert_eq!(context.get("nonexistent"), None);
/// ```
///
/// ## Stage-to-Stage Communication
///
/// ```rust
/// use hexafn_core::PipelineContext;
/// use serde_json::json;
/// 
/// async fn feed_stage_example(context: &mut PipelineContext) {
///     // Feed stage: ingest external data
///     context.set("raw_input".to_string(), json!({
///         "event_type": "user_login",
///         "user_id": "user_123",
///         "timestamp": "2024-01-01T10:00:00Z"
///     }));
/// }
/// 
/// async fn filter_stage_example(context: &mut PipelineContext) {
///     // Filter stage: validate and filter data
///     if let Some(raw_input) = context.get("raw_input") {
///         let event_type = raw_input["event_type"].as_str().unwrap_or("");
///         if event_type == "user_login" {
///             // Clone the value to avoid borrowing conflicts
///             let user_id = raw_input["user_id"].clone();
///             context.set("filter_passed".to_string(), json!(true));
///             context.set("validated_user_id".to_string(), user_id);
///         }
///     }
/// }
/// ```
///
/// ## Complex Data Structures
///
/// ```rust
/// use hexafn_core::PipelineContext;
/// use serde_json::json;
/// 
/// let mut context = PipelineContext::new();
/// 
/// // Store complex nested structures
/// context.set("processing_result".to_string(), json!({
///     "status": "success",
///     "data": {
///         "processed_items": ["item1", "item2", "item3"],
///         "metrics": {
///             "duration_ms": 150,
///             "memory_used_kb": 1024
///         }
///     },
///     "errors": []
/// }));
/// 
/// // Access nested data
/// if let Some(result) = context.get("processing_result") {
///     let status = result["status"].as_str().unwrap_or("unknown");
///     let duration = result["data"]["metrics"]["duration_ms"].as_u64().unwrap_or(0);
///     println!("Processing {} in {}ms", status, duration);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct PipelineContext {
    /// Internal data storage using JSON values for flexibility
    pub data: HashMap<String, serde_json::Value>,
}

impl PipelineContext {
    /// Create a new empty pipeline context.
    ///
    /// Initializes an empty context ready for pipeline execution.
    /// This is the starting point for all pipeline data flow.
    ///
    /// # Returns
    ///
    /// A new, empty `PipelineContext` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_core::PipelineContext;
    /// 
    /// let context = PipelineContext::new();
    /// assert_eq!(context.data.len(), 0);
    /// 
    /// // Context is ready for use
    /// let mut mutable_context = context;
    /// // Add data as needed...
    /// ```
    ///
    /// # Performance
    ///
    /// Creating a new context is lightweight (O(1)) as it only allocates
    /// an empty HashMap. Memory usage grows as data is added.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Get a value from the context.
    ///
    /// Retrieves a value by key, returning a reference to the JSON value.
    /// Returns `None` if the key doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// * `Some(&serde_json::Value)` - Reference to the value if key exists
    /// * `None` - Key not found in context
    ///
    /// # Examples
    ///
    /// ## Basic Retrieval
    ///
    /// ```rust
    /// use hexafn_core::PipelineContext;
    /// use serde_json::json;
    /// 
    /// let mut context = PipelineContext::new();
    /// context.set("name".to_string(), json!("Alice"));
    /// 
    /// // Get existing value
    /// assert_eq!(context.get("name"), Some(&json!("Alice")));
    /// 
    /// // Get non-existent value
    /// assert_eq!(context.get("missing"), None);
    /// ```
    ///
    /// ## Type-Safe Access
    ///
    /// ```rust
    /// use hexafn_core::PipelineContext;
    /// use serde_json::json;
    /// 
    /// let mut context = PipelineContext::new();
    /// context.set("age".to_string(), json!(25));
    /// context.set("active".to_string(), json!(true));
    /// 
    /// // Safe type conversion
    /// if let Some(age_value) = context.get("age") {
    ///     let age = age_value.as_u64().unwrap_or(0);
    ///     println!("Age: {}", age);
    /// }
    /// 
    /// if let Some(active_value) = context.get("active") {
    ///     let is_active = active_value.as_bool().unwrap_or(false);
    ///     println!("Active: {}", is_active);
    /// }
    /// ```
    ///
    /// ## Pattern Matching
    ///
    /// ```rust
    /// use hexafn_core::PipelineContext;
    /// use serde_json::json;
    /// 
    /// let mut context = PipelineContext::new();
    /// context.set("status".to_string(), json!("processing"));
    /// 
    /// match context.get("status") {
    ///     Some(value) if value.as_str() == Some("processing") => {
    ///         println!("Currently processing...");
    ///     }
    ///     Some(value) => {
    ///         println!("Status: {:?}", value);
    ///     }
    ///     None => {
    ///         println!("No status available");
    ///     }
    /// }
    /// ```
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }

    /// Set a value in the context.
    ///
    /// Stores or updates a value for the given key. If the key already exists,
    /// the previous value is replaced.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to store the value under
    /// * `value` - The JSON value to store
    ///
    /// # Examples
    ///
    /// ## Setting Different Types
    ///
    /// ```rust
    /// use hexafn_core::PipelineContext;
    /// use serde_json::json;
    /// 
    /// let mut context = PipelineContext::new();
    /// 
    /// // String value
    /// context.set("name".to_string(), json!("John Doe"));
    /// 
    /// // Numeric value
    /// context.set("count".to_string(), json!(42));
    /// 
    /// // Boolean value
    /// context.set("enabled".to_string(), json!(true));
    /// 
    /// // Array value
    /// context.set("tags".to_string(), json!(["rust", "pipeline", "async"]));
    /// 
    /// // Object value
    /// context.set("config".to_string(), json!({
    ///     "timeout": 30,
    ///     "retries": 3,
    ///     "debug": false
    /// }));
    /// ```
    ///
    /// ## Updating Existing Values
    ///
    /// ```rust
    /// use hexafn_core::PipelineContext;
    /// use serde_json::json;
    /// 
    /// let mut context = PipelineContext::new();
    /// 
    /// // Initial value
    /// context.set("counter".to_string(), json!(0));
    /// assert_eq!(context.get("counter"), Some(&json!(0)));
    /// 
    /// // Update value
    /// context.set("counter".to_string(), json!(1));
    /// assert_eq!(context.get("counter"), Some(&json!(1)));
    /// ```
    ///
    /// ## Building Complex State
    ///
    /// ```rust
    /// use hexafn_core::PipelineContext;
    /// use serde_json::json;
    /// 
    /// let mut context = PipelineContext::new();
    /// 
    /// // Build up state through stages
    /// context.set("input".to_string(), json!("raw data"));
    /// context.set("validated".to_string(), json!(true));
    /// context.set("processed_at".to_string(), json!("2024-01-01T12:00:00Z"));
    /// context.set("output".to_string(), json!({
    ///     "result": "processed data",
    ///     "metadata": {
    ///         "version": "1.0",
    ///         "checksum": "abc123"
    ///     }
    /// }));
    /// ```
    pub fn set(&mut self, key: String, value: serde_json::Value) {
        self.data.insert(key, value);
    }
}

impl Default for PipelineContext {
    /// Create a default pipeline context.
    ///
    /// Equivalent to `PipelineContext::new()`. Provides a default instance
    /// for use with the `Default` trait.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_core::PipelineContext;
    /// 
    /// let context1 = PipelineContext::default();
    /// let context2 = PipelineContext::new();
    /// 
    /// // Both are equivalent
    /// assert_eq!(context1.data.len(), context2.data.len());
    /// ```
    ///
    /// ## Usage in Structs
    ///
    /// ```rust
    /// use hexafn_core::PipelineContext;
    /// 
    /// #[derive(Default)]
    /// struct PipelineExecutor {
    ///     context: PipelineContext,
    ///     stage_count: usize,
    /// }
    /// 
    /// let executor = PipelineExecutor::default();
    /// assert_eq!(executor.context.data.len(), 0);
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn pipeline_context_new_and_default_are_empty() {
        let ctx1 = PipelineContext::new();
        let ctx2 = PipelineContext::default();
        assert_eq!(ctx1.data.len(), 0);
        assert_eq!(ctx2.data.len(), 0);
    }

    #[test]
    fn pipeline_context_set_and_get_basic_types() {
        let mut ctx = PipelineContext::new();
        ctx.set("str".to_string(), json!("abc"));
        ctx.set("num".to_string(), json!(42));
        ctx.set("bool".to_string(), json!(true));
        assert_eq!(ctx.get("str"), Some(&json!("abc")));
        assert_eq!(ctx.get("num"), Some(&json!(42)));
        assert_eq!(ctx.get("bool"), Some(&json!(true)));
        assert_eq!(ctx.get("missing"), None);
    }

    #[test]
    fn pipeline_context_set_overwrites_value() {
        let mut ctx = PipelineContext::new();
        ctx.set("key".to_string(), json!(1));
        assert_eq!(ctx.get("key"), Some(&json!(1)));
        ctx.set("key".to_string(), json!(2));
        assert_eq!(ctx.get("key"), Some(&json!(2)));
    }

    #[test]
    fn pipeline_context_handles_complex_and_nested_data() {
        let mut ctx = PipelineContext::new();
        let obj = json!({"a": 1, "b": [2, 3]});
        ctx.set("obj".to_string(), obj.clone());
        assert_eq!(ctx.get("obj"), Some(&obj));
        if let Some(val) = ctx.get("obj") {
            assert_eq!(val["a"], json!(1));
            assert_eq!(val["b"][1], json!(3));
        }
    }

    #[test]
    fn pipeline_context_type_safe_access() {
        let mut ctx = PipelineContext::new();
        ctx.set("int".to_string(), json!(10));
        ctx.set("bool".to_string(), json!(false));
        assert_eq!(ctx.get("int").unwrap().as_u64(), Some(10));
        assert_eq!(ctx.get("bool").unwrap().as_bool(), Some(false));
        assert_eq!(ctx.get("int").unwrap().as_str(), None);
    }

    #[test]
    fn pipeline_context_clone_is_independent() {
        let mut ctx = PipelineContext::new();
        ctx.set("x".to_string(), json!(1));
        let mut clone = ctx.clone();
        clone.set("x".to_string(), json!(2));
        assert_eq!(ctx.get("x"), Some(&json!(1)));
        assert_eq!(clone.get("x"), Some(&json!(2)));
    }

    #[test]
    fn pipeline_context_stage_to_stage_example() {
        let mut ctx = PipelineContext::new();
        ctx.set("raw_input".to_string(), json!({
            "event_type": "user_login",
            "user_id": "user_123"
        }));
        // Simulate filter stage (avoid borrow conflict)
        if let Some(raw_input) = ctx.get("raw_input") {
            let event_type = raw_input["event_type"].as_str().unwrap_or("");
            if event_type == "user_login" {
                let user_id = raw_input["user_id"].clone();
                ctx.set("filter_passed".to_string(), json!(true));
                ctx.set("validated_user_id".to_string(), user_id);
            }
        }
        assert_eq!(ctx.get("filter_passed"), Some(&json!(true)));
        assert_eq!(ctx.get("validated_user_id"), Some(&json!("user_123")));
    }
}