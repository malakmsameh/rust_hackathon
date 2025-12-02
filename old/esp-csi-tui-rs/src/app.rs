use anyhow::Result;
use std::fs::File;
use std::io::Write;
use chrono::Local;

use crate::models::AppState;
use crate::csi::CSIParser;
use crate::serial::SerialHandler;

pub struct App {
    pub serial: SerialHandler,
    pub parser: CSIParser,
    pub state: AppState,
    pub output_buffer: Vec<String>,
    pub max_lines: usize,
    pub last_command: String,
    pub connection_status: String,
    pub data_display: String,
    pub captured_data: Vec<String>,
    pub is_collecting: bool,
}

impl App {
    /// Create a new app instance with serial connection
    pub fn new(port: &str, baud_rate: u32) -> Result<Self> {
        let serial = SerialHandler::new(port, baud_rate)?;
        let parser = CSIParser::new();
        let state = AppState::default();

        Ok(Self {
            serial,
            parser,
            state,
            output_buffer: Vec::new(),
            max_lines: 30,
            last_command: "None".to_string(),
            connection_status: "Connecting...".to_string(),
            data_display: "Waiting for data...".to_string(),
            captured_data: Vec::new(),
            is_collecting: false,
        })
    }

    /// Send a command to the device
    pub fn send_command(&mut self, command: &str) -> Result<()> {
        self.serial.send_command(command)?;
        self.last_command = command.to_string();
        self.output_buffer.push(format!("✓ Sent: {}", command));
        if self.output_buffer.len() > self.max_lines {
            self.output_buffer.remove(0);
        }
        Ok(())
    }

    /// Read and process data from serial
    pub fn read_data(&mut self) -> Result<()> {
        match self.serial.read_data() {
            Ok(data) if !data.is_empty() => {
                if !self.state.is_connected {
                    self.state.is_connected = true;
                    self.connection_status = "✓ Connected".to_string();
                }

                // Batch buffer operations to avoid repeated bounds checking
                let mut lines_to_add = Vec::new();
                let mut last_data_display = None;
                let mut packets_count = 0;

                for line in data.lines() {
                    let trimmed = line.trim();

                    if trimmed.is_empty() || trimmed.starts_with(">>>") {
                        continue;
                    }

                    lines_to_add.push(trimmed.to_string());

                    if trimmed.contains("WiFi Configuration Set") {
                        self.connection_status = "✓ WiFi configured".to_string();
                    } else if trimmed.contains("WiFi Connected") {
                        self.connection_status = "✓ WiFi Connected".to_string();
                        self.state.is_connected = true;
                    } else if trimmed.contains("CSI") || trimmed.contains("mac:") {
                        packets_count += 1;
                        last_data_display = Some(trimmed.to_string());
                        self.state.is_collecting = true;

                        if self.is_collecting {
                            self.captured_data.push(trimmed.to_string());
                        }
                    } else if trimmed.contains("Collection stopped") || trimmed.contains("stopped") {
                        self.is_collecting = false;
                    }
                }

                // Batch add lines to buffer
                self.output_buffer.extend(lines_to_add);
                
                // Keep buffer size under control
                if self.output_buffer.len() > self.max_lines {
                    let overflow = self.output_buffer.len() - self.max_lines;
                    self.output_buffer.drain(0..overflow);
                }

                // Update display once
                if let Some(display) = last_data_display {
                    self.data_display = format!("Latest: {}", display);
                }
                self.state.stats.packets_received += packets_count;
            }
            _ => {}
        }
        Ok(())
    }

    /// Export captured data to CSV
    pub fn save_to_csv(&self) -> Result<String> {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("csi_data_{}.csv", timestamp);

        let mut file = File::create(&filename)?;

        file.write_all(b"Timestamp,Data\n")?;

        for (idx, data) in self.captured_data.iter().enumerate() {
            let line = format!("{},{}\n", idx, data);
            file.write_all(line.as_bytes())?;
        }

        Ok(filename)
    }

    /// Get status string for display
    pub fn get_status(&self) -> String {
        format!(
            "Status: {} | Collecting: {} | Packets: {}",
            self.connection_status,
            if self.state.is_collecting { "✓ Yes" } else { "No" },
            self.state.stats.packets_received
        )
    }
}
