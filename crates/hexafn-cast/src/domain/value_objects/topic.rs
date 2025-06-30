// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Topic Value Object
//!
//! This module defines the [`Topic`] value object and its associated methods for the HexaCast pub/sub system.
//!
//! Topics represent named channels for publishing and subscribing to messages. Each topic has a retention policy that controls how many events are kept and for how long.
//!
//! ## Example
//!
//! ```rust
//! use hexafn_cast::Topic;
//! let topic = Topic::new("my-topic".to_string()).unwrap();
//! assert!(topic.is_valid());
//! ```

use crate::RetentionPolicy;
use chrono::{DateTime, Utc};
use hexafn_core::{HexaCoreError, HexaError};
use std::fmt;

/// Represents a topic in the pub/sub system.
///
/// A topic is a named channel for publishing and subscribing to messages. Each topic has a name, an optional description, a creation timestamp, and a retention policy that controls how long events are kept.
///
/// # Example
/// ```rust
/// use hexafn_cast::Topic;
/// let topic = Topic::new("demo".to_string()).unwrap();
/// assert_eq!(topic.name(), "demo");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Topic {
    /// The name of the topic.
    name: String,
    /// An optional description of the topic.
    description: String,
    /// The creation timestamp of the topic.
    created_at: DateTime<Utc>,
    /// The retention policy for the topic, which determines how many events are stored and for how long.
    retention_policy: RetentionPolicy,
}

impl fmt::Display for Topic {
    /// Formats the topic as a user-friendly string.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_cast::Topic;
    /// let topic = Topic::new("abc".to_string()).unwrap();
    /// let s = format!("{}", topic);
    /// assert!(s.contains("abc"));
    /// ```
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
    /// # Errors
    /// Returns an error if the name is empty.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_cast::Topic;
    /// let topic = Topic::new("abc".to_string());
    /// assert!(topic.is_ok());
    /// ```
    pub fn new(name: String) -> Result<Self, Box<dyn HexaError>> {
        if name.is_empty() {
            return Err(Box::new(
                HexaCoreError::new("cast.topic.validation.name.empty")
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
    /// use hexafn_cast::Topic;
    /// let topic = Topic::new("abc".to_string()).unwrap().with_description("desc".to_string());
    /// assert_eq!(topic.description(), "desc");
    /// ```
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Checks if the topic is valid.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_cast::Topic;
    /// let topic = Topic::new("abc".to_string()).unwrap();
    /// assert!(topic.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty() && self.retention_policy.max_events > 0
    }

    /// Sets the retention policy for the topic (builder style).
    ///
    /// # Example
    /// ```rust
    /// use hexafn_cast::{Topic, RetentionPolicy};
    /// let policy = RetentionPolicy { max_events: 500, ttl: None };
    /// let topic = Topic::new("abc".to_string()).unwrap().with_retention_policy(policy.clone());
    /// assert_eq!(topic.retention_policy().max_events, 500);
    /// ```
    pub fn with_retention_policy(mut self, policy: RetentionPolicy) -> Self {
        self.retention_policy = policy;
        self
    }

    /// Returns the topic name as a string slice.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_cast::Topic;
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
    /// use hexafn_cast::Topic;
    /// let topic = Topic::new("abc".to_string()).unwrap().with_description("desc".to_string());
    /// assert_eq!(topic.description(), "desc");
    /// ```
    pub fn description(&self) -> &str {
        &self.description
    }
    /// Returns the creation time as a [`chrono::DateTime<Utc>`].
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    /// Returns a reference to the topic's retention policy.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_cast::{Topic, RetentionPolicy};
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
    use crate::RetentionPolicy;
    use chrono::Duration;

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
        let s = format!("{t}");
        assert!(s.contains("topic3"));
        assert!(s.contains("d"));
        let r = t.retention_policy();
        let s2 = format!("{r}");
        assert!(s2.contains("max_events"));
    }

    #[test]
    fn test_is_valid_false_when_max_events_zero() {
        let mut t = Topic::new("topic4".to_string()).unwrap();
        t.retention_policy.max_events = 0;
        assert!(!t.is_valid());
    }

    #[test]
    fn test_with_retention_policy() {
        let policy = RetentionPolicy {
            max_events: 42,
            ttl: Some(Duration::seconds(99)),
        };
        let t = Topic::new("topic5".to_string())
            .unwrap()
            .with_retention_policy(policy.clone());
        assert_eq!(t.retention_policy().max_events, 42);
        assert_eq!(t.retention_policy().ttl.unwrap().num_seconds(), 99);
    }

    #[test]
    fn test_doc_example() {
        let topic = Topic::new("my-topic".to_string()).unwrap();
        assert!(topic.is_valid());
    }
}
