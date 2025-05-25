// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Trigger Condition Value Object
//! 
//! Defines the conditions under which a trigger will execute.
//! Supports timer-based, event-based, and complex composite conditions.

use hexafn_core::types::ValidationError;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use regex::Regex;

/// Logical operators for combining conditions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogicalOperator {
    /// Both conditions must be true
    And,
    /// Either condition must be true
    Or,
    /// Condition must be false
    Not,
}

/// Timer expression for time-based triggers
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimerExpression {
    /// Duration string (e.g., "5s", "10m", "1h")
    duration: String,
    /// Parsed duration for internal use
    #[serde(skip)]
    parsed_duration: Option<Duration>,
}

impl TimerExpression {
    /// Create a new timer expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TimerExpression;
    ///
    /// let timer = TimerExpression::new("5s")?;
    /// let timer = TimerExpression::new("10m")?;
    /// let timer = TimerExpression::new("1h")?;
    /// ```
    pub fn new<S: Into<String>>(duration: S) -> Result<Self, ValidationError> {
        let duration_str = duration.into();
        let parsed = Self::parse_duration(&duration_str)?;
        
        Ok(Self {
            duration: duration_str,
            parsed_duration: Some(parsed),
        })
    }
    
    /// Parse duration string into Duration
    fn parse_duration(duration_str: &str) -> Result<Duration, ValidationError> {
        let re = Regex::new(r"^(\d+)(s|m|h|d)$").map_err(|_| ValidationError::InvalidValue {
            field: "timer_duration".to_string(),
            value: duration_str.to_string(),
            reason: "Invalid regex pattern".to_string(),
        })?;
        
        let captures = re.captures(duration_str).ok_or_else(|| ValidationError::InvalidValue {
            field: "timer_duration".to_string(),
            value: duration_str.to_string(),
            reason: "Duration must be in format: number + unit (s|m|h|d)".to_string(),
        })?;
        
        let number: u64 = captures.get(1).unwrap().as_str().parse().map_err(|_| {
            ValidationError::InvalidValue {
                field: "timer_duration".to_string(),
                value: duration_str.to_string(),
                reason: "Invalid number in duration".to_string(),
            }
        })?;
        
        let unit = captures.get(2).unwrap().as_str();
        
        let duration = match unit {
            "s" => Duration::from_secs(number),
            "m" => Duration::from_secs(number * 60),
            "h" => Duration::from_secs(number * 3600),
            "d" => Duration::from_secs(number * 86400),
            _ => return Err(ValidationError::InvalidValue {
                field: "timer_duration".to_string(),
                value: duration_str.to_string(),
                reason: "Unsupported time unit".to_string(),
            }),
        };
        
        // Validate reasonable duration limits
        if duration.as_secs() == 0 {
            return Err(ValidationError::InvalidValue {
                field: "timer_duration".to_string(),
                value: duration_str.to_string(),
                reason: "Duration must be greater than 0".to_string(),
            });
        }
        
        if duration.as_secs() > 86400 * 30 { // 30 days max
            return Err(ValidationError::InvalidValue {
                field: "timer_duration".to_string(),
                value: duration_str.to_string(),
                reason: "Duration cannot exceed 30 days".to_string(),
            });
        }
        
        Ok(duration)
    }
    
    /// Get the duration string
    pub fn duration_string(&self) -> &str {
        &self.duration
    }
    
    /// Get the parsed duration
    pub fn duration(&self) -> Result<Duration, ValidationError> {
        if let Some(duration) = self.parsed_duration {
            Ok(duration)
        } else {
            Self::parse_duration(&self.duration)
        }
    }
    
    /// Validate the timer expression
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.duration()?; // This will validate and parse
        Ok(())
    }
}

/// Event pattern for event-based triggers
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventPattern {
    /// Event type pattern (e.g., "user.created", "order.*")
    pattern: String,
    /// Whether to use regex matching
    use_regex: bool,
    /// Compiled regex for pattern matching
    #[serde(skip)]
    compiled_regex: Option<Regex>,
}

impl EventPattern {
    /// Create a new event pattern
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::EventPattern;
    ///
    /// // Simple string matching
    /// let pattern = EventPattern::new("user.created")?;
    ///
    /// // Regex pattern matching
    /// let pattern = EventPattern::with_regex("user\\.(created|updated)")?;
    /// ```
    pub fn new<S: Into<String>>(pattern: S) -> Result<Self, ValidationError> {
        let pattern_str = pattern.into();
        Self::validate_pattern(&pattern_str)?;
        
        Ok(Self {
            pattern: pattern_str,
            use_regex: false,
            compiled_regex: None,
        })
    }
    
