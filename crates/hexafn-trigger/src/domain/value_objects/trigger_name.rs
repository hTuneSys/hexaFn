// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Trigger Name Value Object
//! 
//! Represents a validated trigger name with business rules for naming conventions,
//! length restrictions, and character validation.

use hexafn_core::types::ValidationError;
use serde::{Deserialize, Serialize};
use regex::Regex;
use std::fmt;

/// Valid trigger name with business rule validation
///
/// # Business Rules
///
/// - Must be 1-64 characters long
/// - Must start with a letter or underscore
/// - Can contain letters, numbers, hyphens, and underscores
/// - Cannot contain spaces or special characters
/// - Cannot be a reserved name
/// - Must be unique within the trigger namespace
///
/// # Examples
///
/// ```rust
/// use hexafn_trigger::domain::value_objects::TriggerName;
///
/// // Valid names
/// let name = TriggerName::new("user_signup_trigger")?;
/// let name = TriggerName::new("daily-backup")?;
/// let name = TriggerName::new("HeartbeatCheck")?;
/// let name = TriggerName::new("_internal_trigger")?;
///
/// // Invalid names will return ValidationError
/// assert!(TriggerName::new("").is_err());           // Empty
/// assert!(TriggerName::new("123invalid").is_err()); // Starts with number
/// assert!(TriggerName::new("invalid@name").is_err()); // Special characters
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TriggerName(String);

impl TriggerName {
    /// Minimum trigger name length
    pub const MIN_LENGTH: usize = 1;
    
    /// Maximum trigger name length
    pub const MAX_LENGTH: usize = 64;
    
    /// Create a new trigger name with validation
    ///
    /// # Arguments
    ///
    /// * `name` - The trigger name string
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` if:
    /// - Name is empty or too long
    /// - Name contains invalid characters
    /// - Name starts with invalid character
    /// - Name is a reserved keyword
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerName;
    ///
    /// let name = TriggerName::new("user_registration_trigger")?;
    /// assert_eq!(name.value(), "user_registration_trigger");
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Result<Self, ValidationError> {
        let name_str = name.into();
        Self::validate_name(&name_str)?;
        Ok(Self(name_str))
    }
    
    /// Validate trigger name according to business rules
    fn validate_name(name: &str) -> Result<(), ValidationError> {
        // Check length constraints
        if name.is_empty() {
            return Err(ValidationError::EmptyValue {
                field: "trigger_name".to_string(),
            });
        }
        
        if name.len() < Self::MIN_LENGTH {
            return Err(ValidationError::TooShort {
                field: "trigger_name".to_string(),
                length: name.len(),
                min: Self::MIN_LENGTH,
            });
        }
        
        if name.len() > Self::MAX_LENGTH {
            return Err(ValidationError::TooLong {
                field: "trigger_name".to_string(),
                length: name.len(),
                max: Self::MAX_LENGTH,
            });
        }
        
        // Check character constraints
        Self::validate_characters(name)?;
        Self::validate_first_character(name)?;
        Self::validate_not_reserved(name)?;
        
        Ok(())
    }
    
    /// Validate that name contains only allowed characters
    fn validate_characters(name: &str) -> Result<(), ValidationError> {
        // Allowed: letters, numbers, hyphens, underscores
        let valid_chars_regex = Regex::new(r"^[a-zA-Z0-9_-]+$").map_err(|_| {
            ValidationError::InvalidValue {
                field: "trigger_name".to_string(),
                value: name.to_string(),
                reason: "Regex compilation failed".to_string(),
            }
        })?;
        
        if !valid_chars_regex.is_match(name) {
            return Err(ValidationError::InvalidValue {
                field: "trigger_name".to_string(),
                value: name.to_string(),
                reason: "Name can only contain letters, numbers, hyphens, and underscores".to_string(),
            });
        }
        
        Ok(())
    }
    
    /// Validate that name starts with a letter or underscore
    fn validate_first_character(name: &str) -> Result<(), ValidationError> {
        let first_char = name.chars().next().unwrap(); // Safe because we checked empty above
        
        if !first_char.is_alphabetic() && first_char != '_' {
            return Err(ValidationError::InvalidValue {
                field: "trigger_name".to_string(),
                value: name.to_string(),
                reason: "Name must start with a letter or underscore".to_string(),
            });
        }
        
        Ok(())
    }
    
    /// Validate that name is not a reserved keyword
    fn validate_not_reserved(name: &str) -> Result<(), ValidationError> {
        let reserved_names = [
            // System reserved names
            "system", "admin", "root", "default", "config",
            // hexaFn reserved names
            "hexafn", "hexa", "trigger", "condition", "pipeline",
            // 6F Lifecycle reserved names
            "feed", "filter", "format", "function", "forward", "feedback",
            // Module reserved names
            "store", "cast", "run", "watch", "bridge", "core",
            // Common reserved words
            "null", "undefined", "true", "false", "if", "else",
            "for", "while", "return", "function", "class", "struct",
            // Temporal reserved names
            "timer", "schedule", "cron", "interval", "delay",
            // Event reserved names
            "event", "emit", "publish", "subscribe", "topic",
        ];
        
        let lowercase_name = name.to_lowercase();
        if reserved_names.contains(&lowercase_name.as_str()) {
            return Err(ValidationError::InvalidValue {
                field: "trigger_name".to_string(),
                value: name.to_string(),
                reason: format!("'{}' is a reserved name and cannot be used", name),
            });
        }
        
        Ok(())
    }
    
    /// Get the underlying trigger name string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerName;
    ///
    /// let name = TriggerName::new("user_trigger")?;
    /// assert_eq!(name.value(), "user_trigger");
    /// ```
    pub fn value(&self) -> &str {
        &self.0
    }
    
