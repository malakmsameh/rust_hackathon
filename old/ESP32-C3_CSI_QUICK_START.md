# ESP32-C3 CSI Collection - Complete Quick Start Guide

## Table of Contents
1. [Initial Setup](#initial-setup)
2. [Flashing the Device](#flashing-the-device)
3. [Connecting to the Device](#connecting-to-the-device)
4. [Configuration Commands](#configuration-commands)
5. [Starting Collection](#starting-collection)
6. [Complete Examples](#complete-examples)
7. [Troubleshooting](#troubleshooting)

---

## Initial Setup

### Prerequisites
- **Hardware:** ESP32-C3 development board
- **Software:**
  - Rust with ESP support
  - `espflash` tool
  - USB cable for connection

### Install Required Tools

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install ESP tools
cargo install espflash
cargo install ldproxy

# Update Rust nightly
rustup default nightly
rustup component add rust-src
```

### Clone the Repository

```bash
git clone https://github.com/theembeddedrustacean/esp-csi-cli-rs.git
cd esp-csi-cli-rs
```

---

## Flashing the Device

### Step 1: Connect the ESP32-C3
- Plug the ESP32-C3 into your computer via USB

### Step 2: Build the Project

```bash
cargo build --release --features=esp32c3 --target=riscv32imc-unknown-none-elf --bin async_main
```

### Step 3: Flash the Binary

```bash
# Using espflash (recommended)
espflash flash --monitor target/riscv32imc-unknown-none-elf/release/async_main --port /dev/ttyUSB0
```

Or with a single command:
```bash
cargo run --release --features=esp32c3 --target=riscv32imc-unknown-none-elf --bin async_main
```

### Step 4: Wait for Boot
You'll see bootloader messages, then the application starts with:
```
******* Welcome to the CSI Collection CLI utility! *******
Available Commands:
    set-wifi            Configure WiFi settings
    set-traffic         Configure traffic parameters
    set-csi             Configure CSI feature flags
    start               Start the CSI collection process
    show-config         Display current configuration
    reset-config        Reset configurations to defaults
    help                Display help menu

>
```

---

## Connecting to the Device

### Using espflash Monitor

```bash
espflash monitor --port /dev/ttyUSB0
```

Or if you need sudo permissions:
```bash
sudo /home/tuqa-saeed/.cargo/bin/espflash monitor --port /dev/ttyUSB0
```

### Serial Port on Different Systems
- **Linux:** `/dev/ttyUSB0` or `/dev/ttyUSB1`
- **macOS:** `/dev/tty.usbserial-*`
- **Windows:** `COM3`, `COM4`, etc.

### Monitor Commands
- **Type commands:** Enter CLI commands
- **CTRL+R:** Reset the device
- **CTRL+C:** Exit the monitor

---

## Configuration Commands

### 1. Check Current Configuration

```
show-config
```

**Output:**
```
Traffic Configuration:
  Traffic Generation Enabled: false
  Traffic Generation Frequency: 0 Hz

CSI Configuration:
  LLTF Enabled: true
  HTLTF Enabled: true
  STBC HTLTF Enabled: true
  LTF Merge Enabled: true

WiFi Settings:
  WiFi Operation Mode: Sniffer
  ...
```

---

### 2. Configure WiFi Mode

#### **Sniffer Mode** (Passive - Recommended to Start)
Passively monitor all WiFi traffic on a channel without connecting.

```
set-wifi --mode=sniffer --set-channel=6
```

#### **Station Mode** (Connect to Existing WiFi)
Connect to an existing WiFi network and collect CSI data.

```
set-wifi --mode=station --sta-ssid=YourWiFiName --sta-password=YourPassword --set-channel=1
```

**Important:** Replace spaces in SSID/password with underscores (`_`)
```
set-wifi --mode=station --sta-ssid=My_Home_WiFi --sta-password=my_pass_123 --set-channel=1
```

#### **Access Point Mode** (Create Your Own WiFi)
Create a WiFi hotspot and collect CSI from connecting devices.

```
set-wifi --mode=ap --ap-ssid=ESP_CSI_AP --ap-password=testing123 --max-connections=5
```

#### **AP+Station Mode** (Both Simultaneously)
Act as an AP and connect to another network at the same time.

```
set-wifi --mode=ap-station --ap-ssid=ESP_AP --ap-password=pass123 --sta-ssid=RouterName --sta-password=router_pass
```

### WiFi Configuration Options

```
set-wifi [OPTIONS]

Options:
  --mode=<mode>              WiFi mode: ap, station, sniffer, ap-station (default: sniffer)
  --set-channel=<NUMBER>     WiFi channel: 1-13 (default: 1)
  --max-connections=<NUMBER> Max AP connections (default: 1)
  --hide-ssid                Hide SSID if in AP mode
  --ap-ssid=<SSID>          AP network name
  --ap-password=<PASSWORD>   AP password
  --sta-ssid=<SSID>         Station network name to connect to
  --sta-password=<PASSWORD>  Station password
```

**Examples:**
```
set-wifi --mode=sniffer --set-channel=11

set-wifi --mode=station --sta-ssid=HomeWiFi --sta-password=secure_pass --set-channel=6

set-wifi --mode=ap --ap-ssid=ESP32_Sniffer --ap-password=pass123 --max-connections=3

set-wifi --mode=ap --hide-ssid
```

---

### 3. Configure CSI Features

Enable or disable specific CSI data collection features.

```
set-csi [OPTIONS]

Options:
  --disable-lltf              Disable Long Training Field (default: enabled)
  --disable-htltf             Disable HT Training Field (default: enabled)
  --disable-stbc-htltf        Disable STBC HT Training Field (default: enabled)
  --disable-ltf-merge         Disable LTF Merge (default: enabled)
```

**Examples:**
```
# Disable HTLTF only
set-csi --disable-htltf

# Disable multiple features
set-csi --disable-htltf --disable-stbc-htltf

# Keep all enabled (default)
set-csi
```

---

### 4. Configure Traffic Generation

Generate WiFi traffic at regular intervals (useful for active CSI collection).

```
set-traffic --frequency-hz=<NUMBER>

Options:
  --frequency-hz=<NUMBER>  Traffic generation frequency in Hz (default: 0 = disabled)
```

**Examples:**
```
# Generate 10 packets per second
set-traffic --frequency-hz=10

# Generate 1 packet per second
set-traffic --frequency-hz=1

# Disable traffic generation
set-traffic --frequency-hz=0
```

---

### 5. Reset Configuration

Reset all settings to their default values.

```
reset-config
```

**Defaults:**
- Mode: Sniffer
- Channel: 1
- All CSI features: Enabled
- Traffic generation: Disabled

---

## Starting Collection

### Start Indefinitely

Collect CSI data until you press **CTRL+C**:

```
start
```

### Start for Specific Duration

Collect CSI data for a set amount of time (in seconds):

```
start --duration=60      # Collect for 60 seconds
start --duration=300     # Collect for 5 minutes
start --duration=3600    # Collect for 1 hour
```

### CSI Data Output

Once collection starts, you'll see output like:

```
rssi: -45
rate: 11
noise floor: -100
channel: 6
timestamp: 1234567890
sig_len: 64
rx_state: 0
secondary_channel: 0
sgi: 0
ant: 0
ampdu_cnt: 0
sig_mode: 0
mcs: 0
cwb: 0
smoothing: 0
not_sounding: 0
aggregation: 0
stbc: 0
fec_coding: 0
csi_data_len: 128
csi raw data:
[raw binary CSI data bytes...]
```

---

## Complete Examples

### Example 1: Passive Sniffer on Channel 6 (Simplest)

```bash
show-config
set-wifi --mode=sniffer --set-channel=6
start --duration=120
```

**What happens:**
- Device passively monitors WiFi on channel 6
- Collects CSI data from all packets for 2 minutes
- No WiFi connection needed
- Displays CSI information as it's collected

---

### Example 2: Connect to Home WiFi and Collect

```bash
set-wifi --mode=station --sta-ssid=MyHomeWiFi --sta-password=mypassword123 --set-channel=6
show-config
start --duration=300
```

**What happens:**
- Device connects to your home WiFi network
- Collects CSI data from that network
- Runs for 5 minutes
- Displays CSI information in real-time

---

### Example 3: Create AP and Collect from Clients

```bash
set-wifi --mode=ap --ap-ssid=ESP_CSI_Lab --ap-password=testing123 --max-connections=5
set-traffic --frequency-hz=10
show-config
start --duration=600
```

**What happens:**
- Device creates a WiFi hotspot named "ESP_CSI_Lab"
- Generates traffic at 10 packets per second
- Allows up to 5 devices to connect
- Collects CSI data for 10 minutes
- Other devices can connect to this AP and trigger CSI collection

**To connect from another device:**
- Find WiFi network: "ESP_CSI_Lab"
- Password: `testing123`
- Start sending traffic to trigger CSI collection

---

### Example 4: With CSI Features Disabled

```bash
set-csi --disable-htltf --disable-stbc-htltf
set-wifi --mode=sniffer --set-channel=11
show-config
start --duration=180
```

**What happens:**
- Disables HTLTF and STBC HTLTF collection
- Uses only LLTF and LTF Merge
- Monitors channel 11
- Collects for 3 minutes

---

### Example 5: Channel Hopping (Multiple Channels)

```bash
# Channel 1
set-wifi --set-channel=1
start --duration=30

# Channel 6
set-wifi --set-channel=6
start --duration=30

# Channel 11
set-wifi --set-channel=11
start --duration=30
```

**What happens:**
- Collects 30 seconds on channel 1
- Then 30 seconds on channel 6
- Then 30 seconds on channel 11
- You can monitor different parts of the WiFi spectrum

---

## Viewing Help

### Get General Help

```
help
```

### Get Help for Specific Command

```
help set-wifi
help start
help set-csi
help set-traffic
```

---

## Troubleshooting

### Device Not Responding

**Solution:**
1. Press **CTRL+R** to reset the device
2. If still not responding, unplug USB, wait 5 seconds, plug back in
3. Reconnect to monitor: `espflash monitor --port /dev/ttyUSB0`

### Can't Type Commands

**Solution:**
1. Check that the monitor prompt shows `> ` (cursor ready)
2. Try typing `help` and pressing Enter
3. If no response, the application may be stuck

### WiFi Won't Connect (Station Mode)

**Solution:**
1. Ensure target WiFi network is running
2. Check SSID and password are correct (spaces should be underscores)
3. Try a different channel: `set-wifi --set-channel=6`
4. Reset device with **CTRL+R**

### No CSI Data Appearing

**Solution:**
1. Check you're in a WiFi-enabled environment
2. Verify WiFi channel matches your network: `show-config`
3. In Sniffer mode, ensure WiFi traffic exists on that channel
4. Try Access Point mode and connect another device to it

### Serial Port Permission Denied

**Solution:**
```bash
# Add user to dialout group
sudo usermod -a -G dialout $USER
newgrp dialout
```

Then try again without sudo:
```bash
espflash monitor --port /dev/ttyUSB0
```

### Device Stuck at Bootloader

**Solution:**
1. Press **CTRL+C** to exit monitor
2. Unplug device
3. Wait 5 seconds
4. Plug back in
5. Reconnect: `espflash monitor --port /dev/ttyUSB0`

---

## Channel Reference

| Region | Channels |
|--------|----------|
| USA/Canada | 1-11 |
| Europe | 1-13 |
| Japan | 1-14 |
| Most Common | 1, 6, 11 (2.4GHz non-overlapping) |

---

## Tips for Best Results

1. **Use standard channels:** 1, 6, or 11 for 2.4GHz WiFi
2. **Generate traffic:** Use `set-traffic` with a known frequency for better CSI collection
3. **Multiple networks:** Switch between channels to collect data from different networks
4. **Monitor duration:** Start with 60-120 seconds to test, then increase
5. **Save output:** Redirect console output to a file for analysis:
   ```bash
   espflash monitor --port /dev/ttyUSB0 > csi_data.log 2>&1
   ```

---

## Common Workflows

### Quick Test (30 seconds)
```
set-wifi --mode=sniffer --set-channel=6
start --duration=30
```

### Comprehensive Survey (5 minutes)
```
set-wifi --mode=station --sta-ssid=YourNetwork --sta-password=YourPass
set-traffic --frequency-hz=5
set-csi
start --duration=300
```

### Background Monitoring (1 hour)
```
set-wifi --mode=sniffer --set-channel=1
start --duration=3600
```

### Lab Experiment (AP Mode)
```
set-wifi --mode=ap --ap-ssid=Lab_CSI --ap-password=lab123
set-traffic --frequency-hz=20
start --duration=600
```
(Then connect other devices to the "Lab_CSI" network)

---

## Advanced Topics

### Redirecting Output to File

```bash
# Start monitoring and save to file
espflash monitor --port /dev/ttyUSB0 | tee csi_collection_$(date +%Y%m%d_%H%M%S).log
```

### Processing CSI Data

The raw CSI data can be extracted and processed with custom scripts. Each CSI packet contains:
- RSSI (Received Signal Strength)
- Rate information
- Noise floor
- Channel information
- Timestamp
- CSI raw data (128+ bytes depending on configuration)

### Multiple Devices

You can flash multiple ESP32-C3 boards and run collection in parallel:
```bash
espflash monitor --port /dev/ttyUSB0  # Device 1
espflash monitor --port /dev/ttyUSB1  # Device 2 (in another terminal)
```

---

## Need More Help?

- **Repository:** https://github.com/theembeddedrustacean/esp-csi-cli-rs
- **Original Library:** https://docs.rs/esp_csi_rs
- **ESP-IDF Docs:** https://docs.espressif.com/

---

**Last Updated:** December 1, 2025
**Device Tested:** ESP32-C3 (revision v0.4)
