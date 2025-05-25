// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # 6F Lifecycle Flow Phase Constants and Utilities
//! 
//! This module defines the core constants and utility functions for the 6F Lifecycle Flow phases.

/// Feed phase - Data ingestion from external sources
pub const FEED: &str = "feed";

/// Filter phase - Validation and gating logic
pub const FILTER: &str = "filter";

/// Format phase - Data transformation and normalization
pub const FORMAT: &str = "format";

/// Function phase - Core business logic execution
pub const FUNCTION: &str = "function";

/// Forward phase - Output routing and delivery
pub const FORWARD: &str = "forward";

/// Feedback phase - Observability and audit trail
pub const FEEDBACK: &str = "feedback";

/// All 6F phases in execution order
pub const ALL_PHASES: &[&str] = &[FEED, FILTER, FORMAT, FUNCTION, FORWARD, FEEDBACK];

/// Phase execution order mapping
pub const PHASE_ORDER: &[(u8, &str)] = &[
    (1, FEED),
    (2, FILTER),
    (3, FORMAT),
    (4, FUNCTION),
    (5, FORWARD),
    (6, FEEDBACK),
];

/// Maximum number of phases in the 6F lifecycle
pub const MAX_PHASE_COUNT: u8 = 6;

/// Phase names as an enum for type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    Feed,
    Filter,
    Format,
    Function,
    Forward,
    Feedback,
}

impl Phase {
    /// Convert phase enum to string
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::Phase;
    ///
    /// assert_eq!(Phase::Feed.as_str(), "feed");
    /// assert_eq!(Phase::Function.as_str(), "function");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Phase::Feed => FEED,
            Phase::Filter => FILTER,
            Phase::Format => FORMAT,
            Phase::Function => FUNCTION,
            Phase::Forward => FORWARD,
            Phase::Feedback => FEEDBACK,
        }
    }
    
    /// Get the execution order of this phase
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::Phase;
    ///
    /// assert_eq!(Phase::Feed.order(), 1);
    /// assert_eq!(Phase::Feedback.order(), 6);
    /// ```
    pub fn order(&self) -> u8 {
        match self {
            Phase::Feed => 1,
            Phase::Filter => 2,
            Phase::Format => 3,
            Phase::Function => 4,
            Phase::Forward => 5,
            Phase::Feedback => 6,
        }
    }
    
    /// Get the next phase in the 6F flow
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::Phase;
    ///
    /// assert_eq!(Phase::Feed.next(), Some(Phase::Filter));
    /// assert_eq!(Phase::Feedback.next(), None); // Last phase
    /// ```
    pub fn next(&self) -> Option<Phase> {
        match self {
            Phase::Feed => Some(Phase::Filter),
            Phase::Filter => Some(Phase::Format),
            Phase::Format => Some(Phase::Function),
            Phase::Function => Some(Phase::Forward),
            Phase::Forward => Some(Phase::Feedback),
            Phase::Feedback => None,
        }
    }
    
    /// Get the previous phase in the 6F flow
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::Phase;
    ///
    /// assert_eq!(Phase::Filter.previous(), Some(Phase::Feed));
    /// assert_eq!(Phase::Feed.previous(), None); // First phase
    /// ```
    pub fn previous(&self) -> Option<Phase> {
        match self {
            Phase::Feed => None,
            Phase::Filter => Some(Phase::Feed),
            Phase::Format => Some(Phase::Filter),
            Phase::Function => Some(Phase::Format),
            Phase::Forward => Some(Phase::Function),
            Phase::Feedback => Some(Phase::Forward),
        }
    }
    
    /// Check if this is the first phase
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::Phase;
    ///
    /// assert!(Phase::Feed.is_first());
    /// assert!(!Phase::Filter.is_first());
    /// ```
    pub fn is_first(&self) -> bool {
        matches!(self, Phase::Feed)
    }
    
    /// Check if this is the last phase
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::Phase;
    ///
    /// assert!(Phase::Feedback.is_last());
    /// assert!(!Phase::Forward.is_last());
    /// ```
    pub fn is_last(&self) -> bool {
        matches!(self, Phase::Feedback)
    }
    
    /// Get all phases as an iterator
    ///
    /// # Examples
    ///
    /// ```
    /// use hexafn_core::phases::Phase;
    ///
    /// let phases: Vec<Phase> = Phase::all().collect();
    /// assert_eq!(phases.len(), 6);
    /// assert_eq!(phases[0], Phase::Feed);
    /// assert_eq!(phases[5], Phase::Feedback);
    /// ```
    pub fn all() -> impl Iterator<Item = Phase> {
        [
            Phase::Feed,
            Phase::Filter,
            Phase::Format,
            Phase::Function,
            Phase::Forward,
            Phase::Feedback,
        ].into_iter()
    }
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Phase {
    type Err = PhaseParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            FEED => Ok(Phase::Feed),
            FILTER => Ok(Phase::Filter),
            FORMAT => Ok(Phase::Format),
            FUNCTION => Ok(Phase::Function),
            FORWARD => Ok(Phase::Forward),
            FEEDBACK => Ok(Phase::Feedback),
            _ => Err(PhaseParseError::InvalidPhase(s.to_string())),
        }
    }
}

