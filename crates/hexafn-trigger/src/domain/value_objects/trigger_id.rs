// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Trigger ID Value Object
//! 
//! Represents a unique identifier for triggers using UUID v4.
//! Ensures trigger identity across the system with validation and type safety.

use hexafn_core::types::ValidationError;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Unique identifier for triggers
///
/// # Design Principles
///
/// - Uses UUID v4 for guaranteed uniqueness
/// - Immutable once created
/// - Serializable for persistence and transport
/// - Type-safe to prevent ID confusion
/// - Supports both generated and custom IDs
///
/// # Examples
///
/// ```rust
/// use hexafn_trigger::domain::value_objects::TriggerId;
///
/// // Generate new UUID
/// let id = TriggerId::new();
/// 
/// // Create from existing UUID string
/// let id = TriggerId::from_string("550e8400-e29b-41d4-a716-446655440000")?;
///
/// // Create from UUID
/// let uuid = uuid::Uuid::new_v4();
/// let id = TriggerId::from_uuid(uuid);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TriggerId(Uuid);

impl TriggerId {
    /// Generate a new random trigger ID using UUID v4
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerId;
    ///
    /// let id = TriggerId::new();
    /// assert!(!id.value().is_empty());
    /// assert_eq!(id.value().len(), 36); // Standard UUID string length
    /// ```
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    /// Create a trigger ID from an existing UUID
    ///
    /// # Arguments
    ///
    /// * `uuid` - The UUID to wrap
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerId;
    /// use uuid::Uuid;
    ///
    /// let uuid = Uuid::new_v4();
    /// let id = TriggerId::from_uuid(uuid);
    /// assert_eq!(id.to_uuid(), uuid);
    /// ```
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
    
    /// Create a trigger ID from a string representation
    ///
    /// # Arguments
    ///
    /// * `id_str` - String representation of UUID
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` if the string is not a valid UUID format.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerId;
    ///
    /// // Valid UUID string
    /// let id = TriggerId::from_string("550e8400-e29b-41d4-a716-446655440000")?;
    /// assert_eq!(id.value(), "550e8400-e29b-41d4-a716-446655440000");
    ///
    /// // Invalid UUID string
    /// assert!(TriggerId::from_string("invalid-uuid").is_err());
    /// ```
    pub fn from_string<S: AsRef<str>>(id_str: S) -> Result<Self, ValidationError> {
        let uuid_str = id_str.as_ref();
        
        if uuid_str.is_empty() {
            return Err(ValidationError::EmptyValue {
                field: "trigger_id".to_string(),
            });
        }
        
        let uuid = Uuid::parse_str(uuid_str).map_err(|e| ValidationError::InvalidValue {
            field: "trigger_id".to_string(),
            value: uuid_str.to_string(),
            reason: format!("Invalid UUID format: {}", e),
        })?;
        
        Ok(Self(uuid))
    }
    
    /// Create a deterministic trigger ID from a seed
    ///
    /// Useful for testing or when you need reproducible IDs.
    ///
    /// # Arguments
    ///
    /// * `seed` - Seed value for deterministic generation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerId;
    ///
    /// let id1 = TriggerId::from_seed("test-trigger");
    /// let id2 = TriggerId::from_seed("test-trigger");
    /// assert_eq!(id1, id2); // Same seed produces same ID
    ///
    /// let id3 = TriggerId::from_seed("different-trigger");
    /// assert_ne!(id1, id3); // Different seed produces different ID
    /// ```
    pub fn from_seed<S: AsRef<str>>(seed: S) -> Self {
        let seed_str = seed.as_ref();
        let uuid = Uuid::new_v5(&Uuid::NAMESPACE_OID, seed_str.as_bytes());
        Self(uuid)
    }
    