    /// Create an event pattern with regex support
    pub fn with_regex<S: Into<String>>(pattern: S) -> Result<Self, ValidationError> {
        let pattern_str = pattern.into();
        Self::validate_pattern(&pattern_str)?;
        
        let regex = Regex::new(&pattern_str).map_err(|e| ValidationError::InvalidValue {
            field: "event_pattern".to_string(),
            value: pattern_str.clone(),
            reason: format!("Invalid regex pattern: {}", e),
        })?;
        
        Ok(Self {
            pattern: pattern_str,
            use_regex: true,
            compiled_regex: Some(regex),
        })
    }
    
    fn validate_pattern(pattern: &str) -> Result<(), ValidationError> {
        if pattern.is_empty() {
            return Err(ValidationError::EmptyValue {
                field: "event_pattern".to_string(),
            });
        }
        
        if pattern.len() > 255 {
            return Err(ValidationError::TooLong {
                field: "event_pattern".to_string(),
                length: pattern.len(),
                max: 255,
            });
        }
        
        Ok(())
    }
    
    /// Get the pattern string
    pub fn pattern(&self) -> &str {
        &self.pattern
    }
    
    /// Check if pattern uses regex
    pub fn uses_regex(&self) -> bool {
        self.use_regex
    }
    
    /// Test if an event matches this pattern
    pub fn matches(&self, event_type: &str) -> bool {
        if self.use_regex {
            if let Some(ref regex) = self.compiled_regex {
                regex.is_match(event_type)
            } else {
                // Fallback to simple string matching if regex compilation failed
                self.pattern == event_type
            }
        } else {
            // Simple string matching with wildcard support
            if self.pattern.contains('*') {
                let pattern_parts: Vec<&str> = self.pattern.split('*').collect();
                if pattern_parts.len() == 2 {
                    let prefix = pattern_parts[0];
                    let suffix = pattern_parts[1];
                    event_type.starts_with(prefix) && event_type.ends_with(suffix)
                } else {
                    false
                }
            } else {
                self.pattern == event_type
            }
        }
    }
    
    /// Validate the event pattern
    pub fn validate(&self) -> Result<(), ValidationError> {
        Self::validate_pattern(&self.pattern)?;
        
        if self.use_regex && self.compiled_regex.is_none() {
            // Try to compile regex to validate
            Regex::new(&self.pattern).map_err(|e| ValidationError::InvalidValue {
                field: "event_pattern".to_string(),
                value: self.pattern.clone(),
                reason: format!("Invalid regex pattern: {}", e),
            })?;
        }
        
        Ok(())
    }
}

/// Logical expression for complex conditional logic
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogicalExpression {
    /// Expression string (e.g., "x > 10 AND y < 5")
    expression: String,
}

impl LogicalExpression {
    /// Create a new logical expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::LogicalExpression;
    ///
    /// let expr = LogicalExpression::new("temperature > 25")?;
    /// let expr = LogicalExpression::new("count > 10 AND status = 'active'")?;
    /// ```
    pub fn new<S: Into<String>>(expression: S) -> Result<Self, ValidationError> {
        let expr_str = expression.into();
        Self::validate_expression(&expr_str)?;
        
        Ok(Self {
            expression: expr_str,
        })
    }
    
    fn validate_expression(expression: &str) -> Result<(), ValidationError> {
        if expression.is_empty() {
            return Err(ValidationError::EmptyValue {
                field: "logical_expression".to_string(),
            });
        }
        
        if expression.len() > 1000 {
            return Err(ValidationError::TooLong {
                field: "logical_expression".to_string(),
                length: expression.len(),
                max: 1000,
            });
        }
        
        // Basic syntax validation (can be enhanced)
        let allowed_operators = ["AND", "OR", "NOT", ">", "<", ">=", "<=", "=", "!="];
        let has_operator = allowed_operators.iter().any(|op| expression.contains(op));
        
        if !has_operator {
            return Err(ValidationError::InvalidValue {
                field: "logical_expression".to_string(),
                value: expression.to_string(),
                reason: "Expression must contain at least one logical operator".to_string(),
            });
        }
        
        Ok(())
    }
    
    /// Get the expression string
    pub fn expression(&self) -> &str {
        &self.expression
    }
    
    /// Validate the logical expression
    pub fn validate(&self) -> Result<(), ValidationError> {
        Self::validate_expression(&self.expression)
    }
}

/// Trigger condition definitions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerCondition {
    /// Always execute (for testing purposes)
    Always,
    
    /// Never execute (disabled trigger)
    Never,
    
    /// Execute on timer intervals
    Timer(TimerExpression),
    
    /// Execute when specific events occur
    Event(EventPattern),
    
    /// Execute based on logical expressions
    Expression(LogicalExpression),
    
    /// Composite condition with logical operators
    Composite {
        /// Left-hand condition
        left: Box<TriggerCondition>,
        /// Logical operator
        operator: LogicalOperator,
        /// Right-hand condition
        right: Box<TriggerCondition>,
    },
}

