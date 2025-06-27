// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Key-Value Store Domain Contracts
//!
//! This module defines the core types and contracts for the HexaStore key-value storage engine in the hexaFn system.
//!
//! ## Overview
//!
//! - [`KeyValueEntry`]: Represents a single key-value record, including metadata, timestamps, and optional TTL.
//! - [`KvOp`]: Enumerates supported atomic operations for transactional updates.
//! - [`Namespace`]: Provides logical scoping for keys, supporting multi-tenant and modular storage.
//! - [`KvStore`]: Trait for generic, pluggable key-value store implementations (in-memory, file, or external).
//!
//! ## Example Usage
//!
//! ```rust
//! use hexafn_store::domain::contracts::{KeyValueEntry, Namespace, KvOp};
//! use chrono::Utc;
//! use std::collections::HashMap;
//! let ns = Namespace::new("users".to_string()).unwrap();
//! let entry = KeyValueEntry {
//!     key: "user:1".to_string(),
//!     value: b"alice".to_vec(),
//!     metadata: HashMap::new(),
//!     created_at: Utc::now(),
//!     updated_at: Utc::now(),
//!     ttl: None,
//! };
//! assert_eq!(entry.key, "user:1");
//! assert!(ns.is_valid());
//! let op = KvOp::Put;
//! assert_eq!(format!("{}", op), "Put");
//! ```
//!
//! ## Unit Test
//!
//! ```rust
//! use hexafn_store::domain::contracts::{KeyValueEntry, Namespace, KvOp};
//! use chrono::Utc;
//! use std::collections::HashMap;
//! let entry = KeyValueEntry {
//!     key: "k1".to_string(),
//!     value: b"v1".to_vec(),
//!     metadata: HashMap::new(),
//!     created_at: Utc::now(),
//!     updated_at: Utc::now(),
//!     ttl: Some(chrono::Duration::seconds(60)),
//! };
//! assert!(format!("{}", entry).contains("k1"));
//! let ns = Namespace::new("abc".to_string()).unwrap();
//! assert_eq!(ns.value, "abc");
//! assert!(ns.is_valid());
//! assert_eq!(format!("{}", ns), "Namespace('abc')");
//! assert!(Namespace::new("".to_string()).is_err());
//! assert_eq!(format!("{}", KvOp::Put), "Put");
//! ```

use chrono::{DateTime, Duration, Utc};
use hexafn_core::{HexaCoreError, HexaError};
use std::collections::HashMap;

/// Represents a single key-value entry in the HexaStore engine.
///
/// This struct encapsulates all data and metadata for a single record in the key-value store, including the key, value, creation and update timestamps, optional TTL (time-to-live), and arbitrary metadata for indexing or auditing.
///
/// # Use Cases
/// - Storing user sessions, configuration, or cache data
/// - Tracking creation and update times for audit or expiration
/// - Attaching metadata for search, tags, or access control
///
/// # Example
/// ```rust
/// use hexafn_store::domain::contracts::KeyValueEntry;
/// use chrono::Utc;
/// let entry = KeyValueEntry {
///     key: "config:theme".to_string(),
///     value: b"dark".to_vec(),
///     metadata: std::collections::HashMap::new(),
///     created_at: Utc::now(),
///     updated_at: Utc::now(),
///     ttl: None,
/// };
/// assert_eq!(entry.key, "config:theme");
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

/// Supported atomic operations for transactional updates in the key-value store.
///
/// This enum is used to express a batch of changes (put or delete) that should be applied atomically to the store. Useful for implementing transactions, migrations, or batch updates.
///
/// # Example
/// ```rust
/// use hexafn_store::domain::contracts::KvOp;
/// let op = KvOp::Put;
/// assert_eq!(format!("{}", op), "Put");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KvOp {
    /// Put operation for adding or updating an entry.
    Put,
    /// Delete operation for removing an entry.
    Delete,
}

