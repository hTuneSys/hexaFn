// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! DataFormatter trait for hexaFn
//!
//! This trait defines the contract for transforming and validating raw data into formatted data using a specific schema and data format.
//!
//! # Example
//!
//! ```
//! use hexafn_run::DataFormatter;
//! use hexafn_run::Schema;
//! use hexafn_run::RawData;
//! use hexafn_run::FormattedData;
//! use hexafn_run::DataFormat;
//!
//! struct JsonSchema;
//! impl Schema for JsonSchema {
//!     fn data_format(&self) -> DataFormat { DataFormat::Json }
//!     fn validate(&self, data: &[u8]) -> bool {
//!         serde_json::from_slice::<serde_json::Value>(data).is_ok()
//!     }
//! }
//!
//! #[derive(Clone)]
//! struct JsonFormatter;
//! impl DataFormatter for JsonFormatter {
//!     fn transform(&self, input: RawData) -> Result<FormattedData, String> {
//!         // Example: just wrap as FormattedData
//!         Ok(FormattedData {
//!             structured_data: serde_json::from_slice(&input.content).unwrap_or_default(),
//!             schema: None,
//!             format: DataFormat::Json,
//!             validation_errors: vec![],
//!             metadata: input.metadata.clone(),
//!         })
//!     }
//! }
//!
//! let formatter = JsonFormatter;
//! let raw = RawData { content: br#"{"a":1}"#.to_vec(), content_type: "application/json".into(), encoding: "utf-8".into(), metadata: Default::default(), source: "test".into(), timestamp: chrono::Utc::now() };
//! let formatted = formatter.transform(raw).unwrap();
//! assert_eq!(formatted.format, DataFormat::Json);
//! ```
//!
//! # Test
//!
//! See module tests for more examples.

use crate::{DataFormat, FormattedData, RawData, Schema};
use dyn_clone::DynClone;

/// Trait for data transformation and schema validation.
///
/// Implement this trait for each supported data format (JSON, XML, CSV, etc).
///
/// # Example
///
/// ```
/// use hexafn_run::DataFormatter;
/// use hexafn_run::Schema;
/// use hexafn_run::RawData;
/// use hexafn_run::FormattedData;
/// use hexafn_run::DataFormat;
/// struct PlainTextSchema;
/// impl Schema for PlainTextSchema {
///     fn data_format(&self) -> DataFormat { DataFormat::PlainText }
///     fn validate(&self, data: &[u8]) -> bool { std::str::from_utf8(data).is_ok() }
/// }
///
/// #[derive(Clone)]
/// struct PlainTextFormatter;
/// impl DataFormatter for PlainTextFormatter {
///     fn transform(&self, input: RawData) -> Result<FormattedData, String> {
///         Ok(FormattedData {
///             structured_data: serde_json::json!({"text": String::from_utf8_lossy(&input.content)}),
///             schema: None,
///             format: DataFormat::PlainText,
///             validation_errors: vec![],
///             metadata: input.metadata.clone(),
///         })
///     }
/// }
/// let formatter = PlainTextFormatter;
/// let raw = RawData { content: b"hello".to_vec(), content_type: "text/plain".into(), encoding: "utf-8".into(), metadata: Default::default(), source: "test".into(), timestamp: chrono::Utc::now() };
/// let formatted = formatter.transform(raw).unwrap();
/// assert_eq!(formatted.format, DataFormat::PlainText);
/// ```
pub trait DataFormatter: DynClone + Send + Sync {
    /// Transforms raw data into formatted data.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::DataFormatter;
    /// use hexafn_run::RawData;
    /// use hexafn_run::FormattedData;
    /// use hexafn_run::DataFormat;
    /// use serde_json::json;
    ///
    /// #[derive(Clone)]
    /// struct DummyFormatter;
    /// impl DataFormatter for DummyFormatter {
    ///     fn transform(&self, input: RawData) -> Result<FormattedData, String> {
    ///         Ok(FormattedData {
    ///             structured_data: serde_json::json!({"dummy": true}),
    ///             schema: None,
    ///             format: DataFormat::Json,
    ///             validation_errors: vec![],
    ///             metadata: input.metadata.clone(),
    ///         })
    ///     }
    /// }
    /// let formatter = DummyFormatter;
    /// let raw = RawData { content: vec![], content_type: "application/json".into(), encoding: "utf-8".into(), metadata: Default::default(), source: "test".into(), timestamp: chrono::Utc::now() };
    /// let formatted = formatter.transform(raw).unwrap();
    /// assert_eq!(formatted.format, DataFormat::Json);
    /// ```
    fn transform(&self, input: RawData) -> Result<FormattedData, String>;

