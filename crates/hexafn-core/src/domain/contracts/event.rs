// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Event Contract (Core Module)
//!
//! This module defines the core `Event` trait and supporting types for the hexaFn system.
//! Events are the primary means of communication between modules and pipeline stages
//! in the 6F Lifecycle Flow (Feed → Filter → Format → Function → Forward → Feedback).
//!
//! ## Design
//! - Each event has a unique identity (`EventId`)
//! - Events are immutable and serializable
//! - The `Event` trait is implemented by all event types in the system
//!
//! ## Example
//!
//! ```rust
//! use hexafn_core::{Event, EventId};
//! use chrono::{Utc, DateTime};
//! use serde_json::json;
//!
//! #[derive(Debug)]
//! struct UserCreatedEvent {
//!     id: EventId,
//!     user_id: String,
//!     occurred_at: DateTime<Utc>,
//! }
//!
//! impl Event for UserCreatedEvent {
//!     fn event_type(&self) -> &'static str { "user.created" }
//!     fn event_id(&self) -> &EventId { &self.id }
//!     fn timestamp(&self) -> DateTime<Utc> { self.occurred_at }
//!     fn payload(&self) -> serde_json::Value {
//!         json!({ "user_id": self.user_id })
//!     }
//! }
//! ```

use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Value object for unique event identity.
///
/// Wraps a UUID and provides utility methods for creation and conversion.
///
/// # Examples
///
/// ```rust
/// use hexafn_core::EventId;
///
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
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::EventId;
    /// let id = EventId::new();
    /// ```
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Parses an event id from a string.
    ///
    /// # Errors
    /// Returns an error if the string is not a valid UUID.
    ///
    /// # Examples
    ///
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
    fn default() -> Self {
        Self::new()
    }
}

impl Display for EventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

///
/// All events must implement this trait. Events are immutable, serializable,
/// and carry a unique identity, type, timestamp, and payload.
///
/// # Examples
///
/// ```rust
/// use hexafn_core::{Event, EventId};
/// use chrono::Utc;
/// use serde_json::json;
///
/// struct MyEvent {
///     id: EventId,
///     occurred_at: chrono::DateTime<Utc>,
/// }
///
/// impl Event for MyEvent {
///     fn event_type(&self) -> &'static str { "my.event" }
///     fn event_id(&self) -> &EventId { &self.id }
///     fn timestamp(&self) -> chrono::DateTime<Utc> { self.occurred_at }
///     fn payload(&self) -> serde_json::Value { json!({}) }
/// }
/// ```
pub trait Event: Send + Sync {
    /// Returns the static event type identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexafn_core::{Event, EventId};
    /// # use chrono::Utc;
    /// struct Evt { id: EventId, occurred_at: chrono::DateTime<Utc> }
    /// impl Event for Evt {
    ///     fn event_type(&self) -> &'static str { "evt.type" }
    ///     fn event_id(&self) -> &EventId { &self.id }
    ///     fn timestamp(&self) -> chrono::DateTime<Utc> { self.occurred_at }
    ///     fn payload(&self) -> serde_json::Value { serde_json::json!({}) }
    /// }
    /// let e = Evt { id: EventId::new(), occurred_at: Utc::now() };
    /// assert_eq!(e.event_type(), "evt.type");
    /// ```
    fn event_type(&self) -> &'static str;

    /// Returns the unique event id.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexafn_core::{Event, EventId};
    /// # use chrono::Utc;
    /// struct Evt { id: EventId, occurred_at: chrono::DateTime<Utc> }
    /// impl Event for Evt {
    ///     fn event_type(&self) -> &'static str { "evt.type" }
    ///     fn event_id(&self) -> &EventId { &self.id }
    ///     fn timestamp(&self) -> chrono::DateTime<Utc> { self.occurred_at }
    ///     fn payload(&self) -> serde_json::Value { serde_json::json!({}) }
    /// }
    /// let e = Evt { id: EventId::new(), occurred_at: Utc::now() };
    /// let id = e.event_id();
    /// ```
    fn event_id(&self) -> &EventId;

    /// Returns the timestamp when the event occurred.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexafn_core::{Event, EventId};
    /// # use chrono::Utc;
    /// struct Evt { id: EventId, occurred_at: chrono::DateTime<Utc> }
    /// impl Event for Evt {
    ///     fn event_type(&self) -> &'static str { "evt.type" }
    ///     fn event_id(&self) -> &EventId { &self.id }
    ///     fn timestamp(&self) -> chrono::DateTime<Utc> { self.occurred_at }
    ///     fn payload(&self) -> serde_json::Value { serde_json::json!({}) }
    /// }
    /// let e = Evt { id: EventId::new(), occurred_at: Utc::now() };
    /// let ts = e.timestamp();
    /// ```
    fn timestamp(&self) -> DateTime<Utc>;

    /// Returns the event payload as a JSON value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexafn_core::{Event, EventId};
    /// # use chrono::Utc;
    /// # use serde_json::json;
    /// struct Evt { id: EventId, value: i32, occurred_at: chrono::DateTime<Utc> }
    /// impl Event for Evt {
    ///     fn event_type(&self) -> &'static str { "evt.type" }
    ///     fn event_id(&self) -> &EventId { &self.id }
    ///     fn timestamp(&self) -> chrono::DateTime<Utc> { self.occurred_at }
    ///     fn payload(&self) -> serde_json::Value { json!({ "value": self.value }) }
    /// }
    /// let e = Evt { id: EventId::new(), value: 42, occurred_at: Utc::now() };
    /// let payload = e.payload();
    /// assert_eq!(payload["value"], 42);
    /// ```
    fn payload(&self) -> serde_json::Value;
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use serde_json::json;

    #[test]
    fn event_id_uniqueness() {
        let id1 = EventId::new();
        let id2 = EventId::new();
        assert_ne!(id1, id2, "EventId::new() should generate unique IDs");
    }

    #[test]
    fn event_id_from_string_and_to_string() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let event_id = EventId::from_string(uuid_str).unwrap();
        assert_eq!(event_id.to_string(), uuid_str);
    }

    #[derive(Debug)]
    struct TestEvent {
        id: EventId,
        value: i32,
        occurred_at: DateTime<Utc>,
    }

    impl Event for TestEvent {
        fn event_type(&self) -> &'static str {
            "test.event"
        }
        fn event_id(&self) -> &EventId {
            &self.id
        }
        fn timestamp(&self) -> DateTime<Utc> {
            self.occurred_at
        }
        fn payload(&self) -> serde_json::Value {
            json!({ "value": self.value })
        }
    }

    #[test]
    fn test_event_trait_impl() {
        let id = EventId::new();
        let event = TestEvent {
            id: id.clone(),
            value: 42,
            occurred_at: Utc::now(),
        };
        assert_eq!(event.event_type(), "test.event");
        assert_eq!(event.event_id(), &id);
        assert_eq!(event.payload(), json!({ "value": 42 }));
    }
}
