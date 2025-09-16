//! Global Hotkey Unifier
//!
//! A cross-platform global hotkey unifier for consistent hotkey management across different
//! applications and operating systems.
//!
//! # Examples
//!
//! ```rust
//! use global_hotkey_unifier::{HotkeyManager, Hotkey, Modifier};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut manager = HotkeyManager::new()?;
//!
//! // Create a global hotkey (Ctrl+Shift+H)
//! let hotkey = Hotkey::new(&[Modifier::Ctrl, Modifier::Shift], 'H')?;
//! 
//! // Note: Registration will return NotImplemented error in current version
//! match manager.register(hotkey, |_| {
//!     println!("Global hotkey triggered!");
//! }) {
//!     Ok(_) => println!("Hotkey registered"),
//!     Err(_) => println!("Registration not yet implemented"),
//! }
//! # Ok(())
//! # }
//! ```

use std::error::Error;
use std::fmt;

/// Represents keyboard modifiers for hotkeys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modifier {
    /// Control key (Ctrl on Windows/Linux, Cmd on macOS)
    Ctrl,
    /// Alt key (Alt on Windows/Linux, Option on macOS)
    Alt,
    /// Shift key
    Shift,
    /// Windows key on Windows, Super key on Linux, ignored on macOS
    Super,
}

/// Represents a hotkey combination
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hotkey {
    modifiers: Vec<Modifier>,
    key: char,
}

impl Hotkey {
    /// Create a new hotkey with the specified modifiers and key
    pub fn new(modifiers: &[Modifier], key: char) -> Result<Self, HotkeyError> {
        if modifiers.is_empty() {
            return Err(HotkeyError::InvalidHotkey("At least one modifier is required".to_string()));
        }
        
        Ok(Hotkey {
            modifiers: modifiers.to_vec(),
            key,
        })
    }

    /// Get the modifiers for this hotkey
    pub fn modifiers(&self) -> &[Modifier] {
        &self.modifiers
    }

    /// Get the key for this hotkey
    pub fn key(&self) -> char {
        self.key
    }
}

/// Manages global hotkey registration and event handling
pub struct HotkeyManager {
    // Platform-specific implementation will be added here
}

impl HotkeyManager {
    /// Create a new hotkey manager
    pub fn new() -> Result<Self, HotkeyError> {
        Ok(HotkeyManager {})
    }

    /// Register a global hotkey with a callback
    pub fn register<F>(&mut self, _hotkey: Hotkey, _callback: F) -> Result<(), HotkeyError>
    where
        F: Fn(&Hotkey) + Send + 'static,
    {
        // Platform-specific implementation will be added here
        Err(HotkeyError::NotImplemented("Hotkey registration not yet implemented".to_string()))
    }

    /// Start listening for hotkey events
    pub fn listen(&self) -> Result<(), HotkeyError> {
        // Platform-specific implementation will be added here
        Err(HotkeyError::NotImplemented("Event listening not yet implemented".to_string()))
    }

    /// Unregister a hotkey
    pub fn unregister(&mut self, _hotkey: &Hotkey) -> Result<(), HotkeyError> {
        // Platform-specific implementation will be added here
        Err(HotkeyError::NotImplemented("Hotkey unregistration not yet implemented".to_string()))
    }
}

/// Error types for the global hotkey unifier
#[derive(Debug)]
pub enum HotkeyError {
    /// Invalid hotkey configuration
    InvalidHotkey(String),
    /// Platform-specific error
    PlatformError(String),
    /// Feature not yet implemented
    NotImplemented(String),
    /// Hotkey already registered
    AlreadyRegistered(String),
    /// Hotkey not found
    NotFound(String),
}

impl fmt::Display for HotkeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HotkeyError::InvalidHotkey(msg) => write!(f, "Invalid hotkey: {}", msg),
            HotkeyError::PlatformError(msg) => write!(f, "Platform error: {}", msg),
            HotkeyError::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
            HotkeyError::AlreadyRegistered(msg) => write!(f, "Already registered: {}", msg),
            HotkeyError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl Error for HotkeyError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hotkey_creation() {
        let hotkey = Hotkey::new(&[Modifier::Ctrl, Modifier::Shift], 'H').unwrap();
        assert_eq!(hotkey.modifiers(), &[Modifier::Ctrl, Modifier::Shift]);
        assert_eq!(hotkey.key(), 'H');
    }

    #[test]
    fn test_hotkey_requires_modifier() {
        let result = Hotkey::new(&[], 'H');
        assert!(result.is_err());
    }

    #[test]
    fn test_manager_creation() {
        let manager = HotkeyManager::new();
        assert!(manager.is_ok());
    }
}