// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Trigger State Value Object
//! 
//! Represents the current state of a trigger with validation and transition rules.
//! Ensures proper state machine behavior and audit trails for state changes.

use hexafn_core::types::{ValidationError, Timestamp};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Trigger execution states with proper lifecycle management
///
/// # State Machine
///
/// ```text
/// Inactive -> Active -> Executing -> (Success|Failed) -> Active
///                |                                         ^
///                v                                         |
///            Suspended ----------------------------------------
///                |
///                v
///            Archived (terminal state)
/// ```
///
/// # Examples
///
/// ```rust
/// use hexafn_trigger::domain::value_objects::{TriggerState, StateType};
///
/// // Create initial state
/// let state = TriggerState::new(StateType::Inactive);
///
/// // Transition through lifecycle
/// let active_state = state.transition_to(StateType::Active)?;
/// let executing_state = active_state.transition_to(StateType::Executing)?;
/// let completed_state = executing_state.transition_to(StateType::Success)?;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriggerState {
    /// Current state type
    current_state: StateType,
    
    /// Previous state (for audit trail)
    previous_state: Option<StateType>,
    
    /// Timestamp when this state was entered
    entered_at: Timestamp,
    
    /// Optional reason for state change
    reason: Option<String>,
    
    /// Number of times trigger has been executed
    execution_count: u64,
    
    /// Number of consecutive failures
    failure_count: u64,
    
    /// Last execution timestamp
    last_executed_at: Option<Timestamp>,
    
    /// State-specific metadata
    metadata: std::collections::HashMap<String, String>,
}

/// Available trigger states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateType {
    /// Trigger is created but not active
    Inactive,
    
    /// Trigger is active and ready to execute
    Active,
    
    /// Trigger is currently executing
    Executing,
    
    /// Last execution completed successfully
    Success,
    
    /// Last execution failed
    Failed,
    
    /// Trigger is temporarily suspended
    Suspended,
    
    /// Trigger is permanently archived (terminal)
    Archived,
}

/// Possible state transition errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateTransitionError {
    /// Invalid transition from current state
    InvalidTransition {
        from: StateType,
        to: StateType,
        reason: String,
    },
    
    /// Maximum failure count exceeded
    MaxFailuresExceeded {
        current_failures: u64,
        max_allowed: u64,
    },
    
    /// State validation failed
    ValidationFailed {
        state: StateType,
        reason: String,
    },
}

impl fmt::Display for StateTransitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateTransitionError::InvalidTransition { from, to, reason } => {
                write!(f, "Invalid transition from {} to {}: {}", from, to, reason)
            }
            StateTransitionError::MaxFailuresExceeded { current_failures, max_allowed } => {
                write!(f, "Max failures exceeded: {} > {}", current_failures, max_allowed)
            }
            StateTransitionError::ValidationFailed { state, reason } => {
                write!(f, "State validation failed for {}: {}", state, reason)
            }
        }
    }
}

impl std::error::Error for StateTransitionError {}

impl StateType {
    /// Check if transition to another state is valid
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::StateType;
    ///
    /// assert!(StateType::Inactive.can_transition_to(StateType::Active));
    /// assert!(StateType::Active.can_transition_to(StateType::Executing));
    /// assert!(!StateType::Archived.can_transition_to(StateType::Active));
    /// ```
    pub fn can_transition_to(self, target: StateType) -> bool {
        match (self, target) {
            // From Inactive
            (StateType::Inactive, StateType::Active) => true,
            (StateType::Inactive, StateType::Archived) => true,
            
            // From Active
            (StateType::Active, StateType::Executing) => true,
            (StateType::Active, StateType::Suspended) => true,
            (StateType::Active, StateType::Inactive) => true,
            (StateType::Active, StateType::Archived) => true,
            
            // From Executing
            (StateType::Executing, StateType::Success) => true,
            (StateType::Executing, StateType::Failed) => true,
            (StateType::Executing, StateType::Suspended) => true, // Emergency suspend
            
            // From Success
            (StateType::Success, StateType::Active) => true,
            (StateType::Success, StateType::Suspended) => true,
            (StateType::Success, StateType::Archived) => true,
            
            // From Failed
            (StateType::Failed, StateType::Active) => true,
            (StateType::Failed, StateType::Suspended) => true,
            (StateType::Failed, StateType::Archived) => true,
            
            // From Suspended
            (StateType::Suspended, StateType::Active) => true,
            (StateType::Suspended, StateType::Inactive) => true,
            (StateType::Suspended, StateType::Archived) => true,
            
            // From Archived (terminal state)
            (StateType::Archived, _) => false,
            
            // Self-transitions (idempotent)
            (state, target) if state == target => true,
            
            // All other transitions are invalid
            _ => false,
        }
    }
    
