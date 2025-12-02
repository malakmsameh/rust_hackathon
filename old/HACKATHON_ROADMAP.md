# 🎯 Hackathon: esp-csi-tui-rs - Complete Action Plan

## Challenge Overview

Build a **Terminal User Interface (TUI) application in Rust** to visualize and collect WiFi CSI data from ESP32 devices in real-time.

**Key Problem:** Current C/C++ CSI frameworks have limitations in reliability, performance, and scalability. Rust offers memory safety and better performance.

**Your Solution:** Interactive TUI using **Ratatui** crate for real-time CSI visualization and data collection.

---

## 📋 Core Requirements

### 1. **Device Interaction** (Critical)
- [ ] Communicate with ESP32 via serial (using esp-csi-cli-rs)
- [ ] Send configuration commands:
  - WiFi mode, channel, SSID/password
  - CSI features (LLTF, HTLTF, etc.)
  - Traffic generation parameters
- [ ] Receive and parse CSI data streams
- [ ] Display real-time status and metrics

### 2. **Data Visualization** (Critical)
- [ ] **Live CSI Plotting:**
  - Plot CSI amplitude vs subcarrier index
  - Plot CSI phase vs subcarrier index
  - Real-time updating graphs
- [ ] **Multiple Plot Types:**
  - 2D line plots (amplitude/phase)
  - Heatmaps (subcarrier vs time)
  - 3D plots (subcarrier, time, amplitude)
  - Color domain representation

### 3. **Remote Live Streaming** (Important)
- [ ] Stream data to Rerun.io viewer
- [ ] Support local or remote Rerun instance
- [ ] Real-time visualization dashboard

### 4. **Data Storage** (Important)
- [ ] Save to `.rrd` format (Rerun recording format)
- [ ] Save to `.csv` format (for analysis)
- [ ] Timestamped data logging

### 5. **Camera Streaming** (Bonus)
- [ ] Capture video from connected camera
- [ ] Display live camera feed in TUI
- [ ] Sync with CSI data collection

---

## 🏗️ Tech Stack

```
┌─────────────────────────────────┐
│   ESP32-C3 (Hardware)           │
│   esp-csi-cli-rs (CLI Tool)     │
└──────────────┬──────────────────┘
               │ Serial Communication
               ▼
┌─────────────────────────────────┐
│   esp-csi-tui-rs (Main App)     │
│                                 │
│  ┌───────────────────────────┐  │
│  │ Ratatui (TUI Framework)   │  │
│  │ - Widgets                 │  │
│  │ - Layout                  │  │
│  │ - Event Handling          │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │ Serial Communication      │  │
│  │ - serialport crate        │  │
│  │ - Parse CLI responses     │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │ Data Visualization        │  │
│  │ - plotters (2D/3D)        │  │
│  │ - ndarray (numerical)     │  │
│  │ - Complex number math     │  │
│  └───────────────────────────┘  │
│                                 │
│  ┌───────────────────────────┐  │
│  │ Data Storage              │  │
│  │ - CSV writer              │  │
│  │ - Rerun SDK               │  │
│  └───────────────────────────┘  │
└─────────────────────────────────┘
               │
       ┌───────┴────────┬───────────┐
       ▼                ▼           ▼
   CSV File         Rerun.io    Camera Feed
  (Analysis)      (Streaming)   (Sync)
```

---

## 🗂️ Project Structure

```
esp-csi-tui-rs/
├── Cargo.toml
├── src/
│   ├── main.rs                 # Entry point, event loop
│   ├── serial_handler.rs       # Serial communication with ESP
│   ├── csi_parser.rs           # Parse CSI data from CLI output
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── layout.rs           # TUI layout and widgets
│   │   ├── widgets.rs          # Custom widgets
│   │   └── colors.rs           # Color schemes
│   ├── visualization/
│   │   ├── mod.rs
│   │   ├── plots.rs            # Plot generation (2D/3D/Heatmap)
│   │   ├── renderer.rs         # Render plots to TUI
│   │   └── complex_math.rs     # Complex number operations
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── csv_writer.rs       # CSV export
│   │   ├── rrd_writer.rs       # Rerun recording format
│   │   └── data_buffer.rs      # In-memory data buffer
│   ├── streaming/
│   │   ├── mod.rs
│   │   └── rerun_client.rs     # Rerun SDK integration
│   ├── camera/
│   │   ├── mod.rs              # (Bonus feature)
│   │   └── capture.rs
│   ├── models.rs               # Data structures
│   └── app.rs                  # Application state
├── assets/                     # Icons, themes
└── tests/
```

