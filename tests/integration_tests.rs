use global_hotkey_unifier::{HotkeyManager, Hotkey, Modifier, HotkeyError};

#[test]
fn test_hotkey_manager_lifecycle() {
    // Test creating a manager
    let manager = HotkeyManager::new();
    assert!(manager.is_ok());
}

#[test]
fn test_hotkey_creation_with_different_modifiers() {
    // Test various modifier combinations
    let hotkey1 = Hotkey::new(&[Modifier::Ctrl], 'A');
    assert!(hotkey1.is_ok());

    let hotkey2 = Hotkey::new(&[Modifier::Alt, Modifier::Shift], 'B');
    assert!(hotkey2.is_ok());

    let hotkey3 = Hotkey::new(&[Modifier::Ctrl, Modifier::Alt, Modifier::Shift], 'C');
    assert!(hotkey3.is_ok());

    // Test that empty modifiers fail
    let hotkey_empty = Hotkey::new(&[], 'D');
    assert!(hotkey_empty.is_err());
    
    if let Err(HotkeyError::InvalidHotkey(_)) = hotkey_empty {
        // Expected error type
    } else {
        panic!("Expected InvalidHotkey error");
    }
}

#[test]
fn test_hotkey_properties() {
    let modifiers = vec![Modifier::Ctrl, Modifier::Shift];
    let key = 'H';
    
    let hotkey = Hotkey::new(&modifiers, key).unwrap();
    
    assert_eq!(hotkey.modifiers(), &modifiers);
    assert_eq!(hotkey.key(), key);
}

#[test]
fn test_registration_returns_not_implemented() {
    let mut manager = HotkeyManager::new().unwrap();
    let hotkey = Hotkey::new(&[Modifier::Ctrl], 'T').unwrap();
    
    let result = manager.register(hotkey, |_| {});
    assert!(result.is_err());
    
    if let Err(HotkeyError::NotImplemented(_)) = result {
        // Expected error type
    } else {
        panic!("Expected NotImplemented error");
    }
}