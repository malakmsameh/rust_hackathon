# Quick Win Implementation Examples

This document provides concrete code examples for implementing the 5 Quick Wins to get started immediately.

---

## Quick Win #1: Extract main.rs (30 min)

### Step 1: Create `src/app.rs`

```rust
use anyhow::Result;
use std::fs::File;
use std::io::Write;
use chrono::Local;

use crate::models::{AppState, CSIPacket};
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
    pub fn new(port: &str) -> Result<Self> {
        let serial = SerialHandler::new(port, 115200)?;
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

                for line in data.lines() {
                    let trimmed = line.trim();

                    if trimmed.is_empty() || trimmed.starts_with(">>>") {
                        continue;
                    }

                    self.output_buffer.push(trimmed.to_string());

                    if trimmed.contains("WiFi Configuration Set") {
                        self.connection_status = "✓ WiFi configured".to_string();
                    } else if trimmed.contains("WiFi Connected") {
                        self.connection_status = "✓ WiFi Connected".to_string();
                        self.state.is_connected = true;
                    } else if trimmed.contains("CSI") || trimmed.contains("mac:") {
                        self.state.stats.packets_received += 1;
                        self.data_display = format!("Latest: {}", trimmed);
                        self.state.is_collecting = true;

                        if self.is_collecting {
                            self.captured_data.push(trimmed.to_string());
                        }
                    } else if trimmed.contains("Collection stopped") || trimmed.contains("stopped") {
                        self.is_collecting = false;
                    }

                    if self.output_buffer.len() > self.max_lines {
                        self.output_buffer.remove(0);
                    }
                }
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
```

### Step 2: Update `src/main.rs`

Replace the old App struct definition and all methods with:

