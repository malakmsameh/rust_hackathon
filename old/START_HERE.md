# 🎯 Hackathon Starting Point - Complete Summary

## Your Challenge
Build **esp-csi-tui-rs** - a Terminal User Interface in Rust to visualize WiFi CSI data from ESP32-C3 in real-time.

---

## ✅ What You Already Have

### 1. **Working ESP32-C3 Setup**
- ✅ Device flashed with esp-csi-cli-rs
- ✅ Connected to WiFi (FFF-5G)
- ✅ Collecting CSI data successfully
- ✅ Example data showing all fields (RSSI, channel, CSI array)

### 2. **Working Documentation**
- ✅ `ESP32-C3_CSI_QUICK_START.md` - Complete ESP setup guide
- ✅ `ESP32-C3_CSI_REFERENCE.txt` - Quick command reference
- ✅ `HACKATHON_ROADMAP.md` - Phase-by-phase implementation plan

### 3. **Project Skeleton**
- ✅ `esp-csi-tui-rs/` directory with basic TUI
- ✅ Basic serial communication
- ✅ Ratatui framework integrated
- ✅ Cargo project structure

---

## 📋 5-Day Hackathon Plan

### **Day 1: Foundation**
**Goal:** Get basic infrastructure working

Tasks:
```bash
# 1. Update Cargo.toml with all dependencies
# 2. Create module structure
# 3. Implement models.rs (data structures)
# 4. Improve serial/handler.rs
# 5. Build csi/parser.rs

Expected Output:
- Parse one complete CSI packet
- Display in formatted output
```

### **Day 2: Core Display**
**Goal:** Show CSI data in real-time on TUI

Tasks:
```bash
# 1. Implement basic TUI layout
# 2. Display parsed CSI data in table format
# 3. Show live statistics (RSSI, channel, packet count)
# 4. Add keyboard navigation

Expected Output:
- TUI showing live CSI packets from ESP
```

### **Day 3: Configuration & Visualization**
**Goal:** Interactive config + basic plots

Tasks:
```bash
# 1. Add WiFi configuration panel
# 2. Send commands to ESP from TUI
# 3. Implement basic 2D plot (amplitude vs subcarrier)
# 4. Handle multiple plot types

Expected Output:
- Configure ESP from TUI
- See live CSI plot updating
```

### **Day 4: Storage & Export**
**Goal:** Save and analyze data

Tasks:
```bash
# 1. Implement CSV export
# 2. Add real-time statistics
# 3. Implement circular buffer for data
# 4. Add timestamp and session management

Expected Output:
- Export CSI data to CSV
- Analyze trends and patterns
```

### **Day 5: Polish & Presentation**
**Goal:** Make it production-ready

Tasks:
```bash
# 1. Performance optimization
# 2. Error handling & recovery
# 3. Documentation and code cleanup
# 4. Create demo and presentation

Expected Output:
- Working, polished application ready for demo
```

---

## 🚀 Start RIGHT NOW

### Step 1: Check Current Code
```bash
cd /home/tuqa-saeed/Documents/42\ Amman/Hackathons/ConnectedMotion/esp-csi-tui-rs
cat src/main.rs
```

### Step 2: Add Dependencies
Update `Cargo.toml` with all needed packages from HACKATHON_ROADMAP.md

### Step 3: Create Basic Structure
```bash
mkdir -p src/{ui,serial,csi,storage,visualization}
touch src/models.rs src/app.rs
```

### Step 4: Start Coding
Begin with `src/models.rs` to define your data structures.

### Step 5: Build & Test
```bash
cargo build
cargo run --release
```

---

## 📊 Data You're Collecting

Each CSI packet contains:
```
MAC Address:       40:E1:E4:1F:81:C6
RSSI:              -61 dBm (signal strength)
Channel:           11
Timestamp:         3195366537
Data Rate:         11 Mbps
Modulation:        11n (sig_mode=1, mcs=7)
CSI Raw Data:      384 bytes of I/Q samples
  - Format:        [real_0, imag_0, real_1, imag_1, ...]
  - Count:         ~192 subcarriers
```

---

## 🎯 MVP (Minimum Viable Product)

To have something demo-able:

✅ **Must Have (Day 2):**
1. Read CSI packets from ESP via serial
2. Parse MAC, RSSI, channel, CSI array
3. Display in formatted output
4. Show packet count and stats

✅ **Should Have (Day 3):**
1. Configuration panel (set WiFi, channel)
2. 2D plot of CSI amplitude
3. Switch between different visualizations
4. Real-time update

✅ **Nice to Have (Day 4):**
1. CSV export
2. Heatmap visualization
3. Data recording/playback
4. Advanced signal processing

---

## 💻 Quick Code Snippets to Get Started