    /// Get the trigger name as an owned string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerName;
    ///
    /// let name = TriggerName::new("user_trigger")?;
    /// let owned_name: String = name.into_string();
    /// assert_eq!(owned_name, "user_trigger");
    /// ```
    pub fn into_string(self) -> String {
        self.0
    }
    
    /// Check if name matches a pattern (supports wildcards)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerName;
    ///
    /// let name = TriggerName::new("user_signup_trigger")?;
    /// assert!(name.matches_pattern("user_*"));
    /// assert!(name.matches_pattern("*_trigger"));
    /// assert!(!name.matches_pattern("admin_*"));
    /// ```
    pub fn matches_pattern(&self, pattern: &str) -> bool {
        if pattern.contains('*') {
            let pattern_parts: Vec<&str> = pattern.split('*').collect();
            match pattern_parts.len() {
                1 => self.0 == pattern_parts[0], // No wildcard
                2 => {
                    let prefix = pattern_parts[0];
                    let suffix = pattern_parts[1];
                    if prefix.is_empty() {
                        self.0.ends_with(suffix)
                    } else if suffix.is_empty() {
                        self.0.starts_with(prefix)
                    } else {
                        self.0.starts_with(prefix) && self.0.ends_with(suffix)
                    }
                }
                _ => false, // Multiple wildcards not supported
            }
        } else {
            self.0 == pattern
        }
    }
    
    /// Generate a normalized version of the name (lowercase, no special chars)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerName;
    ///
    /// let name = TriggerName::new("User_Signup-Trigger")?;
    /// assert_eq!(name.normalized(), "user_signup_trigger");
    /// ```
    pub fn normalized(&self) -> String {
        self.0
            .to_lowercase()
            .chars()
            .map(|c| if c == '-' { '_' } else { c })
            .collect()
    }
    
    /// Check if name is snake_case
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerName;
    ///
    /// let snake_case = TriggerName::new("user_signup_trigger")?;
    /// assert!(snake_case.is_snake_case());
    ///
    /// let camel_case = TriggerName::new("UserSignupTrigger")?;
    /// assert!(!camel_case.is_snake_case());
    /// ```
    pub fn is_snake_case(&self) -> bool {
        !self.0.chars().any(|c| c.is_uppercase() || c == '-')
    }
    
    /// Check if name is kebab-case
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerName;
    ///
    /// let kebab_case = TriggerName::new("user-signup-trigger")?;
    /// assert!(kebab_case.is_kebab_case());
    ///
    /// let snake_case = TriggerName::new("user_signup_trigger")?;
    /// assert!(!snake_case.is_kebab_case());
    /// ```
    pub fn is_kebab_case(&self) -> bool {
        !self.0.chars().any(|c| c.is_uppercase() || c == '_')
    }
    
    /// Convert to snake_case
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerName;
    ///
    /// let name = TriggerName::new("user-signup-trigger")?;
    /// let snake_case = name.to_snake_case()?;
    /// assert_eq!(snake_case.value(), "user_signup_trigger");
    /// ```
    pub fn to_snake_case(&self) -> Result<TriggerName, ValidationError> {
        let snake_case = self.0
            .chars()
            .map(|c| if c == '-' { '_' } else { c.to_lowercase().next().unwrap_or(c) })
            .collect::<String>();
        
        TriggerName::new(snake_case)
    }
    
    /// Convert to kebab-case
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerName;
    ///
    /// let name = TriggerName::new("user_signup_trigger")?;
    /// let kebab_case = name.to_kebab_case()?;
    /// assert_eq!(kebab_case.value(), "user-signup-trigger");
    /// ```
    pub fn to_kebab_case(&self) -> Result<TriggerName, ValidationError> {
        let kebab_case = self.0
            .chars()
            .map(|c| if c == '_' { '-' } else { c.to_lowercase().next().unwrap_or(c) })
            .collect::<String>();
        
        TriggerName::new(kebab_case)
    }
    
    /// Validate the trigger name (same as constructor validation)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerName;
    ///
    /// let name = TriggerName::new("valid_trigger")?;
    /// assert!(name.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), ValidationError> {
        Self::validate_name(&self.0)
    }
}

impl fmt::Display for TriggerName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for TriggerName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<TriggerName> for String {
    fn from(name: TriggerName) -> String {
        name.0
    }
}

impl PartialEq<str> for TriggerName {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for TriggerName {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for TriggerName {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trigger_name_creation_valid() {
        let valid_names = [
            "user_trigger",
            "UserTrigger",
            "user-trigger",
            "_internal_trigger",
            "trigger123",
            "a",
            "very_long_trigger_name_that_is_still_valid_within_64_chars",
        ];
        
