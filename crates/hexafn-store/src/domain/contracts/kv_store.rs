// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Key-Value Store Domain Contracts
//!
//! This module defines the core types and contracts for the HexaStore key-value storage engine in the hexaFn system.
//!
//! - [`KvStore`]: Trait for generic, pluggable key-value store implementations (in-memory, file, or external).
//!
//! ## Example
//! ```rust
//! use hexafn_store::{KeyValueEntry, Namespace, KvOp};
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
//! ```rust
//! use hexafn_store::{KeyValueEntry, Namespace, KvOp};
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

pub use crate::KeyValueEntry;
pub use crate::KvOp;
pub use crate::Namespace;
use hexafn_core::HexaError;

/// Trait for a generic, pluggable key-value store contract.
///
/// This trait defines the required interface for any HexaStore backend implementation, including in-memory, file-based, or external (e.g., Redis, RocksDB) stores. It supports basic CRUD operations, listing, existence checks, and atomic transactions.
///
/// # Example
/// ```rust
/// use hexafn_store::{KeyValueEntry, Namespace, KvOp};
/// use hexafn_store::KvStore;
/// struct MyStore;
/// impl KvStore for MyStore {
///     fn get(&self, _namespace: &Namespace, _key: &str) -> Result<Option<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(None) }
///     fn put(&mut self, _namespace: &Namespace, _key: &str, _value: KeyValueEntry) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
///     fn delete(&mut self, _namespace: &Namespace, _key: &str) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
///     fn list(&self, _namespace: &Namespace) -> Result<Vec<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(vec![]) }
///     fn exists(&self, _namespace: &Namespace, _key: &str) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(false) }
///     fn transaction(&mut self, _ops: Vec<KvOp>) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
/// }
/// let store = MyStore;
/// let ns = Namespace::new("users".to_string()).unwrap();
/// let result = store.get(&ns, "user:1");
/// assert!(result.is_ok());
/// ```
pub trait KvStore {
    /// Get a value by namespace and key.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_store::KvStore;
    /// use hexafn_store::{Namespace, KeyValueEntry, KvOp};
    /// struct MyStore;
    /// impl KvStore for MyStore {
    ///     fn get(&self, _namespace: &Namespace, _key: &str) -> Result<Option<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(None) }
    ///     fn put(&mut self, _namespace: &Namespace, _key: &str, _value: KeyValueEntry) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    ///     fn delete(&mut self, _namespace: &Namespace, _key: &str) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    ///     fn list(&self, _namespace: &Namespace) -> Result<Vec<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(vec![]) }
    ///     fn exists(&self, _namespace: &Namespace, _key: &str) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(false) }
    ///     fn transaction(&mut self, _ops: Vec<KvOp>) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// }
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
    /// use hexafn_store::KvStore;
    /// use hexafn_store::{Namespace, KeyValueEntry, KvOp};
    /// struct MyStore;
    /// impl KvStore for MyStore {
    ///     fn get(&self, _namespace: &Namespace, _key: &str) -> Result<Option<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(None) }
    ///     fn put(&mut self, _namespace: &Namespace, _key: &str, _value: KeyValueEntry) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    ///     fn delete(&mut self, _namespace: &Namespace, _key: &str) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    ///     fn list(&self, _namespace: &Namespace) -> Result<Vec<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(vec![]) }
    ///     fn exists(&self, _namespace: &Namespace, _key: &str) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(false) }
    ///     fn transaction(&mut self, _ops: Vec<KvOp>) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// }
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
    /// use hexafn_store::KvStore;
    /// use hexafn_store::{Namespace, KeyValueEntry, KvOp};
    /// struct MyStore;
    /// impl KvStore for MyStore {
    ///     fn get(&self, _namespace: &Namespace, _key: &str) -> Result<Option<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(None) }
    ///     fn put(&mut self, _namespace: &Namespace, _key: &str, _value: KeyValueEntry) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    ///     fn delete(&mut self, _namespace: &Namespace, _key: &str) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    ///     fn list(&self, _namespace: &Namespace) -> Result<Vec<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(vec![]) }
    ///     fn exists(&self, _namespace: &Namespace, _key: &str) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(false) }
    ///     fn transaction(&mut self, _ops: Vec<KvOp>) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// }
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
    /// use hexafn_store::KvStore;
    /// use hexafn_store::{Namespace, KeyValueEntry, KvOp};
    /// struct MyStore;
    /// impl KvStore for MyStore {
    ///     fn get(&self, _namespace: &Namespace, _key: &str) -> Result<Option<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(None) }
    ///     fn put(&mut self, _namespace: &Namespace, _key: &str, _value: KeyValueEntry) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    ///     fn delete(&mut self, _namespace: &Namespace, _key: &str) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    ///     fn list(&self, _namespace: &Namespace) -> Result<Vec<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(vec![]) }
    ///     fn exists(&self, _namespace: &Namespace, _key: &str) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(false) }
    ///     fn transaction(&mut self, _ops: Vec<KvOp>) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// }
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
    /// use hexafn_store::KvStore;
    /// use hexafn_store::{Namespace, KeyValueEntry, KvOp};
    /// struct MyStore;
    /// impl KvStore for MyStore {
    ///     fn get(&self, _namespace: &Namespace, _key: &str) -> Result<Option<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(None) }
    ///     fn put(&mut self, _namespace: &Namespace, _key: &str, _value: KeyValueEntry) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    ///     fn delete(&mut self, _namespace: &Namespace, _key: &str) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    ///     fn list(&self, _namespace: &Namespace) -> Result<Vec<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(vec![]) }
    ///     fn exists(&self, _namespace: &Namespace, _key: &str) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(false) }
    ///     fn transaction(&mut self, _ops: Vec<KvOp>) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// }
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
    /// use hexafn_store::KvStore;
    /// use hexafn_store::{Namespace, KeyValueEntry, KvOp};
    /// struct MyStore;
    /// impl KvStore for MyStore {
    ///     fn get(&self, _namespace: &Namespace, _key: &str) -> Result<Option<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(None) }
    ///     fn put(&mut self, _namespace: &Namespace, _key: &str, _value: KeyValueEntry) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    ///     fn delete(&mut self, _namespace: &Namespace, _key: &str) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    ///     fn list(&self, _namespace: &Namespace) -> Result<Vec<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> { Ok(vec![]) }
    ///     fn exists(&self, _namespace: &Namespace, _key: &str) -> Result<bool, Box<dyn hexafn_core::HexaError>> { Ok(false) }
    ///     fn transaction(&mut self, _ops: Vec<KvOp>) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// }
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
    use crate::KeyValueEntry;
    use crate::KvOp;
    use crate::Namespace;
    use chrono::Utc;
    use std::collections::HashMap;