impl TriggerCondition {
    /// Create a timer-based condition
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerCondition;
    ///
    /// let condition = TriggerCondition::timer("5s")?;
    /// let condition = TriggerCondition::timer("10m")?;
    /// ```
    pub fn timer<S: Into<String>>(duration: S) -> Result<Self, ValidationError> {
        Ok(TriggerCondition::Timer(TimerExpression::new(duration)?))
    }
    
    /// Create an event-based condition
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerCondition;
    ///
    /// let condition = TriggerCondition::event("user.created")?;
    /// let condition = TriggerCondition::event("order.*")?;
    /// ```
    pub fn event<S: Into<String>>(pattern: S) -> Result<Self, ValidationError> {
        Ok(TriggerCondition::Event(EventPattern::new(pattern)?))
    }
    
    /// Create an event-based condition with regex
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerCondition;
    ///
    /// let condition = TriggerCondition::event_regex(r"user\.(created|updated)")?;
    /// ```
    pub fn event_regex<S: Into<String>>(pattern: S) -> Result<Self, ValidationError> {
        Ok(TriggerCondition::Event(EventPattern::with_regex(pattern)?))
    }
    
    /// Create an expression-based condition
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerCondition;
    ///
    /// let condition = TriggerCondition::expression("count > 10")?;
    /// ```
    pub fn expression<S: Into<String>>(expression: S) -> Result<Self, ValidationError> {
        Ok(TriggerCondition::Expression(LogicalExpression::new(expression)?))
    }
    
    /// Combine two conditions with AND operator
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerCondition;
    ///
    /// let timer_condition = TriggerCondition::timer("5s")?;
    /// let event_condition = TriggerCondition::event("user.created")?;
    /// let combined = timer_condition.and(event_condition);
    /// ```
    pub fn and(self, other: TriggerCondition) -> TriggerCondition {
        TriggerCondition::Composite {
            left: Box::new(self),
            operator: LogicalOperator::And,
            right: Box::new(other),
        }
    }
    
    /// Combine two conditions with OR operator
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerCondition;
    ///
    /// let timer_condition = TriggerCondition::timer("5s")?;
    /// let event_condition = TriggerCondition::event("user.created")?;
    /// let combined = timer_condition.or(event_condition);
    /// ```
    pub fn or(self, other: TriggerCondition) -> TriggerCondition {
        TriggerCondition::Composite {
            left: Box::new(self),
            operator: LogicalOperator::Or,
            right: Box::new(other),
        }
    }
    
    /// Negate the condition with NOT operator
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::TriggerCondition;
    ///
    /// let condition = TriggerCondition::event("user.deleted")?;
    /// let negated = condition.not();
    /// ```
    pub fn not(self) -> TriggerCondition {
        TriggerCondition::Composite {
            left: Box::new(self),
            operator: LogicalOperator::Not,
            right: Box::new(TriggerCondition::Always), // Placeholder for NOT
        }
    }
    
    /// Check if condition is always true
    pub fn is_always(&self) -> bool {
        matches!(self, TriggerCondition::Always)
    }
    
    /// Check if condition is never true (disabled)
    pub fn is_never(&self) -> bool {
        matches!(self, TriggerCondition::Never)
    }
    
    /// Check if condition is timer-based
    pub fn is_timer(&self) -> bool {
        matches!(self, TriggerCondition::Timer(_))
    }
    
    /// Check if condition is event-based
    pub fn is_event(&self) -> bool {
        matches!(self, TriggerCondition::Event(_))
    }
    
    /// Check if condition is expression-based
    pub fn is_expression(&self) -> bool {
        matches!(self, TriggerCondition::Expression(_))
    }
    
    /// Check if condition is composite
    pub fn is_composite(&self) -> bool {
        matches!(self, TriggerCondition::Composite { .. })
    }
    
    /// Validate the entire condition tree
    pub fn validate(&self) -> Result<(), ValidationError> {
        match self {
            TriggerCondition::Always | TriggerCondition::Never => Ok(()),
            TriggerCondition::Timer(timer) => timer.validate(),
            TriggerCondition::Event(event) => event.validate(),
            TriggerCondition::Expression(expr) => expr.validate(),
            TriggerCondition::Composite { left, right, .. } => {
                left.validate()?;
                right.validate()?;
                Ok(())
            }
        }
    }
}

