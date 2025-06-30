// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Span Entity
//!
//! Represents a single span in a distributed trace for observability and tracing in the hexaFn system.
//!
//! A span models a logical unit of work or operation within a trace, including timing, parent-child relationships, and arbitrary attributes for context propagation and observability.
//!
//! ## Example
//! ```rust
//! use hexafn_watch::Span;
//! use chrono::Utc;
//! use std::collections::HashMap;
//! let span = Span::new(
//!     "span-1".to_string(),
//!     "db_query".to_string(),
//!     Some("root-1".to_string()),
//!     Utc::now(),
//!     None,
//!     HashMap::new(),
//! );
//! assert_eq!(span.name, "db_query");
//! ```

use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Represents a single span in a distributed trace.
///
/// A span models a logical unit of work or operation within a trace, including timing, parent-child relationships, and arbitrary attributes for context propagation and observability.
///
/// # Example
/// ```rust
/// use hexafn_watch::Span;
/// use chrono::Utc;
/// use std::collections::HashMap;
/// let span = Span::new(
///     "span-1".to_string(),
///     "db_query".to_string(),
///     Some("root-1".to_string()),
///     Utc::now(),
///     None,
///     HashMap::new(),
/// );
/// assert_eq!(span.name, "db_query");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    /// Unique identifier for the span.
    pub id: String,
    /// Name of the operation or logical unit.
    pub name: String,
    /// Optional parent span ID for hierarchical traces.
    pub parent_id: Option<String>,
    /// Start timestamp of the span.
    pub start_time: DateTime<Utc>,
    /// Optional end timestamp (None if still open).
    pub end_time: Option<DateTime<Utc>>,
    /// Arbitrary key-value attributes for context propagation.
    pub attributes: HashMap<String, String>,
}

impl Span {
    /// Builder for a new `Span` value object.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_watch::Span;
    /// use chrono::Utc;
    /// use std::collections::HashMap;
    /// let span = Span::new(
    ///     "span-1".to_string(),
    ///     "op".to_string(),
    ///     None,
    ///     Utc::now(),
    ///     None,
    ///     HashMap::new(),
    /// );
    /// assert_eq!(span.name, "op");
    /// ```
    pub fn new(
        id: String,
        name: String,
        parent_id: Option<String>,
        start_time: DateTime<Utc>,
        end_time: Option<DateTime<Utc>>,
        attributes: HashMap<String, String>,
    ) -> Self {
        Self {
            id,
            name,
            parent_id,
            start_time,
            end_time,
            attributes,
        }
    }
}

impl std::fmt::Display for Span {
    /// Formats the span as a human-readable string.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_watch::Span;
    /// use chrono::Utc;
    /// use std::collections::HashMap;
    /// let span = Span::new(
    ///     "s1".to_string(), "op".to_string(), None, Utc::now(), None, HashMap::new()
    /// );
    /// let s = format!("{}", span);
    /// assert!(s.contains("op"));
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Span[id: '{}', name: '{}', parent: {:?}, start: {}, end: {:?}]",
            self.id, self.name, self.parent_id, self.start_time, self.end_time
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use std::collections::HashMap;

    #[test]
    fn test_span_display() {
        let span = Span::new(
            "s1".to_string(),
            "op".to_string(),
            Some("p1".to_string()),
            Utc::now(),
            None,
            HashMap::new(),
        );
        let s = format!("{}", span);
        assert!(s.contains("op"));
        assert!(s.contains("s1"));
    }

    #[test]
    fn test_span_equality_and_clone() {
        let mut attrs = HashMap::new();
        attrs.insert("key".to_string(), "val".to_string());
        let span1 = Span::new(
            "id1".to_string(),
            "span".to_string(),
            None,
            Utc::now(),
            None,
            attrs.clone(),
        );
        let span2 = span1.clone();
        assert_eq!(span1, span2);
        assert_eq!(span1.attributes["key"], "val");
    }

    #[test]
    fn test_span_with_end_time() {
        let now = Utc::now();
        let end = now + Duration::seconds(1);
        let span = Span::new(
            "id2".to_string(),
            "finished".to_string(),
            None,
            now,
            Some(end),
            HashMap::new(),
        );
        assert!(span.end_time.is_some());
        let s = format!("{}", span);
        assert!(s.contains("finished"));
    }

    #[test]
    fn test_span_parent_child() {
        let parent = Span::new(
            "parent".to_string(),
            "parent_span".to_string(),
            None,
            Utc::now(),
            None,
            HashMap::new(),
        );
        let child = Span::new(
            "child".to_string(),
            "child_span".to_string(),
            Some(parent.id.clone()),
            Utc::now(),
            None,
            HashMap::new(),
        );
        assert_eq!(child.parent_id, Some("parent".to_string()));
    }

    #[test]
    fn test_span_attributes() {
        let mut attrs = HashMap::new();
        attrs.insert("user_id".to_string(), "42".to_string());
        let span = Span::new(
            "id3".to_string(),
            "attr_span".to_string(),
            None,
            Utc::now(),
            None,
            attrs.clone(),
        );
        assert_eq!(span.attributes["user_id"], "42");
    }
}
