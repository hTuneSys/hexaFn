// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Distributed Tracing Domain Contracts
//!
//! This module defines the core types and contracts for distributed tracing in the HexaWatch observability system.
//!
//! ## Overview
//!
//! - [`Span`]: Represents a single logical operation or unit of work in a distributed trace, including timing, parent-child relationships, and attributes for context propagation.
//! - [`Trace`]: Trait for distributed tracing systems, supporting span creation, lookup, and completion. Implementations may use in-memory, OpenTelemetry, or custom backends.
//!
//! ## Example Usage
//!
//! ```rust
//! use hexafn_watch::domain::contracts::Span;
//! use chrono::Utc;
//! use std::collections::HashMap;
//! let span = Span {
//!     id: "span-1".to_string(),
//!     name: "http_request".to_string(),
//!     parent_id: None,
//!     start_time: Utc::now(),
//!     end_time: None,
//!     attributes: HashMap::new(),
//! };
//! assert_eq!(span.name, "http_request");
//! ```
//!
//! ## Unit Test
//!
//! ```rust
//! use hexafn_watch::domain::contracts::Span;
//! use chrono::Utc;
//! use std::collections::HashMap;
//! let span = Span {
//!     id: "s1".to_string(),
//!     name: "op".to_string(),
//!     parent_id: Some("p1".to_string()),
//!     start_time: Utc::now(),
//!     end_time: None,
//!     attributes: HashMap::new(),
//! };
//! let s = format!("{}", span);
//! assert!(s.contains("op"));
//! assert!(s.contains("s1"));
//! ```
//!
//! ## Trace Trait Example
//!
//! ```rust
//! use hexafn_watch::domain::contracts::{Trace, Span};
//! struct MyTracer;
//! impl Trace for MyTracer {
//!     fn start_span(&self, name: String) -> Result<Span, Box<dyn hexafn_core::HexaError>> {
//!         Ok(Span {
//!             id: "span-1".to_string(),
//!             name,
//!             parent_id: None,
//!             start_time: chrono::Utc::now(),
//!             end_time: None,
//!             attributes: std::collections::HashMap::new(),
//!         })
//!     }
//!     fn current_span(&self) -> Option<Span> { None }
//!     fn get_trace_id(&self) -> String { "trace-1".to_string() }
//!     fn finish_span(&self, _span: Span) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
//! }
//! let tracer = MyTracer;
//! let span = tracer.start_span("test".to_string()).unwrap();
//! assert_eq!(span.name, "test");
//! ```
//!

use chrono::{DateTime, Utc};
use hexafn_core::HexaError;
use std::collections::HashMap;

/// Represents a single span in a distributed trace.
///
/// A span models a logical unit of work or operation within a trace, including timing, parent-child relationships, and arbitrary attributes for context propagation and observability.
///
/// # Example
/// ```rust
/// use hexafn_watch::domain::contracts::Span;
/// use chrono::Utc;
/// use std::collections::HashMap;
/// let span = Span {
///     id: "span-1".to_string(),
///     name: "db_query".to_string(),
///     parent_id: Some("root-1".to_string()),
///     start_time: Utc::now(),
///     end_time: None,
///     attributes: HashMap::new(),
/// };
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

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Span[id: '{}', name: '{}', parent: {:?}, start: {}, end: {:?}]",
            self.id, self.name, self.parent_id, self.start_time, self.end_time
        )
    }
}

