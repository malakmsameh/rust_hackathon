use serialport::SerialPort;
use std::io::{Read, Write};
use std::time::Duration;
use anyhow::{Result, anyhow};
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver};

pub struct SerialHandler {
    port: Arc<Mutex<Box<dyn SerialPort>>>,
    reader: Receiver<String>,
    initialized: bool,
}

impl SerialHandler {
    /// Open a serial connection to the ESP device
    pub fn new(port_name: &str, baud_rate: u32) -> Result<Self> {
        // Open port with ultra-fast timeout
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(10))  // Ultra-fast timeout
            .stop_bits(serialport::StopBits::One)
            .data_bits(serialport::DataBits::Eight)
            .parity(serialport::Parity::None)
            .flow_control(serialport::FlowControl::None)
            .open()
            .map_err(|e| anyhow!("Failed to open serial port {}: {}", port_name, e))?;

        let port = Arc::new(Mutex::new(port));
        let port_clone = Arc::clone(&port);

        // Spawn a background thread to continuously read from serial
        let (tx, rx) = channel();

        thread::spawn(move || {
            let mut buffer = [0u8; 4096];

            loop {
                if let Ok(mut p) = port_clone.lock() {
                    match p.read(&mut buffer) {
                        Ok(n) if n > 0 => {
                            let data = String::from_utf8_lossy(&buffer[..n]).to_string();
                            let _ = tx.send(data);
                        }
                        _ => {
                            // Yield without sleep for maximum speed
                            std::thread::yield_now();
                        }
                    }
                }
            }
        });

        thread::sleep(Duration::from_millis(500));

        Ok(SerialHandler {
            port,
            reader: rx,
            initialized: true,
        })
    }

    /// Send a command to the device (instant)
    pub fn send_command(&mut self, command: &str) -> Result<()> {
        let cmd = format!("{}\r\n", command);

        if let Ok(mut p) = self.port.lock() {
            p.write_all(cmd.as_bytes())
                .map_err(|e| anyhow!("Failed to send command: {}", e))?;
            p.flush()
                .map_err(|e| anyhow!("Failed to flush: {}", e))?;
        }

        Ok(())
    }

    /// Read available data from serial port
    pub fn read_data(&mut self) -> Result<String> {
        // Try to get any pending data from the channel
        let mut result = String::new();

        // Non-blocking read from channel - get all pending data
        while let Ok(data) = self.reader.try_recv() {
            result.push_str(&data);
        }

        Ok(result)
    }

    /// Check if connection is alive
    pub fn is_connected(&self) -> bool {
        self.initialized
    }
}
