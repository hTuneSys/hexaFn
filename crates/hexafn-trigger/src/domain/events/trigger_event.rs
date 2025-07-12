// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # TriggerEvent Domain Trait
//!
//! This trait defines the contract for all domain events related to triggers in the hexaFn-trigger module.
//!
//! ## Example Usage
//! ```rust
//! use chrono::Utc;
//! use serde_json::json;
//! use hexafn_trigger::domain::events::trigger_event::{TriggerEvent, TriggerCreatedEvent};
//!
//! let event = TriggerCreatedEvent::new(
//!     "trigger-1".to_string(),
//!     Utc::now(),
//!     json!({"name": "Test Trigger"})
//! );
//! assert_eq!(event.trigger_id(), "trigger-1");
//! assert_eq!(event.event_type(), "TriggerCreatedEvent");
//! ```
//!
//! ## Unit Test Example
//! ```rust
//! #[test]
//! fn test_trigger_created_event() {
//!     use chrono::Utc;
//!     use serde_json::json;
//!     use hexafn_trigger::domain::events::trigger_event::{TriggerCreatedEvent, TriggerEvent};
//!
//!     let event = TriggerCreatedEvent::new(
//!         "trigger-1".to_string(),
//!         Utc::now(),
//!         json!({"name": "Test Trigger"})
//!     );
//!     assert_eq!(event.event_type(), "TriggerCreatedEvent");
//! }
//! ```

use chrono::{DateTime, Utc};
use serde_json::Value;

/// Trait for all domain events related to triggers.
///
/// # Example
/// ```rust
/// use chrono::Utc;
/// use serde_json::json;
/// use hexafn_trigger::domain::events::trigger_event::{TriggerCreatedEvent, TriggerEvent};
/// let event = TriggerCreatedEvent::new(
///     "trigger-1".to_string(),
///     Utc::now(),
///     json!({"name": "Test Trigger"})
/// );
/// assert_eq!(event.trigger_id(), "trigger-1");
/// ```
/// # Unit Test
/// ```rust
/// #[test]
/// fn test_event_type() {
///     use chrono::Utc;
///     use serde_json::json;
///     use hexafn_trigger::domain::events::trigger_event::{TriggerCreatedEvent, TriggerEvent};
///     let event = TriggerCreatedEvent::new(
///         "trigger-1".to_string(),
///         Utc::now(),
///         json!({"name": "Test Trigger"})
///     );
///     assert_eq!(event.event_type(), "TriggerCreatedEvent");
/// }
/// ```
pub trait TriggerEvent {
    /// Returns the ID of the trigger that generated this event.
    ///
    /// # Example
    /// ```rust
    /// use chrono::Utc;
    /// use serde_json::json;
    /// use hexafn_trigger::domain::events::trigger_event::{TriggerCreatedEvent, TriggerEvent};
    /// let event = TriggerCreatedEvent::new(
    ///     "trigger-1".to_string(),
    ///     Utc::now(),
    ///     json!({"name": "Test Trigger"})
    /// );
    /// assert_eq!(event.trigger_id(), "trigger-1");
    /// ```
    /// # Unit Test
    /// ```rust
    /// #[test]
    /// fn test_trigger_id() {
    ///     use chrono::Utc;
    ///     use serde_json::json;
    ///     use hexafn_trigger::domain::events::trigger_event::{TriggerCreatedEvent, TriggerEvent};
    ///     let event = TriggerCreatedEvent::new(
    ///         "trigger-1".to_string(),
    ///         Utc::now(),
    ///         json!({"name": "Test Trigger"})
    ///     );
    ///     assert_eq!(event.trigger_id(), "trigger-1");
    /// }
    /// ```
    fn trigger_id(&self) -> String;

    /// Returns the static event type string for this event.
    ///
    /// # Example
    /// ```rust
    /// use chrono::Utc;
    /// use serde_json::json;
    /// use hexafn_trigger::domain::events::trigger_event::{TriggerCreatedEvent, TriggerEvent};
    /// let event = TriggerCreatedEvent::new(
    ///     "trigger-1".to_string(),
    ///     Utc::now(),
    ///     json!({"name": "Test Trigger"})
    /// );
    /// assert_eq!(event.event_type(), "TriggerCreatedEvent");
    /// ```
    /// # Unit Test
    /// ```rust
    /// #[test]
    /// fn test_event_type() {
    ///     use chrono::Utc;
    ///     use serde_json::json;
    ///     use hexafn_trigger::domain::events::trigger_event::{TriggerCreatedEvent, TriggerEvent};
    ///     let event = TriggerCreatedEvent::new(
    ///         "trigger-1".to_string(),
    ///         Utc::now(),
    ///         json!({"name": "Test Trigger"})
    ///     );
    ///     assert_eq!(event.event_type(), "TriggerCreatedEvent");
    /// }
    /// ```
    fn event_type(&self) -> &'static str;