        for name in &valid_names {
            let result = TriggerName::new(*name);
            assert!(result.is_ok(), "Name '{}' should be valid", name);
            assert_eq!(result.unwrap().value(), *name);
        }
    }
    
    #[test]
    fn test_trigger_name_creation_invalid() {
        let invalid_names = [
            "",                              // Empty
            "123invalid",                    // Starts with number
            "invalid@name",                  // Special character
            "invalid name",                  // Space
            "invalid.name",                  // Dot
            "a_very_long_trigger_name_that_exceeds_the_maximum_allowed_length_of_64_characters", // Too long
        ];
        
        for name in &invalid_names {
            let result = TriggerName::new(*name);
            assert!(result.is_err(), "Name '{}' should be invalid", name);
        }
    }
    
    #[test]
    fn test_trigger_name_reserved_names() {
        let reserved_names = [
            "system", "admin", "hexafn", "feed", "filter", "trigger",
            "null", "true", "function", "timer", "event",
        ];
        
        for name in &reserved_names {
            let result = TriggerName::new(*name);
            assert!(result.is_err(), "Reserved name '{}' should be invalid", name);
        }
        
        // Case insensitive check
        let result = TriggerName::new("SYSTEM");
        assert!(result.is_err(), "Reserved name 'SYSTEM' should be invalid");
    }
    
    #[test]
    fn test_trigger_name_length_validation() {
        // Test minimum length
        let min_name = TriggerName::new("a");
        assert!(min_name.is_ok());
        
        // Test maximum length (64 characters)
        let max_name = "a".repeat(64);
        let result = TriggerName::new(&max_name);
        assert!(result.is_ok());
        
        // Test too long (65 characters)
        let too_long = "a".repeat(65);
        let result = TriggerName::new(&too_long);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_trigger_name_pattern_matching() {
        let name = TriggerName::new("user_signup_trigger").unwrap();
        
        assert!(name.matches_pattern("user_*"));
        assert!(name.matches_pattern("*_trigger"));
        assert!(name.matches_pattern("user_signup_trigger"));
        assert!(!name.matches_pattern("admin_*"));
        assert!(!name.matches_pattern("*_event"));
    }
    
    #[test]
    fn test_trigger_name_normalization() {
        let name = TriggerName::new("User_Signup-Trigger").unwrap();
        assert_eq!(name.normalized(), "user_signup_trigger");
        
        let name2 = TriggerName::new("SIMPLE_NAME").unwrap();
        assert_eq!(name2.normalized(), "simple_name");
    }
    
    #[test]
    fn test_trigger_name_case_detection() {
        let snake_case = TriggerName::new("user_signup_trigger").unwrap();
        assert!(snake_case.is_snake_case());
        assert!(!snake_case.is_kebab_case());
        
        let kebab_case = TriggerName::new("user-signup-trigger").unwrap();
        assert!(!kebab_case.is_snake_case());
        assert!(kebab_case.is_kebab_case());
        
        let camel_case = TriggerName::new("UserSignupTrigger").unwrap();
        assert!(!camel_case.is_snake_case());
        assert!(!camel_case.is_kebab_case());
    }
    
    #[test]
    fn test_trigger_name_case_conversion() {
        let kebab_name = TriggerName::new("user-signup-trigger").unwrap();
        let snake_name = kebab_name.to_snake_case().unwrap();
        assert_eq!(snake_name.value(), "user_signup_trigger");
        
        let snake_name = TriggerName::new("user_signup_trigger").unwrap();
        let kebab_name = snake_name.to_kebab_case().unwrap();
        assert_eq!(kebab_name.value(), "user-signup-trigger");
    }
    
    #[test]
    fn test_trigger_name_display() {
        let name = TriggerName::new("test_trigger").unwrap();
        assert_eq!(format!("{}", name), "test_trigger");
    }
    
    #[test]
    fn test_trigger_name_equality() {
        let name = TriggerName::new("test_trigger").unwrap();
        
        assert_eq!(name, "test_trigger");
        assert_eq!(name, &"test_trigger");
        assert_eq!(name, "test_trigger".to_string());
        
        assert_ne!(name, "different_trigger");
    }
    
    #[test]
    fn test_trigger_name_into_string() {
        let name = TriggerName::new("test_trigger").unwrap();
        let string: String = name.into_string();
        assert_eq!(string, "test_trigger");
    }
    
    #[test]
    fn test_trigger_name_as_ref() {
        let name = TriggerName::new("test_trigger").unwrap();
        let str_ref: &str = name.as_ref();
        assert_eq!(str_ref, "test_trigger");
    }
    
    #[test]
    fn test_trigger_name_validation() {
        let name = TriggerName::new("valid_trigger").unwrap();
        assert!(name.validate().is_ok());
    }
    
    #[test]
    fn test_trigger_name_constants() {
        assert_eq!(TriggerName::MIN_LENGTH, 1);
        assert_eq!(TriggerName::MAX_LENGTH, 64);
    }
}