    /// Check if state is terminal (no outgoing transitions except self)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::StateType;
    ///
    /// assert!(StateType::Archived.is_terminal());
    /// assert!(!StateType::Active.is_terminal());
    /// ```
    pub fn is_terminal(self) -> bool {
        matches!(self, StateType::Archived)
    }
    
    /// Check if state allows trigger execution
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::StateType;
    ///
    /// assert!(StateType::Active.allows_execution());
    /// assert!(!StateType::Suspended.allows_execution());
    /// ```
    pub fn allows_execution(self) -> bool {
        matches!(self, StateType::Active)
    }
    
    /// Check if state indicates successful completion
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::StateType;
    ///
    /// assert!(StateType::Success.is_successful());
    /// assert!(!StateType::Failed.is_successful());
    /// ```
    pub fn is_successful(self) -> bool {
        matches!(self, StateType::Success)
    }
    
    /// Check if state indicates failure
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::StateType;
    ///
    /// assert!(StateType::Failed.is_failed());
    /// assert!(!StateType::Success.is_failed());
    /// ```
    pub fn is_failed(self) -> bool {
        matches!(self, StateType::Failed)
    }
    
    /// Check if state is active (ready for execution)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::StateType;
    ///
    /// assert!(StateType::Active.is_active());
    /// assert!(!StateType::Inactive.is_active());
    /// ```
    pub fn is_active(self) -> bool {
        matches!(self, StateType::Active)
    }
    
    /// Check if state is executing
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::StateType;
    ///
    /// assert!(StateType::Executing.is_executing());
    /// assert!(!StateType::Active.is_executing());
    /// ```
    pub fn is_executing(self) -> bool {
        matches!(self, StateType::Executing)
    }
}

impl fmt::Display for StateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state_str = match self {
            StateType::Inactive => "inactive",
            StateType::Active => "active",
            StateType::Executing => "executing",
            StateType::Success => "success",
            StateType::Failed => "failed",
            StateType::Suspended => "suspended",
            StateType::Archived => "archived",
        };
        write!(f, "{}", state_str)
    }
}

impl std::str::FromStr for StateType {
    type Err = ValidationError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "inactive" => Ok(StateType::Inactive),
            "active" => Ok(StateType::Active),
            "executing" => Ok(StateType::Executing),
            "success" => Ok(StateType::Success),
            "failed" => Ok(StateType::Failed),
            "suspended" => Ok(StateType::Suspended),
            "archived" => Ok(StateType::Archived),
            _ => Err(ValidationError::InvalidValue {
                field: "state_type".to_string(),
                value: s.to_string(),
                reason: "Unknown state type".to_string(),
            }),
        }
    }
}

impl TriggerState {
    /// Create a new trigger state
    ///
    /// # Arguments
    ///
    /// * `initial_state` - The initial state type
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::{TriggerState, StateType};
    ///
    /// let state = TriggerState::new(StateType::Inactive);
    /// assert_eq!(state.current_state(), StateType::Inactive);
    /// assert_eq!(state.execution_count(), 0);
    /// ```
    pub fn new(initial_state: StateType) -> Self {
        Self {
            current_state: initial_state,
            previous_state: None,
            entered_at: Timestamp::now(),
            reason: None,
            execution_count: 0,
            failure_count: 0,
            last_executed_at: None,
            metadata: std::collections::HashMap::new(),
        }
    }
    