/// Error for parsing phase from string
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhaseParseError {
    InvalidPhase(String),
}

impl std::fmt::Display for PhaseParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhaseParseError::InvalidPhase(phase) => {
                write!(f, "Invalid phase '{}'. Valid phases are: {}", phase, ALL_PHASES.join(", "))
            }
        }
    }
}

impl std::error::Error for PhaseParseError {}

/// Validate if a phase name is valid
///
/// # Examples
///
/// ```
/// use hexafn_core::phases;
///
/// assert!(phases::is_valid_phase("feed"));
/// assert!(phases::is_valid_phase("filter"));
/// assert!(!phases::is_valid_phase("invalid"));
/// ```
pub fn is_valid_phase(phase: &str) -> bool {
    ALL_PHASES.contains(&phase)
}

/// Get the execution order of a phase
///
/// # Examples
///
/// ```
/// use hexafn_core::phases;
///
/// assert_eq!(phases::get_phase_order("feed"), Some(1));
/// assert_eq!(phases::get_phase_order("feedback"), Some(6));
/// assert_eq!(phases::get_phase_order("invalid"), None);
/// ```
pub fn get_phase_order(phase: &str) -> Option<u8> {
    PHASE_ORDER.iter()
        .find(|(_, name)| *name == phase)
        .map(|(order, _)| *order)
}

/// Get the next phase in the 6F flow
///
/// # Examples
///
/// ```
/// use hexafn_core::phases;
///
/// assert_eq!(phases::next_phase("feed"), Some("filter"));
/// assert_eq!(phases::next_phase("function"), Some("forward"));
/// assert_eq!(phases::next_phase("feedback"), None); // Last phase
/// assert_eq!(phases::next_phase("invalid"), None);
/// ```
pub fn next_phase(current: &str) -> Option<&'static str> {
    let current_order = get_phase_order(current)?;
    if current_order >= MAX_PHASE_COUNT {
        return None; // Last phase
    }
    
    PHASE_ORDER.iter()
        .find(|(order, _)| *order == current_order + 1)
        .map(|(_, name)| *name)
}

/// Get the previous phase in the 6F flow
///
/// # Examples
///
/// ```
/// use hexafn_core::phases;
///
/// assert_eq!(phases::previous_phase("filter"), Some("feed"));
/// assert_eq!(phases::previous_phase("feedback"), Some("forward"));
/// assert_eq!(phases::previous_phase("feed"), None); // First phase
/// assert_eq!(phases::previous_phase("invalid"), None);
/// ```
pub fn previous_phase(current: &str) -> Option<&'static str> {
    let current_order = get_phase_order(current)?;
    if current_order <= 1 {
        return None; // First phase
    }
    
    PHASE_ORDER.iter()
        .find(|(order, _)| *order == current_order - 1)
        .map(|(_, name)| *name)
}

/// Get all phases in execution order
///
/// # Examples
///
/// ```
/// use hexafn_core::phases;
///
/// let phases = phases::get_all_phases();
/// assert_eq!(phases.len(), 6);
/// assert_eq!(phases[0], "feed");
/// assert_eq!(phases[5], "feedback");
/// ```
pub fn get_all_phases() -> &'static [&'static str] {
    ALL_PHASES
}