---

## 📅 Phase-by-Phase Roadmap

### **Phase 1: Foundation (Days 1-2)**

#### Sprint 1.1: Project Setup
- [ ] Create Rust project using Cargo
- [ ] Add dependencies to Cargo.toml
- [ ] Set up basic main loop structure
- [ ] Create data models (CSIPacket, CSIFrame, etc.)

**Key Dependencies:**
```toml
[dependencies]
ratatui = "0.26"
serialport = "4.2"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
csv = "1.2"
ndarray = "0.15"
plotters = "0.3"
rerun = "0.14"
crossterm = "0.27"
```

**Deliverable:** Basic TUI skeleton that runs without crashing

#### Sprint 1.2: Serial Communication
- [ ] Open serial port connection to ESP device
- [ ] Read data from serial buffer
- [ ] Handle serial disconnection gracefully
- [ ] Create SerialHandler struct with read/write methods

**Deliverable:** Can connect to ESP and receive raw data

#### Sprint 1.3: CSI Data Parser
- [ ] Parse CSI output format from ESP
- [ ] Extract: MAC, RSSI, channel, CSI raw data
- [ ] Handle multi-line CSI packets
- [ ] Create structured CSIPacket data type

**Deliverable:** Parse one complete CSI packet successfully

---

### **Phase 2: Core UI & Visualization (Days 2-3)**

#### Sprint 2.1: Ratatui UI Framework
- [ ] Create TUI layout:
  - Top: Status bar (device status, connection, channel)
  - Left: Configuration panel (mode, channel, features)
  - Center: Live plot area
  - Bottom: Stats (packet count, RSSI trend, data rate)
- [ ] Set up event handling (keyboard input)
- [ ] Create reusable widget components

**Deliverable:** Interactive TUI with working navigation

#### Sprint 2.2: Basic Live Plotting (2D)
- [ ] Plot CSI amplitude vs subcarrier index
- [ ] Real-time update on new packets
- [ ] Use Plotters or similar for graph rendering
- [ ] Handle axis scaling and labels

**Key Features:**
```
Subcarrier Index (X-axis: 0-128)
       ↑
   A   │     ╱╲    ╱╲
   m   │    ╱  ╲  ╱  ╲
   p   │   ╱    ╲╱    ╲
   l   │
   i   │
   t   ├─────────────────→
   u   │
   d   │
   e   │
       └────────────────
```

**Deliverable:** Display real-time CSI data on screen

#### Sprint 2.3: Multi-Plot Types
- [ ] Add plot type selector (2D/Heatmap/Phase)
- [ ] 2D Phase plot (phase angle vs subcarrier)
- [ ] Heatmap (subcarrier vs time)
- [ ] Complex visualization (I/Q components)

**Deliverable:** Switch between different plot visualizations

---

### **Phase 3: Data & Control (Days 3-4)**

#### Sprint 3.1: Device Configuration UI
- [ ] Add config panel to TUI:
  - WiFi mode selector (Sniffer/Station/AP)
  - Channel selector (1-13)
  - SSID/Password input fields
  - CSI feature toggles
- [ ] Send commands to ESP via serial
- [ ] Display configuration feedback

**Deliverable:** Configure ESP from TUI instead of manual CLI

#### Sprint 3.2: Data Storage (CSV)
- [ ] Buffer incoming CSI packets in memory
- [ ] Export to CSV format:
  ```csv
  timestamp,mac,rssi,channel,subcarrier_0_real,subcarrier_0_imag,...
  1702632000,40:E1:E4:1F:81:C6,-61,11,-14,5,-14,6,...
  ```