    /// Create an active trigger state (convenience constructor)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::{TriggerState, StateType};
    ///
    /// let state = TriggerState::active();
    /// assert_eq!(state.current_state(), StateType::Active);
    /// ```
    pub fn active() -> Self {
        Self::new(StateType::Active)
    }
    
    /// Create an inactive trigger state (convenience constructor)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::{TriggerState, StateType};
    ///
    /// let state = TriggerState::inactive();
    /// assert_eq!(state.current_state(), StateType::Inactive);
    /// ```
    pub fn inactive() -> Self {
        Self::new(StateType::Inactive)
    }
    
    /// Transition to a new state
    ///
    /// # Arguments
    ///
    /// * `target_state` - The target state to transition to
    ///
    /// # Errors
    ///
    /// Returns `StateTransitionError` if the transition is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::{TriggerState, StateType};
    ///
    /// let state = TriggerState::new(StateType::Inactive);
    /// let active_state = state.transition_to(StateType::Active)?;
    /// assert_eq!(active_state.current_state(), StateType::Active);
    /// assert_eq!(active_state.previous_state(), Some(StateType::Inactive));
    /// ```
    pub fn transition_to(self, target_state: StateType) -> Result<Self, StateTransitionError> {
        self.transition_to_with_reason(target_state, None)
    }
    
    /// Transition to a new state with a reason
    ///
    /// # Arguments
    ///
    /// * `target_state` - The target state to transition to
    /// * `reason` - Optional reason for the transition
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::{TriggerState, StateType};
    ///
    /// let state = TriggerState::new(StateType::Active);
    /// let suspended_state = state.transition_to_with_reason(
    ///     StateType::Suspended,
    ///     Some("Maintenance mode".to_string())
    /// )?;
    /// assert_eq!(suspended_state.reason(), Some("Maintenance mode"));
    /// ```
    pub fn transition_to_with_reason(
        mut self, 
        target_state: StateType, 
        reason: Option<String>
    ) -> Result<Self, StateTransitionError> {
        // Check if transition is valid
        if !self.current_state.can_transition_to(target_state) {
            return Err(StateTransitionError::InvalidTransition {
                from: self.current_state,
                to: target_state,
                reason: format!(
                    "Transition from {} to {} is not allowed", 
                    self.current_state, 
                    target_state
                ),
            });
        }
        
        // Update state
        self.previous_state = Some(self.current_state);
        self.current_state = target_state;
        self.entered_at = Timestamp::now();
        self.reason = reason;
        
        Ok(self)
    }
    
    /// Record successful execution
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::{TriggerState, StateType};
    ///
    /// let state = TriggerState::new(StateType::Executing);
    /// let success_state = state.record_execution_success()?;
    /// assert_eq!(success_state.current_state(), StateType::Success);
    /// assert_eq!(success_state.execution_count(), 1);
    /// assert_eq!(success_state.failure_count(), 0);
    /// ```
    pub fn record_execution_success(mut self) -> Result<Self, StateTransitionError> {
        if !matches!(self.current_state, StateType::Executing) {
            return Err(StateTransitionError::InvalidTransition {
                from: self.current_state,
                to: StateType::Success,
                reason: "Can only record success from executing state".to_string(),
            });
        }
        
        self.execution_count += 1;
        self.failure_count = 0; // Reset failure count on success
        self.last_executed_at = Some(Timestamp::now());
        
        self.transition_to_with_reason(
            StateType::Success, 
            Some("Execution completed successfully".to_string())
        )
    }
    
