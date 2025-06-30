// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # DomainEvent Contract (Core Module)
//!
//! This module defines the [`DomainEvent`] trait for domain-level events in the hexaFn system.
//! Domain events represent significant business occurrences and are used for cross-module
//! communication and audit trails. All domain events extend the core [`Event`] trait but do not
//! require metadata by default.
//!
//! ## Example
//!
//! ```rust
//! use chrono::{Utc, DateTime};
//! use serde_json::json;
//! use hexafn_core::Event;
//! use hexafn_core::DomainEvent;
//! use hexafn_core::EventId;
//!
//! #[derive(Debug)]
//! struct UserRenamedEvent {
//!     id: EventId,
//!     aggregate_id: String,
//!     seq: u64,
//!     occurred_at: DateTime<Utc>,
//!     correlation_id: String,
//!     new_name: String,
//! }
//!
//! impl Event for UserRenamedEvent {
//!     fn event_type(&self) -> &'static str { "user.renamed" }
//!     fn event_id(&self) -> &EventId { &self.id }
//!     fn timestamp(&self) -> DateTime<Utc> { self.occurred_at }
//!     fn payload(&self) -> serde_json::Value {
//!         json!({ "new_name": self.new_name })
//!     }
//! }
//!
//! impl DomainEvent for UserRenamedEvent {
//!     fn aggregate_id(&self) -> &str { &self.aggregate_id }
//!     fn sequence_number(&self) -> u64 { self.seq }
//!     fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
//!     fn correlation_id(&self) -> &str { &self.correlation_id }
//! }
//! let event = UserRenamedEvent {
//!     id: EventId::new(),
//!     aggregate_id: "agg-1".to_string(),
//!     seq: 2,
//!     occurred_at: Utc::now(),
//!     correlation_id: "corr-xyz".to_string(),
//!     new_name: "Alice".to_string(),
//! };
//! assert_eq!(event.aggregate_id(), "agg-1");
//! assert_eq!(event.sequence_number(), 2);
//! assert_eq!(event.correlation_id(), "corr-xyz");
//! ```
//!
//! --- DDD/Hexagonal Architecture Note ---
//! The DomainEvent trait enables the transfer of aggregate-rooted events between modules and the creation of audit trails.
//! Every domain event provides traceability in business processes via aggregate_id, sequence_number, occurred_at, and correlation_id.
//!
//! ---
//!
//! ## Example domain event struct and trait implementation for DDD/hexagonal compliance.
//!
//! ```rust
//! use chrono::{Utc, DateTime};
//! use serde_json::json;
//! use hexafn_core::Event;
//! use hexafn_core::DomainEvent;
//! use hexafn_core::EventId;
//! #[derive(Debug)]
//! struct OrderCreatedEvent {
//!     id: EventId,
//!     aggregate_id: String,
//!     seq: u64,
//!     occurred_at: DateTime<Utc>,
//!     correlation_id: String,
//!     order_total: f64,
//! }
//! impl Event for OrderCreatedEvent {
//!     fn event_type(&self) -> &'static str { "order.created" }
//!     fn event_id(&self) -> &EventId { &self.id }
//!     fn timestamp(&self) -> DateTime<Utc> { self.occurred_at }
//!     fn payload(&self) -> serde_json::Value {
//!         json!({ "order_total": self.order_total })
//!     }
//! }
//! impl DomainEvent for OrderCreatedEvent {
//!     fn aggregate_id(&self) -> &str { &self.aggregate_id }
//!     fn sequence_number(&self) -> u64 { self.seq }
//!     fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
//!     fn correlation_id(&self) -> &str { &self.correlation_id }
//! }
//! let event = OrderCreatedEvent {
//!     id: EventId::new(),
//!     aggregate_id: "order-1".to_string(),
//!     seq: 1,
//!     occurred_at: Utc::now(),
//!     correlation_id: "corr-xyz".to_string(),
//!     order_total: 42.0,
//! };
//! assert_eq!(event.aggregate_id(), "order-1");
//! ```
//!

use super::event::Event;
use chrono::{DateTime, Utc};