- [ ] File picker/saver in TUI
- [ ] Auto-save with timestamped filenames

**Deliverable:** Save and load CSI data in CSV format

#### Sprint 3.3: Real-Time Statistics
- [ ] Display live metrics:
  - Packets received: X
  - RSSI trend (avg, min, max)
  - Data rate (packets/sec)
  - Collection duration
  - Buffer usage
- [ ] Update at 10 Hz refresh rate

**Deliverable:** Live dashboard showing collection metrics

---

### **Phase 4: Advanced Features (Days 4-5)**

#### Sprint 4.1: Rerun.io Integration
- [ ] Set up Rerun SDK connection
- [ ] Send 3D plot data to Rerun viewer
- [ ] Support local Rerun instance
- [ ] Stream data in real-time to remote instance

**Deliverable:** View CSI data in Rerun viewer while collecting

#### Sprint 4.2: Data Recording (.rrd format)
- [ ] Use Rerun recording format
- [ ] Save timestamped recordings
- [ ] Playback capability for recorded sessions

**Deliverable:** Record and playback CSI collection sessions

#### Sprint 4.3: Advanced Visualization
- [ ] 3D plot (subcarrier, time, amplitude)
- [ ] Spectral analysis (FFT of CSI)
- [ ] Waterfall plot (frequency vs time)
- [ ] Color domain visualization

**Deliverable:** Multiple advanced visualization options

---

### **Phase 5: Polish & Optimization (Day 5)**

#### Sprint 5.1: Performance
- [ ] Optimize TUI rendering (reduce flicker)
- [ ] Handle high-frequency data streams (1000+ packets/sec)
- [ ] Memory-efficient buffering
- [ ] Profile and optimize bottlenecks

#### Sprint 5.2: Error Handling
- [ ] Graceful serial disconnection handling
- [ ] Invalid data packet recovery
- [ ] User-friendly error messages in TUI
- [ ] Reconnection logic

#### Sprint 5.3: Documentation & Demo
- [ ] Create README with usage instructions
- [ ] Write inline code documentation
- [ ] Create demo GIF/video showing features
- [ ] Prepare presentation slides

---

## 🎬 Getting Started NOW

### **Immediate Actions (Today):**

#### Step 1: Create Project Structure
```bash
cd /home/tuqa-saeed/Documents/42\ Amman/Hackathons/ConnectedMotion
cargo new esp-csi-tui-rs
cd esp-csi-tui-rs
```

#### Step 2: Set Up Dependencies
Update `Cargo.toml`:
```toml
[package]
name = "esp-csi-tui-rs"
version = "0.1.0"
edition = "2021"

[dependencies]
ratatui = "0.26"
crossterm = "0.27"
serialport = "4.2"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
csv = "1.2"
ndarray = "0.15"
plotters = "0.3"
thiserror = "1.0"
log = "0.4"
env_logger = "0.11"
```

#### Step 3: Start with Main Loop
Create `src/main.rs`:
```rust
use ratatui::{
    backend::{CrosstermBackend, Backend},
    Terminal,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Main loop
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(10),
                ])
                .split(f.size());

            let title = Paragraph::new("ESP-CSI TUI Collector")
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);
        })?;

        // Handle user input
        if let Ok(true) = crossterm::event::poll(std::time::Duration::from_millis(100)) {
            if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
                if key.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    Ok(())
}
```

#### Step 4: Test Build
```bash
cargo build
cargo run
```

**Deliverable:** Running TUI window with title bar

---

## 📊 Recommended Module Development Order

```
1. models.rs          → Define data structures
2. serial_handler.rs  → Read from ESP
3. csi_parser.rs      → Parse CSI data
4. app.rs             → Application state
5. ui/layout.rs       → TUI structure
6. visualization/     → Plot rendering
7. storage/           → CSV/RRD export
8. streaming/         → Rerun integration
```

---

## 💡 Implementation Tips

