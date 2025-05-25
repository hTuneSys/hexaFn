// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Phase Execution Context
//! 
//! This module provides the PhaseContext struct for tracking phase execution state and metadata.

use super::lifecycle::{get_phase_order, next_phase, previous_phase};
use std::collections::HashMap;
use std::time::Instant;

/// Phase execution context
/// 
/// Tracks the execution state and metadata for a single phase in the 6F Lifecycle Flow.
/// This includes timing information, correlation IDs for tracing, and custom metadata.
#[derive(Debug, Clone)]
pub struct PhaseContext {
    /// Current phase name
    pub phase: String,
    /// Execution order (1-6 for 6F phases)
    pub order: u8,
    /// Correlation ID for distributed tracing
    pub correlation_id: Option<String>,
    /// Start timestamp for duration tracking
    pub started_at: Instant,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

impl PhaseContext {
    /// Create a new phase context
    ///
    /// Initializes a new context for the specified phase with current timestamp.
    /// The execution order is automatically determined from the phase name.
    ///
    /// # Arguments
    ///
    /// * `phase` - The name of the 6F phase ("feed", "filter", etc.)
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::{PhaseContext, FEED};
    ///
    /// let context = PhaseContext::new(FEED);
    /// assert_eq!(context.phase, "feed");
    /// assert_eq!(context.order, 1);
    /// assert!(context.correlation_id.is_none());
    /// assert!(context.metadata.is_empty());
    /// ```
    pub fn new(phase: &str) -> Self {
        let order = get_phase_order(phase).unwrap_or(0);
        
        Self {
            phase: phase.to_string(),
            order,
            correlation_id: None,
            started_at: Instant::now(),
            metadata: HashMap::new(),
        }
    }
    
    /// Set correlation ID for distributed tracing
    ///
    /// Associates a correlation ID with this phase context to enable
    /// distributed tracing across service boundaries.
    ///
    /// # Arguments
    ///
    /// * `correlation_id` - Unique identifier for tracing this execution
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::{PhaseContext, FEED};
    ///
    /// let context = PhaseContext::new(FEED)
    ///     .with_correlation_id("trace-123");
    /// assert_eq!(context.correlation_id, Some("trace-123".to_string()));
    /// ```
    pub fn with_correlation_id(mut self, correlation_id: impl Into<String>) -> Self {
        self.correlation_id = Some(correlation_id.into());
        self
    }
    
    /// Add metadata to the context
    ///
    /// Attaches key-value metadata to the phase context for additional
    /// tracking and debugging information.
    ///
    /// # Arguments
    ///
    /// * `key` - Metadata key
    /// * `value` - Metadata value
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::{PhaseContext, FEED};
    ///
    /// let context = PhaseContext::new(FEED)
    ///     .with_metadata("source", "webhook")
    ///     .with_metadata("content_type", "application/json");
    ///     
    /// assert_eq!(context.metadata.get("source"), Some(&"webhook".to_string()));
    /// assert_eq!(context.metadata.get("content_type"), Some(&"application/json".to_string()));
    /// ```
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
    
    /// Get execution duration since context creation
    ///
    /// Returns the elapsed time since this context was created, useful
    /// for performance monitoring and debugging.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::{PhaseContext, FEED};
    /// use std::time::Duration;
    ///
    /// let context = PhaseContext::new(FEED);
    /// std::thread::sleep(Duration::from_millis(1));
    /// assert!(context.duration().as_millis() > 0);
    /// ```
    pub fn duration(&self) -> std::time::Duration {
        self.started_at.elapsed()
    }
    
    /// Check if this is the first phase in the 6F flow
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::{PhaseContext, FEED, FILTER};
    ///
    /// let feed_context = PhaseContext::new(FEED);
    /// assert!(feed_context.is_first_phase());
    ///
    /// let filter_context = PhaseContext::new(FILTER);
    /// assert!(!filter_context.is_first_phase());
    /// ```
    pub fn is_first_phase(&self) -> bool {
        self.order == 1
    }
    
    /// Check if this is the last phase in the 6F flow
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::{PhaseContext, FEEDBACK, FORWARD};
    ///
    /// let feedback_context = PhaseContext::new(FEEDBACK);
    /// assert!(feedback_context.is_last_phase());
    ///
    /// let forward_context = PhaseContext::new(FORWARD);
    /// assert!(!forward_context.is_last_phase());
    /// ```
    pub fn is_last_phase(&self) -> bool {
        self.order == 6
    }
    
    /// Get the next phase name in the 6F flow
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::{PhaseContext, FEED, FEEDBACK};
    ///
    /// let context = PhaseContext::new(FEED);
    /// assert_eq!(context.next_phase(), Some("filter"));
    ///
    /// let feedback_context = PhaseContext::new(FEEDBACK);
    /// assert_eq!(feedback_context.next_phase(), None); // Last phase
    /// ```
    pub fn next_phase(&self) -> Option<&'static str> {
        next_phase(&self.phase)
    }
    
