// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! FormattedData value object for hexaFn
//!
//! Represents normalized and validated data with optional schema and metadata.
//!
//! # Example
//!
//! ```
//! use hexafn_run::FormattedData;
//! use hexafn_run::DataFormat;
//! use serde_json::json;
//! use std::collections::HashMap;
//! let data = FormattedData {
//!     structured_data: json!({"k": 1}),
//!     schema: None,
//!     format: DataFormat::Json,
//!     validation_errors: vec![],
//!     metadata: HashMap::new(),
//! };
//! assert_eq!(data.format.as_str(), "json");
//! ```
//!
//! # Test
//!
//! ```
//! use hexafn_run::FormattedData;
//! use hexafn_run::DataFormat;
//! use serde_json::json;
//! use std::collections::HashMap;
//! let data = FormattedData {
//!     structured_data: json!({}),
//!     schema: None,
//!     format: DataFormat::Json,
//!     validation_errors: vec![],
//!     metadata: HashMap::new(),
//! };
//! assert!(data.structured_data.is_object());
//! ```

use crate::Schema;

/// Normalized and validated data with optional schema and metadata.
///
/// # Example
/// ```
/// use hexafn_run::FormattedData;
/// use hexafn_run::DataFormat;
/// use serde_json::json;
/// use std::collections::HashMap;
/// let data = FormattedData {
///     structured_data: json!({"foo": "bar"}),
///     schema: None,
///     format: DataFormat::Json,
///     validation_errors: vec![],
///     metadata: HashMap::new(),
/// };
/// assert_eq!(data.format.as_str(), "json");
/// ```
pub struct FormattedData {
    /// Structured data in JSON format
    pub structured_data: serde_json::Value,
    /// Optional schema for validation
    pub schema: Option<Box<dyn Schema>>,
    /// Data format used for serialization
    pub format: DataFormat,
    /// Validation errors if any
    pub validation_errors: Vec<Box<dyn HexaError>>,
    /// Additional metadata
    pub metadata: std::collections::HashMap<String, String>,
}

use crate::DataFormat;
use hexafn_core::HexaError;
use std::fmt;

impl fmt::Display for FormattedData {
    /// Formats the FormattedData for display.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::FormattedData;
    /// use hexafn_run::DataFormat;
    /// use serde_json::json;
    /// use std::collections::HashMap;
    /// let data = FormattedData {
    ///     structured_data: json!({"foo": 1}),
    ///     schema: None,
    ///     format: DataFormat::Json,
    ///     validation_errors: vec![],
    ///     metadata: HashMap::new(),
    /// };
    /// let s = format!("{}", data);
    /// assert!(s.contains("FormattedData"));
    /// assert!(s.contains("format: json"));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FormattedData {{ format: {}, schema: {}, validation_errors: {}, metadata: {} }}",
            self.format.as_str(),
            if self.schema.is_some() {
                "Some"
            } else {
                "None"
            },
            self.validation_errors.len(),
            self.metadata.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hexafn_core::HexaCoreError;
    use serde_json::json;
    use std::collections::HashMap;

    fn dummy_schema() -> Option<Box<dyn Schema>> {
        None
    }
    fn dummy_error() -> Box<dyn HexaError> {
        // Replace with a real error if available
        Box::new(HexaCoreError::new("run.formatted_data.dummy_error"))
    }

    #[test]
    fn test_struct_fields() {
        let data = FormattedData {
            structured_data: json!({"a": 1}),
            schema: dummy_schema(),
            format: DataFormat::Json,
            validation_errors: vec![],
            metadata: HashMap::new(),
        };
        assert_eq!(data.format, DataFormat::Json);
        assert!(data.structured_data.is_object());
    }

    #[test]
    fn test_with_validation_error() {
        let err = dummy_error();
        let data = FormattedData {
            structured_data: json!({}),
            schema: None,
            format: DataFormat::Json,
            validation_errors: vec![err],
            metadata: HashMap::new(),
        };
        assert_eq!(data.validation_errors.len(), 1);
    }

    #[test]
    fn test_display() {
        let data = FormattedData {
            structured_data: json!({"foo": 1}),
            schema: None,
            format: DataFormat::Json,
            validation_errors: vec![],
            metadata: HashMap::new(),
        };
        let s = format!("{}", data);
        assert!(s.contains("FormattedData"));
        assert!(s.contains("format: json"));
    }
}