    /// Record failed execution
    ///
    /// # Arguments
    ///
    /// * `error_message` - Error message for the failure
    /// * `max_failures` - Maximum allowed consecutive failures
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::{TriggerState, StateType};
    ///
    /// let state = TriggerState::new(StateType::Executing);
    /// let failed_state = state.record_execution_failure("Network timeout", 3)?;
    /// assert_eq!(failed_state.current_state(), StateType::Failed);
    /// assert_eq!(failed_state.failure_count(), 1);
    /// ```
    pub fn record_execution_failure(
        mut self, 
        error_message: &str, 
        max_failures: u64
    ) -> Result<Self, StateTransitionError> {
        if !matches!(self.current_state, StateType::Executing) {
            return Err(StateTransitionError::InvalidTransition {
                from: self.current_state,
                to: StateType::Failed,
                reason: "Can only record failure from executing state".to_string(),
            });
        }
        
        self.execution_count += 1;
        self.failure_count += 1;
        self.last_executed_at = Some(Timestamp::now());
        
        // Check if max failures exceeded
        if self.failure_count > max_failures {
            return Err(StateTransitionError::MaxFailuresExceeded {
                current_failures: self.failure_count,
                max_allowed: max_failures,
            });
        }
        
        self.transition_to_with_reason(
            StateType::Failed, 
            Some(format!("Execution failed: {}", error_message))
        )
    }
    
    /// Start execution (transition from Active to Executing)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::{TriggerState, StateType};
    ///
    /// let state = TriggerState::new(StateType::Active);
    /// let executing_state = state.start_execution()?;
    /// assert_eq!(executing_state.current_state(), StateType::Executing);
    /// ```
    pub fn start_execution(self) -> Result<Self, StateTransitionError> {
        self.transition_to_with_reason(
            StateType::Executing, 
            Some("Execution started".to_string())
        )
    }
    
    /// Suspend the trigger
    ///
    /// # Arguments
    ///
    /// * `reason` - Reason for suspension
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::{TriggerState, StateType};
    ///
    /// let state = TriggerState::new(StateType::Active);
    /// let suspended_state = state.suspend("Maintenance required")?;
    /// assert_eq!(suspended_state.current_state(), StateType::Suspended);
    /// ```
    pub fn suspend<S: Into<String>>(self, reason: S) -> Result<Self, StateTransitionError> {
        self.transition_to_with_reason(
            StateType::Suspended, 
            Some(reason.into())
        )
    }
    
    /// Archive the trigger (terminal state)
    ///
    /// # Arguments
    ///
    /// * `reason` - Reason for archiving
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::{TriggerState, StateType};
    ///
    /// let state = TriggerState::new(StateType::Inactive);
    /// let archived_state = state.archive("No longer needed")?;
    /// assert_eq!(archived_state.current_state(), StateType::Archived);
    /// assert!(archived_state.is_terminal());
    /// ```
    pub fn archive<S: Into<String>>(self, reason: S) -> Result<Self, StateTransitionError> {
        self.transition_to_with_reason(
            StateType::Archived, 
            Some(reason.into())
        )
    }
    
    /// Resume from suspended state
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::{TriggerState, StateType};
    ///
    /// let suspended_state = TriggerState::new(StateType::Suspended);
    /// let active_state = suspended_state.resume()?;
    /// assert_eq!(active_state.current_state(), StateType::Active);
    /// ```
    pub fn resume(self) -> Result<Self, StateTransitionError> {
        if !matches!(self.current_state, StateType::Suspended) {
            return Err(StateTransitionError::InvalidTransition {
                from: self.current_state,
                to: StateType::Active,
                reason: "Can only resume from suspended state".to_string(),
            });
        }
        
        self.transition_to_with_reason(
            StateType::Active, 
            Some("Resumed from suspension".to_string())
        )
    }
    
    /// Get current state
    pub fn current_state(&self) -> StateType {
        self.current_state
    }
    
    /// Get previous state
    pub fn previous_state(&self) -> Option<StateType> {
        self.previous_state
    }
    
    /// Get timestamp when current state was entered
    pub fn entered_at(&self) -> &Timestamp {
        &self.entered_at
    }
    