impl std::fmt::Display for KvOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KvOp::Put => write!(f, "Put"),
            KvOp::Delete => write!(f, "Delete"),
        }
    }
}

/// Logical namespace for key scoping in the HexaStore engine.
///
/// Namespaces allow for multi-tenant, modular, or isolated storage within a single key-value backend. Each namespace acts as a logical partition for keys, supporting use cases like per-user, per-module, or per-environment data separation.
///
/// # Use Cases
/// - Isolating data for different users, teams, or modules
/// - Supporting multi-tenant SaaS or plugin architectures
/// - Enforcing access control or retention policies per namespace
///
/// # Example
/// ```rust
/// use hexafn_store::domain::contracts::Namespace;
/// let ns = Namespace::new("settings".to_string()).unwrap();
/// assert_eq!(ns.value, "settings");
/// assert!(ns.is_valid());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Namespace {
    /// The string value of the namespace.
    pub value: String,
}

impl Namespace {
    /// Create a new namespace. Returns error if name is empty or contains invalid chars.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_store::domain::contracts::Namespace;
    /// let ns = Namespace::new("users".to_string());
    /// assert!(ns.is_ok());
    /// let ns = Namespace::new("".to_string());
    /// assert!(ns.is_err());
    /// ```
    pub fn new(name: String) -> Result<Self, Box<dyn HexaError>> {
        if name.trim().is_empty() {
            return Err(Box::new(
                HexaCoreError::new("store.namespace.validation.empty")
                    .with_message("Namespace name cannot be empty"),
            ));
        }
        // Optionally: add more validation for allowed chars
        Ok(Self { value: name })
    }
    /// Returns true if the namespace is valid (non-empty, no spaces).
    ///
    /// # Example
    /// ```rust
    /// use hexafn_store::domain::contracts::Namespace;
    /// let ns = Namespace::new("users".to_string()).unwrap();
    /// assert!(ns.is_valid());
    /// let ns = Namespace { value: "".to_string() };
    /// assert!(!ns.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        !self.value.trim().is_empty() && !self.value.contains(' ')
    }
}

impl std::fmt::Display for Namespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Namespace('{}')", self.value)
    }
}