```rust
// Module declarations
mod app;
mod models;
mod csi;
mod serial;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use std::io;
use std::time::Duration;

fn main() -> Result<()> {
    env_logger::init();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    match App::new("/dev/ttyUSB0") {
        Ok(mut app) => {
            let res = run_app(&mut terminal, &mut app);

            // Restore terminal
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;

            if let Err(err) = res {
                println!("Error: {:?}", err);
            }
            Ok(())
        }
        Err(e) => {
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;

            eprintln!("❌ Failed to connect to ESP32!");
            eprintln!("Error: {}", e);
            eprintln!("\nTroubleshooting:");
            eprintln!("1. Ensure ESP32 is plugged in via USB");
            eprintln!("2. Check: ls /dev/ttyUSB*");
            eprintln!("3. Run with sudo: sudo cargo run");
            Err(e)
        }
    }
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        app.read_data()?;

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                    Constraint::Percentage(25),
                    Constraint::Length(12),
                ])
                .split(f.area());

            // Header
            let header = Paragraph::new(vec![
                Line::from(
                    vec![
                        Span::styled("ESP32 CSI Monitor", Style::default().fg(Color::Cyan)),
                        Span::raw(" - Real-time WiFi Channel State Information Visualization"),
                    ]
                ),
            ])
            .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Cyan)))
            .style(Style::default().fg(Color::White));
            f.render_widget(header, chunks[0]);

            // Status Panel
            let status_lines = vec![
                Line::from(vec![
                    Span::styled("Connection: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&app.connection_status),
                ]),
                Line::from(vec![
                    Span::styled("Last Command: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&app.last_command),
                ]),
            ];
            let status = Paragraph::new(status_lines)
                .block(Block::default().borders(Borders::ALL).title("Status"))
                .style(Style::default().fg(Color::Green));
            f.render_widget(status, chunks[1]);

            // Device Output Window
            let output_text: Vec<Line> = app
                .output_buffer
                .iter()
                .map(|line| Line::from(line.clone()))
                .collect();

            let output = Paragraph::new(output_text)
                .block(Block::default().borders(Borders::ALL).title("Device Output"))
                .wrap(Wrap { trim: false })
                .style(Style::default().fg(Color::Gray));
            f.render_widget(output, chunks[2]);

            // Live Data Display
            let data_display = Paragraph::new(vec![
                Line::from(vec![
                    Span::styled("Packets: ", Style::default().fg(Color::Magenta)),
                    Span::raw(app.state.stats.packets_received.to_string()),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Latest Data: ", Style::default().fg(Color::Magenta)),
                ]),
                Line::from(Span::styled(&app.data_display, Style::default().fg(Color::White))),
            ])
            .block(Block::default().borders(Borders::ALL).title("Live Data"))
            .style(Style::default().fg(Color::Magenta));
            f.render_widget(data_display, chunks[3]);

            // Commands Panel
            let commands_text = vec![
                Line::from(Span::styled("━━━ QUICK COMMANDS ━━━", Style::default().fg(Color::Yellow))),
                Line::from(""),
                Line::from(vec![
                    Span::styled("w", Style::default().fg(Color::Green)),
                    Span::raw(" = Sniffer  |  "),
                    Span::styled("c", Style::default().fg(Color::Green)),
                    Span::raw(" = Start  |  "),
                    Span::styled("x", Style::default().fg(Color::Green)),
                    Span::raw(" = Stop"),
                ]),
                Line::from(vec![
                    Span::styled("1", Style::default().fg(Color::Green)),
                    Span::raw(" = Traffic 10Hz  |  "),
                    Span::styled("2", Style::default().fg(Color::Green)),
                    Span::raw(" = CSI Config"),
                ]),
                Line::from(vec![
                    Span::styled("s", Style::default().fg(Color::Green)),
                    Span::raw(" = Show Config  |  "),
                    Span::styled("h", Style::default().fg(Color::Green)),
                    Span::raw(" = Help"),
                ]),
                Line::from(vec![
                    Span::styled("e", Style::default().fg(Color::Cyan)),
                    Span::raw(" = Export CSV  |  "),
                    Span::styled("q", Style::default().fg(Color::Red)),
                    Span::raw(" = Quit"),
                ]),
            ];
            let commands = Paragraph::new(commands_text)
                .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Yellow)))
                .style(Style::default().fg(Color::White));
            f.render_widget(commands, chunks[4]);
        })?;

        if event::poll(Duration::from_millis(1))? {
            if let Ok(Event::Key(key)) = event::read() {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                    KeyCode::Char('h') | KeyCode::Char('H') => {
                        app.send_command("help")?;
                    }
                    KeyCode::Char('w') | KeyCode::Char('W') => {
                        app.send_command("set-wifi --mode=sniffer --set-channel=6")?;
                    }
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        app.send_command("start --duration=60")?;
                        app.is_collecting = true;
                    }
                    KeyCode::Char('x') | KeyCode::Char('X') => {
                        app.send_command("stop")?;
                        app.is_collecting = false;
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        app.send_command("show-config")?;
                    }
                    KeyCode::Char('1') => {
                        app.send_command("set-traffic --frequency-hz=10")?;
                    }
                    KeyCode::Char('2') => {
                        app.send_command("set-csi")?;
                    }
                    KeyCode::Char('e') | KeyCode::Char('E') => {
                        match app.save_to_csv() {
                            Ok(filename) => {
                                app.output_buffer.push(format!("✓ Data exported to: {}", filename));
                            }
                            Err(e) => {
                                app.output_buffer.push(format!("✗ Export failed: {}", e));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
```

---

## Quick Win #2: Add Config File Support (45 min)

### Create `src/config.rs`

```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub serial: SerialConfig,
    pub collection: CollectionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialConfig {
    pub port: String,
    pub baud_rate: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionConfig {
    pub default_channel: u8,
    pub default_duration: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            serial: SerialConfig {
                port: "/dev/ttyUSB0".to_string(),
                baud_rate: 115200,
            },
            collection: CollectionConfig {
                default_channel: 6,
                default_duration: 60,
            },
        }
    }
}

/// Get config file path: ~/.esp-csi/config.toml
pub fn get_config_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".esp-csi");
    path.push("config.toml");
    path
}

/// Load configuration from file or use defaults
pub fn load_config() -> Result<Config> {
    let config_path = get_config_path();

    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}

/// Save configuration to file
pub fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path();
    
    // Create parent directory if needed
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let content = toml::to_string_pretty(config)?;
    std::fs::write(&config_path, content)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.serial.port, "/dev/ttyUSB0");
        assert_eq!(config.serial.baud_rate, 115200);
        assert_eq!(config.collection.default_channel, 6);
    }

    #[test]
    fn test_serialize_config() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("/dev/ttyUSB0"));
    }
}
```