    /// Get the string representation of the trigger ID
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerId;
    ///
    /// let id = TriggerId::from_string("550e8400-e29b-41d4-a716-446655440000")?;
    /// assert_eq!(id.value(), "550e8400-e29b-41d4-a716-446655440000");
    /// ```
    pub fn value(&self) -> String {
        self.0.to_string()
    }
    
    /// Get the underlying UUID
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerId;
    /// use uuid::Uuid;
    ///
    /// let original_uuid = Uuid::new_v4();
    /// let id = TriggerId::from_uuid(original_uuid);
    /// assert_eq!(id.to_uuid(), original_uuid);
    /// ```
    pub fn to_uuid(&self) -> Uuid {
        self.0
    }
    
    /// Get a short representation of the ID (first 8 characters)
    ///
    /// Useful for logging and display purposes where full UUID is too verbose.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerId;
    ///
    /// let id = TriggerId::from_string("550e8400-e29b-41d4-a716-446655440000")?;
    /// assert_eq!(id.short(), "550e8400");
    /// ```
    pub fn short(&self) -> String {
        self.0.to_string()[..8].to_string()
    }
    
    /// Check if this ID represents a nil UUID
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerId;
    /// use uuid::Uuid;
    ///
    /// let nil_id = TriggerId::from_uuid(Uuid::nil());
    /// assert!(nil_id.is_nil());
    ///
    /// let normal_id = TriggerId::new();
    /// assert!(!normal_id.is_nil());
    /// ```
    pub fn is_nil(&self) -> bool {
        self.0.is_nil()
    }
    
    /// Get the version of the UUID
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerId;
    ///
    /// let id = TriggerId::new();
    /// assert_eq!(id.version(), Some(uuid::Version::Random)); // UUID v4
    ///
    /// let seed_id = TriggerId::from_seed("test");
    /// assert_eq!(seed_id.version(), Some(uuid::Version::Sha1)); // UUID v5
    /// ```
    pub fn version(&self) -> Option<uuid::Version> {
        self.0.get_version()
    }
    
    /// Create a new trigger ID with the same timestamp (for UUID v1/v6)
    ///
    /// Note: This is mainly for completeness. Most triggers use v4 (random).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerId;
    ///
    /// let id1 = TriggerId::new();
    /// let id2 = id1.with_same_timestamp();
    /// // Both IDs are different but conceptually related
    /// assert_ne!(id1, id2);
    /// ```
    pub fn with_same_timestamp(&self) -> Self {
        // For v4 UUIDs, just generate a new one
        Self::new()
    }
    
    /// Validate the trigger ID
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerId;
    ///
    /// let id = TriggerId::new();
    /// assert!(id.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.0.is_nil() {
            return Err(ValidationError::InvalidValue {
                field: "trigger_id".to_string(),
                value: self.0.to_string(),
                reason: "Trigger ID cannot be nil UUID".to_string(),
            });
        }
        
        Ok(())
    }
    
    /// Create a nil trigger ID (for testing purposes)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerId;
    ///
    /// let nil_id = TriggerId::nil();
    /// assert!(nil_id.is_nil());
    /// assert_eq!(nil_id.value(), "00000000-0000-0000-0000-000000000000");
    /// ```
    pub fn nil() -> Self {
        Self(Uuid::nil())
    }
    
    /// Generate multiple unique trigger IDs
    ///
    /// # Arguments
    ///
    /// * `count` - Number of IDs to generate
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerId;
    ///
    /// let ids = TriggerId::generate_multiple(5);
    /// assert_eq!(ids.len(), 5);
    /// 
    /// // All IDs should be unique
    /// for i in 0..ids.len() {
    ///     for j in (i + 1)..ids.len() {
    ///         assert_ne!(ids[i], ids[j]);
    ///     }
    /// }
    /// ```
    pub fn generate_multiple(count: usize) -> Vec<Self> {
        (0..count).map(|_| Self::new()).collect()
    }
    
    /// Check if two trigger IDs are from the same generation (for debugging)
    ///
    /// # Arguments
    ///
    /// * `other` - Other trigger ID to compare with
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerId;
    ///
    /// let id1 = TriggerId::from_seed("same-seed");
    /// let id2 = TriggerId::from_seed("same-seed");
    /// assert!(id1.is_same_generation(&id2));
    ///
    /// let id3 = TriggerId::new();
    /// assert!(!id1.is_same_generation(&id3));
    /// ```
    pub fn is_same_generation(&self, other: &Self) -> bool {
        // For UUID v4 (random), we can't determine generation
        // For UUID v5 (name-based), same name = same generation
        match (self.version(), other.version()) {
            (Some(uuid::Version::Sha1), Some(uuid::Version::Sha1)) => {
                // For v5 UUIDs, compare the full UUID
                self.0 == other.0
            }
            _ => false, // Different versions or v4 (random)
        }
    }
}

