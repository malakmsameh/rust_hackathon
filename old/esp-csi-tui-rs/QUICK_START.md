# Quick Start Guide - Refactored esp-csi-tui-rs

## Compilation

```bash
cd esp-csi-tui-rs
cargo build              # Debug build (~1.7s)
cargo build --release   # Optimized release build
```

**Binary Location:** `target/debug/esp-csi-tui-rs` (50 MB debug)

## Running the Application

```bash
./target/debug/esp-csi-tui-rs
```

## Configuration

Edit or create `~/.esp-csi/config.toml`:

```toml
[serial]
port = "/dev/ttyUSB0"      # Serial port path
baud_rate = 115200         # Baud rate

[collection]
channel = 6                # WiFi channel
duration_seconds = 60      # Collection duration
```

Default values used if file doesn't exist.

## Keyboard Controls

| Key | Action |
|-----|--------|
| `q` / `Q` | Quit application |
| `h` / `H` | Send help command |
| `w` / `W` | Enable WiFi sniffer mode |
| `c` / `C` | Start collection (60 seconds) |
| `x` / `X` | Stop collection |
| `s` / `S` | Show device configuration |
| `1` | Set traffic mode (10 Hz) |
| `2` | Set CSI mode |
| `e` / `E` | Export data to CSV |

## Data Export

CSV files are exported to: `~/.esp-csi/exports/`

Filename format: `csi_data_YYYYMMDD_HHMMSS.csv`

**Exported Fields:**
- timestamp_us - Microsecond timestamp
- mac_address - Source device MAC
- rssi_dbm - Signal strength (dBm)
- channel - WiFi channel number
- rate_mbps - Data rate
- signal_length - Packet length
- subcarrier_count - Number of subcarriers
- amplitude_peak - Peak CSI amplitude
- amplitude_mean - Average amplitude

**Metadata Header:**
```
# Export Time: 2024-12-02 17:45:30.123456 +00:00
# Packet Count: 1250
# Collection Status: Active
```

## Project Structure

```
src/
├── main.rs              # UI event loop & rendering (278 lines)
├── app.rs              # App state management (126 lines)
├── config.rs           # Config file system (92 lines)
├── models.rs           # Data structures (161 lines)
├── csi/
│   ├── parser.rs       # Packet parsing (269 lines)
│   └── processor.rs    # Data pipeline (74 lines)
├── serial/
│   └── handler.rs      # Serial I/O (93 lines)
├── storage/
│   └── csv.rs          # CSV export (127 lines)
└── visualization/
    └── plots.rs        # ASCII plots (107 lines)
```

## Module Overview

### app.rs
Application state and lifecycle management
- `App::new()` - Initialize with serial connection
- `send_command()` - Send commands to device
- `read_data()` - Parse serial output
- `save_to_csv()` - Export packet data

### config.rs
TOML-based configuration system
- `load_config()` - Load config from file with fallback defaults
- `save_config()` - Save settings to file

### csi/processor.rs
Data pipeline integration
- Bridges raw serial data to structured CSIPacket vectors
- Integrated into data flow: Serial → Parser → Processor → Buffer

### visualization/plots.rs
Real-time ASCII/Unicode visualization
- `plot_amplitude()` - Renders subcarrier amplitudes with Unicode blocks
- `plot_bar_chart()` - Renders data distribution charts

### storage/csv.rs
Structured data export with metadata
- `CSIRecord` struct - Serde-serializable packet record
- `export_with_metadata()` - CSV export with comment headers
- Metadata includes: export time, packet count, collection status

## Debugging

Enable verbose logging:

```bash
RUST_LOG=debug ./target/debug/esp-csi-tui-rs
```

Check compilation details:

```bash
cargo build -v              # Verbose build output
cargo build --message-format=json  # JSON diagnostics
```

## Performance Targets

- Parser latency: < 5ms per packet
- UI refresh rate: 30 FPS (33ms)
- Memory footprint: < 50 MB (packet buffer size dependent)

## Known Limitations

1. Device firmware doesn't support stopping collection once started
2. Collection runs for specified duration or indefinitely if unsupported
3. Single serial port connection at a time
4. Buffer limited by available RAM

## Future Enhancements

- [ ] Event handler extraction module (ui/handlers.rs)
- [ ] Configuration validation on startup
- [ ] Extended visualization types (heatmaps, spectrograms)
- [ ] Unit tests for each module
- [ ] Database backend option
- [ ] Network packet streaming

## Troubleshooting

**Error: "Could not find serial port"**
- Check device is connected: `ls /dev/tty*`
- Update config.toml with correct port

**Error: "Could not determine home directory"**
- Set HOME environment variable
- Or ensure ~/.esp-csi/ directory exists

**Error: "CSV export failed"**
- Ensure ~/.esp-csi/exports/ directory exists
- Check disk space and permissions

**No data appearing in plots**
- Verify device is in collection mode (press `c`)
- Check RSSI status line shows positive packet count
- Increase packet collection duration

## Building for Release

```bash
cargo build --release
strip target/release/esp-csi-tui-rs  # Optional: reduce binary size
```

Release binary will be significantly smaller and faster than debug build.

## Development Workflow

1. Make code changes
2. `cargo build` to compile
3. `cargo run` to test
4. `cargo test` to run unit tests
5. `cargo clippy` for code quality checks

---

**Last Updated:** 2024-12-02
**Refactoring Status:** ✅ Complete
**Compilation:** ✅ Success (0 errors, 15 warnings)
