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
    pub last_rendered_count: usize,  // Track last plot render size
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
            last_rendered_count: 0,
        }
    }
}