### **For Serial Communication:**
```rust
use serialport::SerialPort;

pub struct SerialHandler {
    port: Box<dyn SerialPort>,
}

impl SerialHandler {
    pub fn new(port_name: &str) -> Result<Self> {
        let port = serialport::new(port_name, 115200)
            .timeout(Duration::from_millis(100))
            .open()?;
        Ok(Self { port })
    }

    pub fn read_line(&mut self) -> Result<String> {
        let mut buffer = [0; 256];
        let n = self.port.read(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
    }

    pub fn send_command(&mut self, cmd: &str) -> Result<()> {
        self.port.write_all(cmd.as_bytes())?;
        self.port.write_all(b"\n")?;
        Ok(())
    }
}
```

### **For CSI Parsing:**
```rust
#[derive(Debug, Clone)]
pub struct CSIPacket {
    pub mac: String,
    pub rssi: i32,
    pub channel: u8,
    pub timestamp: u64,
    pub csi_data: Vec<i32>,  // I/Q interleaved
}

impl CSIPacket {
    pub fn parse(output: &str) -> Result<Self> {
        // Parse esp-csi-cli-rs output
        // Extract fields using regex or line parsing
    }

    pub fn get_amplitude(&self) -> Vec<f32> {
        // Convert I/Q to amplitude
        self.csi_data.chunks(2)
            .map(|iq| ((iq[0]^2 + iq[1]^2) as f32).sqrt())
            .collect()
    }

    pub fn get_phase(&self) -> Vec<f32> {
        // Convert I/Q to phase
        self.csi_data.chunks(2)
            .map(|iq| (iq[1] as f32).atan2(iq[0] as f32))
            .collect()
    }
}
```

### **For Ratatui Plotting:**
```rust
use ratatui::widgets::canvas::Canvas;

pub fn draw_csi_plot(amplitude: &[f32]) -> Canvas {
    Canvas::default()
        .block(Block::default().title("CSI Amplitude").borders(Borders::ALL))
        .paint(|ctx| {
            // Plot amplitude on canvas
            for (i, &amp) in amplitude.iter().enumerate() {
                ctx.draw(&Point {
                    x: i as f64,
                    y: amp as f64,
                });
            }
        })
        .x_bounds([0.0, 128.0])
        .y_bounds([0.0, 100.0])
}
```

---

## 🎯 Minimum Viable Product (MVP)

To have a working demo by end of hackathon, focus on MVP:

✅ **Must Have (MVP):**
1. Serial connection to ESP
2. Parse and display CSI packets
3. Live 2D plot (amplitude vs subcarrier)
4. Configuration UI (WiFi mode, channel)
5. CSV export
6. Real-time stats display

🎁 **Nice to Have:**
1. Multiple plot types
2. Rerun.io integration
3. Heatmaps
4. 3D visualization

🌟 **Bonus (if time):**
1. Camera streaming
2. Advanced signal processing
3. Recording/playback

---

## 📚 Useful Resources

- **Ratatui Docs:** https://docs.rs/ratatui/latest/ratatui/
- **Ratatui Examples:** https://github.com/ratatui/ratatui/tree/main/examples
- **Serial Port:** https://docs.rs/serialport/latest/serialport/
- **Plotters:** https://docs.rs/plotters/latest/plotters/
- **Rerun SDK:** https://docs.rs/rerun/latest/rerun/

---

## 🎤 Tech Advisor Support

**Schedule:** https://calendar.app.google/Wh6e2BZ5FRUPMY9V8

Use your sessions to discuss:
- Architecture decisions
- Performance optimizations
- Visualization best practices
- Rerun.io integration details

---

## 🎬 Presentation Strategy

When presenting, showcase:
1. **Problem:** CSI collection tools lack interactive visualization
2. **Solution:** TUI-based collector in memory-safe Rust
3. **Demo:** Live CSI collection with multiple visualizations
4. **Impact:** Easier CSI research for WiFi sensing community

---

## 🚀 Let's Start!

**Your next immediate action:**

```bash
cd esp-csi-tui-rs
cargo new .
# Start with Phase 1, Sprint 1.1
```

Good luck with your hackathon! 🎉

Let me know if you need help with any specific part of the implementation!
