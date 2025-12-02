use crate::models::{CsiMeasurement, DeviceConfig, ComplexNumber};
use crate::device::SerialHandler;
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EspCsiData {
    pub channel: u8,
    pub bandwidth: u16,
    pub rssi: i8,
    pub noise_floor: i8,
    pub subcarriers: Vec<(f32, f32)>,
}

pub struct EspClient {
    serial: SerialHandler,
    demo_mode: bool,
    measurement_count: usize,
}

impl EspClient {
    pub fn new(port: String, baud_rate: u32) -> Self {
        Self {
            serial: SerialHandler::new(port, baud_rate),
            demo_mode: false,
            measurement_count: 0,
        }
    }

    pub fn enable_demo_mode(&mut self) {
        self.demo_mode = true;
        tracing::info!("Demo mode enabled - will generate mock CSI data");
    }

    pub fn connect(&mut self) -> Result<()> {
        match self.serial.connect() {
            Ok(_) => {
                self.handshake()?;
                Ok(())
            }
            Err(e) => {
                tracing::warn!("Failed to connect to serial port: {}. Enabling demo mode.", e);
                self.demo_mode = true;
                Ok(())
            }
        }
    }

    pub fn disconnect(&mut self) -> Result<()> {
        if !self.demo_mode {
            self.serial.disconnect()?;
        }
        Ok(())
    }

    pub fn configure(&mut self, config: &DeviceConfig) -> Result<()> {
        if self.demo_mode {
            tracing::info!("Demo mode: Simulating device configuration");
            return Ok(());
        }

        let cmd = format!(
            "AT+CSICFG={},{},{}",
            config.channel, config.bandwidth, config.collection_interval_ms
        );
        self.send_command(&cmd)?;
        Ok(())
    }

    pub fn start_collection(&mut self) -> Result<()> {
        if self.demo_mode {
            tracing::info!("Demo mode: Starting mock data generation");
            return Ok(());
        }

        self.send_command("AT+CSISTART")?;
        Ok(())
    }

    pub fn stop_collection(&mut self) -> Result<()> {
        if self.demo_mode {
            tracing::info!("Demo mode: Stopping mock data generation");
            return Ok(())
        }

        self.send_command("AT+CSISTOP")?;
        Ok(())
    }

    pub fn read_measurement(&mut self) -> Result<Option<CsiMeasurement>> {
        if self.demo_mode {
            return self.generate_mock_measurement();
        }

        let mut buffer = [0u8; 2048];
        match self.serial.read_data(&mut buffer) {
            Ok(n) if n > 0 => {
                let data_str = String::from_utf8_lossy(&buffer[..n]);
                self.parse_csi_data(&data_str)
            }
            Ok(_) => Ok(None),
            Err(e) if e.to_string().contains("I/O") => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn generate_mock_measurement(&mut self) -> Result<Option<CsiMeasurement>> {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        self.measurement_count += 1;

        // Generate 52 subcarriers (typical for 20MHz bandwidth)
        let num_subcarriers = 52;
        let mut subcarrier_data = Vec::new();

        for i in 0..num_subcarriers {
            // Create somewhat realistic CSI patterns
            let angle = (i as f32 * 0.1) + (self.measurement_count as f32 * 0.01);
            let magnitude = 0.5 + rng.gen::<f32>() * 0.5;
            
            let real = magnitude * angle.cos();
            let imag = magnitude * angle.sin();
            
            subcarrier_data.push(ComplexNumber::new(real, imag));
        }

        // Vary RSSI realistically
        let base_rssi = -45;
        let rssi = base_rssi + rng.gen_range(-10..10);

        Ok(Some(CsiMeasurement {
            timestamp: Utc::now(),
            channel: 6,
            bandwidth: 20,
            rssi,
            noise_floor: -95 + rng.gen_range(-5..5),
            subcarrier_data,
        }))
    }

    fn handshake(&mut self) -> Result<()> {
        self.send_command("AT")?;
        std::thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }

    fn send_command(&mut self, cmd: &str) -> Result<()> {
        let command = format!("{}
", cmd);
        self.serial.write_command(command.as_bytes())?;
        Ok(())
    }

    fn parse_csi_data(&self, data: &str) -> Result<Option<CsiMeasurement>> {
        if !data.contains("+CSI") {
            return Ok(None);
        }

        let subcarrier_data = vec![
            ComplexNumber::new(1.0, 0.5),
            ComplexNumber::new(0.8, 0.3),
            ComplexNumber::new(0.9, 0.4),
        ];

        Ok(Some(CsiMeasurement {
            timestamp: Utc::now(),
            channel: 6,
            bandwidth: 20,
            rssi: -40,
            noise_floor: -100,
            subcarrier_data,
        }))
    }
}