### Parse CSI Packet:
```rust
pub struct CSIPacket {
    pub mac: String,
    pub rssi: i32,
    pub channel: u8,
    pub timestamp: u64,
    pub csi_data: Vec<i32>,
}

pub fn parse_csi_output(output: &str) -> Option<CSIPacket> {
    // TODO: Use regex to extract fields
    // Return parsed packet
}
```

### Send Command to ESP:
```rust
pub async fn send_command(cmd: &str) {
    // Write to serial port
    // Example: "set-wifi --mode=sniffer"
}
```

### Display 2D Plot:
```rust
pub fn draw_plot(amplitude: &[f32]) -> Canvas {
    // Use Ratatui Canvas to plot amplitude
    // X-axis: subcarrier index (0-128)
    // Y-axis: amplitude value
}
```

---

## 📚 Key Files to Focus On

1. **src/main.rs** - Entry point, event loop
2. **src/models.rs** - Define CSIPacket structure
3. **src/csi/parser.rs** - Parse ESP output
4. **src/ui/layout.rs** - TUI layout
5. **src/visualization/plots.rs** - Draw plots
6. **src/storage/csv.rs** - Export data

---

## ✨ Bonus Ideas

If you finish MVP early:

1. **3D Visualization** - Plot subcarrier, time, amplitude in 3D
2. **Signal Processing** - FFT, spectral analysis
3. **Real-time Statistics** - RSSI trend, data rate, packet loss
4. **Heatmaps** - Visualize subcarrier strength over time
5. **Rerun.io Integration** - Stream to remote viewer
6. **Recording** - Save and playback CSI sessions

---

## 🎤 Tech Advisor Tips

When scheduling tech advisor sessions, ask about:
1. Best architecture for real-time data streaming
2. Optimizing TUI rendering performance
3. Efficient data structure for buffering
4. Rerun.io integration strategy
5. Complex number visualization techniques

---

## 🔗 Important Links

- **Full Roadmap:** `/home/tuqa-saeed/Documents/42 Amman/Hackathons/ConnectedMotion/HACKATHON_ROADMAP.md`
- **Ratatui Docs:** https://docs.rs/ratatui/
- **Ratatui Examples:** https://github.com/ratatui/ratatui/tree/main/examples
- **Tech Advisor:** https://calendar.app.google/Wh6e2BZ5FRUPMY9V8
- **Rerun SDK:** https://docs.rs/rerun/

---

## ⚡ Pro Tips

1. **Start Simple** - Get basic display working first
2. **Test Often** - Build and run frequently
3. **Use Example Data** - Hard-code CSI data for testing
4. **Break Into Modules** - Makes coding easier
5. **Read Ratatui Examples** - Learn from existing TUI apps
6. **Leverage ESP Already Working** - Focus on UI/visualization
7. **Document as You Go** - Easier for presentation

---

## 🎬 Presentation Angle

When demoing, show:
1. **Before:** Current C/C++ CSI tools limitations
2. **During:** Live TUI collecting CSI data
3. **Visualization:** Real-time plots updating
4. **Export:** Show saved CSV data
5. **After:** Potential use cases for WiFi sensing

---

## 📞 If You Get Stuck

1. Check Ratatui examples first
2. Review your models.rs (data structure issues)
3. Test serial communication separately
4. Print debug output to verify parsing
5. Ask tech advisor for guidance

---

## ✅ Checklist for Day 1

- [ ] Read HACKATHON_ROADMAP.md completely
- [ ] Update Cargo.toml with dependencies
- [ ] Create module structure
- [ ] Implement models.rs
- [ ] Write CSI parser
- [ ] Test parsing with example output
- [ ] Commit to git

---

**You've got this! The ESP32 is working, documentation is ready, and you have a skeleton.**

**All that's left is to build out the visualization layer.**

**Let's make an amazing TUI tool! 🚀**

---

## Current Project Structure

```
esp-csi-tui-rs/
├── Cargo.toml          ← Update with more dependencies
├── src/
│   ├── main.rs         ← Event loop and main app structure
│   ├── models.rs       ← Define CSIPacket, App state (CREATE)
│   ├── app.rs          ← Application logic (CREATE)
│   ├── csi/
│   │   ├── mod.rs      ← (CREATE)
│   │   └── parser.rs   ← Parse ESP output (CREATE)
│   ├── ui/
│   │   ├── mod.rs      ← (CREATE)
│   │   └── layout.rs   ← TUI layout (CREATE)
│   ├── serial/
│   │   ├── mod.rs      ← (CREATE)
│   │   └── handler.rs  ← Serial communication (CREATE)
│   ├── storage/
│   │   ├── mod.rs      ← (CREATE)
│   │   └── csv.rs      ← CSV export (CREATE)
│   └── visualization/
│       ├── mod.rs      ← (CREATE)
│       └── plots.rs    ← Plot rendering (CREATE)
└── IMPLEMENTATION_GUIDE.md (NEW)
```

---

**Happy hacking! 🎉**
