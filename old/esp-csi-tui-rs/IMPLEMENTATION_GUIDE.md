# esp-csi-tui-rs Implementation Guide

## Current Status

✅ Basic skeleton exists with:
- Ratatui TUI framework
- Serial port communication
- Basic command interface
- Output display

❌ Still needed:
- Proper module structure
- CSI data parsing
- Real visualization
- Storage/export features
- Device configuration panel

---

## Next Steps to Implement

### Step 1: Enhance Cargo.toml

Add missing dependencies for full feature set:

```toml
[dependencies]
ratatui = "0.28"
crossterm = "0.28"
anyhow = "1.0"
serialport = "4.3"
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
csv = "1.2"
ndarray = "0.15"
regex = "1"
log = "0.4"
env_logger = "0.11"
thiserror = "1.0"
```

### Step 2: Create Module Structure

```bash
mkdir -p src/{ui,serial,csi,storage,visualization}
touch src/{
    ui/mod.rs,
    ui/layout.rs,
    serial/mod.rs,
    serial/handler.rs,
    csi/mod.rs,
    csi/parser.rs,
    storage/mod.rs,
    storage/csv.rs,
    visualization/mod.rs,
    visualization/plots.rs,
    models.rs,
    app.rs
}
```

### Step 3: Start with data models (models.rs)

Define your core data structures for CSI handling.

### Step 4: Build serial handler (serial/handler.rs)

Improve serial communication with proper error handling.

### Step 5: Implement CSI parser (csi/parser.rs)

Parse esp-csi-cli-rs output into structured data.

### Step 6: Build TUI components (ui/layout.rs)

Create configuration panel and visualization areas.

### Step 7: Add visualization (visualization/plots.rs)

Implement 2D/3D plotting for CSI data.

### Step 8: Implement storage (storage/csv.rs)

Export data to CSV and other formats.

---

## Quick Wins (Do These First)

### ✅ Quick Win 1: Improve Serial Communication
Make it more robust and add error handling.

### ✅ Quick Win 2: Parse CSI Packets
Extract MAC, RSSI, and CSI data from output.

### ✅ Quick Win 3: Display Live Data
Show parsed data in a nicely formatted table.

### ✅ Quick Win 4: Add Visualization Section
Show a simple ASCII plot of CSI amplitude.

### ✅ Quick Win 5: Add Configuration UI
Interactive input fields for WiFi settings.

---

## Build Command

```bash
cd esp-csi-tui-rs
cargo build --release
```

## Run Command

```bash
cargo run --release
```

---

## Good luck! Start with the modules and incrementally build up the features.
