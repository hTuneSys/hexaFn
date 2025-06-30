// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! Schema trait for data format validation in hexaFn
//!
//! This trait defines a contract for checking if a given byte slice matches a specific data format (JSON, XML, CSV, etc).
//! Implementations should provide format-specific validation logic.
//!
//! # Example
//!
//! ```
//! use hexafn_run::Schema;
//! use hexafn_run::DataFormat;
//!
//! struct JsonSchema;
//!
//! impl Schema for JsonSchema {
//!     fn data_format(&self) -> DataFormat {
//!         DataFormat::Json
//!     }
//!     fn validate(&self, data: &[u8]) -> bool {
//!         serde_json::from_slice::<serde_json::Value>(data).is_ok()
//!     }
//! }
//!
//! let schema = JsonSchema;
//! assert_eq!(schema.data_format(), DataFormat::Json);
//! assert!(schema.validate(br#"{"a":1}"#));
//! assert!(!schema.validate(br"not json"));
//! ```
//!
//! # Test
//!
//! See the module tests for more examples.

use crate::DataFormat;

/// Trait for data format schema validation.
///
/// Implement this trait for each supported data format (JSON, XML, CSV, etc).
///
/// # Example
///
/// ```
/// use hexafn_run::Schema;
/// use hexafn_run::DataFormat;
///
/// struct PlainTextSchema;
///
/// impl Schema for PlainTextSchema {
///     fn data_format(&self) -> DataFormat {
///         DataFormat::PlainText
///     }
///     fn validate(&self, data: &[u8]) -> bool {
///         std::str::from_utf8(data).is_ok()
///     }
/// }
///
/// let schema = PlainTextSchema;
/// assert_eq!(schema.data_format(), DataFormat::PlainText);
/// assert!(schema.validate(b"hello world"));
/// assert!(!schema.validate(&[0, 159, 146, 150]));
/// ```
pub trait Schema: Send + Sync {
    /// Returns the supported data format for this schema.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::Schema;
    /// use hexafn_run::DataFormat;
    /// struct CsvSchema;
    /// impl Schema for CsvSchema {
    ///     fn data_format(&self) -> DataFormat { DataFormat::Csv }
    ///     fn validate(&self, _data: &[u8]) -> bool { true }
    /// }
    /// let schema = CsvSchema;
    /// assert_eq!(schema.data_format(), DataFormat::Csv);
    /// ```
    fn data_format(&self) -> DataFormat;

    /// Checks if the given data matches the schema's format.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::Schema;
    /// use hexafn_run::DataFormat;
    /// use serde_yaml;
    ///
    /// struct YamlSchema;
    /// impl Schema for YamlSchema {
    ///     fn data_format(&self) -> DataFormat { DataFormat::Yaml }
    ///     fn validate(&self, data: &[u8]) -> bool {
    ///         serde_yaml::from_slice::<serde_yaml::Value>(data).is_ok()
    ///     }
    /// }
    /// let schema = YamlSchema;
    /// assert!(schema.validate(b"foo: bar\nnum: 42"));
    /// assert!(!schema.validate(b"not: yaml: ["));
    /// ```
    fn validate(&self, data: &[u8]) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DataFormat;

    struct JsonSchema;
    impl Schema for JsonSchema {
        fn data_format(&self) -> DataFormat {
            DataFormat::Json
        }
        fn validate(&self, data: &[u8]) -> bool {
            serde_json::from_slice::<serde_json::Value>(data).is_ok()
        }
    }

    struct PlainTextSchema;
    impl Schema for PlainTextSchema {
        fn data_format(&self) -> DataFormat {
            DataFormat::PlainText
        }
        fn validate(&self, data: &[u8]) -> bool {
            std::str::from_utf8(data).is_ok()
        }
    }

    #[test]
    fn test_json_schema_valid() {
        let schema = JsonSchema;
        assert!(schema.validate(br#"{"a":1}"#));
        assert!(!schema.validate(br"not json"));
    }

    #[test]
    fn test_plain_text_schema_valid() {
        let schema = PlainTextSchema;
        assert!(schema.validate(b"hello world"));
        assert!(!schema.validate(&[0, 159, 146, 150]));
    }

    #[test]
    fn test_data_format_enum() {
        let schema = JsonSchema;
        assert_eq!(schema.data_format(), DataFormat::Json);
        let schema = PlainTextSchema;
        assert_eq!(schema.data_format(), DataFormat::PlainText);
    }
}