/// Trait for domain-level events in the hexaFn system.
///
/// Domain events extend the core [`Event`] trait and provide additional context
/// for aggregate association, sequencing, and correlation. They are used to
/// signal important business changes within the domain.
///
/// # Examples
///
/// ```rust
/// use chrono::{Utc, DateTime};
/// use serde_json::json;
/// use hexafn_core::Event;
/// use hexafn_core::DomainEvent;
/// use hexafn_core::EventId;
///
/// struct MyDomainEvent {
///     id: EventId,
///     aggregate_id: String,
///     seq: u64,
///     occurred_at: DateTime<Utc>,
///     correlation_id: String,
/// }
///
/// impl Event for MyDomainEvent {
///     fn event_type(&self) -> &'static str { "my.domain_event" }
///     fn event_id(&self) -> &EventId { &self.id }
///     fn timestamp(&self) -> DateTime<Utc> { self.occurred_at }
///     fn payload(&self) -> serde_json::Value { json!({}) }
/// }
///
/// impl DomainEvent for MyDomainEvent {
///     fn aggregate_id(&self) -> &str { &self.aggregate_id }
///     fn sequence_number(&self) -> u64 { self.seq }
///     fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
///     fn correlation_id(&self) -> &str { &self.correlation_id }
/// }
/// let event = MyDomainEvent {
///     id: EventId::new(),
///     aggregate_id: "agg-123".to_string(),
///     seq: 1,
///     occurred_at: Utc::now(),
///     correlation_id: "corr-1".to_string(),
/// };
/// assert_eq!(event.aggregate_id(), "agg-123");
/// assert_eq!(event.sequence_number(), 1);
/// assert_eq!(event.correlation_id(), "corr-1");
/// ```
pub trait DomainEvent: Event {
    /// Returns the aggregate id associated with this domain event.
    ///
    /// The aggregate id identifies the domain aggregate (entity or root)
    /// to which this event belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use chrono::{Utc, DateTime};
    /// # use hexafn_core::Event;
    /// # use hexafn_core::DomainEvent;
    /// # use hexafn_core::EventId;
    /// struct MyDomainEvent {
    ///     id: EventId,
    ///     aggregate_id: String,
    ///     seq: u64,
    ///     occurred_at: DateTime<Utc>,
    ///     correlation_id: String,
    /// }
    /// impl Event for MyDomainEvent {
    ///     fn event_type(&self) -> &'static str { "my.domain_event" }
    ///     fn event_id(&self) -> &EventId { &self.id }
    ///     fn timestamp(&self) -> DateTime<Utc> { self.occurred_at }
    ///     fn payload(&self) -> serde_json::Value { serde_json::json!({}) }
    /// }
    /// impl DomainEvent for MyDomainEvent {
    ///     fn aggregate_id(&self) -> &str { &self.aggregate_id }
    ///     fn sequence_number(&self) -> u64 { self.seq }
    ///     fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    ///     fn correlation_id(&self) -> &str { &self.correlation_id }
    /// }
    /// let event = MyDomainEvent {
    ///     id: EventId::new(),
    ///     aggregate_id: "agg-123".to_string(),
    ///     seq: 1,
    ///     occurred_at: Utc::now(),
    ///     correlation_id: "corr-1".to_string(),
    /// };
    /// assert_eq!(event.aggregate_id(), "agg-123");
    /// ```
    fn aggregate_id(&self) -> &str;

    /// Returns the sequence number of the event within the aggregate.
    ///
    /// Sequence numbers are used to order events for a given aggregate.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use chrono::{Utc, DateTime};
    /// # use hexafn_core::Event;
    /// # use hexafn_core::DomainEvent;
    /// # use hexafn_core::EventId;
    /// struct MyDomainEvent {
    ///     id: EventId,
    ///     aggregate_id: String,
    ///     seq: u64,
    ///     occurred_at: DateTime<Utc>,
    ///     correlation_id: String,
    /// }
    /// impl Event for MyDomainEvent {
    ///     fn event_type(&self) -> &'static str { "my.domain_event" }
    ///     fn event_id(&self) -> &EventId { &self.id }
    ///     fn timestamp(&self) -> DateTime<Utc> { self.occurred_at }
    ///     fn payload(&self) -> serde_json::Value { serde_json::json!({}) }
    /// }
    /// impl DomainEvent for MyDomainEvent {
    ///     fn aggregate_id(&self) -> &str { &self.aggregate_id }
    ///     fn sequence_number(&self) -> u64 { self.seq }
    ///     fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    ///     fn correlation_id(&self) -> &str { &self.correlation_id }
    /// }
    /// let event = MyDomainEvent {
    ///     id: EventId::new(),
    ///     aggregate_id: "agg-123".to_string(),
    ///     seq: 42,
    ///     occurred_at: Utc::now(),
    ///     correlation_id: "corr-1".to_string(),
    /// };
    /// assert_eq!(event.sequence_number(), 42);
    /// ```
    fn sequence_number(&self) -> u64;

    /// Returns the time when the domain event occurred.
    ///
    /// This timestamp should reflect the actual business occurrence time.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use chrono::{Utc, DateTime};
    /// # use hexafn_core::Event;
    /// # use hexafn_core::DomainEvent;
    /// # use hexafn_core::EventId;
    /// struct MyDomainEvent {
    ///     id: EventId,
    ///     aggregate_id: String,
    ///     seq: u64,
    ///     occurred_at: DateTime<Utc>,
    ///     correlation_id: String,
    /// }
    /// impl Event for MyDomainEvent {
    ///     fn event_type(&self) -> &'static str { "my.domain_event" }
    ///     fn event_id(&self) -> &EventId { &self.id }
    ///     fn timestamp(&self) -> DateTime<Utc> { self.occurred_at }
    ///     fn payload(&self) -> serde_json::Value { serde_json::json!({}) }
    /// }
    /// impl DomainEvent for MyDomainEvent {
    ///     fn aggregate_id(&self) -> &str { &self.aggregate_id }
    ///     fn sequence_number(&self) -> u64 { self.seq }
    ///     fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    ///     fn correlation_id(&self) -> &str { &self.correlation_id }
    /// }
    /// let event = MyDomainEvent {
    ///     id: EventId::new(),
    ///     aggregate_id: "agg-123".to_string(),
    ///     seq: 1,
    ///     occurred_at: Utc::now(),
    ///     correlation_id: "corr-1".to_string(),
    /// };
    /// let ts = event.occurred_at();
    /// assert!(ts <= Utc::now());
    /// ```
    fn occurred_at(&self) -> DateTime<Utc>;

