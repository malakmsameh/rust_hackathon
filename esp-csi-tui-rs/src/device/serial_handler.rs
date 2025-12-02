use serialport::SerialPort;
use std::io::{Read, Write};
use std::time::Duration;
use anyhow::{Result, anyhow};

pub struct SerialHandler {
    port: Option<Box<dyn SerialPort>>,
    port_name: String,
    baud_rate: u32,
}

impl SerialHandler {
    pub fn new(port_name: String, baud_rate: u32) -> Self {
        Self {
            port: None,
            port_name,
            baud_rate,
        }
    }

    pub fn connect(&mut self) -> Result<()> {
        match serialport::new(&self.port_name, self.baud_rate)
            .timeout(Duration::from_secs(1))
            .open()
        {
            Ok(port) => {
                self.port = Some(port);
                tracing::info!("Connected to serial port: {}", self.port_name);
                Ok(())
            }
            Err(e) => Err(anyhow!("Failed to open serial port: {}", e)),
        }
    }

    pub fn disconnect(&mut self) -> Result<()> {
        self.port = None;
        tracing::info!("Disconnected from serial port");
        Ok(())
    }

    pub fn write_command(&mut self, command: &[u8]) -> Result<()> {
        if let Some(ref mut port) = self.port {
            port.write_all(command)
                .map_err(|e| anyhow!("Failed to write to serial port: {}", e))?;
            port.flush()
                .map_err(|e| anyhow!("Failed to flush serial port: {}", e))?;
            Ok(())
        } else {
            Err(anyhow!("Serial port not connected"))
        }
    }

    pub fn read_data(&mut self, buffer: &mut [u8]) -> Result<usize> {
        if let Some(ref mut port) = self.port {
            port.read(buffer)
                .map_err(|e| anyhow!("Failed to read from serial port: {}", e))
        } else {
            Err(anyhow!("Serial port not connected"))
        }
    }

    pub fn is_connected(&self) -> bool {
        self.port.is_some()
    }
}
