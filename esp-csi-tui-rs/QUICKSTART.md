# Quick Start Guide

## Installation

### Prerequisites
- Rust 1.70+ (you have 1.91.1 ✓)
- Linux (Ubuntu/Debian recommended)

### Build
```bash
cd esp-csi-tui-rs
cargo build --release
```

Build time: ~2-3 minutes on first build.

## Running the Application

### Test Run (Without ESP Device)
```bash
cargo run
```

You should see a TUI interface with:
- Header showing "Disconnected"
- Left panel with device configuration
- Right panel for visualization
- Footer with keyboard shortcuts

### With ESP32 Device

1. **Connect ESP32** via USB

2. **Find serial port**:
```bash
ls /dev/ttyUSB* /dev/ttyACM*
```

3. **Set permissions** (if needed):
```bash
sudo usermod -a -G dialout $USER
# Logout and login again
```

4. **Update configuration** in `src/models/mod.rs`:
```rust
port: "/dev/ttyUSB0".to_string(),  // Your port here
```

5. **Run and connect**:
```bash
cargo run
```
Then press `c` to connect.

## Basic Workflow

### Data Collection
1. Start app: `cargo run`
2. Press `c` - Connect to device
3. Press `s` - Start collection  
4. Watch data accumulate in header
5. Press `t` - Toggle between plots
6. Press `e` - Stop collection
7. Press `q` - Quit

### Data Export
Currently manual (call from code):
```rust
use esp_csi_tui_rs::storage::CsvStorage;

CsvStorage::save(&measurements, Path::new("data.csv"))?;
```

## UI Overview

```
┌────────────────────────────────────────────────┐
│ esp-csi-tui-rs | Connected | Collecting: Yes   │ ← Header
├───────────────┬────────────────────────────────┤
│ Device Config │                                │
│               │    Visualization Area          │
│ Port: /dev..  │                                │ ← Content
│ Baud: 115200  │    (2D Plot / Heatmap / etc)   │
│ Channel: 6    │                                │
│ BW: 20 MHz    │                                │
├───────────────┴────────────────────────────────┤
│ q: Quit | c: Connect | s: Start | e: Stop      │ ← Footer
└────────────────────────────────────────────────┘
```

## Troubleshooting

### "Permission denied" on serial port
```bash
sudo chmod 666 /dev/ttyUSB0  # Quick fix
# Or permanently:
sudo usermod -a -G dialout $USER
```

### Terminal not rendering correctly
- Resize terminal to at least 80x24
- Use a modern terminal (gnome-terminal, kitty, alacritty)
- Ensure UTF-8 encoding

### No data appearing
- Check ESP is programmed with CSI firmware
- Verify AT command format matches your device
- Check baud rate matches device settings
- Look at logs: `RUST_LOG=debug cargo run`

## Development Workflow

### Running with logs
```bash
RUST_LOG=debug cargo run 2> debug.log
# Check debug.log in another terminal
```

### Quick rebuild
```bash
cargo check  # Fast syntax check
cargo build  # Full build
```

### Testing changes
```bash
cargo test   # Run tests
cargo clippy # Lint
cargo fmt    # Format code
```

## Next Steps

1. **Customize Device Protocol**: Edit `src/device/esp_client.rs` to match your ESP firmware
2. **Add Visualizations**: Implement actual plot rendering in `src/ui/components/plots.rs`
3. **Integrate Rerun.io**: Complete streaming in `src/streaming/rerun_client.rs`
4. **Add Export UI**: Create menu for CSV/RRD export
5. **Implement Camera**: Add bonus video streaming feature

## Resources

- **Code**: All modules in `src/`
- **Docs**: `README.md`, `ARCHITECTURE.md`
- **Examples**: Check `src/main.rs` for entry point
- **Logs**: Enable with `RUST_LOG=trace`

## Getting Help

Check logs first:
```bash
RUST_LOG=trace cargo run 2>&1 | tee app.log
```

Common issues:
- Serial errors → Check device connection
- UI frozen → Press `q` to quit gracefully
- Build errors → `cargo clean && cargo build`

## Performance Tips

- Release build for production: `cargo build --release`
- Limit measurement buffer size to prevent memory growth
- Use `--release` for smooth 60 Hz rendering

## What Works Now

✅ TUI rendering  
✅ Keyboard navigation  
✅ App state management  
✅ Module structure  
✅ Serial port handling  
✅ Basic CSI data models  

## What Needs Implementation

⚠️ ESP AT command protocol (customize to your device)  
⚠️ Actual plot rendering (integrate Plotly or tui-widgets)  
⚠️ Rerun.io HTTP API calls  
⚠️ CSV/RRD file writing (structure ready)  
⚠️ Camera streaming (bonus feature)  

## Success Criteria

You've successfully set up the project when:
1. `cargo build` completes without errors ✓
2. `cargo run` shows the TUI interface ✓
3. Keyboard controls are responsive ✓
4. Can connect to serial device (with ESP connected)

Happy hacking! 🚀
