# Global Hotkey Unifier

A cross-platform global hotkey unifier for consistent hotkey management across different applications and operating systems.

## Overview

Global Hotkey Unifier provides a unified interface for registering and managing global hotkeys that work consistently across Windows, macOS, and Linux. It allows applications to register system-wide keyboard shortcuts that can be triggered regardless of which application currently has focus.

## Features

- **Cross-platform support**: Works on Windows, macOS, and Linux
- **Global hotkey registration**: Register system-wide keyboard shortcuts
- **Event-driven architecture**: Asynchronous hotkey event handling
- **Conflict detection**: Detect and handle hotkey conflicts
- **Configuration management**: Save and load hotkey configurations
- **Application integration**: Easy integration with existing applications

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
global-hotkey-unifier = "0.1.0"
```

## Quick Start

```rust
use global_hotkey_unifier::{HotkeyManager, Hotkey, Modifier};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = HotkeyManager::new()?;
    
    // Register a global hotkey (Ctrl+Shift+H)
    let hotkey = Hotkey::new(&[Modifier::Ctrl, Modifier::Shift], 'H')?;
    manager.register(hotkey, |_| {
        println!("Global hotkey triggered!");
    })?;
    
    // Start listening for hotkey events
    manager.listen()?;
    
    Ok(())
}
```

## Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| Windows  | ✅ Planned | Using Windows API |
| macOS    | ✅ Planned | Using Carbon/Cocoa APIs |
| Linux    | ✅ Planned | Using X11/Wayland |

## Building

To build the project:

```bash
cargo build
```

To run tests:

```bash
cargo test
```

To run the example:

```bash
cargo run
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the need for consistent global hotkey management across platforms
- Built with Rust for performance and safety