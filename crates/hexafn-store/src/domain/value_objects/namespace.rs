// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Namespace Value Object
//!
//! Logical namespace for key scoping in the HexaStore engine.
//!
//! Namespaces allow for multi-tenant, modular, or isolated storage within a single key-value backend. Each namespace acts as a logical partition for keys, supporting use cases like per-user, per-module, or per-environment data separation.
//!
//! # Example
//!
//! ```rust
//! use hexafn_store::Namespace;
//! let ns = Namespace::new("settings".to_string()).unwrap();
//! assert_eq!(ns.value, "settings");
//! assert!(ns.is_valid());
//! ```

use hexafn_core::{HexaCoreError, HexaError};

/// Logical namespace for key scoping in the HexaStore engine.
///
/// Namespaces allow for multi-tenant, modular, or isolated storage within a single key-value backend. Each namespace acts as a logical partition for keys, supporting use cases like per-user, per-module, or per-environment data separation.
///
/// # Example
/// ```rust
/// use hexafn_store::Namespace;
/// let ns = Namespace::new("users".to_string()).unwrap();
/// assert_eq!(ns.value, "users");
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
    /// use hexafn_store::Namespace;
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
    /// use hexafn_store::Namespace;
    /// let ns = Namespace { value: "abc".to_string() };
    /// assert!(ns.is_valid());
    /// let ns = Namespace { value: "".to_string() };
    /// assert!(!ns.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        !self.value.trim().is_empty() && !self.value.contains(' ')
    }
    /// Returns true if this namespace is a reserved system namespace (starts with "_sys:").
    ///
    /// # Example
    /// ```rust
    /// use hexafn_store::Namespace;
    /// let ns = Namespace { value: "_sys:internal".to_string() };
    /// assert!(ns.is_system_namespace());
    /// let ns = Namespace { value: "users".to_string() };
    /// assert!(!ns.is_system_namespace());
    /// ```
    pub fn is_system_namespace(&self) -> bool {
        self.value.starts_with("_sys:")
    }
}

impl std::fmt::Display for Namespace {
    /// Formats the namespace as a user-friendly string.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_store::Namespace;
    /// let ns = Namespace { value: "abc".to_string() };
    /// assert_eq!(format!("{}", ns), "Namespace('abc')");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Namespace('{}')", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        let ns = Namespace::new("users".to_string());
        assert!(ns.is_ok());
        let ns = ns.unwrap();
        assert_eq!(ns.value, "users");
    }

    #[test]
    fn test_new_empty() {
        let ns = Namespace::new("".to_string());
        assert!(ns.is_err());
    }

    #[test]
    fn test_is_valid() {
        let ns = Namespace {
            value: "abc".to_string(),
        };
        assert!(ns.is_valid());
        let ns = Namespace {
            value: "".to_string(),
        };
        assert!(!ns.is_valid());
        let ns = Namespace {
            value: "with space".to_string(),
        };
        assert!(!ns.is_valid());
    }

    #[test]
    fn test_is_system_namespace() {
        let ns = Namespace {
            value: "_sys:internal".to_string(),
        };
        assert!(ns.is_system_namespace());
        let ns = Namespace {
            value: "users".to_string(),
        };
        assert!(!ns.is_system_namespace());
    }

    #[test]
    fn test_display() {
        let ns = Namespace {
            value: "abc".to_string(),
        };
        assert_eq!(format!("{ns}"), "Namespace('abc')");
    }

    #[test]
    fn test_doc_examples() {
        // Example from struct-level doc
        let ns = Namespace::new("settings".to_string()).unwrap();
        assert_eq!(ns.value, "settings");
        assert!(ns.is_valid());
        // Example from is_valid doc
        let ns = Namespace {
            value: "abc".to_string(),
        };
        assert!(ns.is_valid());
        let ns = Namespace {
            value: "".to_string(),
        };
        assert!(!ns.is_valid());
        // Example from is_system_namespace doc
        let ns = Namespace {
            value: "_sys:internal".to_string(),
        };
        assert!(ns.is_system_namespace());
    }
}
