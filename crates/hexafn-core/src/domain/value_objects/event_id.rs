// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # EventId Value Object
//!
//! This module defines the [`EventId`] struct, a value object for unique event identity in the hexaFn framework.
//! It wraps a UUID and provides utility methods for creation, conversion, and parsing. Used in all event-related
//! domain contracts and value objects (see [`crate::Event`]).
//!
//! ## Usage Example
//!
//! ```rust
//! use hexafn_core::EventId;
//!
//! let id = EventId::new();
//! let id_str = id.to_string();
//! let parsed = EventId::from_string(&id_str).unwrap();
//! assert_eq!(id, parsed);
//! ```
//!
//! ## DDD/Hexagonal Architecture Note
//!
//! This value object should be used in all event-related domain contracts and value objects to ensure
//! a consistent event identity model throughout the system.

use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Value object for unique event identity.
///
/// Wraps a UUID and provides utility methods for creation and conversion.
///
/// # Example
/// ```
/// use hexafn_core::EventId;
/// let id = EventId::new();
/// let id_str = id.to_string();
/// let parsed = EventId::from_string(&id_str).unwrap();
/// assert_eq!(id, parsed);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventId(pub Uuid);

impl EventId {
    /// Creates a new random event id.
    ///
    /// # Example
    /// ```
    /// use hexafn_core::EventId;
    /// let id = EventId::new();
    /// assert_eq!(id.to_string().len(), 36);
    /// ```
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Parses an event id from a string.
    ///
    /// # Errors
    /// Returns an error if the string is not a valid UUID.
    ///
    /// # Example
    /// ```
    /// use hexafn_core::EventId;
    /// let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
    /// let event_id = EventId::from_string(uuid_str).unwrap();
    /// assert_eq!(event_id.to_string(), uuid_str);
    /// ```
    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl Default for EventId {
    /// Returns a new random event id as the default value.
    ///
    /// # Example
    /// ```
    /// use hexafn_core::EventId;
    /// let id = EventId::default();
    /// assert_eq!(id.to_string().len(), 36);
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl Display for EventId {
    /// Formats the event id as a string (UUID format).
    ///
    /// # Example
    /// ```
    /// use hexafn_core::EventId;
    /// let id = EventId::new();
    /// let s = id.to_string();
    /// assert_eq!(s.len(), 36);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_to_string() {
        let id = EventId::new();
        let s = id.to_string();
        assert_eq!(s.len(), 36);
    }

    #[test]
    fn test_from_string_success() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let event_id = EventId::from_string(uuid_str).unwrap();
        assert_eq!(event_id.to_string(), uuid_str);
    }

    #[test]
    fn test_from_string_failure() {
        let invalid = "not-a-uuid";
        assert!(EventId::from_string(invalid).is_err());
    }

    #[test]
    fn test_default() {
        let id = EventId::default();
        assert_eq!(id.to_string().len(), 36);
    }

    #[test]
    fn test_equality_and_clone() {
        let id1 = EventId::new();
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_serde_roundtrip() {
        let id = EventId::new();
        let json = serde_json::to_string(&id).unwrap();
        let de: EventId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, de);
    }
}