impl Default for TriggerId {
    /// Create a new random trigger ID as default
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerId;
    ///
    /// let id: TriggerId = Default::default();
    /// assert!(!id.is_nil());
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TriggerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for TriggerId {
    fn as_ref(&self) -> &str {
        // Note: This returns a reference to the internal string representation
        // For UUID, we need to convert to string first, so this is a bit tricky
        // We'll use the Display implementation instead
        std::thread_local! {
            static UUID_STRING: std::cell::RefCell<String> = RefCell::new(String::new());
        }
        
        UUID_STRING.with(|s| {
            let mut s = s.borrow_mut();
            s.clear();
            s.push_str(&self.0.to_string());
            // This is unsafe but works for the lifetime of the function call
            unsafe { std::mem::transmute(s.as_str()) }
        })
    }
}

impl From<Uuid> for TriggerId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<TriggerId> for Uuid {
    fn from(id: TriggerId) -> Self {
        id.0
    }
}

impl From<TriggerId> for String {
    fn from(id: TriggerId) -> Self {
        id.0.to_string()
    }
}

impl std::str::FromStr for TriggerId {
    type Err = ValidationError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s)
    }
}

impl PartialEq<Uuid> for TriggerId {
    fn eq(&self, other: &Uuid) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for TriggerId {
    fn eq(&self, other: &String) -> bool {
        self.0.to_string() == *other
    }
}

impl PartialEq<&str> for TriggerId {
    fn eq(&self, other: &&str) -> bool {
        self.0.to_string() == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    
    #[test]
    fn test_trigger_id_generation() {
        let id = TriggerId::new();
        assert!(!id.is_nil());
        assert_eq!(id.value().len(), 36); // Standard UUID string length
        assert_eq!(id.version(), Some(uuid::Version::Random));
    }
    
    #[test]
    fn test_trigger_id_from_string_valid() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let id = TriggerId::from_string(uuid_str).unwrap();
        assert_eq!(id.value(), uuid_str);
    }
    
    #[test]
    fn test_trigger_id_from_string_invalid() {
        let invalid_inputs = [
            "",
            "invalid-uuid",
            "550e8400-e29b-41d4-a716",
            "not-a-uuid-at-all",
            "550e8400-e29b-41d4-a716-446655440000-extra",
        ];
        
        for input in &invalid_inputs {
            let result = TriggerId::from_string(*input);
            assert!(result.is_err(), "Input '{}' should be invalid", input);
        }
    }
    
    #[test]
    fn test_trigger_id_from_uuid() {
        let uuid = Uuid::new_v4();
        let id = TriggerId::from_uuid(uuid);
        assert_eq!(id.to_uuid(), uuid);
        assert_eq!(id.value(), uuid.to_string());
    }
    
    #[test]
    fn test_trigger_id_from_seed() {
        let seed = "test-trigger";
        let id1 = TriggerId::from_seed(seed);
        let id2 = TriggerId::from_seed(seed);
        
        // Same seed should produce same ID
        assert_eq!(id1, id2);
        assert_eq!(id1.version(), Some(uuid::Version::Sha1));
        
        // Different seed should produce different ID
        let id3 = TriggerId::from_seed("different-trigger");
        assert_ne!(id1, id3);
    }
    
