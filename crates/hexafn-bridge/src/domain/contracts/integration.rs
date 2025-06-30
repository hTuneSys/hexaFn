// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

//! # Integration Contracts
//!
//! This module defines the [`Integration`] trait for external system integrations in the Bridge module.
//!
//! ## Example Usage
//!
//! ```rust
//! use hexafn_bridge::Integration;
//! use hexafn_bridge::ConnectionStatus;
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
//! ## Doc Test: Custom Implementation
//!
//! ```rust
//! use hexafn_bridge::Integration;
//! use hexafn_bridge::ConnectionStatus;
//! use hexafn_core::HexaError;
//!
//! struct CustomIntegration;
//!
//! impl Integration for CustomIntegration {
//!     fn connect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
//!     fn disconnect(&self) -> Result<(), Box<dyn HexaError>> { Ok(()) }
//!     fn is_connected(&self) -> bool { false }
//!     fn get_name(&self) -> &str { "custom" }
//!     fn get_status(&self) -> ConnectionStatus { ConnectionStatus::Disconnected }
//! }
//!
//! let integration = CustomIntegration;
//! assert!(!integration.is_connected());
//! assert_eq!(integration.get_status(), ConnectionStatus::Disconnected);
//! assert_eq!(integration.get_name(), "custom");
//! ```

use crate::ConnectionStatus;
use hexafn_core::HexaError;

/// Trait for external integration contracts.
///
/// This trait defines the required interface for any external system integration
/// (e.g., webhook, API, SaaS connector) in the Bridge module. Implementors must
/// provide connection lifecycle management and status reporting.
///
/// # Example
/// ```rust
/// use hexafn_bridge::Integration;
/// use hexafn_bridge::ConnectionStatus;
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
    /// use hexafn_bridge::Integration;
    /// use hexafn_bridge::ConnectionStatus;
    /// use hexafn_core::HexaError;
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
    /// use hexafn_bridge::Integration;
    /// use hexafn_bridge::ConnectionStatus;
    /// use hexafn_core::HexaError;
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
    /// use hexafn_bridge::Integration;
    /// use hexafn_bridge::ConnectionStatus;
    /// use hexafn_core::HexaError;
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
    /// use hexafn_bridge::Integration;
    /// use hexafn_bridge::ConnectionStatus;
    /// use hexafn_core::HexaError;
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
    /// use hexafn_bridge::Integration;
    /// use hexafn_bridge::ConnectionStatus;
    /// use hexafn_core::HexaError;
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
    use crate::ConnectionStatus;

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
    fn test_integration_status_connected() {
        let int = TestIntegration {
            connected: true,
            name: "test",
        };
        assert!(int.is_connected());
        assert_eq!(int.get_status(), ConnectionStatus::Connected);
        assert_eq!(int.get_name(), "test");
    }

    #[test]
    fn test_integration_status_disconnected() {
        let int = TestIntegration {
            connected: false,
            name: "test",
        };
        assert!(!int.is_connected());
        assert_eq!(int.get_status(), ConnectionStatus::Disconnected);
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

    #[test]
    fn test_trait_object_usage() {
        let int = TestIntegration {
            connected: true,
            name: "traitobj",
        };
        let obj: &dyn Integration = &int;
        assert!(obj.connect().is_ok());
        assert_eq!(obj.get_name(), "traitobj");
        assert_eq!(obj.get_status(), ConnectionStatus::Connected);
    }

    #[test]
    fn test_multiple_integrations() {
        let a = TestIntegration {
            connected: true,
            name: "A",
        };
        let b = TestIntegration {
            connected: false,
            name: "B",
        };
        assert!(a.is_connected());
        assert!(!b.is_connected());
        assert_eq!(a.get_status(), ConnectionStatus::Connected);
        assert_eq!(b.get_status(), ConnectionStatus::Disconnected);
    }
}
