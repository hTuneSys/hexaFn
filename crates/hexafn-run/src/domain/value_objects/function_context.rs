// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! FunctionContext for hexaFn
//!
//! This struct represents the execution context for a function, including inputs, metadata, and environment variables.
//!
//! # Example
//!
//! ```
//! use hexafn_run::FunctionContext;
//! use serde_json::json;
//!
//! let mut ctx = FunctionContext::new();
//! ctx.inputs.insert("x".to_string(), json!(42));
//! ctx.metadata.insert("trace_id".to_string(), "abc123".to_string());
//! ctx.environment.insert("PATH".to_string(), "/usr/bin".to_string());
//! assert_eq!(ctx.get_input("x"), Some(&json!(42)));
//! assert_eq!(ctx.get_metadata("trace_id"), Some(&"abc123".to_string()));
//! ```
//!
//! # Test
//!
//! See module tests for more examples.

/// Execution context for a function, including inputs, metadata, and environment variables.
///
/// # Example
/// ```
/// use hexafn_run::FunctionContext;
/// use serde_json::json;
/// let mut ctx = FunctionContext::new();
/// ctx.inputs.insert("foo".to_string(), json!(123));
/// assert_eq!(ctx.get_input("foo"), Some(&json!(123)));
/// ```
pub struct FunctionContext {
    /// Input parameters for the function
    pub inputs: std::collections::HashMap<String, serde_json::Value>,
    /// Metadata about the function execution
    pub metadata: std::collections::HashMap<String, String>,
    /// Environment variables for the function execution
    pub environment: std::collections::HashMap<String, String>,
}

impl FunctionContext {
    /// Creates a new `FunctionContext` with empty inputs, metadata, and environment.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::FunctionContext;
    /// let ctx = FunctionContext::new();
    /// assert!(ctx.inputs.is_empty());
    /// assert!(ctx.metadata.is_empty());
    /// assert!(ctx.environment.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            inputs: std::collections::HashMap::new(),
            metadata: std::collections::HashMap::new(),
            environment: std::collections::HashMap::new(),
        }
    }

    /// Gets an input value by key.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::FunctionContext;
    /// use serde_json::json;
    /// let mut ctx = FunctionContext::new();
    /// ctx.inputs.insert("foo".to_string(), json!(1));
    /// assert_eq!(ctx.get_input("foo"), Some(&json!(1)));
    /// ```
    pub fn get_input(&self, key: &str) -> Option<&serde_json::Value> {
        self.inputs.get(key)
    }

    /// Sets an output value by key (alias for inserting into inputs).
    ///
    /// # Example
    /// ```
    /// use hexafn_run::FunctionContext;
    /// use serde_json::json;
    /// let mut ctx = FunctionContext::new();
    /// ctx.set_output("result".to_string(), json!(99));
    /// assert_eq!(ctx.get_input("result"), Some(&json!(99)));
    /// ```
    pub fn set_output(&mut self, key: String, value: serde_json::Value) {
        self.inputs.insert(key, value);
    }

    /// Gets metadata by key.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::FunctionContext;
    /// let mut ctx = FunctionContext::new();
    /// ctx.metadata.insert("trace_id".to_string(), "abc123".to_string());
    /// assert_eq!(ctx.get_metadata("trace_id"), Some(&"abc123".to_string()));
    /// ```
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

//TODO add comment
impl Default for FunctionContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new_context_is_empty() {
        let ctx = FunctionContext::new();
        assert!(ctx.inputs.is_empty());
        assert!(ctx.metadata.is_empty());
        assert!(ctx.environment.is_empty());
    }

    #[test]
    fn test_get_and_set_input() {
        let mut ctx = FunctionContext::new();
        ctx.inputs.insert("foo".to_string(), json!(123));
        assert_eq!(ctx.get_input("foo"), Some(&json!(123)));
        ctx.set_output("bar".to_string(), json!(456));
        assert_eq!(ctx.get_input("bar"), Some(&json!(456)));
    }

    #[test]
    fn test_get_metadata() {
        let mut ctx = FunctionContext::new();
        ctx.metadata
            .insert("trace_id".to_string(), "abc123".to_string());
        assert_eq!(ctx.get_metadata("trace_id"), Some(&"abc123".to_string()));
        assert_eq!(ctx.get_metadata("missing"), None);
    }

    #[test]
    fn test_environment() {
        let mut ctx = FunctionContext::new();
        ctx.environment
            .insert("PATH".to_string(), "/usr/bin".to_string());
        assert_eq!(ctx.environment.get("PATH"), Some(&"/usr/bin".to_string()));
    }
}
