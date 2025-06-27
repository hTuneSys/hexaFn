// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Topic Domain Contracts
//!
//! This module provides the core domain types for topic management and retention policy in the HexaCast pub/sub system.
//!
//! Topics represent named channels for publishing and subscribing to messages. Each topic has a retention policy that controls how many events are kept and for how long.
//!
//! ## Example Usage
//!
//! ```rust
//! use hexafn_cast::domain::contracts::{Topic, RetentionPolicy};
//! // Create a topic with a name and default retention policy
//! let topic = Topic::new("my-topic".to_string()).unwrap()
//!     .with_description("A topic for user events".to_string());
//! assert!(topic.is_valid());
//! // Access topic metadata
//! println!("{}", topic);
//! ```
//!
//! ## Test
//!
//! ```rust
//! use hexafn_cast::domain::contracts::Topic;
//! let topic = Topic::new("test".to_string());
//! assert!(topic.is_ok());
//! ```

use chrono::{DateTime, Duration, Utc};
use hexafn_core::{HexaCoreError, HexaError};
use std::fmt;

/// Retention policy configuration for a topic.
///
/// A retention policy determines how many events are stored for a topic and for how long.
/// This helps control memory usage and event replay behavior in the pub/sub system.
///
/// - `max_events`: The maximum number of events to retain in the topic.
/// - `ttl`: The optional time-to-live (in seconds) for each event. If `None`, events are kept until `max_events` is reached.
///
/// # Example
/// ```rust
/// use hexafn_cast::domain::contracts::RetentionPolicy;
/// let policy = RetentionPolicy { max_events: 100, ttl: Some(chrono::Duration::seconds(60)) };
/// assert_eq!(policy.max_events, 100);
/// assert_eq!(policy.ttl.unwrap().num_seconds(), 60);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetentionPolicy {
    /// The maximum number of events to retain in the topic.
    pub max_events: u64,
    /// The optional time-to-live for each event (in chrono::Duration).
    pub ttl: Option<Duration>,
}

impl fmt::Display for RetentionPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "max_events: {}", self.max_events)?;
        if let Some(ttl) = self.ttl {
            write!(f, ", ttl: {}s", ttl.num_seconds())
        } else {
            write!(f, ", ttl: None")
        }
    }
}

/// Represents a topic in the pub/sub system.
///
/// A topic is a named channel for publishing and subscribing to messages. Each topic has a name, an optional description, a creation timestamp, and a retention policy that controls how long events are kept.
///
/// Topics are created using [`Topic::new`], and can be further configured using builder-style methods such as [`with_description`].
///
/// # Example
/// ```rust
/// use hexafn_cast::domain::contracts::Topic;
/// let topic = Topic::new("demo".to_string()).unwrap().with_description("Demo topic".to_string());
/// assert_eq!(topic.name(), "demo");
/// assert_eq!(topic.description(), "Demo topic");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Topic {
    name: String,
    description: String,
    created_at: DateTime<Utc>,
    retention_policy: RetentionPolicy,
}

impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Topic[name: '{}', desc: '{}', created_at: {}, retention: {}]",
            self.name, self.description, self.created_at, self.retention_policy
        )
    }
}

impl Topic {
    /// Creates a new topic with the given name and default retention policy.
    ///
    /// The default retention policy keeps up to 1000 events and does not set a TTL.
    ///
    /// # Errors
    /// Returns an error if the name is empty.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_cast::domain::contracts::Topic;
    /// let topic = Topic::new("abc".to_string());
    /// assert!(topic.is_ok());
    /// let topic = Topic::new(String::new());
    /// assert!(topic.is_err());
    /// ```
    pub fn new(name: String) -> Result<Self, Box<dyn HexaError>> {
        if name.is_empty() {
            return Err(Box::new(
                HexaCoreError::new("cast.topic.validation.empty_name")
                    .with_message("Topic name cannot be empty"),
            ));
        }
        Ok(Self {
            name,
            description: String::new(),
            created_at: Utc::now(),
            retention_policy: RetentionPolicy {
                max_events: 1000,
                ttl: None,
            },
        })
    }

