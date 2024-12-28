# Display-Aware Screenshot Tool 📸

A professional Rust-based screenshot utility that intelligently handles display scaling and rotation. This tool accurately captures screen content by accounting for system DPI settings and additional scaling factors.

## Features ✨

- **Smart Display Scaling** 🔍

  - Automatic DPI scaling detection
  - Dynamic scaling factor determination
  - Handles Windows display scaling settings

- **Professional Output** 💼

  - High-quality screenshots with correct dimensions
  - Detailed metadata in filenames
  - Beautiful console output with display information

- **Robust Error Handling** 🛡️
  - Graceful fallbacks for scaling detection
  - Clear error messages
  - Type-safe operations

## Usage 🚀

### Basic Screenshot

```rust
use screenshots::Screen;

fn main() -> Result<(), Box<dyn Error>> {
    // Get all displays
    let displays = DisplayAwareCapture::all_displays()?;

    // Capture from the first display
    let capture = &displays[0];

    // Take a 500x500 screenshot at position (0,0)
    let image = capture.capture_scaled_area(0, 0, 500, 500)?;

    // Save with metadata
    capture.save_screenshot(&image, "my_screenshot", 500, "output")?;

    Ok(())
}
```

### Display Information

```rust
fn main() -> Result<(), Box<dyn Error>> {
    let displays = DisplayAwareCapture::all_displays()?;

    for (i, display) in displays.iter().enumerate() {
        display.print_display_info(i);
    }

    Ok(())
}
```

## Example Output 🖥️

```
📺 Display #1
├─ 🆔 ID: \\.\DISPLAY1
├─ 📍 Position: (0, 0)
├─ 🖥️  Resolution
│  ├─ Logical: 1536x864
│  └─ Physical: 1920x1080
├─ 📏 Scaling
│  ├─ DPI Scale: 1.25x (125%)
│  ├─ Extra Scale: 1.56x
│  └─ Total Scale: 1.95x (195%)
├─ 🔄 Rotation: 0°
└─ 🎯 Primary: Yes
```

## Development 🛠️

### Requirements

- Rust 1.70 or higher
- Windows OS (for display scaling features)

### Dependencies

- `screenshots`: Screen capture functionality
- `anyhow`: Error handling
- `proptest`: Property-based testing (optional)

### Testing

Run the standard test suite:

```bash
cargo test --example display_info
```

Run with property-based tests:

```bash
cargo test --example display_info --features proptest
```

## Architecture 🏗️

The project is built around two main structs:

1. `ScalingConfig`: Handles scaling calculations and factor determination

   - Dynamically determines actual scaling factors
   - Manages DPI and total scaling values
   - Provides coordinate and dimension scaling utilities

2. `DisplayAwareCapture`: Main capture interface
   - Integrates with the system display API
   - Manages screenshot capture and saving
   - Provides detailed display information

## Contributing 🤝

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License 📄

This project is licensed under the MIT License - see the LICENSE file for details.
