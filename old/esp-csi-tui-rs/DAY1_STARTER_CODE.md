# Day 1 Starter Code - Models & Parser

Copy these files to your `esp-csi-tui-rs/src/` directory to get started immediately.

---

## File 1: src/models.rs

```rust
use serde::{Deserialize, Serialize};

/// Represents a single CSI packet from the ESP device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CSIPacket {
    /// MAC address of the source device
    pub mac: String,

    /// Received Signal Strength Indicator in dBm
    pub rssi: i32,

    /// WiFi channel number
    pub channel: u8,

    /// Unix timestamp of packet capture
    pub timestamp: u64,

    /// Data rate in Mbps
    pub rate: u8,

    /// Noise floor level
    pub noise_floor: i32,

    /// Signal length
    pub sig_len: u16,

    /// RX state
    pub rx_state: u8,

    /// Secondary channel
    pub secondary_channel: u8,

    /// Short Guard Interval flag
    pub sgi: u8,

    /// Antenna number
    pub ant: u8,

    /// AMPDU count
    pub ampdu_cnt: u8,

    /// Signal mode (11n, 11ac, etc.)
    pub sig_mode: u8,

    /// Modulation and Coding Scheme
    pub mcs: u8,

    /// Channel bandwidth
    pub cwb: u8,

    /// Smoothing flag
    pub smoothing: u8,

    /// Not sounding flag
    pub not_sounding: u8,

    /// Aggregation flag
    pub aggregation: u8,

    /// STBC flag
    pub stbc: u8,

    /// FEC coding flag
    pub fec_coding: u8,

    /// Data length
    pub data_length: u16,

    /// Raw CSI data (I/Q interleaved): [I0, Q0, I1, Q1, ...]
    pub csi_data: Vec<i32>,
}

impl CSIPacket {
    /// Get amplitude for each subcarrier
    pub fn get_amplitude(&self) -> Vec<f32> {
        self.csi_data
            .chunks(2)
            .map(|iq| {
                let i = iq.get(0).copied().unwrap_or(0) as f32;
                let q = iq.get(1).copied().unwrap_or(0) as f32;
                (i * i + q * q).sqrt()
            })
            .collect()
    }

    /// Get phase for each subcarrier
    pub fn get_phase(&self) -> Vec<f32> {
        self.csi_data
            .chunks(2)
            .map(|iq| {
                let i = iq.get(0).copied().unwrap_or(0) as f32;
                let q = iq.get(1).copied().unwrap_or(0) as f32;
                q.atan2(i)
            })
            .collect()
    }

    /// Get number of subcarriers
    pub fn subcarrier_count(&self) -> usize {
        self.csi_data.len() / 2
    }
}

/// Represents collected statistics
#[derive(Debug, Clone, Default)]
pub struct CollectionStats {
    pub packets_received: u64,
    pub rssi_min: i32,
    pub rssi_max: i32,
    pub rssi_avg: i32,
    pub start_time: u64,
    pub data_rate: f32,
}

/// Application state
#[derive(Debug, Clone)]
pub struct AppState {
    pub is_connected: bool,
    pub is_collecting: bool,
    pub current_config: DeviceConfig,
    pub latest_packet: Option<CSIPacket>,
    pub packet_buffer: Vec<CSIPacket>,
    pub stats: CollectionStats,
}

/// Device configuration
#[derive(Debug, Clone)]
pub struct DeviceConfig {
    pub wifi_mode: String,
    pub channel: u8,
    pub ssid: String,
    pub password: String,
    pub traffic_frequency: u32,
    pub csi_features_enabled: bool,
}

impl Default for DeviceConfig {
    fn default() -> Self {
        Self {
            wifi_mode: "sniffer".to_string(),
            channel: 1,
            ssid: String::new(),
            password: String::new(),
            traffic_frequency: 0,
            csi_features_enabled: true,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            is_connected: false,
            is_collecting: false,
            current_config: DeviceConfig::default(),
            latest_packet: None,
            packet_buffer: Vec::new(),
            stats: CollectionStats::default(),
        }
    }
}
```

---

## File 2: src/csi/mod.rs

```rust
pub mod parser;

pub use parser::CSIParser;
```

---

## File 3: src/csi/parser.rs