    /// Validates the raw data against the given schema.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::Schema;
    /// use hexafn_run::DataFormatter;
    /// use hexafn_run::RawData;
    /// use hexafn_run::DataFormat;
    /// use hexafn_run::FormattedData;
    /// use serde_json::json;
    /// struct JsonSchema;
    /// impl Schema for JsonSchema {
    ///     fn data_format(&self) -> DataFormat { DataFormat::Json }
    ///     fn validate(&self, data: &[u8]) -> bool {
    ///         serde_json::from_slice::<serde_json::Value>(data).is_ok()
    ///     }
    /// }
    ///
    /// #[derive(Clone)]
    /// struct JsonFormatter;
    /// impl DataFormatter for JsonFormatter {
    ///     fn transform(&self, input: RawData) -> Result<FormattedData, String> {
    ///         unimplemented!()
    ///     }
    ///     fn validate_schema(&self, data: RawData, schema: Box<dyn Schema>) -> Result<(), String> {
    ///         if schema.validate(&data.content) {
    ///             Ok(())
    ///         } else {
    ///             Err("Validation failed".into())
    ///         }
    ///     }
    /// }
    /// let formatter = JsonFormatter;
    /// let schema = Box::new(JsonSchema);
    /// let raw = RawData { content: br#"{"a":1}"#.to_vec(), content_type: "application/json".into(), encoding: "utf-8".into(), metadata: Default::default(), source: "test".into(), timestamp: chrono::Utc::now() };
    /// assert!(formatter.validate_schema(raw, schema).is_ok());
    /// ```
    fn validate_schema(&self, _data: RawData, _schema: Box<dyn Schema>) -> Result<(), String> {
        Ok(())
    }

    /// Normalizes types in the raw data and returns formatted data.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::DataFormatter;
    /// use hexafn_run::RawData;
    /// use hexafn_run:: FormattedData;
    /// use hexafn_run::DataFormat;
    /// use serde_json::json;
    ///
    /// #[derive(Clone)]
    /// struct DummyFormatter;
    /// impl DataFormatter for DummyFormatter {
    ///     fn transform(&self, input: RawData) -> Result<FormattedData, String> {
    ///         Ok(FormattedData {
    ///             structured_data: serde_json::json!({"dummy": true}),
    ///             schema: None,
    ///             format: DataFormat::Json,
    ///             validation_errors: vec![],
    ///             metadata: input.metadata.clone(),
    ///         })
    ///     }
    /// }
    /// let formatter = DummyFormatter;
    /// let raw = RawData { content: vec![], content_type: "application/json".into(), encoding: "utf-8".into(), metadata: Default::default(), source: "test".into(), timestamp: chrono::Utc::now() };
    /// let formatted = formatter.normalize_types(raw).unwrap();
    /// assert_eq!(formatted.format, DataFormat::Json);
    /// ```
    fn normalize_types(&self, data: RawData) -> Result<FormattedData, String> {
        self.transform(data)
    }

    /// Returns the supported data formats for this formatter.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::DataFormatter;
    /// use hexafn_run::DataFormat;
    /// use hexafn_run::RawData;
    /// use hexafn_run::FormattedData;
    ///
    /// #[derive(Clone)]
    /// struct CsvFormatter;
    /// impl DataFormatter for CsvFormatter {
    ///     fn transform(&self, _input: RawData) -> Result<FormattedData, String> { unimplemented!() }
    ///     fn get_supported_formats(&self) -> Vec<DataFormat> {
    ///         vec![DataFormat::Csv]
    ///     }
    /// }
    /// let formatter = CsvFormatter;
    /// assert_eq!(formatter.get_supported_formats(), vec![DataFormat::Csv]);
    /// ```
    fn get_supported_formats(&self) -> Vec<DataFormat> {
        vec![DataFormat::Json]
    }
}

dyn_clone::clone_trait_object!(DataFormatter);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DataFormat, FormattedData, RawData, Schema};
    use std::collections::HashMap;

    struct DummySchema;
    impl Schema for DummySchema {
        fn data_format(&self) -> DataFormat {
            DataFormat::Json
        }
        fn validate(&self, data: &[u8]) -> bool {
            serde_json::from_slice::<serde_json::Value>(data).is_ok()
        }
    }

    #[derive(Clone)]
    struct DummyFormatter;
    impl DataFormatter for DummyFormatter {
        fn transform(&self, input: RawData) -> Result<FormattedData, String> {
            Ok(FormattedData {
                structured_data: serde_json::json!({"dummy": true}),
                schema: None,
                format: DataFormat::Json,
                validation_errors: vec![],
                metadata: input.metadata.clone(),
            })
        }
        fn validate_schema(&self, data: RawData, schema: Box<dyn Schema>) -> Result<(), String> {
            if schema.validate(&data.content) {
                Ok(())
            } else {
                Err("Validation failed".into())
            }
        }
    }

    #[test]
    fn test_transform() {
        let formatter = DummyFormatter;
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

    #[test]
    fn test_validate_schema() {
        let formatter = DummyFormatter;
        let schema = Box::new(DummySchema);
        let raw = RawData {
            content: br#"{"a":1}"#.to_vec(),
            content_type: "application/json".into(),
            encoding: "utf-8".into(),
            metadata: HashMap::new(),
            source: "test".into(),
            timestamp: chrono::Utc::now(),
        };
        assert!(formatter.validate_schema(raw, schema).is_ok());
    }

    #[test]
    fn test_get_supported_formats() {
        let formatter = DummyFormatter;
        assert_eq!(formatter.get_supported_formats(), vec![DataFormat::Json]);
    }
}