    /// Get the previous phase name in the 6F flow
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::{PhaseContext, FILTER, FEED};
    ///
    /// let context = PhaseContext::new(FILTER);
    /// assert_eq!(context.previous_phase(), Some("feed"));
    ///
    /// let feed_context = PhaseContext::new(FEED);
    /// assert_eq!(feed_context.previous_phase(), None); // First phase
    /// ```
    pub fn previous_phase(&self) -> Option<&'static str> {
        previous_phase(&self.phase)
    }
    
    /// Create a context for the next phase with copied metadata
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::{PhaseContext, FEED};
    ///
    /// let feed_context = PhaseContext::new(FEED)
    ///     .with_correlation_id("trace-123")
    ///     .with_metadata("source", "webhook");
    ///
    /// let filter_context = feed_context.create_next_context().unwrap();
    /// assert_eq!(filter_context.phase, "filter");
    /// assert_eq!(filter_context.correlation_id, Some("trace-123".to_string()));
    /// assert_eq!(filter_context.metadata.get("source"), Some(&"webhook".to_string()));
    /// ```
    pub fn create_next_context(&self) -> Option<PhaseContext> {
        let next_phase_name = self.next_phase()?;
        
        Some(PhaseContext::new(next_phase_name)
            .with_correlation_id(self.correlation_id.as_ref()?.clone())
            .with_metadata_map(self.metadata.clone()))
    }
    
    /// Add multiple metadata entries at once
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::{PhaseContext, FEED};
    /// use std::collections::HashMap;
    ///
    /// let mut metadata = HashMap::new();
    /// metadata.insert("source".to_string(), "webhook".to_string());
    /// metadata.insert("type".to_string(), "json".to_string());
    ///
    /// let context = PhaseContext::new(FEED)
    ///     .with_metadata_map(metadata);
    ///     
    /// assert_eq!(context.metadata.get("source"), Some(&"webhook".to_string()));
    /// assert_eq!(context.metadata.get("type"), Some(&"json".to_string()));
    /// ```
    pub fn with_metadata_map(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata.extend(metadata);
        self
    }
    
    /// Get a metadata value by key
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::{PhaseContext, FEED};
    ///
    /// let context = PhaseContext::new(FEED)
    ///     .with_metadata("source", "webhook");
    ///     
    /// assert_eq!(context.get_metadata("source"), Some("webhook"));
    /// assert_eq!(context.get_metadata("missing"), None);
    /// ```
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }
    
    /// Check if metadata contains a specific key
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::{PhaseContext, FEED};
    ///
    /// let context = PhaseContext::new(FEED)
    ///     .with_metadata("source", "webhook");
    ///     
    /// assert!(context.has_metadata("source"));
    /// assert!(!context.has_metadata("missing"));
    /// ```
    pub fn has_metadata(&self, key: &str) -> bool {
        self.metadata.contains_key(key)
    }
    
    /// Update the correlation ID
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::{PhaseContext, FEED};
    ///
    /// let mut context = PhaseContext::new(FEED);
    /// context.set_correlation_id("new-trace-id");
    /// assert_eq!(context.correlation_id, Some("new-trace-id".to_string()));
    /// ```
    pub fn set_correlation_id(&mut self, correlation_id: impl Into<String>) {
        self.correlation_id = Some(correlation_id.into());
    }
    
    /// Add a single metadata entry
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::{PhaseContext, FEED};
    ///
    /// let mut context = PhaseContext::new(FEED);
    /// context.add_metadata("source", "webhook");
    /// assert_eq!(context.metadata.get("source"), Some(&"webhook".to_string()));
    /// ```
    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }
    
    /// Get execution duration in milliseconds
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::{PhaseContext, FEED};
    /// use std::time::Duration;
    ///
    /// let context = PhaseContext::new(FEED);
    /// std::thread::sleep(Duration::from_millis(5));
    /// assert!(context.duration_millis() >= 5);
    /// ```
    pub fn duration_millis(&self) -> u128 {
        self.duration().as_millis()
    }
    
    /// Reset the start time (useful for phase timing)
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::{PhaseContext, FEED};
    /// use std::time::Duration;
    ///
    /// let mut context = PhaseContext::new(FEED);
    /// std::thread::sleep(Duration::from_millis(5));
    /// 
    /// let old_duration = context.duration_millis();
    /// context.reset_start_time();
    /// let new_duration = context.duration_millis();
    /// 
    /// assert!(old_duration > new_duration);
    /// ```
    pub fn reset_start_time(&mut self) {
        self.started_at = Instant::now();
    }
}

