# Running the ESP32 CSI TUI Application

## Quick Start

### Option 1: Use the Run Script (Easiest)
```bash
cd ~/Documents/42\ Amman/Hackathons/ConnectedMotion/esp-csi-tui-rs
./run.sh
```

This script will:
- ✓ Check if ESP32 is connected
- ✓ Handle permissions automatically
- ✓ Launch the TUI

### Option 2: Run with Cargo (Standard Rust way)
```bash
cd ~/Documents/42\ Amman/Hackathons/ConnectedMotion/esp-csi-tui-rs
sudo cargo run
```

### Option 3: Run the Compiled Binary Directly
```bash
sudo ~/Documents/42\ Amman/Hackathons/ConnectedMotion/esp-csi-tui-rs/target/debug/esp-csi-tui-rs
```

---

## ⚠️ Important: Need sudo!

The application **must run with sudo** because:
- Serial port access requires root permissions
- `/dev/ttyUSB0` is owned by root:dialout group

If you get "Permission denied", just use `sudo`:
```bash
sudo cargo run
```

---

## 📋 Requirements Before Running

1. **ESP32 Plugged In**
   ```bash
   ls /dev/ttyUSB*
   # Should show /dev/ttyUSB0
   ```

2. **ESP32 Flashed**
   - The device should have esp-csi-cli-rs firmware already flashed
   - See `ESP32-C3_CSI_QUICK_START.md` for flashing instructions

3. **USB Cable Working**
   - Test with espflash: `sudo espflash monitor --port /dev/ttyUSB0`

---

## 🎮 Using the TUI

Once running, you'll see:

```
┌─ ESP32 CSI Monitor - Real-time WiFi Channel State Information Visualization ─┐
│                                                                               │
├─ Status ──────────────────────────────────────────────────────────────────┤
│ Connection: Connecting...                                                  │
│ Last Command: show-config                                                 │
│                                                                               │
├─ Device Output ────────────────────────────────────────────────────────────┤
│ ✓ Sent: set-wifi --mode=sniffer --set-channel=6                          │
│ ✓ Sent: start --duration=120                                              │
│ ✓ Sent: set-traffic --frequency-hz=10                                     │
│                                                                               │
├─ Live Data ────────────────────────────────────────────────────────────────┤
│ Packets: 42                                                                │
│ Latest Data: mac: 40:E1:E4:1F:81:C6 rssi: -61                            │
│                                                                               │
├─ QUICK COMMANDS ──────────────────────────────────────────────────────────┤
│ w = Sniffer Mode | c = Start Collection | 1 = Traffic 10Hz               │
│ s = Show Config  | h = Help  | 2 = CSI Config                            │
│                                                                               │
│ q = Quit Application                                                       │
└───────────────────────────────────────────────────────────────────────────┘
```

### Keyboard Commands:
- **w** → Start sniffer mode
- **c** → Start CSI collection for 120 seconds
- **1** → Generate WiFi traffic at 10Hz
- **2** → Configure CSI settings
- **s** → Show current device configuration
- **h** → Show device help
- **q** → Quit the application

---

## 🐛 Troubleshooting

### "Failed to connect to ESP32"
```bash
# Check if device is detected
ls /dev/ttyUSB*

# If not found:
1. Check USB cable
2. Plug device into different USB port
3. Try: dmesg | tail -20
```

### "Permission denied"
```bash
# Make sure you use sudo:
sudo cargo run
# or
sudo ./run.sh
```

### "Device seems unresponsive"
```bash
# Try connecting with espflash first
sudo espflash monitor --port /dev/ttyUSB0

# This will show if device is actually responding
```

### Hang on startup
```bash
# Press Ctrl+C and try again
# Sometimes first run needs a moment
```

---

## 💡 Development Notes

The TUI application:
- Connects to ESP32 at `/dev/ttyUSB0`, 115200 baud
- Sends commands and reads responses in real-time
- Displays status, device output, live data, and commands
- Can be modified to add more visualization features

To rebuild after changes:
```bash
cargo build --release
```

For debugging:
```bash
RUST_LOG=debug sudo cargo run
```

---

## 🚀 Next Steps

Once this is working:
1. Test with real WiFi traffic collection
2. Add data visualization (2D plots)
3. Implement CSV export
4. Add statistics dashboard
5. Integrate Rerun.io for live streaming

Happy monitoring! 📊
