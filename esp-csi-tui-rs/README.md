# esp-csi-tui-rs

A Terminal User Interface (TUI) application for WiFi CSI (Channel State Information) data collection and visualization, built in Rust using the Ratatui framework.

## Overview

This project implements a complete TUI-based solution for collecting, visualizing, and streaming CSI data from ESP32 devices. It addresses the limitations of existing C/C++ frameworks by leveraging Rust's memory safety and performance characteristics.

## Features

### Core Features
- **Device Interaction**: Serial communication with ESP32 devices for CSI collection
- **Real-time Data Visualization**: 
  - 2D Magnitude and Phase plots
  - Heatmap visualization
  - Color Domain representation
  - Time-series plots
- **Data Storage**:
  - CSV format for raw data export
  - RRD format for Rerun.io viewer compatibility
- **Remote Live Streaming**: Send CSI data to Rerun.io instances
- **Interactive TUI**: Full keyboard-driven interface

### Bonus Feature
- Camera video streaming support (infrastructure ready)

## Project Structure

```
src/
├── main.rs                 # Application entry point
├── models/                 # Data structures
│   └── mod.rs            # CSI measurements, device config, app state
├── ui/                    # Terminal User Interface
│   ├── app.rs           # Main app logic and event handling
│   ├── terminal.rs      # Terminal setup/cleanup
│   └── components/      # UI components
│       ├── layout.rs    # Layout definitions
│       ├── plots.rs     # Plot rendering
│       └── status.rs    # Header/footer widgets
├── device/              # ESP device communication
│   ├── serial_handler.rs # Serial port communication
│   └── esp_client.rs    # ESP32-specific protocol
├── visualization/       # Data processing & visualization
│   ├── renderer.rs      # Plot rendering with Plotly
│   └── charts.rs        # Chart configuration
├── storage/             # Data persistence
│   ├── csv_storage.rs   # CSV export
│   └── rrd_storage.rs   # Rerun.io format
├── streaming/           # Live data streaming
│   └── rerun_client.rs  # Rerun.io integration
└── commands/            # Command execution
    └── mod.rs
```

## Dependencies

- **ratatui**: Terminal UI framework
- **crossterm**: Cross-platform terminal manipulation
- **tokio**: Async runtime
- **serialport**: Serial port communication
- **plotly**: Plotting library
- **serde/serde_json**: Serialization
- **csv**: CSV handling
- **chrono**: Time handling
- **tracing**: Logging

## Building & Running

### Build
```bash
cd esp-csi-tui-rs
cargo build --release
```

### Run
```bash
cargo run
```

## Keyboard Controls

| Key | Action |
|-----|--------|
| `q` | Quit application |
| `c` | Connect to device |
| `s` | Start data collection |
| `e` | Stop data collection |
| `t` | Toggle between visualization tabs |
| `Esc` | Return to main view |
| `h` | Show help |

## Usage Workflow

1. **Connect**: Press `c` to connect to ESP32 device
2. **Configure**: Set WiFi channel, bandwidth, and collection interval
3. **Collect**: Press `s` to start collecting CSI data
4. **Visualize**: Use `t` to switch between different plot types
5. **Export**: Save data as CSV or RRD format
6. **Stream**: Optionally stream to Rerun.io for remote analysis

## Data Formats

### CSV Format
Exports subcarrier-level data with timestamps:
- timestamp, channel, bandwidth, rssi, noise_floor
- subcarrier_index, real, imag, magnitude, phase

### RRD Format
JSON-based format compatible with Rerun.io viewer for playback and analysis

## ESP32 AT Commands

The application uses standard AT commands to control the ESP device:
- `AT` - Handshake
- `AT+CSICFG=<channel>,<bw>,<interval>` - Configure CSI collection
- `AT+CSISTART` - Start collection
- `AT+CSISTOP` - Stop collection

## Configuration

Default configuration (can be customized):
- Port: `/dev/ttyUSB0`
- Baud Rate: `115200`
- WiFi Channel: `6`
- Bandwidth: `20 MHz`
- Collection Interval: `100 ms`

## Architecture Notes

### Async Design
Uses Tokio for non-blocking I/O operations, allowing responsive TUI even during data collection.

### Modular Structure
Each component (device, visualization, storage, streaming) is independently testable and replaceable.

### Memory Safety
Leverages Rust's type system for safe concurrent operations and memory management.

## Future Enhancements

- 3D visualization with real-time rendering
- Advanced signal processing (FFT, filtering)
- Multi-device support
- Web interface
- Machine learning integration for anomaly detection
- Camera stream integration (bonus feature)

## Testing

Run tests with:
```bash
cargo test
```

## Troubleshooting

**Can't connect to serial port**:
- Check device is connected: `ls /dev/tty*`
- Verify permissions: `sudo usermod -a -G dialout $USER`
- Try different baud rate in config

**TUI display issues**:
- Ensure terminal supports 256 colors
- Try resizing terminal window
- Check terminal encoding is UTF-8

## Contributing

See CONTRIBUTING.md for guidelines.

## License

MIT License

## Acknowledgments

Built for the ConnectedMotion Hackathon 2024-2025, addressing WiFi CSI collection challenges in the ESP ecosystem.

## Resources

- [Ratatui Documentation](https://docs.rs/ratatui/)
- [Rerun Documentation](https://www.rerun.io/)
- [ESP-IDF Documentation](https://docs.espressif.com/projects/esp-idf/)