    /// Returns the correlation id for this domain event.
    ///
    /// Correlation ids are used to trace related events across processes.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use chrono::{Utc, DateTime};
    /// # use hexafn_core::Event;
    /// # use hexafn_core::DomainEvent;
    /// # use hexafn_core::EventId;
    /// struct MyDomainEvent {
    ///     id: EventId,
    ///     aggregate_id: String,
    ///     seq: u64,
    ///     occurred_at: DateTime<Utc>,
    ///     correlation_id: String,
    /// }
    /// impl Event for MyDomainEvent {
    ///     fn event_type(&self) -> &'static str { "my.domain_event" }
    ///     fn event_id(&self) -> &EventId { &self.id }
    ///     fn timestamp(&self) -> DateTime<Utc> { self.occurred_at }
    ///     fn payload(&self) -> serde_json::Value { serde_json::json!({}) }
    /// }
    /// impl DomainEvent for MyDomainEvent {
    ///     fn aggregate_id(&self) -> &str { &self.aggregate_id }
    ///     fn sequence_number(&self) -> u64 { self.seq }
    ///     fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    ///     fn correlation_id(&self) -> &str { &self.correlation_id }
    /// }
    /// let event = MyDomainEvent {
    ///     id: EventId::new(),
    ///     aggregate_id: "agg-123".to_string(),
    ///     seq: 1,
    ///     occurred_at: Utc::now(),
    ///     correlation_id: "corr-xyz".to_string(),
    /// };
    /// assert_eq!(event.correlation_id(), "corr-xyz");
    /// ```
    fn correlation_id(&self) -> &str;
}

/// # DomainEvent Trait Unit Tests
///
/// These tests validate the behavior of the DomainEvent trait and its integration with Event and EventId.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Event;
    use crate::EventId;
    use chrono::Utc;
    use serde_json::json;

    #[derive(Debug)]
    struct TestDomainEvent {
        id: EventId,
        aggregate_id: String,
        seq: u64,
        occurred_at: chrono::DateTime<Utc>,
        correlation_id: String,
        value: i32,
    }

    impl Event for TestDomainEvent {
        fn event_type(&self) -> &'static str {
            "test.domain_event"
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

    impl DomainEvent for TestDomainEvent {
        fn aggregate_id(&self) -> &str {
            &self.aggregate_id
        }
        fn sequence_number(&self) -> u64 {
            self.seq
        }
        fn occurred_at(&self) -> chrono::DateTime<Utc> {
            self.occurred_at
        }
        fn correlation_id(&self) -> &str {
            &self.correlation_id
        }
    }

    #[test]
    fn test_domain_event_trait_impl() {
        let id = EventId::new();
        let now = Utc::now();
        let event = TestDomainEvent {
            id: id.clone(),
            aggregate_id: "agg-1".to_string(),
            seq: 7,
            occurred_at: now,
            correlation_id: "corr-123".to_string(),
            value: 99,
        };
        assert_eq!(event.aggregate_id(), "agg-1");
        assert_eq!(event.sequence_number(), 7);
        assert_eq!(event.correlation_id(), "corr-123");
        assert_eq!(event.event_type(), "test.domain_event");
        assert_eq!(event.event_id(), &id);
        assert_eq!(event.payload(), json!({ "value": 99 }));
        assert_eq!(event.occurred_at(), now);
    }

    #[test]
    fn test_domain_event_sequence_and_correlation() {
        let event = TestDomainEvent {
            id: EventId::new(),
            aggregate_id: "agg-2".to_string(),
            seq: 42,
            occurred_at: Utc::now(),
            correlation_id: "corr-xyz".to_string(),
            value: 1,
        };
        assert_eq!(event.sequence_number(), 42);
        assert_eq!(event.correlation_id(), "corr-xyz");
    }

    #[test]
    fn test_domain_event_occurred_at_is_now() {
        let now = Utc::now();
        let event = TestDomainEvent {
            id: EventId::new(),
            aggregate_id: "agg-3".to_string(),
            seq: 1,
            occurred_at: now,
            correlation_id: "corr-abc".to_string(),
            value: 0,
        };
        assert_eq!(event.occurred_at(), now);
    }
}