impl std::fmt::Display for TriggerCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TriggerCondition::Always => write!(f, "Always"),
            TriggerCondition::Never => write!(f, "Never"),
            TriggerCondition::Timer(timer) => write!(f, "Timer({})", timer.duration_string()),
            TriggerCondition::Event(event) => write!(f, "Event({})", event.pattern()),
            TriggerCondition::Expression(expr) => write!(f, "Expression({})", expr.expression()),
            TriggerCondition::Composite { left, operator, right } => {
                let op_str = match operator {
                    LogicalOperator::And => "AND",
                    LogicalOperator::Or => "OR",
                    LogicalOperator::Not => "NOT",
                };
                if *operator == LogicalOperator::Not {
                    write!(f, "NOT {}", left)
                } else {
                    write!(f, "({} {} {})", left, op_str, right)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_timer_expression_creation() {
        let timer = TimerExpression::new("5s").unwrap();
        assert_eq!(timer.duration_string(), "5s");
        assert_eq!(timer.duration().unwrap().as_secs(), 5);
    }
    
    #[test]
    fn test_timer_expression_validation() {
        assert!(TimerExpression::new("5s").is_ok());
        assert!(TimerExpression::new("10m").is_ok());
        assert!(TimerExpression::new("1h").is_ok());
        assert!(TimerExpression::new("1d").is_ok());
        
        assert!(TimerExpression::new("0s").is_err());
        assert!(TimerExpression::new("invalid").is_err());
        assert!(TimerExpression::new("31d").is_err()); // Over 30 days
    }
    
    #[test]
    fn test_event_pattern_creation() {
        let pattern = EventPattern::new("user.created").unwrap();
        assert_eq!(pattern.pattern(), "user.created");
        assert!(!pattern.uses_regex());
    }
    
    #[test]
    fn test_event_pattern_matching() {
        let pattern = EventPattern::new("user.*").unwrap();
        assert!(pattern.matches("user.created"));
        assert!(pattern.matches("user.updated"));
        assert!(!pattern.matches("order.created"));
        
        let exact_pattern = EventPattern::new("user.created").unwrap();
        assert!(exact_pattern.matches("user.created"));
        assert!(!exact_pattern.matches("user.updated"));
    }
    
    #[test]
    fn test_event_pattern_regex() {
        let pattern = EventPattern::with_regex(r"user\.(created|updated)").unwrap();
        assert!(pattern.uses_regex());
        assert!(pattern.matches("user.created"));
        assert!(pattern.matches("user.updated"));
        assert!(!pattern.matches("user.deleted"));
    }
    
    #[test]
    fn test_logical_expression_creation() {
        let expr = LogicalExpression::new("count > 10").unwrap();
        assert_eq!(expr.expression(), "count > 10");
    }
    
    #[test]
    fn test_logical_expression_validation() {
        assert!(LogicalExpression::new("count > 10").is_ok());
        assert!(LogicalExpression::new("x = 5 AND y < 3").is_ok());
        assert!(LogicalExpression::new("").is_err());
        assert!(LogicalExpression::new("no operators").is_err());
    }
    
    #[test]
    fn test_trigger_condition_creation() {
        let timer = TriggerCondition::timer("5s").unwrap();
        assert!(timer.is_timer());
        
        let event = TriggerCondition::event("user.created").unwrap();
        assert!(event.is_event());
        
        let expr = TriggerCondition::expression("count > 10").unwrap();
        assert!(expr.is_expression());
        
        assert!(TriggerCondition::Always.is_always());
        assert!(TriggerCondition::Never.is_never());
    }
    
    #[test]
    fn test_trigger_condition_composition() {
        let timer = TriggerCondition::timer("5s").unwrap();
        let event = TriggerCondition::event("user.created").unwrap();
        
        let and_condition = timer.clone().and(event.clone());
        assert!(and_condition.is_composite());
        
        let or_condition = timer.clone().or(event.clone());
        assert!(or_condition.is_composite());
        
        let not_condition = timer.not();
        assert!(not_condition.is_composite());
    }
    
    #[test]
    fn test_trigger_condition_validation() {
        let valid_timer = TriggerCondition::timer("5s").unwrap();
        assert!(valid_timer.validate().is_ok());
        
        let valid_event = TriggerCondition::event("user.created").unwrap();
        assert!(valid_event.validate().is_ok());
        
        let valid_expr = TriggerCondition::expression("count > 10").unwrap();
        assert!(valid_expr.validate().is_ok());
        
        let composite = valid_timer.and(valid_event);
        assert!(composite.validate().is_ok());
    }
    
    #[test]
    fn test_trigger_condition_display() {
        let timer = TriggerCondition::timer("5s").unwrap();
        assert_eq!(format!("{}", timer), "Timer(5s)");
        
        let event = TriggerCondition::event("user.created").unwrap();
        assert_eq!(format!("{}", event), "Event(user.created)");
        
        let expr = TriggerCondition::expression("count > 10").unwrap();
        assert_eq!(format!("{}", expr), "Expression(count > 10)");
        
        let composite = timer.clone().and(event.clone());
        let display = format!("{}", composite);
        assert!(display.contains("Timer(5s)"));
        assert!(display.contains("Event(user.created)"));
        assert!(display.contains("AND"));
    }
}