    /// Get reason for current state
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
    
    /// Get execution count
    pub fn execution_count(&self) -> u64 {
        self.execution_count
    }
    
    /// Get failure count
    pub fn failure_count(&self) -> u64 {
        self.failure_count
    }
    
    /// Get last execution timestamp
    pub fn last_executed_at(&self) -> Option<&Timestamp> {
        self.last_executed_at.as_ref()
    }
    
    /// Get state metadata
    pub fn metadata(&self) -> &std::collections::HashMap<String, String> {
        &self.metadata
    }
    
    /// Add metadata to the state
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::{TriggerState, StateType};
    ///
    /// let mut state = TriggerState::new(StateType::Active);
    /// state.add_metadata("retry_count", "3");
    /// assert_eq!(state.metadata().get("retry_count"), Some(&"3".to_string()));
    /// ```
    pub fn add_metadata<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.metadata.insert(key.into(), value.into());
    }
    
    /// Check if trigger can be executed in current state
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::{TriggerState, StateType};
    ///
    /// let active_state = TriggerState::new(StateType::Active);
    /// assert!(active_state.can_execute());
    ///
    /// let suspended_state = TriggerState::new(StateType::Suspended);
    /// assert!(!suspended_state.can_execute());
    /// ```
    pub fn can_execute(&self) -> bool {
        self.current_state.allows_execution()
    }
    
    /// Check if state is terminal
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::{TriggerState, StateType};
    ///
    /// let archived_state = TriggerState::new(StateType::Archived);
    /// assert!(archived_state.is_terminal());
    /// ```
    pub fn is_terminal(&self) -> bool {
        self.current_state.is_terminal()
    }
    
    /// Get the age of current state (time since entered)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::{TriggerState, StateType};
    /// use std::time::Duration;
    ///
    /// let state = TriggerState::new(StateType::Active);
    /// let age = state.state_age();
    /// assert!(age >= Duration::ZERO);
    /// ```
    pub fn state_age(&self) -> std::time::Duration {
        self.entered_at.elapsed()
    }
    
    /// Validate current state consistency
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexafn_trigger::domain::value_objects::{TriggerState, StateType};
    ///
    /// let state = TriggerState::new(StateType::Active);
    /// assert!(state.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Validate execution count consistency
        if self.execution_count == 0 && self.last_executed_at.is_some() {
            return Err(ValidationError::InvalidValue {
                field: "execution_count".to_string(),
                value: "0".to_string(),
                reason: "Execution count is 0 but last_executed_at is set".to_string(),
            });
        }
        
        // Validate failure count
        if self.failure_count > self.execution_count {
            return Err(ValidationError::InvalidValue {
                field: "failure_count".to_string(),
                value: self.failure_count.to_string(),
                reason: "Failure count cannot exceed execution count".to_string(),
            });
        }
        
        // Validate state transition consistency
        if let Some(prev_state) = self.previous_state {
            if !prev_state.can_transition_to(self.current_state) {
                return Err(ValidationError::InvalidValue {
                    field: "state_transition".to_string(),
                    value: format!("{} -> {}", prev_state, self.current_state),
                    reason: "Invalid state transition recorded".to_string(),
                });
            }
        }
        
        Ok(())
    }
}