/// Trait for a generic, pluggable key-value store contract.
///
/// This trait defines the required interface for any HexaStore backend implementation, including in-memory, file-based, or external (e.g., Redis, RocksDB) stores. It supports basic CRUD operations, listing, existence checks, and atomic transactions.
///
/// # Use Cases
/// - Abstracting over different storage backends for testing or deployment
/// - Supporting hot-swappable or multi-backend architectures
/// - Enabling atomic batch updates for consistency and migration
///
/// # Example
/// ```rust
/// use hexafn_store::domain::contracts::{KvStore, Namespace, KeyValueEntry,KvOp};
/// struct MyStore;
/// impl KvStore for MyStore {
///     fn get(&self, _namespace: &Namespace, _key: &str) -> Result<Option<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(None) }
///     fn put(&mut self, _namespace: &Namespace, _key: &str, _value: KeyValueEntry) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
///     fn delete(&mut self, _namespace: &Namespace, _key: &str) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
///     fn list(&self, _namespace: &Namespace) -> Result<Vec<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(vec![]) }
///     fn exists(&self, _namespace: &Namespace, _key: &str) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(false) }
///     fn transaction(&mut self, _ops: Vec<KvOp>) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
/// }
/// ```
pub trait KvStore {
    /// Get a value by namespace and key.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_store::domain::contracts::{KvStore, Namespace, KeyValueEntry,KvOp};
    /// # struct MyStore;
    /// # impl KvStore for MyStore {
    /// #   fn get(&self, _namespace: &Namespace, _key: &str) -> Result<Option<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(None) }
    /// #   fn put(&mut self, _namespace: &Namespace, _key: &str, _value: KeyValueEntry) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// #   fn delete(&mut self, _namespace: &Namespace, _key: &str) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// #   fn list(&self, _namespace: &Namespace) -> Result<Vec<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(vec![]) }
    /// #   fn exists(&self, _namespace: &Namespace, _key: &str) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(false) }
    /// #   fn transaction(&mut self, _ops: Vec<KvOp>) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// # }
    /// let store = MyStore;
    /// let ns = Namespace::new("users".to_string()).unwrap();
    /// let result = store.get(&ns, "user:1");
    /// assert!(result.is_ok());
    /// ```
    fn get(
        &self,
        namespace: &Namespace,
        key: &str,
    ) -> Result<Option<KeyValueEntry>, Box<dyn HexaError>>;
    /// Put a value by namespace and key.
    ///
    /// # Example
    /// ```rust
    /// # use hexafn_store::domain::contracts::{KvStore, Namespace, KeyValueEntry,KvOp};
    /// # struct MyStore;
    /// # impl KvStore for MyStore {
    /// #   fn get(&self, _namespace: &Namespace, _key: &str) -> Result<Option<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(None) }
    /// #   fn put(&mut self, _namespace: &Namespace, _key: &str, _value: KeyValueEntry) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// #   fn delete(&mut self, _namespace: &Namespace, _key: &str) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// #   fn list(&self, _namespace: &Namespace) -> Result<Vec<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(vec![]) }
    /// #   fn exists(&self, _namespace: &Namespace, _key: &str) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(false) }
    /// #   fn transaction(&mut self, _ops: Vec<KvOp>) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// # }
    /// let mut store = MyStore;
    /// let ns = Namespace::new("users".to_string()).unwrap();
    /// let entry = KeyValueEntry {
    ///     key: "user:1".to_string(),
    ///     value: b"alice".to_vec(),
    ///     metadata: std::collections::HashMap::new(),
    ///     created_at: chrono::Utc::now(),
    ///     updated_at: chrono::Utc::now(),
    ///     ttl: None,
    /// };
    /// let result = store.put(&ns, "user:1", entry);
    /// assert!(result.is_ok());
    /// ```
    fn put(
        &mut self,
        namespace: &Namespace,
        key: &str,
        value: KeyValueEntry,
    ) -> Result<(), Box<dyn HexaError>>;
    /// Delete a value by namespace and key.
    ///
    /// # Example
    /// ```rust
    /// # use hexafn_store::domain::contracts::{KvStore, Namespace, KeyValueEntry,KvOp};
    /// # struct MyStore;
    /// # impl KvStore for MyStore {
    /// #   fn get(&self, _namespace: &Namespace, _key: &str) -> Result<Option<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(None) }
    /// #   fn put(&mut self, _namespace: &Namespace, _key: &str, _value: KeyValueEntry) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// #   fn delete(&mut self, _namespace: &Namespace, _key: &str) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// #   fn list(&self, _namespace: &Namespace) -> Result<Vec<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(vec![]) }
    /// #   fn exists(&self, _namespace: &Namespace, _key: &str) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(false) }
    /// #   fn transaction(&mut self, _ops: Vec<KvOp>) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// # }
    /// let mut store = MyStore;
    /// let ns = Namespace::new("users".to_string()).unwrap();
    /// let result = store.delete(&ns, "user:1");
    /// assert!(result.is_ok());
    /// ```
    fn delete(&mut self, namespace: &Namespace, key: &str) -> Result<(), Box<dyn HexaError>>;
    /// List all entries in a namespace.
    ///
    /// # Example
    /// ```rust
    /// # use hexafn_store::domain::contracts::{KvStore, Namespace, KeyValueEntry,KvOp};
    /// # struct MyStore;
    /// # impl KvStore for MyStore {
    /// #   fn get(&self, _namespace: &Namespace, _key: &str) -> Result<Option<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(None) }
    /// #   fn put(&mut self, _namespace: &Namespace, _key: &str, _value: KeyValueEntry) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// #   fn delete(&mut self, _namespace: &Namespace, _key: &str) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// #   fn list(&self, _namespace: &Namespace) -> Result<Vec<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(vec![]) }
    /// #   fn exists(&self, _namespace: &Namespace, _key: &str) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(false) }
    /// #   fn transaction(&mut self, _ops: Vec<KvOp>) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// # }
    /// let store = MyStore;
    /// let ns = Namespace::new("users".to_string()).unwrap();
    /// let result = store.list(&ns);
    /// assert!(result.is_ok());
    /// ```
    fn list(&self, namespace: &Namespace) -> Result<Vec<KeyValueEntry>, Box<dyn HexaError>>;
    /// Check if a key exists in a namespace.
    ///
    /// # Example
    /// ```rust
    /// # use hexafn_store::domain::contracts::{KvStore, Namespace, KeyValueEntry,KvOp};
    /// # struct MyStore;
    /// # impl KvStore for MyStore {
    /// #   fn get(&self, _namespace: &Namespace, _key: &str) -> Result<Option<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(None) }
    /// #   fn put(&mut self, _namespace: &Namespace, _key: &str, _value: KeyValueEntry) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// #   fn delete(&mut self, _namespace: &Namespace, _key: &str) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// #   fn list(&self, _namespace: &Namespace) -> Result<Vec<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(vec![]) }
    /// #   fn exists(&self, _namespace: &Namespace, _key: &str) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(false) }
    /// #   fn transaction(&mut self, _ops: Vec<KvOp>) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// # }
    /// let store = MyStore;
    /// let ns = Namespace::new("users".to_string()).unwrap();
    /// let result = store.exists(&ns, "user:1");
    /// assert!(result.is_ok());
    /// ```
    fn exists(&self, namespace: &Namespace, key: &str) -> Result<bool, Box<dyn HexaError>>;
    /// Execute a batch of operations atomically.
    ///
    /// # Example
    /// ```rust
    /// # use hexafn_store::domain::contracts::{KvStore, Namespace, KeyValueEntry, KvOp};
    /// # struct MyStore;
    /// # impl KvStore for MyStore {
    /// #   fn get(&self, _namespace: &Namespace, _key: &str) -> Result<Option<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(None) }
    /// #   fn put(&mut self, _namespace: &Namespace, _key: &str, _value: KeyValueEntry) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// #   fn delete(&mut self, _namespace: &Namespace, _key: &str) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// #   fn list(&self, _namespace: &Namespace) -> Result<Vec<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(vec![]) }
    /// #   fn exists(&self, _namespace: &Namespace, _key: &str) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(false) }
    /// #   fn transaction(&mut self, _ops: Vec<KvOp>) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// # }
    /// let mut store = MyStore;
    /// let ops = vec![KvOp::Put, KvOp::Delete];
    /// let result = store.transaction(ops);
    /// assert!(result.is_ok());
    /// ```
    fn transaction(&mut self, ops: Vec<KvOp>) -> Result<(), Box<dyn HexaError>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::collections::HashMap;

    #[test]
    fn test_key_value_entry_display() {
        let entry = KeyValueEntry {
            key: "k1".to_string(),
            value: b"v1".to_vec(),
            metadata: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            ttl: Some(chrono::Duration::seconds(60)),
        };
        let s = format!("{}", entry);
        assert!(s.contains("k1"));
        assert!(s.contains("60s"));
    }

    #[test]
    fn test_kvop_display() {
        assert_eq!(format!("{}", KvOp::Put), "Put");
        assert_eq!(format!("{}", KvOp::Delete), "Delete");
    }

    #[test]
    fn test_namespace_new_and_display() {
        let ns = Namespace::new("abc".to_string()).unwrap();
        assert_eq!(ns.value, "abc");
        assert!(ns.is_valid());
        assert_eq!(format!("{}", ns), "Namespace('abc')");
        assert!(Namespace::new("".to_string()).is_err());
    }
}
