// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # PipelineContext Value Object
//!
//! This module defines the [`PipelineContext`] struct, a value object for shared context and data flow
//! between pipeline stages in the hexaFn framework. It provides a flexible, JSON-serializable key-value
//! store for passing information between stages, supporting the 6F lifecycle (Feed, Filter, Format, Function, Forward, Feedback).
//!
//! ## Usage Example
//!
//! ```rust
//! use hexafn_core::PipelineContext;
//! use serde_json::json;
//!
//! let mut context = PipelineContext::new();
//! context.set("user_id".to_string(), json!(123));
//! assert_eq!(context.get("user_id"), Some(&json!(123)));
//! ```
//!
//! ## DDD/Hexagonal Architecture Note
//!
//! This value object should be used in all pipeline-related domain contracts and value objects to ensure
//! a consistent, type-safe, and serializable context model throughout the system.

use std::collections::HashMap;

/// Shared context for pipeline execution.
///
/// Provides a data store for passing information between pipeline stages.
/// Each stage can read data set by previous stages and set data for subsequent stages.
///
/// # Example
/// ```
/// use hexafn_core::PipelineContext;
/// use serde_json::json;
/// let mut context = PipelineContext::new();
/// context.set("foo".to_string(), json!(42));
/// assert_eq!(context.get("foo"), Some(&json!(42)));
/// ```
#[derive(Debug, Clone)]
pub struct PipelineContext {
    /// Internal data storage using JSON values for flexibility
    pub data: HashMap<String, serde_json::Value>,
}

impl PipelineContext {
    /// Create a new empty pipeline context.
    ///
    /// # Example
    /// ```
    /// use hexafn_core::PipelineContext;
    /// let context = PipelineContext::new();
    /// assert_eq!(context.data.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Get a value from the context.
    ///
    /// # Example
    /// ```
    /// use hexafn_core::PipelineContext;
    /// use serde_json::json;
    /// let mut context = PipelineContext::new();
    /// context.set("foo".to_string(), json!(123));
    /// assert_eq!(context.get("foo"), Some(&json!(123)));
    /// assert_eq!(context.get("bar"), None);
    /// ```
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }

    /// Set a value in the context.
    ///
    /// # Example
    /// ```
    /// use hexafn_core::PipelineContext;
    /// use serde_json::json;
    /// let mut context = PipelineContext::new();
    /// context.set("foo".to_string(), json!(true));
    /// assert_eq!(context.get("foo"), Some(&json!(true)));
    /// ```
    pub fn set(&mut self, key: String, value: serde_json::Value) {
        self.data.insert(key, value);
    }
}

impl Default for PipelineContext {
    /// Create a default pipeline context.
    ///
    /// # Example
    /// ```
    /// use hexafn_core::PipelineContext;
    /// let context = PipelineContext::default();
    /// assert_eq!(context.data.len(), 0);
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
    fn test_new_and_default() {
        let ctx1 = PipelineContext::new();
        let ctx2 = PipelineContext::default();
        assert_eq!(ctx1.data.len(), 0);
        assert_eq!(ctx2.data.len(), 0);
    }

    #[test]
    fn test_set_and_get() {
        let mut ctx = PipelineContext::new();
        ctx.set("key1".to_string(), json!(123));
        ctx.set("key2".to_string(), json!("abc"));
        assert_eq!(ctx.get("key1"), Some(&json!(123)));
        assert_eq!(ctx.get("key2"), Some(&json!("abc")));
        assert_eq!(ctx.get("missing"), None);
    }

    #[test]
    fn test_overwrite_value() {
        let mut ctx = PipelineContext::new();
        ctx.set("counter".to_string(), json!(1));
        assert_eq!(ctx.get("counter"), Some(&json!(1)));
        ctx.set("counter".to_string(), json!(2));
        assert_eq!(ctx.get("counter"), Some(&json!(2)));
    }

    #[test]
    fn test_complex_data() {
        let mut ctx = PipelineContext::new();
        ctx.set("obj".to_string(), json!({"a": 1, "b": [2, 3]}));
        let val = ctx.get("obj").unwrap();
        assert_eq!(val["a"], 1);
        assert_eq!(val["b"], json!([2, 3]));
    }
}