    struct DummyStore;
    impl KvStore for DummyStore {
        fn get(
            &self,
            _namespace: &Namespace,
            _key: &str,
        ) -> Result<Option<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> {
            Ok(None)
        }
        fn put(
            &mut self,
            _namespace: &Namespace,
            _key: &str,
            _value: KeyValueEntry,
        ) -> Result<(), Box<dyn hexafn_core::HexaError>> {
            Ok(())
        }
        fn delete(
            &mut self,
            _namespace: &Namespace,
            _key: &str,
        ) -> Result<(), Box<dyn hexafn_core::HexaError>> {
            Ok(())
        }
        fn list(
            &self,
            _namespace: &Namespace,
        ) -> Result<Vec<KeyValueEntry>, Box<dyn hexafn_core::HexaError>> {
            Ok(vec![])
        }
        fn exists(
            &self,
            _namespace: &Namespace,
            _key: &str,
        ) -> Result<bool, Box<dyn hexafn_core::HexaError>> {
            Ok(false)
        }
        fn transaction(&mut self, _ops: Vec<KvOp>) -> Result<(), Box<dyn hexafn_core::HexaError>> {
            Ok(())
        }
    }

    #[test]
    fn test_kvstore_trait_usage() {
        let mut store = DummyStore;
        let ns = Namespace::new("users".to_string()).unwrap();
        let entry = KeyValueEntry {
            key: "user:1".to_string(),
            value: b"alice".to_vec(),
            metadata: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            ttl: None,
        };
        assert!(store.get(&ns, "user:1").is_ok());
        assert!(store.put(&ns, "user:1", entry.clone()).is_ok());
        assert!(store.delete(&ns, "user:1").is_ok());
        assert!(store.list(&ns).is_ok());
        assert!(store.exists(&ns, "user:1").is_ok());
        assert!(store.transaction(vec![KvOp::Put, KvOp::Delete]).is_ok());
    }
}
