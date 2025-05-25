// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Identifier Types
//!
//! Core identifier value objects for hexaFn system.
//! All identifiers are strongly typed to prevent confusion.

use super::{TypeError, TypeResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use uuid::Uuid;

/// Shared validation logic for all identifier types
fn validate_identifier(value: &String) -> TypeResult<()> {
    if value.is_empty() {
        return Err(TypeError::TooShort { length: 0, min: 1 });
    }

    if value.len() > 255 {
        return Err(TypeError::TooLong {
            length: value.len(),
            max: 255,
        });
    }

    // Allow alphanumeric, hyphens, and underscores
    if !value
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(TypeError::InvalidFormat {
            value: value.to_string(),
        });
    }

    Ok(())
}

/// Correlation ID for tracing related operations
///
/// Used throughout the hexaFn 6F Lifecycle Flow to trace events across
/// Feed → Filter → Format → Function → Forward → Feedback phases.
///
/// # Examples
///
/// ```
/// use hexafn_core::types::CorrelationId;
///
/// // Create a new random correlation ID
/// let id = CorrelationId::new();
/// assert!(!id.value().is_empty());
/// assert_eq!(id.value().len(), 36); // UUID format
///
/// // Create from custom string
/// let custom_id = CorrelationId::from_string("pipeline-trace-001").unwrap();
/// assert_eq!(custom_id.value(), "pipeline-trace-001");
///
/// // Invalid formats will fail
/// assert!(CorrelationId::from_string("invalid@id").is_err());
/// ```
///
/// # Validation Rules
///
/// - Minimum length: 1 character
/// - Maximum length: 255 characters  
/// - Allowed characters: alphanumeric, hyphens (-), underscores (_)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CorrelationId(String);

impl CorrelationId {
    /// Create a new random correlation ID
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::types::CorrelationId;
    ///
    /// let id1 = CorrelationId::new();
    /// let id2 = CorrelationId::new();
    ///
    /// // Each call generates unique ID
    /// assert_ne!(id1, id2);
    /// ```
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Create correlation ID from string with validation
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::types::CorrelationId;
    ///
    /// // Valid IDs
    /// assert!(CorrelationId::from_string("trace-123").is_ok());
    /// assert!(CorrelationId::from_string("pipeline_execution_001").is_ok());
    ///
    /// // Invalid IDs  
    /// assert!(CorrelationId::from_string("").is_err());
    /// assert!(CorrelationId::from_string("invalid@symbol").is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `TypeError` if:
    /// - String is empty
    /// - String is longer than 255 characters
    /// - String contains invalid characters (only alphanumeric, -, _ allowed)
    pub fn from_string(value: impl Into<String>) -> TypeResult<Self> {
        let value = value.into();
        validate_identifier(&value)?; // 🔧 Shared validation function
        Ok(Self(value))
    }

    /// Get the underlying string value
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Default for CorrelationId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for CorrelationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for CorrelationId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid.to_string())
    }
}

/// Trace ID for distributed tracing
///
/// # Examples
///
/// ```
/// use hexafn_core::types::TraceId;
///
/// // Create a new trace ID
/// let trace_id = TraceId::new();
/// assert!(!trace_id.value().is_empty());
///
/// // Create from string
/// let custom_trace = TraceId::from_string("trace-456").unwrap();
/// assert_eq!(custom_trace.value(), "trace-456");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TraceId(String);

impl TraceId {
    /// Create a new random trace ID
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Create trace ID from string with validation
    pub fn from_string(value: impl Into<String>) -> TypeResult<Self> {
        let value = value.into();
        validate_identifier(&value)?; // 🔧 Fixed: Use shared validation
        Ok(Self(value))
    }

    /// Get the underlying string value
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Default for TraceId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for TraceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for TraceId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid.to_string())
    }
}

/// Event ID for domain events
///
/// # Examples
///
/// ```
/// use hexafn_core::types::EventId;
///
/// // Create a new event ID
/// let event_id = EventId::new();
/// assert!(!event_id.value().is_empty());
///
/// // Create from string
/// let custom_event = EventId::from_string("event-789").unwrap();
/// assert_eq!(custom_event.value(), "event-789");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventId(String);

impl EventId {
    /// Create a new random event ID
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Create event ID from string with validation
    pub fn from_string(value: impl Into<String>) -> TypeResult<Self> {
        let value = value.into();
        validate_identifier(&value)?; // 🔧 Fixed: Use shared validation
        Ok(Self(value))
    }

    /// Get the underlying string value
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for EventId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid.to_string())
    }
}