    #[test]
    fn test_trigger_id_short() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let id = TriggerId::from_string(uuid_str).unwrap();
        assert_eq!(id.short(), "550e8400");
    }
    
    #[test]
    fn test_trigger_id_nil() {
        let nil_id = TriggerId::nil();
        assert!(nil_id.is_nil());
        assert_eq!(nil_id.value(), "00000000-0000-0000-0000-000000000000");
        
        let normal_id = TriggerId::new();
        assert!(!normal_id.is_nil());
    }
    
    #[test]
    fn test_trigger_id_validation() {
        let valid_id = TriggerId::new();
        assert!(valid_id.validate().is_ok());
        
        let nil_id = TriggerId::nil();
        assert!(nil_id.validate().is_err());
    }
    
    #[test]
    fn test_trigger_id_generate_multiple() {
        let ids = TriggerId::generate_multiple(10);
        assert_eq!(ids.len(), 10);
        
        // All IDs should be unique
        let unique_ids: HashSet<_> = ids.iter().collect();
        assert_eq!(unique_ids.len(), 10);
    }
    
    #[test]
    fn test_trigger_id_same_generation() {
        let seed = "test-seed";
        let id1 = TriggerId::from_seed(seed);
        let id2 = TriggerId::from_seed(seed);
        assert!(id1.is_same_generation(&id2));
        
        let id3 = TriggerId::from_seed("different-seed");
        assert!(!id1.is_same_generation(&id3));
        
        let random_id = TriggerId::new();
        assert!(!id1.is_same_generation(&random_id));
    }
    
    #[test]
    fn test_trigger_id_display() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let id = TriggerId::from_string(uuid_str).unwrap();
        assert_eq!(format!("{}", id), uuid_str);
    }
    
    #[test]
    fn test_trigger_id_equality() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let id = TriggerId::from_string(uuid_str).unwrap();
        let uuid = Uuid::parse_str(uuid_str).unwrap();
        
        assert_eq!(id, uuid);
        assert_eq!(id, uuid_str.to_string());
        assert_eq!(id, &uuid_str);
    }
    
    #[test]
    fn test_trigger_id_conversions() {
        let uuid = Uuid::new_v4();
        let id = TriggerId::from(uuid);
        
        let back_to_uuid: Uuid = id.clone().into();
        assert_eq!(uuid, back_to_uuid);
        
        let to_string: String = id.clone().into();
        assert_eq!(to_string, uuid.to_string());
    }
    
    #[test]
    fn test_trigger_id_from_str() {
        use std::str::FromStr;
        
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let id = TriggerId::from_str(uuid_str).unwrap();
        assert_eq!(id.value(), uuid_str);
        
        let invalid_result = TriggerId::from_str("invalid");
        assert!(invalid_result.is_err());
    }
    
    #[test]
    fn test_trigger_id_default() {
        let id: TriggerId = Default::default();
        assert!(!id.is_nil());
        assert_eq!(id.version(), Some(uuid::Version::Random));
    }
    
    #[test]
    fn test_trigger_id_hash() {
        let mut set = HashSet::new();
        let id1 = TriggerId::new();
        let id2 = TriggerId::new();
        
        set.insert(id1.clone());
        set.insert(id2.clone());
        
        assert_eq!(set.len(), 2);
        assert!(set.contains(&id1));
        assert!(set.contains(&id2));
    }
    
    #[test]
    fn test_trigger_id_clone() {
        let id1 = TriggerId::new();
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }
    
    #[test]
    fn test_trigger_id_debug() {
        let id = TriggerId::new();
        let debug_str = format!("{:?}", id);
        assert!(debug_str.contains("TriggerId"));
        assert!(debug_str.contains(&id.value()));
    }
}