impl std::fmt::Display for PhaseContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PhaseContext(phase={}, order={}, duration={}ms, correlation_id={:?})",
            self.phase,
            self.order,
            self.duration_millis(),
            self.correlation_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phases::lifecycle::{FEED, FILTER, FORMAT, FEEDBACK};
    
    #[test]
    fn test_phase_context_creation() {
        let context = PhaseContext::new(FEED);
        assert_eq!(context.phase, FEED);
        assert_eq!(context.order, 1);
        assert!(context.correlation_id.is_none());
        assert!(context.metadata.is_empty());
    }
    
    #[test]
    fn test_phase_context_with_correlation_id() {
        let context = PhaseContext::new(FILTER)
            .with_correlation_id("trace-123");
        assert_eq!(context.correlation_id, Some("trace-123".to_string()));
    }
    
    #[test]
    fn test_phase_context_with_metadata() {
        let context = PhaseContext::new(FORMAT)
            .with_metadata("source", "webhook")
            .with_metadata("type", "json");
        
        assert_eq!(context.metadata.get("source"), Some(&"webhook".to_string()));
        assert_eq!(context.metadata.get("type"), Some(&"json".to_string()));
    }
    
    #[test]
    fn test_phase_context_duration() {
        let context = PhaseContext::new(FEED);
        std::thread::sleep(std::time::Duration::from_millis(1));
        let duration = context.duration();
        assert!(duration.as_millis() > 0);
    }
    
    #[test]
    fn test_phase_context_position_checks() {
        let feed_context = PhaseContext::new(FEED);
        assert!(feed_context.is_first_phase());
        assert!(!feed_context.is_last_phase());
        
        let feedback_context = PhaseContext::new(FEEDBACK);
        assert!(!feedback_context.is_first_phase());
        assert!(feedback_context.is_last_phase());
        
        let filter_context = PhaseContext::new(FILTER);
        assert!(!filter_context.is_first_phase());
        assert!(!filter_context.is_last_phase());
    }
    
    #[test]
    fn test_phase_context_navigation() {
        let context = PhaseContext::new(FORMAT);
        assert_eq!(context.next_phase(), Some("function"));
        assert_eq!(context.previous_phase(), Some("filter"));
        
        let feed_context = PhaseContext::new(FEED);
        assert_eq!(feed_context.next_phase(), Some("filter"));
        assert_eq!(feed_context.previous_phase(), None);
        
        let feedback_context = PhaseContext::new(FEEDBACK);
        assert_eq!(feedback_context.next_phase(), None);
        assert_eq!(feedback_context.previous_phase(), Some("forward"));
    }
    
    #[test]
    fn test_create_next_context() {
        let feed_context = PhaseContext::new(FEED)
            .with_correlation_id("trace-123")
            .with_metadata("source", "webhook");
        
        let filter_context = feed_context.create_next_context().unwrap();
        assert_eq!(filter_context.phase, "filter");
        assert_eq!(filter_context.correlation_id, Some("trace-123".to_string()));
        assert_eq!(filter_context.metadata.get("source"), Some(&"webhook".to_string()));
        
        let feedback_context = PhaseContext::new(FEEDBACK);
        assert!(feedback_context.create_next_context().is_none());
    }
    
    #[test]
    fn test_metadata_operations() {
        let mut context = PhaseContext::new(FEED);
        
        // Test add_metadata
        context.add_metadata("key1", "value1");
        assert!(context.has_metadata("key1"));
        assert_eq!(context.get_metadata("key1"), Some("value1"));
        
        // Test has_metadata
        assert!(!context.has_metadata("missing"));
        
        // Test get_metadata
        assert_eq!(context.get_metadata("missing"), None);
        
        // Test with_metadata_map
        let mut metadata = HashMap::new();
        metadata.insert("key2".to_string(), "value2".to_string());
        metadata.insert("key3".to_string(), "value3".to_string());
        
        let context2 = PhaseContext::new(FILTER)
            .with_metadata_map(metadata);
        
        assert_eq!(context2.get_metadata("key2"), Some("value2"));
        assert_eq!(context2.get_metadata("key3"), Some("value3"));
    }
    
    #[test]
    fn test_correlation_id_operations() {
        let mut context = PhaseContext::new(FEED);
        
        // Test set_correlation_id
        context.set_correlation_id("new-id");
        assert_eq!(context.correlation_id, Some("new-id".to_string()));
    }
    
    #[test]
    fn test_timing_operations() {
        let mut context = PhaseContext::new(FEED);
        std::thread::sleep(std::time::Duration::from_millis(2));
        
        let duration1 = context.duration_millis();
        assert!(duration1 >= 2);
        
        // Reset timing
        context.reset_start_time();
        std::thread::sleep(std::time::Duration::from_millis(1));
        
        let duration2 = context.duration_millis();
        assert!(duration2 < duration1);
        assert!(duration2 >= 1);
    }
    
    #[test]
    fn test_display_implementation() {
        let context = PhaseContext::new(FEED)
            .with_correlation_id("test-123");
        
        let display_string = format!("{}", context);
        assert!(display_string.contains("phase=feed"));
        assert!(display_string.contains("order=1"));
        assert!(display_string.contains("correlation_id=Some(\"test-123\")"));
    }
}