```rust
use regex::Regex;
use crate::models::CSIPacket;

pub struct CSIParser {
    mac_regex: Regex,
    rssi_regex: Regex,
    channel_regex: Regex,
    csi_data_regex: Regex,
}

impl CSIParser {
    pub fn new() -> Self {
        Self {
            mac_regex: Regex::new(r"mac:\s+([0-9a-fA-F:]+)").unwrap(),
            rssi_regex: Regex::new(r"rssi:\s+(-?\d+)").unwrap(),
            channel_regex: Regex::new(r"channel:\s+(\d+)").unwrap(),
            csi_data_regex: Regex::new(r"csi raw data:\s*\[([^\]]+)\]").unwrap(),
        }
    }

    /// Parse a multi-line CSI output block
    pub fn parse_packet(&self, output: &str) -> Option<CSIPacket> {
        let mut packet = CSIPacket {
            mac: self.extract_field(&self.mac_regex, output)?,
            rssi: output
                .lines()
                .find(|l| l.contains("rssi:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            channel: output
                .lines()
                .find(|l| l.contains("channel:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            timestamp: output
                .lines()
                .find(|l| l.contains("timestamp:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            rate: output
                .lines()
                .find(|l| l.contains("rate:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            noise_floor: output
                .lines()
                .find(|l| l.contains("noise floor:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            sig_len: output
                .lines()
                .find(|l| l.contains("sig len:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            rx_state: output
                .lines()
                .find(|l| l.contains("rx state:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            secondary_channel: output
                .lines()
                .find(|l| l.contains("secondary channel:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            sgi: output
                .lines()
                .find(|l| l.contains("sgi:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            ant: output
                .lines()
                .find(|l| l.contains("ant:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            ampdu_cnt: output
                .lines()
                .find(|l| l.contains("ampdu cnt:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            sig_mode: output
                .lines()
                .find(|l| l.contains("sig_mode:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            mcs: output
                .lines()
                .find(|l| l.contains("mcs:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            cwb: output
                .lines()
                .find(|l| l.contains("cwb:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            smoothing: output
                .lines()
                .find(|l| l.contains("smoothing:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            not_sounding: output
                .lines()
                .find(|l| l.contains("not sounding:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            aggregation: output
                .lines()
                .find(|l| l.contains("aggregation:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            stbc: output
                .lines()
                .find(|l| l.contains("stbc:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            fec_coding: output
                .lines()
                .find(|l| l.contains("fec coding:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            data_length: output
                .lines()
                .find(|l| l.contains("data length:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            csi_data: Vec::new(),
        };

        // Parse CSI data array
        packet.csi_data = self.extract_csi_array(output)?;

        Some(packet)
    }

    fn extract_field(&self, regex: &Regex, text: &str) -> Option<String> {
        regex.captures(text).and_then(|c| c.get(1)).map(|m| m.as_str().to_string())
    }

    fn extract_csi_array(&self, output: &str) -> Option<Vec<i32>> {
        let data_line = output
            .lines()
            .find(|l| l.starts_with("[0,") || l.starts_with("[-"))?;

        // Parse array of numbers: [1, 2, 3, ...]
        let cleaned = data_line
            .trim_matches(|c| c == '[' || c == ']')
            .trim();

        let values: Vec<i32> = cleaned
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        if values.is_empty() {
            None
        } else {
            Some(values)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_packet() {
        let sample_output = r#"
mac: 40:E1:E4:1F:81:C6
rssi: -61
rate: 11
noise floor: 161
channel: 11
timestamp: 3195366537
sig len: 100
rx state: 0
secondary channel: 0
sgi: 0
ant: 0
ampdu cnt: 0
sig_mode: 1
mcs: 7
cwb: 0
smoothing: 1
not sounding: 1
aggregation: 1
stbc: 1
fec coding: 0
sig_len: 100
data length: 384
csi raw data:
[-14, 5, -14, 6, -14, 7]
"#;

        let parser = CSIParser::new();
        let packet = parser.parse_packet(sample_output);

        assert!(packet.is_some());
        let pkt = packet.unwrap();
        assert_eq!(pkt.mac, "40:E1:E4:1F:81:C6");
        assert_eq!(pkt.rssi, -61);
        assert_eq!(pkt.channel, 11);
    }
}
```

---

## File 4: Update src/main.rs

Add these at the top to include your modules:

```rust
mod models;
mod csi;

use models::{AppState, CSIPacket};
use csi::CSIParser;
```

---

## How to Use These Files

1. Create the files in your src/ directory
2. Add to Cargo.toml:
   ```toml
   regex = "1"
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   ```
3. Build: `cargo build`
4. Test: `cargo test`

---

## Next Steps After This

1. Test the parser with real ESP output
2. Build UI layout to display CSIPacket
3. Implement real-time update loop
4. Add plotting of amplitude

You're now ready to build the visualization layer! 🚀
