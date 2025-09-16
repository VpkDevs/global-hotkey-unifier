use global_hotkey_unifier::{HotkeyManager, Hotkey, Modifier};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Global Hotkey Unifier - Basic Example");
    println!("=====================================\n");

    // Create a hotkey manager
    let mut manager = HotkeyManager::new()?;
    println!("Created hotkey manager");

    // Create some example hotkeys
    let hotkeys = vec![
        Hotkey::new(&[Modifier::Ctrl, Modifier::Shift], 'A')?,
        Hotkey::new(&[Modifier::Alt], '1')?, // Using '1' instead of 'F1' for now
        Hotkey::new(&[Modifier::Ctrl, Modifier::Alt], 'D')?,
    ];

    // Try to register each hotkey
    for (i, hotkey) in hotkeys.iter().enumerate() {
        println!("Hotkey {}: {:?}+{}", i + 1, hotkey.modifiers(), hotkey.key());
        
        match manager.register(hotkey.clone(), move |h| {
            println!("Hotkey {} triggered: {:?}+{}", i + 1, h.modifiers(), h.key());
        }) {
            Ok(_) => println!("  ✓ Registered successfully"),
            Err(e) => println!("  ⚠ Registration failed: {}", e),
        }
    }

    println!("\nNote: This example demonstrates the API structure.");
    println!("Platform-specific implementations will be added in future versions.");

    Ok(())
}