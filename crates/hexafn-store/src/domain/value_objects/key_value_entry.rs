// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # KeyValueEntry Value Object
//!
//! Represents a single key-value entry in the HexaStore engine.
//!
//! This struct encapsulates all data and metadata for a single record in the key-value store, including the key, value, creation and update timestamps, optional TTL (time-to-live), and arbitrary metadata for indexing or auditing.
//!
//! ## Example
//! ```rust
//! use chrono::Utc;
//! use hexafn_store::KeyValueEntry;
//! let entry = KeyValueEntry::new(
//!     "config:theme".to_string(),
//!     b"dark".to_vec(),
//!     None,
//! );
//! assert_eq!(entry.key, "config:theme");
//! ```

use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;

/// Represents a single key-value entry in the HexaStore engine.
///
/// This struct encapsulates all data and metadata for a single record in the key-value store, including the key, value, creation and update timestamps, optional TTL (time-to-live), and arbitrary metadata for indexing or auditing.
///
/// # Example
/// ```rust
/// use chrono::Utc;
/// use hexafn_store::KeyValueEntry;
/// let entry = KeyValueEntry::new(
///     "user:1".to_string(),
///     b"data".to_vec(),
///     None,
/// );
/// assert_eq!(entry.key, "user:1");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyValueEntry {
    /// The key of the entry.
    pub key: String,
    /// The value of the entry.
    pub value: Vec<u8>,
    /// Metadata associated with the entry.
    pub metadata: HashMap<String, String>,
    /// The creation timestamp of the entry.
    pub created_at: DateTime<Utc>,
    /// The last updated timestamp of the entry.
    pub updated_at: DateTime<Utc>,
    /// The time-to-live (TTL) duration of the entry, if any.
    pub ttl: Option<Duration>,
}

impl std::fmt::Display for KeyValueEntry {
    /// Formats the key-value entry for display.
    ///
    /// # Example
    /// ```rust
    /// use chrono::Utc;
    /// use hexafn_store::KeyValueEntry;
    /// let entry = KeyValueEntry::new(
    ///     "k".to_string(),
    ///     vec![1, 2, 3],
    ///     None,
    /// );
    /// let s = format!("{}", entry);
    /// assert!(s.contains("key: 'k'"));
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KeyValueEntry[key: '{}', value: {} bytes, created_at: {}, updated_at: {}, ttl: {}]",
            self.key,
            self.value.len(),
            self.created_at,
            self.updated_at,
            match &self.ttl {
                Some(ttl) => format!("{}s", ttl.num_seconds()),
                None => "None".to_string(),
            }
        )
    }
}

impl KeyValueEntry {
    /// Creates a new `KeyValueEntry` with the given key, value, and optional TTL.
    ///
    /// # Example
    /// ```rust
    /// use chrono::Utc;
    /// use hexafn_store::KeyValueEntry;
    /// let entry = KeyValueEntry::new(
    ///     "user:1".to_string(),
    ///     b"data".to_vec(),
    ///     None,
    /// );
    /// assert_eq!(entry.key, "user:1");
    /// ```
    pub fn new(key: String, value: Vec<u8>, ttl: Option<Duration>) -> Self {
        let now = Utc::now();
        Self {
            key,
            value,
            metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
            ttl,
        }
    }

    /// Returns true if the entry is expired, given the current time.
    ///
    /// # Example
    /// ```rust
    /// use chrono::{Utc, Duration};
    /// use hexafn_store::KeyValueEntry;
    /// let mut entry = KeyValueEntry::new("k".to_string(), vec![], Some(Duration::seconds(1)));
    /// assert!(!entry.is_expired(Utc::now()));
    /// ```
    pub fn is_expired(&self, now: DateTime<Utc>) -> bool {
        match self.ttl {
            Some(ttl) => self.created_at + ttl < now,
            None => false,
        }
    }

    /// Gets a metadata value by key.
    ///
    /// # Example
    /// ```rust
    /// use chrono::Utc;
    /// use hexafn_store::KeyValueEntry;
    /// let mut entry = KeyValueEntry::new("k".to_string(), vec![], None);
    /// entry.metadata.insert("foo".to_string(), "bar".to_string());
    /// assert_eq!(entry.get_metadata("foo"), Some(&"bar".to_string()));
    /// ```
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_new_entry() {
        let entry = KeyValueEntry::new("a".to_string(), b"b".to_vec(), None);
        assert_eq!(entry.key, "a");
        assert_eq!(entry.value, b"b".to_vec());
        assert!(entry.metadata.is_empty());
        assert!(entry.ttl.is_none());
    }

    #[test]
    fn test_display() {
        let entry = KeyValueEntry::new("k".to_string(), vec![1, 2, 3], Some(Duration::seconds(10)));
        let s = format!("{}", entry);
        assert!(s.contains("key: 'k'"));
        assert!(s.contains("value: 3 bytes"));
        assert!(s.contains("ttl: 10s"));
    }

    #[test]
    fn test_is_expired() {
        let entry = KeyValueEntry {
            key: "k".to_string(),
            value: vec![],
            metadata: HashMap::new(),
            created_at: Utc::now() - Duration::seconds(20),
            updated_at: Utc::now() - Duration::seconds(20),
            ttl: Some(Duration::seconds(10)),
        };
        assert!(entry.is_expired(Utc::now()));
    }

    #[test]
    fn test_metadata_access() {
        let mut entry = KeyValueEntry::new("k".to_string(), vec![], None);
        entry.metadata.insert("foo".to_string(), "bar".to_string());
        assert_eq!(entry.get_metadata("foo"), Some(&"bar".to_string()));
        assert_eq!(entry.get_metadata("baz"), None);
    }

    #[test]
    fn test_expired_false_when_no_ttl() {
        let entry = KeyValueEntry::new("k".to_string(), vec![], None);
        assert!(!entry.is_expired(Utc::now()));
    }

    #[test]
    fn test_metadata_empty() {
        let entry = KeyValueEntry::new("k".to_string(), vec![], None);
        assert!(entry.metadata.is_empty());
    }
}
