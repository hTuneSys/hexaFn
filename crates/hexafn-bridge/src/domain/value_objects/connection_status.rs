// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # ConnectionStatus Value Object
//!
//! This module defines the [`ConnectionStatus`] enum, which represents the connection state of an external integration in the Bridge module.
//!
//! ## Usage
//!
//! The enum is used by implementations of the [`crate::domain::contracts::Integration`] trait to indicate their current state for diagnostics, health checks, and orchestration.
//!
//! ## Example
//!
//! ```rust
//! use hexafn_bridge::ConnectionStatus;
//!
//! fn print_status(status: ConnectionStatus) {
//!     match status {
//!         ConnectionStatus::Connected => println!("Connected!"),
//!         ConnectionStatus::Disconnected => println!("Disconnected!"),
//!         ConnectionStatus::Error => println!("Error state!"),
//!     }
//! }
//!
//! let status = ConnectionStatus::Connected;
//! print_status(status);
//! ```
/// Represents the connection status of an external integration.
///
/// This enum is used to indicate the current state of an integration implementing the [`crate::domain::contracts::Integration`] trait.
/// It is useful for diagnostics, health checks, and orchestrating integration lifecycles.
///
/// # Variants
/// - [`ConnectionStatus::Connected`]: The integration is currently connected and operational.
/// - [`ConnectionStatus::Disconnected`]: The integration is not connected.
/// - [`ConnectionStatus::Error`]: The integration is in an error state (e.g., failed to connect or lost connection).
///
/// # Example
///
/// ```rust
/// use hexafn_bridge::ConnectionStatus;
///
/// fn print_status(status: ConnectionStatus) {
///     match status {
///         ConnectionStatus::Connected => println!("Connected!"),
///         ConnectionStatus::Disconnected => println!("Disconnected!"),
///         ConnectionStatus::Error => println!("Error state!"),
///     }
/// }
///
/// let status = ConnectionStatus::Connected;
/// print_status(status);
/// ```
///
/// # Doc Test: Pattern Matching
///
/// ```rust
/// use hexafn_bridge::ConnectionStatus;
///
/// fn status_message(status: ConnectionStatus) -> &'static str {
///     match status {
///         ConnectionStatus::Connected => "Integration is up.",
///         ConnectionStatus::Disconnected => "Integration is down.",
///         ConnectionStatus::Error => "Integration error!",
///     }
/// }
/// assert_eq!(status_message(ConnectionStatus::Connected), "Integration is up.");
/// assert_eq!(status_message(ConnectionStatus::Disconnected), "Integration is down.");
/// assert_eq!(status_message(ConnectionStatus::Error), "Integration error!");
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConnectionStatus {
    /// The integration is currently connected.
    Connected,
    /// The integration is currently disconnected.
    Disconnected,
    /// The integration is in an error state.
    Error,
}

impl std::fmt::Display for ConnectionStatus {
    /// Formats the connection status as a user-friendly string.
    ///
    /// # Example
    /// ```rust
    /// use hexafn_bridge::ConnectionStatus;
    /// assert_eq!(ConnectionStatus::Connected.to_string(), "Connected");
    /// assert_eq!(ConnectionStatus::Disconnected.to_string(), "Disconnected");
    /// assert_eq!(ConnectionStatus::Error.to_string(), "Error");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionStatus::Connected => write!(f, "Connected"),
            ConnectionStatus::Disconnected => write!(f, "Disconnected"),
            ConnectionStatus::Error => write!(f, "Error"),
        }
    }
}

impl ConnectionStatus {
    /// Returns true if the status is [`ConnectionStatus::Connected`].
    ///
    /// # Example
    /// ```rust
    /// use hexafn_bridge::ConnectionStatus;
    /// assert!(ConnectionStatus::Connected.is_connected());
    /// assert!(!ConnectionStatus::Disconnected.is_connected());
    /// assert!(!ConnectionStatus::Error.is_connected());
    /// ```
    pub fn is_connected(&self) -> bool {
        matches!(self, ConnectionStatus::Connected)
    }

    /// Returns true if the status is [`ConnectionStatus::Disconnected`].
    ///
    /// # Example
    /// ```rust
    /// use hexafn_bridge::ConnectionStatus;
    /// assert!(ConnectionStatus::Disconnected.is_disconnected());
    /// assert!(!ConnectionStatus::Connected.is_disconnected());
    /// assert!(!ConnectionStatus::Error.is_disconnected());
    /// ```
    pub fn is_disconnected(&self) -> bool {
        matches!(self, ConnectionStatus::Disconnected)
    }

    /// Returns true if the status is [`ConnectionStatus::Error`].
    ///
    /// # Example
    /// ```rust
    /// use hexafn_bridge::ConnectionStatus;
    /// assert!(ConnectionStatus::Error.is_error());
    /// assert!(!ConnectionStatus::Connected.is_error());
    /// assert!(!ConnectionStatus::Disconnected.is_error());
    /// ```
    pub fn is_error(&self) -> bool {
        matches!(self, ConnectionStatus::Error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_for_connection_status() {
        assert_eq!(ConnectionStatus::Connected.to_string(), "Connected");
        assert_eq!(ConnectionStatus::Disconnected.to_string(), "Disconnected");
        assert_eq!(ConnectionStatus::Error.to_string(), "Error");
    }

    #[test]
    fn test_equality_and_copy() {
        let a = ConnectionStatus::Connected;
        let b = a; // Copy, not clone
        assert_eq!(a, b);
        let c = ConnectionStatus::Disconnected;
        assert_ne!(a, c);
    }

    #[test]
    fn test_match_usage() {
        fn status_str(status: ConnectionStatus) -> &'static str {
            match status {
                ConnectionStatus::Connected => "ok",
                ConnectionStatus::Disconnected => "no",
                ConnectionStatus::Error => "err",
            }
        }
        assert_eq!(status_str(ConnectionStatus::Connected), "ok");
        assert_eq!(status_str(ConnectionStatus::Disconnected), "no");
        assert_eq!(status_str(ConnectionStatus::Error), "err");
    }

    #[test]
    fn test_all_variants_are_covered() {
        // This test ensures all enum variants are handled and can be iterated if needed.
        let statuses = [
            ConnectionStatus::Connected,
            ConnectionStatus::Disconnected,
            ConnectionStatus::Error,
        ];
        for status in statuses.iter() {
            let s = status.to_string();
            assert!(s == "Connected" || s == "Disconnected" || s == "Error");
        }
    }

    #[test]
    fn test_debug_trait() {
        let status = ConnectionStatus::Connected;
        let debug_str = format!("{:?}", status);
        assert_eq!(debug_str, "Connected");
    }
}