/// Trait for distributed tracing and span management.
///
/// This trait defines the contract for distributed tracing systems, supporting span creation, lookup, and completion. Implementations may provide in-memory, OpenTelemetry, or custom backends.
///
/// # Example
/// ```rust
/// use hexafn_watch::domain::contracts::{Trace, Span};
/// struct MyTracer;
/// impl Trace for MyTracer {
///     fn start_span(&self, name: String) -> Result<Span, Box<dyn hexafn_core::HexaError>> {
///         Ok(Span {
///             id: "span-1".to_string(),
///             name,
///             parent_id: None,
///             start_time: chrono::Utc::now(),
///             end_time: None,
///             attributes: std::collections::HashMap::new(),
///         })
///     }
///     fn current_span(&self) -> Option<Span> { None }
///     fn get_trace_id(&self) -> String { "trace-1".to_string() }
///     fn finish_span(&self, _span: Span) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
/// }
/// let tracer = MyTracer;
/// let span = tracer.start_span("test".to_string()).unwrap();
/// assert_eq!(span.name, "test");
/// ```
pub trait Trace {
    /// Start a new span with the given name.
    ///
    /// This method creates and returns a new span, which represents a logical unit of work in the trace. The span should be finished with [`finish_span`] when the operation completes.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_watch::domain::contracts::{Trace, Span};
    /// struct MyTracer;
    /// impl Trace for MyTracer {
    ///     fn start_span(&self, name: String) -> Result<Span, Box<dyn hexafn_core::HexaError>> {
    ///         Ok(Span {
    ///             id: "span-1".to_string(),
    ///             name,
    ///             parent_id: None,
    ///             start_time: chrono::Utc::now(),
    ///             end_time: None,
    ///             attributes: std::collections::HashMap::new(),
    ///         })
    ///     }
    ///     fn current_span(&self) -> Option<Span> { None }
    ///     fn get_trace_id(&self) -> String { "trace-1".to_string() }
    ///     fn finish_span(&self, _span: Span) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// }
    /// let tracer = MyTracer;
    /// let span = tracer.start_span("db_query".to_string()).unwrap();
    /// assert_eq!(span.name, "db_query");
    /// ```
    fn start_span(&self, name: String) -> Result<Span, Box<dyn HexaError>>;

    /// Get the current active span, if any.
    ///
    /// This method returns the currently active span in the trace context, or `None` if no span is active. Useful for propagating context or adding child spans.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_watch::domain::contracts::{Trace, Span};
    /// struct MyTracer;
    /// impl Trace for MyTracer {
    ///     fn start_span(&self, _name: String) -> Result<Span, Box<dyn hexafn_core::HexaError>> { unimplemented!() }
    ///     fn current_span(&self) -> Option<Span> { None }
    ///     fn get_trace_id(&self) -> String { "trace-1".to_string() }
    ///     fn finish_span(&self, _span: Span) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// }
    /// let tracer = MyTracer;
    /// assert!(tracer.current_span().is_none());
    /// ```
    fn current_span(&self) -> Option<Span>;

    /// Get the unique trace ID for the current trace context.
    ///
    /// This method returns a globally unique identifier for the trace, which can be used to correlate logs, metrics, and spans across distributed systems.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_watch::domain::contracts::{Trace,Span};
    /// struct MyTracer;
    /// impl Trace for MyTracer {
    ///     fn start_span(&self, _name: String) -> Result<Span, Box<dyn hexafn_core::HexaError>> { unimplemented!() }
    ///     fn current_span(&self) -> Option<Span> { None }
    ///     fn get_trace_id(&self) -> String { "trace-1234".to_string() }
    ///     fn finish_span(&self, _span: Span) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// }
    /// let tracer = MyTracer;
    /// assert_eq!(tracer.get_trace_id(), "trace-1234");
    /// ```
    fn get_trace_id(&self) -> String;

    /// Finish and record the given span.
    ///
    /// This method marks the span as completed and records it in the tracing backend. It should be called exactly once for each span created with [`start_span`].
    ///
    /// # Example
    /// ```rust
    /// use hexafn_watch::domain::contracts::{Trace, Span};
    /// struct MyTracer;
    /// impl Trace for MyTracer {
    ///     fn start_span(&self, name: String) -> Result<Span, Box<dyn hexafn_core::HexaError>> {
    ///         Ok(Span {
    ///             id: "span-1".to_string(),
    ///             name,
    ///             parent_id: None,
    ///             start_time: chrono::Utc::now(),
    ///             end_time: None,
    ///             attributes: std::collections::HashMap::new(),
    ///         })
    ///     }
    ///     fn current_span(&self) -> Option<Span> { None }
    ///     fn get_trace_id(&self) -> String { "trace-1".to_string() }
    ///     fn finish_span(&self, _span: Span) -> Result<(), Box<dyn hexafn_core::HexaError>> { Ok(()) }
    /// }
    /// let tracer = MyTracer;
    /// let span = tracer.start_span("api_call".to_string()).unwrap();
    /// let result = tracer.finish_span(span);
    /// assert!(result.is_ok());
    /// ```
    fn finish_span(&self, span: Span) -> Result<(), Box<dyn HexaError>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use std::collections::HashMap;

