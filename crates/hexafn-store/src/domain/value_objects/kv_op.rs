// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # KvOp Value Object
//!
//! Supported atomic operations for transactional updates in the key-value store.
//!
//! This enum is used to express a batch of changes (put or delete) that should be applied atomically to the store. Useful for implementing transactions, migrations, or batch updates.
//!
//! # Example
//!
//! ```rust
//! use hexafn_store::KvOp;
//! let op = KvOp::Put;
//! assert_eq!(format!("{}", op), "Put");
//! ```

/// Supported atomic operations for transactional updates in the key-value store.
///
/// This enum is used to express a batch of changes (put or delete) that should be applied atomically to the store.
///
/// # Example
/// ```rust
/// use hexafn_store::KvOp;
/// let op = KvOp::Delete;
/// assert_eq!(format!("{}", op), "Delete");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KvOp {
    /// Put operation for adding or updating an entry.
    Put,
    /// Delete operation for removing an entry.
    Delete,
}

impl std::fmt::Display for KvOp {
    /// Formats the KvOp as a user-friendly string.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_store::KvOp;
    /// assert_eq!(KvOp::Put.to_string(), "Put");
    /// assert_eq!(KvOp::Delete.to_string(), "Delete");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KvOp::Put => write!(f, "Put"),
            KvOp::Delete => write!(f, "Delete"),
        }
    }
}

impl KvOp {
    /// Returns true if this operation is a Put.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_store::KvOp;
    /// assert!(KvOp::Put.is_put());
    /// assert!(!KvOp::Delete.is_put());
    /// ```
    pub fn is_put(&self) -> bool {
        matches!(self, KvOp::Put)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_put() {
        let op = KvOp::Put;
        assert_eq!(format!("{}", op), "Put");
    }

    #[test]
    fn test_display_delete() {
        let op = KvOp::Delete;
        assert_eq!(format!("{}", op), "Delete");
    }

    #[test]
    fn test_is_put() {
        assert!(KvOp::Put.is_put());
        assert!(!KvOp::Delete.is_put());
    }

    #[test]
    fn test_enum_equality() {
        assert_eq!(KvOp::Put, KvOp::Put);
        assert_eq!(KvOp::Delete, KvOp::Delete);
        assert_ne!(KvOp::Put, KvOp::Delete);
    }

    #[test]
    fn test_doc_examples() {
        // Example from enum-level doc
        let op = KvOp::Delete;
        assert_eq!(format!("{}", op), "Delete");
        // Example from is_put doc
        assert!(KvOp::Put.is_put());
        assert!(!KvOp::Delete.is_put());
    }
}
