// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! Data format value object for hexaFn
//!
//! Represents supported data serialization formats in hexaFn.
//!
//! # Example
//!
//! ```
//! use hexafn_run::DataFormat;
//! use std::str::FromStr;
//! assert_eq!(DataFormat::from_str("json"), Ok(DataFormat::Json));
//! assert_eq!(DataFormat::Toml.as_str(), "toml");
//! ```
//!
//! # Test
//!
//! ```
//! use hexafn_run::DataFormat;
//! use std::str::FromStr;
//! assert_eq!(DataFormat::from_str("unknown"), Err(()));
//! ```

use std::str::FromStr;

/// Represents supported data serialization formats in hexaFn.
///
/// # Example
///
/// ```
/// use hexafn_run::DataFormat;
/// use std::str::FromStr;
/// assert_eq!(DataFormat::from_str("json"), Ok(DataFormat::Json));
/// assert_eq!(DataFormat::Toml.as_str(), "toml");
/// ```
///
/// # Test
///
/// ```
/// use hexafn_run::DataFormat;
/// use std::str::FromStr;
/// assert_eq!(DataFormat::from_str("unknown"), Err(()));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFormat {
    /// JavaScript Object Notation
    Json,
    /// Extensible Markup Language
    Xml,
    /// Comma-Separated Values
    Csv,
    /// YAML Ain't Markup Language
    Yaml,
    /// Tom's Obvious, Minimal Language
    Toml,
    /// Arbitrary binary data
    Binary,
    /// UTF-8 plain text
    PlainText,
}

impl DataFormat {
    /// Returns the canonical lowercase string representation of the format.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::DataFormat;
    /// assert_eq!(DataFormat::Yaml.as_str(), "yaml");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            DataFormat::Json => "json",
            DataFormat::Xml => "xml",
            DataFormat::Csv => "csv",
            DataFormat::Yaml => "yaml",
            DataFormat::Toml => "toml",
            DataFormat::Binary => "binary",
            DataFormat::PlainText => "plaintext",
        }
    }
}

impl FromStr for DataFormat {
    type Err = ();

    /// Parse a string into a DataFormat.
    ///
    /// # Example
    /// ```
    /// use hexafn_run::DataFormat;
    /// let format = "csv".parse::<DataFormat>();
    /// assert_eq!(format, Ok(DataFormat::Csv));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "json" => Ok(DataFormat::Json),
            "xml" => Ok(DataFormat::Xml),
            "csv" => Ok(DataFormat::Csv),
            "yaml" => Ok(DataFormat::Yaml),
            "toml" => Ok(DataFormat::Toml),
            "binary" => Ok(DataFormat::Binary),
            "plaintext" | "plain" | "text" => Ok(DataFormat::PlainText),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_str() {
        assert_eq!(DataFormat::Json.as_str(), "json");
        assert_eq!(DataFormat::Xml.as_str(), "xml");
        assert_eq!(DataFormat::Csv.as_str(), "csv");
        assert_eq!(DataFormat::Yaml.as_str(), "yaml");
        assert_eq!(DataFormat::Toml.as_str(), "toml");
        assert_eq!(DataFormat::Binary.as_str(), "binary");
        assert_eq!(DataFormat::PlainText.as_str(), "plaintext");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(DataFormat::from_str("json"), Ok(DataFormat::Json));
        assert_eq!(DataFormat::from_str("XML"), Ok(DataFormat::Xml));
        assert_eq!(DataFormat::from_str("Csv"), Ok(DataFormat::Csv));
        assert_eq!(DataFormat::from_str("yaml"), Ok(DataFormat::Yaml));
        assert_eq!(DataFormat::from_str("TOML"), Ok(DataFormat::Toml));
        assert_eq!(DataFormat::from_str("binary"), Ok(DataFormat::Binary));
        assert_eq!(DataFormat::from_str("plaintext"), Ok(DataFormat::PlainText));
        assert_eq!(DataFormat::from_str("plain"), Ok(DataFormat::PlainText));
        assert_eq!(DataFormat::from_str("text"), Ok(DataFormat::PlainText));
        assert_eq!(DataFormat::from_str("unknown"), Err(()));
    }
}
