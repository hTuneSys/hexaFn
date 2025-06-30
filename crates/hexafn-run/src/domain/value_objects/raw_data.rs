// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! RawData value object for hexaFn
//!
//! Represents raw input data with metadata and source information.
//!
//! # Example
//!
//! ```
//! use hexafn_run::RawData;
//! use chrono::Utc;
//! use std::collections::HashMap;
//!
//! let data = RawData {
//!     content: b"hello".to_vec(),
//!     content_type: "text/plain".to_string(),
//!     encoding: "utf-8".to_string(),
//!     metadata: HashMap::new(),
//!     source: "test".to_string(),
//!     timestamp: Utc::now(),
//! };
//! assert_eq!(data.content_type, "text/plain");
//! ```
//!
//! # Test
//!
//! ```
//! use hexafn_run::RawData;
//! use chrono::Utc;
//! use std::collections::HashMap;
//!
//! let data = RawData {
//!     content: vec![],
//!     content_type: "application/json".to_string(),
//!     encoding: "utf-8".to_string(),
//!     metadata: HashMap::new(),
//!     source: "unit-test".to_string(),
//!     timestamp: Utc::now(),
//! };
//! assert_eq!(data.content_type, "application/json");
//! ```

/// Raw input data with metadata and source information.
///
/// # Example
///
/// ```
/// use hexafn_run::RawData;
/// use chrono::Utc;
/// use std::collections::HashMap;
/// let data = RawData {
///     content: b"abc".to_vec(),
///     content_type: "text/plain".to_string(),
///     encoding: "utf-8".to_string(),
///     metadata: HashMap::new(),
///     source: "example".to_string(),
///     timestamp: Utc::now(),
/// };
/// assert_eq!(data.source, "example");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawData {
    /// The raw content as a byte vector.
    pub content: Vec<u8>,
    /// The MIME type of the content.
    pub content_type: String,
    /// The character encoding of the content.
    pub encoding: String,
    /// Additional metadata associated with the data.
    pub metadata: std::collections::HashMap<String, String>,
    /// The source of the data, e.g., "user input", "file upload".
    pub source: String,
    /// The timestamp when the data was created or received.
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl std::fmt::Display for RawData {
    /// Formats the RawData for display.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::RawData;
    /// use chrono::Utc;
    /// use std::collections::HashMap;
    /// let data = RawData {
    ///     content: b"abc".to_vec(),
    ///     content_type: "text/plain".to_string(),
    ///     encoding: "utf-8".to_string(),
    ///     metadata: HashMap::new(),
    ///     source: "display-doc".to_string(),
    ///     timestamp: Utc::now(),
    /// };
    /// let s = format!("{}", data);
    /// assert!(s.contains("RawData"));
    /// assert!(s.contains("content_type: text/plain"));
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RawData {{ content: [u8; {}], content_type: {}, encoding: {}, source: {}, timestamp: {} }}",
            self.content.len(),
            self.content_type,
            self.encoding,
            self.source,
            self.timestamp
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::collections::HashMap;

    #[test]
    fn test_raw_data_fields() {
        let mut meta = HashMap::new();
        meta.insert("k".to_string(), "v".to_string());
        let data = RawData {
            content: b"data".to_vec(),
            content_type: "application/octet-stream".to_string(),
            encoding: "utf-8".to_string(),
            metadata: meta.clone(),
            source: "unit-test".to_string(),
            timestamp: Utc::now(),
        };
        assert_eq!(data.content, b"data".to_vec());
        assert_eq!(data.content_type, "application/octet-stream");
        assert_eq!(data.encoding, "utf-8");
        assert_eq!(data.metadata, meta);
        assert_eq!(data.source, "unit-test");
    }

    #[test]
    fn test_display() {
        let data = RawData {
            content: b"abc".to_vec(),
            content_type: "text/plain".to_string(),
            encoding: "utf-8".to_string(),
            metadata: HashMap::new(),
            source: "display-test".to_string(),
            timestamp: Utc::now(),
        };
        let s = format!("{data}");
        assert!(s.contains("RawData"));
        assert!(s.contains("content_type: text/plain"));
        assert!(s.contains("encoding: utf-8"));
        assert!(s.contains("source: display-test"));
    }
}
