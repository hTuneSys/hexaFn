// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Integration Contracts
//!
//! This module defines the [`Integration`] trait and the [`ConnectionStatus`] enum for external system integrations in the Bridge module.
//!
//! ## Example Usage
//!
//! ```rust
//! use hexafn_bridge::domain::contracts::{Integration, ConnectionStatus};
//! use hexafn_core::HexaError;
//!
//! struct ExampleIntegration;
//!
//! impl Integration for ExampleIntegration {
//!     fn connect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
//!     fn disconnect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
//!     fn is_connected(&self) -> bool { true }
//!     fn get_name(&self) -> &str { "example" }
//!     fn get_status(&self) -> ConnectionStatus { ConnectionStatus::Connected }
//! }
//!
//! let integration = ExampleIntegration;
//! assert_eq!(integration.is_connected(), true);
//! assert_eq!(integration.get_status(), ConnectionStatus::Connected);
//! assert_eq!(integration.get_name(), "example");
//! ```
//!
//! ## Test
//!
//! ```rust
//! use hexafn_bridge::domain::contracts::{Integration, ConnectionStatus};
//! use hexafn_core::HexaError;
//!
//! struct TestIntegration { connected: bool }
//!
//! impl Integration for TestIntegration {
//!     fn connect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
//!     fn disconnect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
//!     fn is_connected(&self) -> bool { self.connected }
//!     fn get_name(&self) -> &str { "test" }
//!     fn get_status(&self) -> ConnectionStatus {
//!         if self.connected { ConnectionStatus::Connected } else { ConnectionStatus::Disconnected }
//!     }
//! }
//!
//! let int = TestIntegration { connected: true };
//! assert_eq!(int.is_connected(), true);
//! assert_eq!(int.get_status(), ConnectionStatus::Connected);
//! ```
//!
//! ## Test
//!
//! ```rust
//! use hexafn_bridge::domain::contracts::{Integration, ConnectionStatus};
//! use hexafn_core::HexaError;
//!
//! struct TestIntegration { connected: bool }
//!
//! impl Integration for TestIntegration {
//!     fn connect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
//!     fn disconnect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
//!     fn is_connected(&self) -> bool { self.connected }
//!     fn get_name(&self) -> &str { "test" }
//!     fn get_status(&self) -> ConnectionStatus {
//!         if self.connected { ConnectionStatus::Connected } else { ConnectionStatus::Disconnected }
//!     }
//! }
//!
//! let int = TestIntegration { connected: false };
//! assert_eq!(int.is_connected(), false);
//! assert_eq!(int.get_status(), ConnectionStatus::Disconnected);
//! ```

use hexafn_core::HexaError;

/// Represents the connection status of an external integration.
///
/// This enum is used to indicate the current state of an integration implementing the [`Integration`] trait.
/// It is useful for diagnostics, health checks, and orchestrating integration lifecycles.
///
/// # Variants
/// - [`Connected`]: The integration is currently connected and operational.
/// - [`Disconnected`]: The integration is not connected.
/// - [`Error`]: The integration is in an error state (e.g., failed to connect or lost connection).
///
/// # Example
///
/// ```rust
/// use hexafn_bridge::domain::contracts::ConnectionStatus;
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionStatus::Connected => write!(f, "Connected"),
            ConnectionStatus::Disconnected => write!(f, "Disconnected"),
            ConnectionStatus::Error => write!(f, "Error"),
        }
    }
}

/// Trait for external integration contracts.
///
/// This trait defines the required interface for any external system integration
/// (e.g., webhook, API, SaaS connector) in the Bridge module. Implementors must
/// provide connection lifecycle management and status reporting.
///
/// # Example
/// ```rust
/// use hexafn_bridge::domain::contracts::{Integration, ConnectionStatus};
/// use hexafn_core::HexaError;
///
/// struct DummyIntegration;
///
/// impl Integration for DummyIntegration {
///     fn connect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
///     fn disconnect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
///     fn is_connected(&self) -> bool { true }
///     fn get_name(&self) -> &str { "dummy" }
///     fn get_status(&self) -> ConnectionStatus { ConnectionStatus::Connected }
/// }
///
/// let integration = DummyIntegration;
/// assert_eq!(integration.is_connected(), true);
/// assert_eq!(integration.get_status(), ConnectionStatus::Connected);
/// assert_eq!(integration.get_name(), "dummy");
/// ```
pub trait Integration {
    /// Establishes a connection to the external system.
    ///
    /// # Example
    /// ```rust
    /// # use hexafn_bridge::domain::contracts::{Integration, ConnectionStatus};
    /// # use hexafn_core::HexaError;
    /// struct MyIntegration;
    /// impl Integration for MyIntegration {
    ///     fn connect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    ///     fn disconnect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    ///     fn is_connected(&self) -> bool { true }
    ///     fn get_name(&self) -> &str { "my" }
    ///     fn get_status(&self) -> ConnectionStatus { ConnectionStatus::Connected }
    /// }
    /// let i = MyIntegration;
    /// assert!(i.connect().is_ok());
    /// ```
    ///
    /// # Errors
    /// Returns a [`HexaError`] if the connection attempt fails.
    fn connect(&self) -> Result<(), Box<dyn HexaError>>;