    /// Returns the timestamp of when the event occurred.
    ///
    /// # Example
    /// ```rust
    /// use chrono::Utc;
    /// use serde_json::json;
    /// use hexafn_trigger::domain::events::trigger_event::{TriggerCreatedEvent, TriggerEvent};
    /// let now = Utc::now();
    /// let event = TriggerCreatedEvent::new(
    ///     "trigger-1".to_string(),
    ///     now,
    ///     json!({"name": "Test Trigger"})
    /// );
    /// assert_eq!(event.timestamp(), now);
    /// ```
    /// # Unit Test
    /// ```rust
    /// #[test]
    /// fn test_timestamp() {
    ///     use chrono::Utc;
    ///     use serde_json::json;
    ///     use hexafn_trigger::domain::events::trigger_event::{TriggerCreatedEvent, TriggerEvent};
    ///     let now = Utc::now();
    ///     let event = TriggerCreatedEvent::new(
    ///         "trigger-1".to_string(),
    ///         now,
    ///         json!({"name": "Test Trigger"})
    ///     );
    ///     assert_eq!(event.timestamp(), now);
    /// }
    /// ```
    fn timestamp(&self) -> DateTime<Utc>;

    /// Returns the event payload as a serde_json::Value.
    ///
    /// # Example
    /// ```rust
    /// use chrono::Utc;
    /// use serde_json::json;
    /// use hexafn_trigger::domain::events::trigger_event::{TriggerCreatedEvent, TriggerEvent};
    /// let payload = json!({"name": "Test Trigger"});
    /// let event = TriggerCreatedEvent::new(
    ///     "trigger-1".to_string(),
    ///     Utc::now(),
    ///     payload.clone()
    /// );
    /// assert_eq!(event.payload(), payload);
    /// ```
    /// # Unit Test
    /// ```rust
    /// #[test]
    /// fn test_payload() {
    ///     use chrono::Utc;
    ///     use serde_json::json;
    ///     use hexafn_trigger::domain::events::trigger_event::{TriggerCreatedEvent, TriggerEvent};
    ///     let payload = json!({"name": "Test Trigger"});
    ///     let event = TriggerCreatedEvent::new(
    ///         "trigger-1".to_string(),
    ///         Utc::now(),
    ///         payload.clone()
    ///     );
    ///     assert_eq!(event.payload(), payload);
    /// }
    /// ```
    fn payload(&self) -> Value;
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use serde_json::json;

    #[test]
    fn test_trigger_created_event_trait() {
        let event = TriggerCreatedEvent::new(
            "trigger-1".to_string(),
            Utc::now(),
            json!({"name": "Test Trigger"}),
        );
        assert_eq!(event.event_type(), "TriggerCreatedEvent");
        assert_eq!(event.trigger_id(), "trigger-1");
    }

    #[test]
    fn test_trigger_fired_event_trait() {
        let event = TriggerFiredEvent::new(
            "trigger-1".to_string(),
            "event-1".to_string(),
            Utc::now(),
            json!({"value": 42}),
        );
        assert_eq!(event.event_type(), "TriggerFiredEvent");
        assert_eq!(event.event_id, "event-1");
    }

    #[test]
    fn test_trigger_deactivated_event_trait() {
        let event = TriggerDeactivatedEvent::new("trigger-1".to_string(), Utc::now());
        assert_eq!(event.event_type(), "TriggerDeactivatedEvent");
        assert_eq!(event.trigger_id(), "trigger-1");
    }

    #[test]
    fn test_payload_method() {
        let payload = json!({"name": "Test Trigger"});
        let event = TriggerCreatedEvent::new("trigger-1".to_string(), Utc::now(), payload.clone());
        assert_eq!(event.payload(), payload);
    }
}
