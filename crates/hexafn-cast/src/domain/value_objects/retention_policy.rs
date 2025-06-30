// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! Value object for topic retention policy in the HexaCast domain.
//!
//! A `RetentionPolicy` determines how many events are stored for a topic and for how long.
//! This helps control memory usage and event replay behavior in the pub/sub system.
//!
//! # Example
//! ```rust
//! use chrono::Duration;
//! use hexafn_cast::RetentionPolicy;
//!
//! let policy = RetentionPolicy::new(100, Some(Duration::seconds(60)));
//! assert_eq!(policy.max_events, 100);
//! assert_eq!(policy.ttl.unwrap().num_seconds(), 60);
//! ```

use chrono::Duration;
use hexafn_core::HexaCoreError;
use std::fmt;

/// Retention policy configuration for a topic.
///
/// A retention policy determines how many events are stored for a topic and for how long.
/// This helps control memory usage and event replay behavior in the pub/sub system.
///
/// - `max_events`: The maximum number of events to retain in the topic.
/// - `ttl`: The optional time-to-live (in chrono::Duration) for each event. If `None`, events are kept until `max_events` is reached.
///
/// # Example
/// ```rust
/// use chrono::Duration;
/// use hexafn_cast::RetentionPolicy;
/// let policy = RetentionPolicy::new(10, Some(Duration::seconds(30)));
/// assert_eq!(policy.max_events, 10);
/// assert_eq!(policy.ttl.unwrap().num_seconds(), 30);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetentionPolicy {
    /// The maximum number of events to retain in the topic.
    pub max_events: u64,
    /// The optional time-to-live for each event (in chrono::Duration).
    pub ttl: Option<Duration>,
}

impl RetentionPolicy {
    /// Creates a new `RetentionPolicy` with the given max_events and optional ttl.
    ///
    /// # Example
    /// ```rust
    /// use chrono::Duration;
    /// use hexafn_cast::RetentionPolicy;
    /// let policy = RetentionPolicy::new(100, Some(Duration::seconds(60)));
    /// assert_eq!(policy.max_events, 100);
    /// assert_eq!(policy.ttl.unwrap().num_seconds(), 60);
    /// ```
    pub fn new(max_events: u64, ttl: Option<Duration>) -> Self {
        Self { max_events, ttl }
    }

    /// Validates the retention policy.
    ///
    /// Returns `Ok(())` if the policy is valid, or a [`HexaCoreError`] if invalid.
    ///
    /// # Rules
    /// - `max_events` must be greater than 0.
    /// - If `ttl` is provided, it must be positive.
    ///
    /// # Example
    /// ```rust
    /// use chrono::Duration;
    /// use hexafn_cast::RetentionPolicy;
    /// let policy = RetentionPolicy::new(1, Some(Duration::seconds(1)));
    /// assert!(policy.validate().is_ok());
    /// let invalid = RetentionPolicy::new(0, None);
    /// assert!(invalid.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<(), HexaCoreError> {
        if self.max_events == 0 {
            return Err(
                HexaCoreError::new("cast.retention_policy.max_events.invalid")
                    .with_message("max_events must be greater than 0".to_string()),
            );
        }
        if let Some(ttl) = self.ttl {
            if ttl.num_seconds() <= 0 {
                return Err(HexaCoreError::new("cast.retention_policy.ttl.invalid")
                    .with_message("ttl must be positive".to_string()));
            }
        }
        Ok(())
    }
}

impl fmt::Display for RetentionPolicy {
    /// Formats the retention policy as a human-readable string.
    ///
    /// # Example
    /// ```rust
    /// use chrono::Duration;
    /// use hexafn_cast::RetentionPolicy;
    /// let policy = RetentionPolicy::new(5, Some(Duration::seconds(10)));
    /// assert_eq!(format!("{}", policy), "max_events: 5, ttl: 10s");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "max_events: {}", self.max_events)?;
        if let Some(ttl) = self.ttl {
            write!(f, ", ttl: {}s", ttl.num_seconds())
        } else {
            write!(f, ", ttl: None")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_new_and_fields() {
        let policy = RetentionPolicy::new(100, Some(Duration::seconds(60)));
        assert_eq!(policy.max_events, 100);
        assert_eq!(policy.ttl, Some(Duration::seconds(60)));
    }

    #[test]
    fn test_display_with_ttl() {
        let policy = RetentionPolicy::new(10, Some(Duration::seconds(30)));
        assert_eq!(format!("{}", policy), "max_events: 10, ttl: 30s");
    }

    #[test]
    fn test_display_without_ttl() {
        let policy = RetentionPolicy::new(5, None);
        assert_eq!(format!("{}", policy), "max_events: 5, ttl: None");
    }

    #[test]
    fn test_validate_valid() {
        let policy = RetentionPolicy::new(1, Some(Duration::seconds(1)));
        assert!(policy.validate().is_ok());
        let policy = RetentionPolicy::new(1, None);
        assert!(policy.validate().is_ok());
    }

    #[test]
    fn test_validate_invalid_max_events() {
        let policy = RetentionPolicy::new(0, Some(Duration::seconds(10)));
        let err = policy.validate();
        assert!(err.is_err());
        let msg = format!("{}", err.unwrap_err());
        assert!(msg.contains("max_events must be greater than 0"));
    }

    #[test]
    fn test_validate_invalid_ttl() {
        let policy = RetentionPolicy::new(10, Some(Duration::seconds(0)));
        let err = policy.validate();
        assert!(err.is_err());
        let msg = format!("{}", err.unwrap_err());
        assert!(msg.contains("ttl must be positive"));

        let policy = RetentionPolicy::new(10, Some(Duration::seconds(-5)));
        let err = policy.validate();
        assert!(err.is_err());
        let msg = format!("{}", err.unwrap_err());
        assert!(msg.contains("ttl must be positive"));
    }

    #[test]
    fn test_equality() {
        let a = RetentionPolicy::new(10, Some(Duration::seconds(5)));
        let b = RetentionPolicy::new(10, Some(Duration::seconds(5)));
        let c = RetentionPolicy::new(10, None);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_doc_example() {
        // Example from struct-level doc
        let policy = RetentionPolicy::new(10, Some(Duration::seconds(30)));
        assert_eq!(policy.max_events, 10);
        assert_eq!(policy.ttl.unwrap().num_seconds(), 30);
    }

    #[test]
    fn test_doc_display_example() {
        // Example from Display doc
        let policy = RetentionPolicy::new(5, Some(Duration::seconds(10)));
        assert_eq!(format!("{}", policy), "max_events: 5, ttl: 10s");
    }

    #[test]
    fn test_doc_validate_example() {
        // Example from validate doc
        let policy = RetentionPolicy::new(1, Some(Duration::seconds(1)));
        assert!(policy.validate().is_ok());
        let invalid = RetentionPolicy::new(0, None);
        assert!(invalid.validate().is_err());
    }
}