/// Get phase order mapping
///
/// # Examples
///
/// ```
/// use hexafn_core::phases;
///
/// let order_map = phases::get_phase_order_map();
/// assert_eq!(order_map.len(), 6);
/// assert_eq!(order_map[0], (1, "feed"));
/// assert_eq!(order_map[5], (6, "feedback"));
/// ```
pub fn get_phase_order_map() -> &'static [(u8, &'static str)] {
    PHASE_ORDER
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    
    #[test]
    fn test_phase_constants() {
        assert_eq!(FEED, "feed");
        assert_eq!(FILTER, "filter");
        assert_eq!(FORMAT, "format");
        assert_eq!(FUNCTION, "function");
        assert_eq!(FORWARD, "forward");
        assert_eq!(FEEDBACK, "feedback");
    }
    
    #[test]
    fn test_all_phases_contains_all() {
        assert_eq!(ALL_PHASES.len(), 6);
        assert!(ALL_PHASES.contains(&FEED));
        assert!(ALL_PHASES.contains(&FILTER));
        assert!(ALL_PHASES.contains(&FORMAT));
        assert!(ALL_PHASES.contains(&FUNCTION));
        assert!(ALL_PHASES.contains(&FORWARD));
        assert!(ALL_PHASES.contains(&FEEDBACK));
    }
    
    #[test]
    fn test_phase_order() {
        assert_eq!(get_phase_order(FEED), Some(1));
        assert_eq!(get_phase_order(FILTER), Some(2));
        assert_eq!(get_phase_order(FORMAT), Some(3));
        assert_eq!(get_phase_order(FUNCTION), Some(4));
        assert_eq!(get_phase_order(FORWARD), Some(5));
        assert_eq!(get_phase_order(FEEDBACK), Some(6));
        assert_eq!(get_phase_order("invalid"), None);
    }
    
    #[test]
    fn test_phase_validation() {
        assert!(is_valid_phase(FEED));
        assert!(is_valid_phase(FILTER));
        assert!(is_valid_phase(FORMAT));
        assert!(is_valid_phase(FUNCTION));
        assert!(is_valid_phase(FORWARD));
        assert!(is_valid_phase(FEEDBACK));
        assert!(!is_valid_phase("invalid"));
        assert!(!is_valid_phase(""));
    }
    
    #[test]
    fn test_next_phase() {
        assert_eq!(next_phase(FEED), Some(FILTER));
        assert_eq!(next_phase(FILTER), Some(FORMAT));
        assert_eq!(next_phase(FORMAT), Some(FUNCTION));
        assert_eq!(next_phase(FUNCTION), Some(FORWARD));
        assert_eq!(next_phase(FORWARD), Some(FEEDBACK));
        assert_eq!(next_phase(FEEDBACK), None); // Last phase
        assert_eq!(next_phase("invalid"), None);
    }
    
    #[test]
    fn test_previous_phase() {
        assert_eq!(previous_phase(FEED), None); // First phase
        assert_eq!(previous_phase(FILTER), Some(FEED));
        assert_eq!(previous_phase(FORMAT), Some(FILTER));
        assert_eq!(previous_phase(FUNCTION), Some(FORMAT));
        assert_eq!(previous_phase(FORWARD), Some(FUNCTION));
        assert_eq!(previous_phase(FEEDBACK), Some(FORWARD));
        assert_eq!(previous_phase("invalid"), None);
    }
    
    #[test]
    fn test_phase_enum() {
        let phase = Phase::Feed;
        assert_eq!(phase.as_str(), FEED);
        assert_eq!(phase.order(), 1);
        assert!(phase.is_first());
        assert!(!phase.is_last());
        assert_eq!(phase.next(), Some(Phase::Filter));
        assert_eq!(phase.previous(), None);
    }
    
    #[test]
    fn test_phase_enum_all() {
        let phases: Vec<Phase> = Phase::all().collect();
        assert_eq!(phases.len(), 6);
        
        for (i, phase) in phases.iter().enumerate() {
            assert_eq!(phase.order(), i as u8 + 1);
        }
    }
    
    #[test]
    fn test_phase_from_str() {
        assert_eq!(Phase::from_str("feed"), Ok(Phase::Feed));
        assert_eq!(Phase::from_str("filter"), Ok(Phase::Filter));
        assert_eq!(Phase::from_str("format"), Ok(Phase::Format));
        assert_eq!(Phase::from_str("function"), Ok(Phase::Function));
        assert_eq!(Phase::from_str("forward"), Ok(Phase::Forward));
        assert_eq!(Phase::from_str("feedback"), Ok(Phase::Feedback));
        
        assert!(Phase::from_str("invalid").is_err());
        assert!(matches!(
            Phase::from_str("invalid"),
            Err(PhaseParseError::InvalidPhase(_))
        ));
    }
    
    #[test]
    fn test_phase_display() {
        assert_eq!(format!("{}", Phase::Feed), "feed");
        assert_eq!(format!("{}", Phase::Function), "function");
        assert_eq!(format!("{}", Phase::Feedback), "feedback");
    }
    
    #[test]
    fn test_phase_navigation_consistency() {
        // Test that next/previous are consistent
        for phase in Phase::all() {
            if let Some(next) = phase.next() {
                assert_eq!(next.previous(), Some(phase));
            }
            
            if let Some(previous) = phase.previous() {
                assert_eq!(previous.next(), Some(phase));
            }
        }
    }
    
    #[test]
    fn test_get_all_phases() {
        let phases = get_all_phases();
        assert_eq!(phases.len(), 6);
        assert_eq!(phases, &[FEED, FILTER, FORMAT, FUNCTION, FORWARD, FEEDBACK]);
    }
    
    #[test]
    fn test_get_phase_order_map() {
        let order_map = get_phase_order_map();
        assert_eq!(order_map.len(), 6);
        
        for (i, (order, phase)) in order_map.iter().enumerate() {
            assert_eq!(*order, i as u8 + 1);
            assert_eq!(*phase, ALL_PHASES[i]);
        }
    }
}