    /// Sets the description for the topic. Returns a new topic with the updated description.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_cast::domain::contracts::Topic;
    /// let topic = Topic::new("abc".to_string()).unwrap().with_description("desc".to_string());
    /// assert_eq!(topic.description(), "desc");
    /// ```
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Checks if the topic is valid.
    ///
    /// A topic is valid if its name is not empty and its retention policy allows at least one event.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_cast::domain::contracts::{Topic, RetentionPolicy};
    /// let policy = RetentionPolicy { max_events: 500, ttl: Some(chrono::Duration::seconds(120)) };
    /// let topic = Topic::new("abc".to_string()).unwrap()
    ///     .with_retention_policy(policy.clone());
    /// assert!(topic.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty() && self.retention_policy.max_events > 0
    }

    /// Sets the retention policy for the topic (builder style).
    ///
    /// This method replaces the topic's retention policy with the provided one and returns a new topic instance.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_cast::domain::contracts::{Topic, RetentionPolicy};
    /// let policy = RetentionPolicy { max_events: 500, ttl: Some(chrono::Duration::seconds(120)) };
    /// let topic = Topic::new("abc".to_string()).unwrap()
    ///     .with_retention_policy(policy.clone());
    /// assert_eq!(topic.retention_policy().max_events, 500);
    /// assert_eq!(topic.retention_policy().ttl.unwrap().num_seconds(), 120);
    /// ```
    pub fn with_retention_policy(mut self, policy: RetentionPolicy) -> Self {
        self.retention_policy = policy;
        self
    }

    /// Returns the topic name as a string slice.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_cast::domain::contracts::Topic;
    /// let topic = Topic::new("abc".to_string()).unwrap();
    /// assert_eq!(topic.name(), "abc");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the topic description as a string slice.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_cast::domain::contracts::Topic;
    /// let topic = Topic::new("abc".to_string()).unwrap().with_description("desc".to_string());
    /// assert_eq!(topic.description(), "desc");
    /// ```
    pub fn description(&self) -> &str {
        &self.description
    }
    /// Returns the creation time as a chrono::DateTime<Utc>.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_cast::domain::contracts::Topic;
    /// let topic = Topic::new("abc".to_string()).unwrap();
    /// let _ = topic.created_at();
    /// ```
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    /// Returns a reference to the topic's retention policy.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_cast::domain::contracts::{Topic, RetentionPolicy};
    /// let topic = Topic::new("abc".to_string()).unwrap();
    /// let policy: &RetentionPolicy = topic.retention_policy();
    /// assert!(policy.max_events > 0);
    /// ```
    pub fn retention_policy(&self) -> &RetentionPolicy {
        &self.retention_policy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topic_new_valid() {
        let t = Topic::new("topic1".to_string());
        assert!(t.is_ok());
        let t = t.unwrap();
        assert_eq!(t.name(), "topic1");
        assert!(t.is_valid());
    }

    #[test]
    fn test_topic_new_empty_name() {
        let t = Topic::new(String::new());
        assert!(t.is_err());
    }

    #[test]
    fn test_with_description() {
        let t = Topic::new("topic2".to_string())
            .unwrap()
            .with_description("desc".to_string());
        assert_eq!(t.description(), "desc");
    }

    #[test]
    fn test_display_topic_and_retention() {
        let t = Topic::new("topic3".to_string())
            .unwrap()
            .with_description("d".to_string());
        let s = format!("{}", t);
        assert!(s.contains("topic3"));
        assert!(s.contains("d"));
        let r = t.retention_policy();
        let s2 = format!("{}", r);
        assert!(s2.contains("max_events"));
    }

    #[test]
    fn test_is_valid_false_when_max_events_zero() {
        let mut t = Topic::new("topic4".to_string()).unwrap();
        t.retention_policy.max_events = 0;
        assert!(!t.is_valid());
    }
}