### Update `Cargo.toml`

Add these dependencies:

```toml
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
dirs = "5.0"
```

### Update `src/main.rs`

Modify to use config:

```rust
mod config;
// ... other mods

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Load configuration
    let config = config::load_config()?;

    // Setup terminal...
    // ... existing code ...

    // Use config for serial port and baud rate
    match App::new(&config.serial.port, config.serial.baud_rate) {
        Ok(mut app) => {
            // ... rest of main
        }
        Err(e) => {
            eprintln!("Error: Could not connect to {}", config.serial.port);
            eprintln!("Details: {}", e);
            Err(e)
        }
    }
}
```

### Update `src/app.rs`

Modify App::new to accept baud_rate:

```rust
impl App {
    pub fn new(port: &str, baud_rate: u32) -> Result<Self> {
        let serial = SerialHandler::new(port, baud_rate)?;
        // ... rest of implementation
    }
}
```

---

## Quick Win #3: Fix Parser Integration (1 hour)

### Create `src/csi/processor.rs`

```rust
use crate::models::CSIPacket;
use crate::csi::CSIParser;

pub struct CSIProcessor {
    parser: CSIParser,
}

impl CSIProcessor {
    pub fn new() -> Self {
        Self {
            parser: CSIParser::new(),
        }
    }

    /// Process raw output and extract CSI packets
    pub fn process_output(&self, output: &str) -> Vec<CSIPacket> {
        let mut packets = Vec::new();

        // Split by logical blocks (empty lines or specific markers)
        let blocks: Vec<&str> = output.split("\n\n").collect();

        for block in blocks {
            if let Some(packet) = self.parser.parse_packet(block) {
                packets.push(packet);
            }
        }

        packets
    }
}

impl Default for CSIProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_single_packet() {
        let sample = r#"mac: 40:E1:E4:1F:81:C6
rssi: -61
channel: 11
timestamp: 1000000
rate: 11
noise floor: 161
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
data length: 384
csi raw data:
[-14, 5, -14, 6, -14, 7]"#;

        let processor = CSIProcessor::new();
        let packets = processor.process_output(sample);
        
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].mac, "40:E1:E4:1F:81:C6");
    }
}
```

### Update `src/csi/mod.rs`

```rust
pub mod parser;
pub mod processor;

pub use parser::CSIParser;
pub use processor::CSIProcessor;
```

### Update `src/app.rs`

```rust
use crate::csi::{CSIParser, CSIProcessor};

pub struct App {
    pub serial: SerialHandler,
    pub parser: CSIParser,
    pub processor: CSIProcessor,  // Add this
    pub state: AppState,
    // ... rest of fields
}

impl App {
    pub fn new(port: &str, baud_rate: u32) -> Result<Self> {
        let serial = SerialHandler::new(port, baud_rate)?;
        let parser = CSIParser::new();
        let processor = CSIProcessor::new();  // Initialize
        let state = AppState::default();

        Ok(Self {
            serial,
            parser,
            processor,
            state,
            // ... rest
        })
    }

    pub fn read_data(&mut self) -> Result<()> {
        match self.serial.read_data() {
            Ok(data) if !data.is_empty() => {
                if !self.state.is_connected {
                    self.state.is_connected = true;
                    self.connection_status = "✓ Connected".to_string();
                }

                // Process and extract CSI packets
                let packets = self.processor.process_output(&data);
                
                for packet in packets {
                    self.state.stats.packets_received += 1;
                    self.state.latest_packet = Some(packet.clone());
                    self.state.packet_buffer.push(packet.clone());
                    
                    self.data_display = format!(
                        "MAC: {} | RSSI: {}dBm | Ch: {}",
                        packet.mac, packet.rssi, packet.channel
                    );
                    
                    if self.is_collecting {
                        // Store for export
                        self.captured_data.push(serde_json::to_string(&packet)?);
                    }
                }

                // Also add raw lines to output buffer
                for line in data.lines() {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() && !trimmed.starts_with(">>>") {
                        self.output_buffer.push(trimmed.to_string());
                        if self.output_buffer.len() > self.max_lines {
                            self.output_buffer.remove(0);
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
```