    /// Disconnects from the external system.
    ///
    /// # Example
    /// ```rust
    /// # use hexafn_bridge::domain::contracts::{Integration, ConnectionStatus};
    /// # use hexafn_core::HexaError;
    /// struct MyIntegration;
    /// impl Integration for MyIntegration {
    ///     fn connect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    ///     fn disconnect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    ///     fn is_connected(&self) -> bool { false }
    ///     fn get_name(&self) -> &str { "my" }
    ///     fn get_status(&self) -> ConnectionStatus { ConnectionStatus::Disconnected }
    /// }
    /// let i = MyIntegration;
    /// assert!(i.disconnect().is_ok());
    /// ```
    ///
    /// # Errors
    /// Returns a [`HexaError`] if the disconnection attempt fails.
    fn disconnect(&self) -> Result<(), Box<dyn HexaError>>;

    /// Returns true if the integration is currently connected.
    ///
    /// # Example
    /// ```rust
    /// # use hexafn_bridge::domain::contracts::{Integration, ConnectionStatus};
    /// # use hexafn_core::HexaError;
    /// struct MyIntegration;
    /// impl Integration for MyIntegration {
    ///     fn connect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    ///     fn disconnect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    ///     fn is_connected(&self) -> bool { true }
    ///     fn get_name(&self) -> &str { "my" }
    ///     fn get_status(&self) -> ConnectionStatus { ConnectionStatus::Connected }
    /// }
    /// let i = MyIntegration;
    /// assert_eq!(i.is_connected(), true);
    /// ```
    fn is_connected(&self) -> bool;

    /// Returns the name of the integration (for diagnostics/logging).
    ///
    /// # Example
    /// ```rust
    /// # use hexafn_bridge::domain::contracts::{Integration, ConnectionStatus};
    /// # use hexafn_core::HexaError;
    /// struct MyIntegration;
    /// impl Integration for MyIntegration {
    ///     fn connect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    ///     fn disconnect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    ///     fn is_connected(&self) -> bool { false }
    ///     fn get_name(&self) -> &str { "my" }
    ///     fn get_status(&self) -> ConnectionStatus { ConnectionStatus::Disconnected }
    /// }
    /// let i = MyIntegration;
    /// assert_eq!(i.get_name(), "my");
    /// ```
    fn get_name(&self) -> &str;

    /// Returns the current [`ConnectionStatus`] of the integration.
    ///
    /// # Example
    /// ```rust
    /// # use hexafn_bridge::domain::contracts::{Integration, ConnectionStatus};
    /// # use hexafn_core::HexaError;
    /// struct MyIntegration;
    /// impl Integration for MyIntegration {
    ///     fn connect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    ///     fn disconnect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
    ///     fn is_connected(&self) -> bool { false }
    ///     fn get_name(&self) -> &str { "my" }
    ///     fn get_status(&self) -> ConnectionStatus { ConnectionStatus::Disconnected }
    /// }
    /// let i = MyIntegration;
    /// assert_eq!(i.get_status(), ConnectionStatus::Disconnected);
    /// ```
    fn get_status(&self) -> ConnectionStatus;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestIntegration {
        connected: bool,
        name: &'static str,
    }

    impl Integration for TestIntegration {
        fn connect(&self) -> Result<(), Box<dyn HexaError>> {
            Ok(())
        }
        fn disconnect(&self) -> Result<(), Box<dyn HexaError>> {
            Ok(())
        }
        fn is_connected(&self) -> bool {
            self.connected
        }
        fn get_name(&self) -> &str {
            self.name
        }
        fn get_status(&self) -> ConnectionStatus {
            if self.connected {
                ConnectionStatus::Connected
            } else {
                ConnectionStatus::Disconnected
            }
        }
    }

    #[test]
    fn test_integration_status() {
        let int = TestIntegration {
            connected: true,
            name: "test",
        };
        assert!(int.is_connected());
        assert_eq!(int.get_status(), ConnectionStatus::Connected);
        assert_eq!(int.get_name(), "test");
    }

    #[test]
    fn test_integration_disconnect() {
        let int = TestIntegration {
            connected: false,
            name: "test",
        };
        assert!(!int.is_connected());
        assert_eq!(int.get_status(), ConnectionStatus::Disconnected);
    }

    #[test]
    fn test_display_for_connection_status() {
        assert_eq!(ConnectionStatus::Connected.to_string(), "Connected");
        assert_eq!(ConnectionStatus::Disconnected.to_string(), "Disconnected");
        assert_eq!(ConnectionStatus::Error.to_string(), "Error");
    }

    #[test]
    fn test_trait_methods_are_callable() {
        let int = TestIntegration {
            connected: true,
            name: "integration",
        };
        assert!(int.connect().is_ok());
        assert!(int.disconnect().is_ok());
        assert_eq!(int.get_name(), "integration");
        assert_eq!(int.get_status(), ConnectionStatus::Connected);
    }

    #[test]
    fn test_trait_methods_with_different_names() {
        let int = TestIntegration {
            connected: false,
            name: "other",
        };
        assert_eq!(int.get_name(), "other");
        assert_eq!(int.get_status(), ConnectionStatus::Disconnected);
    }
}
