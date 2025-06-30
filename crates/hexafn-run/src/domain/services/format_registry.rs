// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! FormatterRegistry for hexaFn
//!
//! This registry provides a thread-safe way to register and retrieve data formatters by name.
//! It is used to manage multiple implementations of the `DataFormatter` trait.
//!
//! # Example
//!
//! ```
//! use hexafn_run::FormatterRegistry;
//! use hexafn_run::DataFormatter;
//! use hexafn_run::RawData;
//! use hexafn_run::FormattedData;
//! use hexafn_run::DataFormat;
//! use std::sync::Arc;
//!
//! #[derive(Clone)]
//! struct DummyFormatter;
//! impl DataFormatter for DummyFormatter {
//!     fn transform(&self, _input: RawData) -> Result<FormattedData, String> {
//!         Err("not implemented".into())
//!     }
//! }
//!
//! let registry = FormatterRegistry::new();
//! registry.register("dummy", Box::new(DummyFormatter));
//! assert!(registry.get("dummy").is_some());
//! ```
//!
//! # Test
//!
//! See module tests for more examples.

use crate::DataFormatter;
use dyn_clone::clone_box;
use std::collections::HashMap;
use std::sync::RwLock;

/// Thread-safe registry for data formatters.
///
/// Allows registering and retrieving `DataFormatter` implementations by name.
///
/// # Example
/// ```
/// use hexafn_run::FormatterRegistry;
/// use hexafn_run::DataFormatter;
/// use hexafn_run::RawData;
/// use hexafn_run::FormattedData;
///
/// #[derive(Clone)]
/// struct DummyFormatter;
/// impl DataFormatter for DummyFormatter {
///     fn transform(&self, _input: RawData) -> Result<FormattedData, String> {
///         Err("not implemented".into())
///     }
/// }
/// let registry = FormatterRegistry::new();
/// registry.register("dummy", Box::new(DummyFormatter));
/// assert!(registry.get("dummy").is_some());
/// ```
pub struct FormatterRegistry {
    /// A thread-safe registry for data formatters.
    map: RwLock<HashMap<String, Box<dyn DataFormatter>>>,
}

impl FormatterRegistry {
    /// Creates a new, empty registry.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::FormatterRegistry;
    /// let registry = FormatterRegistry::new();
    /// assert!(registry.get("any").is_none());
    /// ```
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }
    /// Registers a formatter under the given name.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::FormatterRegistry;
    /// use hexafn_run::DataFormatter;
    /// use hexafn_run::RawData;
    /// use hexafn_run::FormattedData;
    /// #[derive(Clone)]
    /// struct DummyFormatter;
    /// impl DataFormatter for DummyFormatter {
    ///     fn transform(&self, _input: RawData) -> Result<FormattedData, String> { Err("not implemented".into()) }
    /// }
    /// let registry = FormatterRegistry::new();
    /// registry.register("dummy", Box::new(DummyFormatter));
    /// assert!(registry.get("dummy").is_some());
    /// ```
    pub fn register(&self, name: &str, formatter: Box<dyn DataFormatter>) {
        self.map
            .write()
            .unwrap()
            .insert(name.to_string(), formatter);
    }
    /// Retrieves a formatter by name, if it exists.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::FormatterRegistry;
    /// use hexafn_run::DataFormatter;
    /// use hexafn_run::RawData;
    /// use hexafn_run::FormattedData;
    ///
    /// #[derive(Clone)]
    /// struct DummyFormatter;
    /// impl DataFormatter for DummyFormatter {
    ///     fn transform(&self, _input: RawData) -> Result<FormattedData, String> { Err("not implemented".into()) }
    /// }
    /// let registry = FormatterRegistry::new();
    /// registry.register("dummy", Box::new(DummyFormatter));
    /// assert!(registry.get("dummy").is_some());
    /// assert!(registry.get("unknown").is_none());
    /// ```
    pub fn get(&self, name: &str) -> Option<Box<dyn DataFormatter>> {
        self.map
            .read()
            .unwrap()
            .get(name)
            .map(|formatter| clone_box(&**formatter))
    }
}

//TODO add comments for methods and struct
impl Default for FormatterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DataFormat, DataFormatter, FormattedData, RawData};
    use std::collections::HashMap;

    #[derive(Clone)]
    struct DummyFormatter;
    impl DataFormatter for DummyFormatter {
        fn transform(&self, _input: RawData) -> Result<FormattedData, String> {
            Ok(FormattedData {
                structured_data: serde_json::json!({"dummy": true}),
                schema: None,
                format: DataFormat::Json,
                validation_errors: vec![],
                metadata: HashMap::new(),
            })
        }
    }

    #[test]
    fn test_register_and_get() {
        let registry = FormatterRegistry::new();
        registry.register("dummy", Box::new(DummyFormatter));
        assert!(registry.get("dummy").is_some());
        assert!(registry.get("unknown").is_none());
    }

    #[test]
    fn test_transform_via_registry() {
        let registry = FormatterRegistry::new();
        registry.register("dummy", Box::new(DummyFormatter));
        let formatter = registry.get("dummy").unwrap();
        let raw = RawData {
            content: br#"{"a":1}"#.to_vec(),
            content_type: "application/json".into(),
            encoding: "utf-8".into(),
            metadata: HashMap::new(),
            source: "test".into(),
            timestamp: chrono::Utc::now(),
        };
        let formatted = formatter.transform(raw).unwrap();
        assert_eq!(formatted.format, DataFormat::Json);
    }
}
