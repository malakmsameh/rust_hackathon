use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsiMeasurement {
    pub timestamp: DateTime<Utc>,
    pub channel: u8,
    pub bandwidth: u16,
    pub rssi: i8,
    pub noise_floor: i8,
    pub subcarrier_data: Vec<ComplexNumber>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ComplexNumber {
    pub real: f32,
    pub imag: f32,
}

impl ComplexNumber {
    pub fn new(real: f32, imag: f32) -> Self {
        Self { real, imag }
    }

    pub fn magnitude(&self) -> f32 {
        (self.real.powi(2) + self.imag.powi(2)).sqrt()
    }

    pub fn phase(&self) -> f32 {
        self.imag.atan2(self.real)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub port: String,
    pub baud_rate: u32,
    pub channel: u8,
    pub bandwidth: u16,
    pub collection_interval_ms: u64,
}

impl Default for DeviceConfig {
    fn default() -> Self {
        Self {
            port: "/dev/ttyUSB0".to_string(),
            baud_rate: 115200,
            channel: 6,
            bandwidth: 20,
            collection_interval_ms: 100,
        }
    }
}

#[derive(Debug)]
pub struct AppState {
    pub measurements: Vec<CsiMeasurement>,
    pub device_config: DeviceConfig,
    pub is_connected: bool,
    pub is_collecting: bool,
    pub current_tab: usize,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            measurements: Vec::new(),
            device_config: DeviceConfig::default(),
            is_connected: false,
            is_collecting: false,
            current_tab: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceCommand {
    Configure(DeviceConfig),
    StartCollection,
    StopCollection,
    Query,
    Disconnect,
}

#[derive(Debug, Clone)]
pub struct StreamingConfig {
    pub enabled: bool,
    pub server_url: String,
    pub format: StreamFormat,
}

#[derive(Debug, Clone)]
pub enum StreamFormat {
    Rrd,
    Json,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            server_url: "http://localhost:9090".to_string(),
            format: StreamFormat::Json,
        }
    }
}