    #[test]
    fn test_span_display() {
        let span = Span {
            id: "s1".to_string(),
            name: "op".to_string(),
            parent_id: Some("p1".to_string()),
            start_time: Utc::now(),
            end_time: None,
            attributes: HashMap::new(),
        };
        let s = format!("{}", span);
        assert!(s.contains("op"));
        assert!(s.contains("s1"));
    }

    #[test]
    fn test_span_equality_and_clone() {
        let mut attrs = HashMap::new();
        attrs.insert("key".to_string(), "val".to_string());
        let span1 = Span {
            id: "id1".to_string(),
            name: "span".to_string(),
            parent_id: None,
            start_time: Utc::now(),
            end_time: None,
            attributes: attrs.clone(),
        };
        let span2 = span1.clone();
        assert_eq!(span1, span2);
        assert_eq!(span1.attributes["key"], "val");
    }

    #[test]
    fn test_span_with_end_time() {
        let now = Utc::now();
        let end = now + Duration::seconds(1);
        let span = Span {
            id: "id2".to_string(),
            name: "finished".to_string(),
            parent_id: None,
            start_time: now,
            end_time: Some(end),
            attributes: HashMap::new(),
        };
        assert!(span.end_time.is_some());
        let s = format!("{}", span);
        assert!(s.contains("finished"));
    }

    #[test]
    fn test_span_parent_child() {
        let parent = Span {
            id: "parent".to_string(),
            name: "parent_span".to_string(),
            parent_id: None,
            start_time: Utc::now(),
            end_time: None,
            attributes: HashMap::new(),
        };
        let child = Span {
            id: "child".to_string(),
            name: "child_span".to_string(),
            parent_id: Some(parent.id.clone()),
            start_time: Utc::now(),
            end_time: None,
            attributes: HashMap::new(),
        };
        assert_eq!(child.parent_id, Some("parent".to_string()));
    }

    struct MockError;
    impl std::fmt::Debug for MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MockError")
        }
    }
    impl std::fmt::Display for MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MockError")
        }
    }
    impl hexafn_core::HexaError for MockError {
        fn error_code(&self) -> &'static str {
            "mock"
        }
        fn error_message(&self) -> &str {
            "mock error"
        }
        fn error_kind(&self) -> hexafn_core::HexaErrorKind {
            hexafn_core::HexaErrorKind::Unknown
        }
        fn error_severity(&self) -> hexafn_core::HexaErrorSeverity {
            hexafn_core::HexaErrorSeverity::Low
        }
        fn source(&self) -> Option<&(dyn std::error::Error + Send + Sync + 'static)> {
            None
        }
        fn to_log_entry(&self) -> String {
            "mock log".to_string()
        }
    }

    struct MockTracer {
        pub last_span: std::cell::RefCell<Option<Span>>,
        pub trace_id: String,
    }
    impl MockTracer {
        fn new(trace_id: &str) -> Self {
            Self {
                last_span: std::cell::RefCell::new(None),
                trace_id: trace_id.to_string(),
            }
        }
    }
    impl Trace for MockTracer {
        fn start_span(&self, name: String) -> Result<Span, Box<dyn hexafn_core::HexaError>> {
            let span = Span {
                id: format!("{}-span", name),
                name: name.clone(),
                parent_id: None,
                start_time: Utc::now(),
                end_time: None,
                attributes: HashMap::new(),
            };
            self.last_span.replace(Some(span.clone()));
            Ok(span)
        }
        fn current_span(&self) -> Option<Span> {
            self.last_span.borrow().clone()
        }
        fn get_trace_id(&self) -> String {
            self.trace_id.clone()
        }
        fn finish_span(&self, span: Span) -> Result<(), Box<dyn hexafn_core::HexaError>> {
            if span.name.is_empty() {
                Err(Box::new(MockError))
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn test_trace_trait_impl() {
        let tracer = MockTracer::new("trace-xyz");
        let span = tracer.start_span("work".to_string()).unwrap();
        assert_eq!(span.name, "work");
        assert!(tracer.current_span().is_some());
        assert_eq!(tracer.get_trace_id(), "trace-xyz");
        assert!(tracer.finish_span(span).is_ok());
    }

    #[test]
    fn test_trace_finish_span_error() {
        let tracer = MockTracer::new("trace-err");
        let span = Span {
            id: "id".to_string(),
            name: "".to_string(),
            parent_id: None,
            start_time: Utc::now(),
            end_time: None,
            attributes: HashMap::new(),
        };
        let result = tracer.finish_span(span);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.error_code(), "mock");
    }
}
