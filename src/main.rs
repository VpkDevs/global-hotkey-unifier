use global_hotkey_unifier::{HotkeyManager, Hotkey, Modifier};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Global Hotkey Unifier v{}", env!("CARGO_PKG_VERSION"));
    println!("A cross-platform global hotkey unifier\n");

    // Create a new hotkey manager
    let mut manager = HotkeyManager::new()?;
    println!("✓ Hotkey manager initialized");

    // Try to create a sample hotkey
    match Hotkey::new(&[Modifier::Ctrl, Modifier::Shift], 'H') {
        Ok(hotkey) => {
            println!("✓ Created sample hotkey: Ctrl+Shift+H");
            println!("  Modifiers: {:?}", hotkey.modifiers());
            println!("  Key: {}", hotkey.key());
            
            // Try to register the hotkey (will fail with NotImplemented for now)
            match manager.register(hotkey, |h| {
                println!("Hotkey triggered: {:?}", h);
            }) {
                Ok(_) => println!("✓ Hotkey registered successfully"),
                Err(e) => println!("⚠ Hotkey registration: {}", e),
            }
        }
        Err(e) => {
            println!("✗ Failed to create hotkey: {}", e);
        }
    }

    println!("\nNote: Platform-specific implementations are not yet complete.");
    println!("This is the basic project structure for future development.");

    Ok(())
}