---

## Quick Win #4: Implement ASCII Plot (1.5 hours)

### Create `src/visualization/plots.rs`

```rust
use crate::models::CSIPacket;

pub struct ASCIIPlotter {
    width: usize,
    height: usize,
}

impl ASCIIPlotter {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    /// Create ASCII plot of amplitude data
    pub fn plot_amplitude(&self, packets: &[CSIPacket]) -> String {
        if packets.is_empty() {
            return "No data".to_string();
        }

        let mut plot = String::new();

        // Get amplitude data from most recent packet
        let latest = &packets[packets.len() - 1];
        let amplitudes = latest.get_amplitude();

        if amplitudes.is_empty() {
            return "No amplitude data".to_string();
        }

        // Find min/max for scaling
        let max_amp = amplitudes
            .iter()
            .fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let min_amp = amplitudes
            .iter()
            .fold(f32::INFINITY, |a, &b| a.min(b));
        let range = (max_amp - min_amp).max(1.0);

        // Plot from top to bottom
        for row in (0..self.height).rev() {
            let threshold = min_amp + (range * row as f32 / self.height as f32);

            plot.push('│');

            for amp in &amplitudes {
                if *amp > threshold {
                    plot.push('█');
                } else if *amp > threshold - range / (self.height as f32 * 2.0) {
                    plot.push('▄');
                } else {
                    plot.push(' ');
                }
            }

            plot.push('│');
            plot.push('\n');
        }

        // Add bottom border
        plot.push('└');
        for _ in 0..amplitudes.len().min(self.width) {
            plot.push('─');
        }
        plot.push('┘');

        plot
    }

    /// Create ASCII bar chart
    pub fn plot_bar_chart(&self, values: &[f32], label: &str) -> String {
        let mut output = String::new();
        output.push_str(&format!("{}\n", label));

        let max_val = values
            .iter()
            .fold(f32::NEG_INFINITY, |a, &b| a.max(b))
            .max(1.0);

        for (i, &val) in values.iter().enumerate().take(self.width) {
            let bar_len = ((val / max_val) * 30.0) as usize;
            output.push_str(&format!("{:3}: ", i));
            for _ in 0..bar_len {
                output.push('█');
            }
            output.push_str(&format!(" {:.2}\n", val));
        }

        output
    }
}

impl Default for ASCIIPlotter {
    fn default() -> Self {
        Self::new(50, 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plotter_creation() {
        let plotter = ASCIIPlotter::new(80, 20);
        assert_eq!(plotter.width, 80);
        assert_eq!(plotter.height, 20);
    }
}
```

### Create `src/visualization/mod.rs`

```rust
pub mod plots;

pub use plots::ASCIIPlotter;
```

### Update `src/main.rs` to show plots

Add in the UI rendering section:

```rust
// Import at top
use crate::visualization::ASCIIPlotter;

// In the render closure, add a plot panel
let plotter = ASCIIPlotter::new(60, 8);

if !app.state.packet_buffer.is_empty() {
    let plot_str = plotter.plot_amplitude(&app.state.packet_buffer);
    let plot = Paragraph::new(plot_str)
        .block(Block::default().borders(Borders::ALL).title("Amplitude Plot"))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(plot, chunks[3]);  // Adjust constraint as needed
}
```

---

## Quick Win #5: Improve CSV Export (45 min)

### Create `src/storage/csv.rs`