impl fmt::Display for TriggerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "TriggerState[{}] (executions: {}, failures: {})", 
            self.current_state, 
            self.execution_count, 
            self.failure_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_state_type_transitions() {
        // Test valid transitions
        assert!(StateType::Inactive.can_transition_to(StateType::Active));
        assert!(StateType::Active.can_transition_to(StateType::Executing));
        assert!(StateType::Executing.can_transition_to(StateType::Success));
        assert!(StateType::Executing.can_transition_to(StateType::Failed));
        assert!(StateType::Success.can_transition_to(StateType::Active));
        assert!(StateType::Failed.can_transition_to(StateType::Active));
        
        // Test invalid transitions
        assert!(!StateType::Inactive.can_transition_to(StateType::Executing));
        assert!(!StateType::Archived.can_transition_to(StateType::Active));
        assert!(!StateType::Success.can_transition_to(StateType::Executing));
        
        // Test self-transitions (idempotent)
        assert!(StateType::Active.can_transition_to(StateType::Active));
    }
    
    #[test]
    fn test_state_type_properties() {
        assert!(StateType::Archived.is_terminal());
        assert!(!StateType::Active.is_terminal());
        
        assert!(StateType::Active.allows_execution());
        assert!(!StateType::Suspended.allows_execution());
        
        assert!(StateType::Success.is_successful());
        assert!(!StateType::Failed.is_successful());
        
        assert!(StateType::Failed.is_failed());
        assert!(!StateType::Success.is_failed());
    }
    
    #[test]
    fn test_state_type_display() {
        assert_eq!(format!("{}", StateType::Active), "active");
        assert_eq!(format!("{}", StateType::Executing), "executing");
        assert_eq!(format!("{}", StateType::Archived), "archived");
    }
    
    #[test]
    fn test_state_type_from_str() {
        use std::str::FromStr;
        
        assert_eq!(StateType::from_str("active").unwrap(), StateType::Active);
        assert_eq!(StateType::from_str("INACTIVE").unwrap(), StateType::Inactive);
        assert_eq!(StateType::from_str("Executing").unwrap(), StateType::Executing);
        
        assert!(StateType::from_str("invalid").is_err());
    }
    
    #[test]
    fn test_trigger_state_creation() {
        let state = TriggerState::new(StateType::Inactive);
        assert_eq!(state.current_state(), StateType::Inactive);
        assert_eq!(state.previous_state(), None);
        assert_eq!(state.execution_count(), 0);
        assert_eq!(state.failure_count(), 0);
        assert!(state.last_executed_at().is_none());
        
        let active_state = TriggerState::active();
        assert_eq!(active_state.current_state(), StateType::Active);
        
        let inactive_state = TriggerState::inactive();
        assert_eq!(inactive_state.current_state(), StateType::Inactive);
    }
    
    #[test]
    fn test_trigger_state_transitions() {
        let state = TriggerState::new(StateType::Inactive);
        
        // Valid transition
        let active_state = state.transition_to(StateType::Active).unwrap();
        assert_eq!(active_state.current_state(), StateType::Active);
        assert_eq!(active_state.previous_state(), Some(StateType::Inactive));
        
        // Invalid transition should fail
        let archived_state = TriggerState::new(StateType::Archived);
        let result = archived_state.transition_to(StateType::Active);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_execution_lifecycle() {
        let state = TriggerState::new(StateType::Active);
        
        // Start execution
        let executing_state = state.start_execution().unwrap();
        assert_eq!(executing_state.current_state(), StateType::Executing);
        
        // Record success
        let success_state = executing_state.record_execution_success().unwrap();
        assert_eq!(success_state.current_state(), StateType::Success);
        assert_eq!(success_state.execution_count(), 1);
        assert_eq!(success_state.failure_count(), 0);
        assert!(success_state.last_executed_at().is_some());
    }
    
    #[test]
    fn test_execution_failure() {
        let state = TriggerState::new(StateType::Active);
        let executing_state = state.start_execution().unwrap();
        
        // Record failure
        let failed_state = executing_state
            .record_execution_failure("Network error", 3)
            .unwrap();
        assert_eq!(failed_state.current_state(), StateType::Failed);
        assert_eq!(failed_state.execution_count(), 1);
        assert_eq!(failed_state.failure_count(), 1);
        assert!(failed_state.reason().unwrap().contains("Network error"));
    }
    
    #[test]
    fn test_max_failures_exceeded() {
        let mut state = TriggerState::new(StateType::Active);
        
        // Simulate multiple failures
        for i in 1..=3 {
            let executing_state = state.start_execution().unwrap();
            state = executing_state
                .record_execution_failure("Test error", 3)
                .unwrap();
            assert_eq!(state.failure_count(), i);
        }
        
        // Next failure should exceed limit
        let executing_state = state.start_execution().unwrap();
        let result = executing_state.record_execution_failure("Test error", 3);
        assert!(matches!(result, Err(StateTransitionError::MaxFailuresExceeded { .. })));
    }
    
    #[test]
    fn test_suspension_and_resume() {
        let state = TriggerState::new(StateType::Active);
        
        // Suspend
        let suspended_state = state.suspend("Maintenance required").unwrap();
        assert_eq!(suspended_state.current_state(), StateType::Suspended);
        assert_eq!(suspended_state.reason(), Some("Maintenance required"));
        assert!(!suspended_state.can_execute());
        
        // Resume
        let resumed_state = suspended_state.resume().unwrap();
        assert_eq!(resumed_state.current_state(), StateType::Active);
        assert_eq!(resumed_state.reason(), Some("Resumed from suspension"));
        assert!(resumed_state.can_execute());
    }
    
    #[test]
    fn test_archiving() {
        let state = TriggerState::new(StateType::Inactive);
        
        let archived_state = state.archive("No longer needed").unwrap();
        assert_eq!(archived_state.current_state(), StateType::Archived);
        assert!(archived_state.is_terminal());
        assert_eq!(archived_state.reason(), Some("No longer needed"));
        
        // Cannot transition from archived state
        let result = archived_state.transition_to(StateType::Active);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_metadata() {
        let mut state = TriggerState::new(StateType::Active);
        
        state.add_metadata("retry_count", "3");
        state.add_metadata("last_error", "timeout");
        
        assert_eq!(state.metadata().get("retry_count"), Some(&"3".to_string()));
        assert_eq!(state.metadata().get("last_error"), Some(&"timeout".to_string()));
    }
    
    #[test]
    fn test_state_validation() {
        let state = TriggerState::new(StateType::Active);
        assert!(state.validate().is_ok());
        
        // Create invalid state manually (normally not possible through API)
        let mut invalid_state = TriggerState::new(StateType::Active);
        invalid_state.failure_count = 5;
        invalid_state.execution_count = 3; // failures > executions
        assert!(invalid_state.validate().is_err());
    }
    
    #[test]
    fn test_state_age() {
        let state = TriggerState::new(StateType::Active);
        let age = state.state_age();
        assert!(age >= std::time::Duration::ZERO);
        assert!(age < std::time::Duration::from_secs(1)); // Should be very recent
    }
    
    #[test]
    fn test_state_display() {
        let state = TriggerState::new(StateType::Active);
        let display_str = format!("{}", state);
        assert!(display_str.contains("TriggerState[active]"));
        assert!(display_str.contains("executions: 0"));
        assert!(display_str.contains("failures: 0"));
    }
    
    #[test]
    fn test_transition_with_reason() {
        let state = TriggerState::new(StateType::Active);
        let suspended_state = state
            .transition_to_with_reason(
                StateType::Suspended, 
                Some("Custom reason".to_string())
            )
            .unwrap();
        
        assert_eq!(suspended_state.current_state(), StateType::Suspended);
        assert_eq!(suspended_state.reason(), Some("Custom reason"));
    }
    
    #[test]
    fn test_failure_count_reset_on_success() {
        let mut state = TriggerState::new(StateType::Active);
        
        // Fail once
        let executing_state = state.start_execution().unwrap();
        state = executing_state
            .record_execution_failure("Test error", 3)
            .unwrap();
        assert_eq!(state.failure_count(), 1);
        
        // Then succeed
        let executing_state = state.transition_to(StateType::Active).unwrap()
            .start_execution().unwrap();
        state = executing_state.record_execution_success().unwrap();
        
        // Failure count should be reset
        assert_eq!(state.failure_count(), 0);
        assert_eq!(state.execution_count(), 2);
    }
}