/// Timestamp for events with timezone awareness
///
/// # Examples
///
/// ```
/// use hexafn_core::types::Timestamp;
///
/// // Create current timestamp
/// let now = Timestamp::now();
/// assert!(now.timestamp() > 0);
///
/// // Parse from RFC3339 string
/// let parsed = Timestamp::from_rfc3339("2025-01-25T10:30:00Z").unwrap();
/// assert_eq!(parsed.to_rfc3339(), "2025-01-25T10:30:00+00:00");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    /// Create timestamp for current time
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Create timestamp from DateTime
    pub fn from_datetime(datetime: DateTime<Utc>) -> Self {
        Self(datetime)
    }

    /// Create timestamp from RFC3339 string
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::types::Timestamp;
    ///
    /// let timestamp = Timestamp::from_rfc3339("2025-01-25T10:30:00Z").unwrap();
    /// assert!(timestamp.timestamp() > 0);
    /// ```
    pub fn from_rfc3339(value: &str) -> TypeResult<Self> {
        let datetime = DateTime::parse_from_rfc3339(value)
            .map_err(|e| TypeError::InvalidTimestamp {
                reason: format!("RFC3339 parse error: {}", e),
            })?
            .with_timezone(&Utc);
        Ok(Self(datetime))
    }

    /// Get the underlying DateTime
    pub fn datetime(&self) -> DateTime<Utc> {
        self.0
    }

    /// Convert to RFC3339 string
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }

    /// Get Unix timestamp in seconds
    pub fn timestamp(&self) -> i64 {
        self.0.timestamp()
    }

    /// Get Unix timestamp in milliseconds
    pub fn timestamp_millis(&self) -> i64 {
        self.0.timestamp_millis()
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_rfc3339())
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(datetime: DateTime<Utc>) -> Self {
        Self(datetime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correlation_id_creation() {
        let id1 = CorrelationId::new();
        let id2 = CorrelationId::new();

        // Each ID should be unique
        assert_ne!(id1, id2);

        // Should be valid UUID format
        assert!(!id1.value().is_empty());
        assert_eq!(id1.value().len(), 36); // UUID string length
    }

    #[test]
    fn test_correlation_id_validation() {
        // Valid cases
        assert!(CorrelationId::from_string("valid-id-123").is_ok());
        assert!(CorrelationId::from_string("test_trace_001").is_ok());

        // Invalid cases
        assert!(CorrelationId::from_string("").is_err());
        assert!(CorrelationId::from_string("invalid@id").is_err());
        assert!(CorrelationId::from_string("a".repeat(256)).is_err());
    }

    #[test]
    fn test_trace_id_operations() {
        let trace_id = TraceId::new();

        // Should be displayable
        let display_str = format!("{}", trace_id);
        assert!(!display_str.is_empty());

        // Should be convertible from UUID
        let uuid = Uuid::new_v4();
        let trace_from_uuid = TraceId::from(uuid);
        assert_eq!(trace_from_uuid.value(), uuid.to_string());
    }

    #[test]
    fn test_event_id_serialization() {
        let event_id = EventId::new();

        // Should serialize/deserialize correctly
        let json = serde_json::to_string(&event_id).unwrap();
        let deserialized: EventId = serde_json::from_str(&json).unwrap();

        assert_eq!(event_id, deserialized);
    }

    #[test]
    fn test_timestamp_operations() {
        let now = Timestamp::now();

        // Should be convertible to RFC3339
        let rfc3339 = now.to_rfc3339();
        let parsed = Timestamp::from_rfc3339(&rfc3339).unwrap();

        // Should be approximately equal (within same second)
        assert_eq!(now.timestamp(), parsed.timestamp());

        // Should provide millisecond precision
        assert!(now.timestamp_millis() > 0);
    }

    #[test]
    fn test_timestamp_ordering() {
        let earlier = Timestamp::now();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let later = Timestamp::now();

        assert!(earlier < later);
        assert!(later > earlier);
    }

    #[test]
    fn test_type_error_display() {
        let error = TypeError::InvalidFormat {
            value: "invalid@id".to_string(),
        };

        let error_message = format!("{}", error);
        assert!(error_message.contains("Invalid identifier format"));
        assert!(error_message.contains("invalid@id"));
    }

    #[test]
    fn test_shared_validation_consistency() {
        // All identifier types should follow same validation rules
        let long_string = "a".repeat(256);
        let test_cases = vec![
            ("valid-id-123", true),
            ("test_trace_001", true),
            ("", false),
            ("invalid@id", false),
            (long_string.as_str(), false),
        ];

        for (input, should_be_valid) in test_cases {
            let correlation_result = CorrelationId::from_string(input);
            let trace_result = TraceId::from_string(input);
            let event_result = EventId::from_string(input);

            assert_eq!(correlation_result.is_ok(), should_be_valid);
            assert_eq!(trace_result.is_ok(), should_be_valid);
            assert_eq!(event_result.is_ok(), should_be_valid);
        }
    }

    #[test]
    fn test_identifier_display_consistency() {
        let uuid = Uuid::new_v4();
        let uuid_str = uuid.to_string();

        let correlation_id = CorrelationId::from(uuid);
        let trace_id = TraceId::from(uuid);
        let event_id = EventId::from(uuid);

        assert_eq!(format!("{}", correlation_id), uuid_str);
        assert_eq!(format!("{}", trace_id), uuid_str);
        assert_eq!(format!("{}", event_id), uuid_str);
    }

    #[test]
    fn test_timestamp_precision() {
        let timestamp = Timestamp::now();
        let rfc3339 = timestamp.to_rfc3339();

        // Test that RFC3339 roundtrip preserves precision
        let parsed = Timestamp::from_rfc3339(&rfc3339).unwrap();
        assert_eq!(timestamp.timestamp_millis(), parsed.timestamp_millis());
    }
}