```rust
use crate::models::CSIPacket;
use serde::Serialize;
use std::path::Path;
use anyhow::Result;

#[derive(Serialize)]
pub struct CSIRecord {
    pub timestamp_us: u64,
    pub mac_address: String,
    pub rssi_dbm: i32,
    pub channel: u8,
    pub rate_mbps: u8,
    pub signal_length: u16,
    pub subcarrier_count: usize,
    pub amplitude_peak: f32,
    pub amplitude_mean: f32,
}

impl From<&CSIPacket> for CSIRecord {
    fn from(packet: &CSIPacket) -> Self {
        let amplitudes = packet.get_amplitude();
        
        let peak = amplitudes
            .iter()
            .fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        
        let mean = if !amplitudes.is_empty() {
            amplitudes.iter().sum::<f32>() / amplitudes.len() as f32
        } else {
            0.0
        };

        Self {
            timestamp_us: packet.timestamp,
            mac_address: packet.mac.clone(),
            rssi_dbm: packet.rssi,
            channel: packet.channel,
            rate_mbps: packet.rate,
            signal_length: packet.sig_len,
            subcarrier_count: packet.subcarrier_count(),
            amplitude_peak: peak,
            amplitude_mean: mean,
        }
    }
}

pub struct CSVExporter;

impl CSVExporter {
    pub fn export_packets(
        packets: &[CSIPacket],
        output_path: &Path,
    ) -> Result<()> {
        let mut writer = csv::Writer::from_path(output_path)?;

        for packet in packets {
            let record = CSIRecord::from(packet);
            writer.serialize(record)?;
        }

        writer.flush()?;
        Ok(())
    }

    pub fn export_with_metadata(
        packets: &[CSIPacket],
        output_path: &Path,
        metadata: &[(String, String)],
    ) -> Result<()> {
        let mut file = std::fs::File::create(output_path)?;
        use std::io::Write;

        // Write metadata as comments
        for (key, value) in metadata {
            writeln!(file, "# {}: {}", key, value)?;
        }
        writeln!(file)?;

        let mut writer = csv::Writer::from_writer(file);

        for packet in packets {
            let record = CSIRecord::from(packet);
            writer.serialize(record)?;
        }

        writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_conversion() {
        let packet = CSIPacket {
            mac: "00:11:22:33:44:55".to_string(),
            rssi: -50,
            channel: 6,
            timestamp: 1000000,
            rate: 65,
            noise_floor: 161,
            sig_len: 100,
            rx_state: 0,
            secondary_channel: 0,
            sgi: 0,
            ant: 0,
            ampdu_cnt: 0,
            sig_mode: 1,
            mcs: 7,
            cwb: 0,
            smoothing: 1,
            not_sounding: 1,
            aggregation: 1,
            stbc: 1,
            fec_coding: 0,
            data_length: 384,
            csi_data: vec![10, 5, 20, 8, 15, 12],
        };

        let record = CSIRecord::from(&packet);
        assert_eq!(record.mac_address, "00:11:22:33:44:55");
        assert_eq!(record.rssi_dbm, -50);
        assert_eq!(record.channel, 6);
    }
}
```

### Create `src/storage/mod.rs`

```rust
pub mod csv;

pub use csv::{CSIRecord, CSVExporter};
```

### Update `src/app.rs` CSV export

```rust
use crate::storage::CSVExporter;

impl App {
    pub fn save_to_csv(&self) -> Result<String> {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("csi_data_{}.csv", timestamp);

        let metadata = vec![
            ("Export Date".to_string(), chrono::Local::now().to_rfc3339()),
            ("Packets Collected".to_string(), self.state.packet_buffer.len().to_string()),
            ("Connection Status".to_string(), self.connection_status.clone()),
        ];

        let path = std::path::Path::new(&filename);
        CSVExporter::export_with_metadata(
            &self.state.packet_buffer,
            path,
            &metadata,
        )?;

        Ok(filename)
    }
}
```

---

## Integration Checklist

After implementing all 5 Quick Wins:

- [ ] `cargo build` compiles without errors
- [ ] `cargo run` starts the application
- [ ] Configuration loads from ~/.esp-csi/config.toml (or defaults)
- [ ] CSI packets parse correctly
- [ ] ASCII plot displays
- [ ] CSV export creates properly formatted files
- [ ] All quick wins work together seamlessly
- [ ] No compiler warnings

---

## Next Steps After Quick Wins

1. **Test thoroughly** - Verify each quick win works independently and together
2. **Commit progress** - Create commits for each quick win
3. **Team feedback** - Get feedback on code organization
4. **Move to Phase 2** - Begin working on visualization and storage enhancements

---

This provides a clear starting point with concrete, copy-paste-ready code!
