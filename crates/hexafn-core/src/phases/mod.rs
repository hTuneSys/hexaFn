// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # 6F Lifecycle Flow Phases Module
//! 
//! This module provides the core phase definitions and utilities for the hexaFn 6F Lifecycle Flow:
//! Feed → Filter → Format → Function → Forward → Feedback

pub mod lifecycle;
pub mod context;
pub mod result;

// Re-export commonly used items for convenience
pub use lifecycle::*;
pub use context::PhaseContext;
pub use result::PhaseResult;

/// Module version for compatibility tracking
pub const PHASES_MODULE_VERSION: &str = "0.1.0";

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_module_integration() {
        // Test that all re-exports work
        let _phases = ALL_PHASES;
        let _context = PhaseContext::new(FEED);
        let _result = PhaseResult::Success;
        
        // Test phase navigation
        assert_eq!(next_phase(FEED), Some(FILTER));
        assert_eq!(previous_phase(FILTER), Some(FEED));
        
        // Test context integration
        let context = PhaseContext::new(FORMAT)
            .with_correlation_id("test-123")
            .with_metadata("source", "test");
        
        assert_eq!(context.phase, FORMAT);
        assert_eq!(context.correlation_id, Some("test-123".to_string()));
        assert!(context.metadata.contains_key("source"));
    }
    
    #[test]
    fn test_all_6f_phases_integration() {
        // Ensure all 6F phases work together
        let phases = [FEED, FILTER, FORMAT, FUNCTION, FORWARD, FEEDBACK];
        
        for (i, phase) in phases.iter().enumerate() {
            let context = PhaseContext::new(phase);
            assert_eq!(context.order, i as u8 + 1);
            
            if i > 0 {
                assert_eq!(context.previous_phase(), Some(phases[i - 1]));
            } else {
                assert_eq!(context.previous_phase(), None);
            }
            
            if i < phases.len() - 1 {
                assert_eq!(context.next_phase(), Some(phases[i + 1]));
            } else {
                assert_eq!(context.next_phase(), None);
            }
        }
    }
}