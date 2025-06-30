// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Event Contract (Core Module)
//!
//! This module defines the core `Event` trait and supporting types for the hexaFn system.
//! Events are the primary means of communication between modules and pipeline stages
//! in the 6F Lifecycle Flow (Feed → Filter → Format → Function → Forward → Feedback).
//!
//! ## Design
//! - Each event has a unique identity ([`EventId`](crate::EventId))
//! - Events are immutable and serializable
//! - The [`Event`] trait is implemented by all event types in the system
//!
//! ## Example
//!
//! ```rust
//! use hexafn_core::Event;
//! use hexafn_core::EventId;
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
//! let event = UserCreatedEvent {
//!     id: EventId::new(),
//!     user_id: "abc123".to_string(),
//!     occurred_at: Utc::now(),
//! };
//! assert_eq!(event.event_type(), "user.created");
//! ```

/// Core event contract for all domain and integration events in hexaFn.
///
/// All events must implement this trait. Events are immutable, serializable,
/// and carry a unique identity, type, timestamp, and payload.
///
/// # Examples
///
/// ```rust
/// use hexafn_core::Event;
/// use hexafn_core::EventId;
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
/// let e = MyEvent { id: EventId::new(), occurred_at: Utc::now() };
/// assert_eq!(e.event_type(), "my.event");
/// ```
pub trait Event: Send + Sync {
    /// Returns the static event type identifier.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_core::Event;
    /// use hexafn_core::EventId;
    /// use chrono::Utc;
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
    /// ```rust
    /// use hexafn_core::Event;
    /// use hexafn_core::EventId;
    /// use chrono::Utc;
    /// struct Evt { id: EventId, occurred_at: chrono::DateTime<Utc> }
    /// impl Event for Evt {
    ///     fn event_type(&self) -> &'static str { "evt.type" }
    ///     fn event_id(&self) -> &EventId { &self.id }
    ///     fn timestamp(&self) -> chrono::DateTime<Utc> { self.occurred_at }
    ///     fn payload(&self) -> serde_json::Value { serde_json::json!({}) }
    /// }
    /// let e = Evt { id: EventId::new(), occurred_at: Utc::now() };
    /// let id = e.event_id();
    /// assert!(id.to_string().len() > 0);
    /// ```
    fn event_id(&self) -> &crate::EventId;

    /// Returns the timestamp when the event occurred.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_core::Event;
    /// use hexafn_core::EventId;
    /// use chrono::Utc;
    /// struct Evt { id: EventId, occurred_at: chrono::DateTime<Utc> }
    /// impl Event for Evt {
    ///     fn event_type(&self) -> &'static str { "evt.type" }
    ///     fn event_id(&self) -> &EventId { &self.id }
    ///     fn timestamp(&self) -> chrono::DateTime<Utc> { self.occurred_at }
    ///     fn payload(&self) -> serde_json::Value { serde_json::json!({}) }
    /// }
    /// let e = Evt { id: EventId::new(), occurred_at: Utc::now() };
    /// let ts = e.timestamp();
    /// assert!(ts.timestamp() > 0);
    /// ```
    fn timestamp(&self) -> chrono::DateTime<chrono::Utc>;

    /// Returns the event payload as a JSON value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_core::Event;
    /// use hexafn_core::EventId;
    /// use chrono::Utc;
    /// use serde_json::json;
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

/// # Event Trait Unit Tests
///
/// These tests validate the behavior of the Event trait and EventId value object.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::EventId;
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
        occurred_at: chrono::DateTime<Utc>,
    }

    impl Event for TestEvent {
        fn event_type(&self) -> &'static str {
            "test.event"
        }
        fn event_id(&self) -> &EventId {
            &self.id
        }
        fn timestamp(&self) -> chrono::DateTime<Utc> {
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

    #[test]
    fn test_event_timestamp_is_now() {
        let event = TestEvent {
            id: EventId::new(),
            value: 1,
            occurred_at: Utc::now(),
        };
        let now = Utc::now();
        // Allow a small difference due to timing
        assert!((now.timestamp() - event.timestamp().timestamp()).abs() < 2);
    }

    #[test]
    fn test_event_payload_json_structure() {
        let event = TestEvent {
            id: EventId::new(),
            value: 99,
            occurred_at: Utc::now(),
        };
        let payload = event.payload();
        assert_eq!(payload["value"], 99);
    }
}
