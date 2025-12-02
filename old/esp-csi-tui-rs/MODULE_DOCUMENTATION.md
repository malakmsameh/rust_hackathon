# Module Interface Documentation

Complete API reference for all refactored modules in esp-csi-tui-rs.

## app.rs - Application State Management

### Struct: App
Central application state container.

```rust
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
```

### Methods

```rust
impl App {
    /// Create a new app instance with serial connection
    pub fn new(port: &str, baud_rate: u32) -> Result<Self>
    
    /// Send a command to the connected device
    pub fn send_command(&mut self, command: &str) -> Result<()>
    
    /// Read and parse data from serial connection
    pub fn read_data(&mut self) -> Result<()>
    
    /// Export collected packets to CSV
    pub fn save_to_csv(&self) -> Result<String>
    
    /// Get formatted status string
    pub fn get_status(&self) -> String
}
```

---

## config.rs - Configuration Management

### Struct: Config
Main configuration container.

```rust
pub struct Config {
    pub serial: SerialConfig,
    pub collection: CollectionConfig,
}
```

### Struct: SerialConfig
Serial port settings.

```rust
pub struct SerialConfig {
    pub port: String,              // e.g., "/dev/ttyUSB0"
    pub baud_rate: u32,           // e.g., 115200
}
```

### Struct: CollectionConfig
Data collection settings.

```rust
pub struct CollectionConfig {
    pub channel: u8,              // WiFi channel (1-14)
    pub duration_seconds: u64,    // Collection duration
}
```

### Functions

```rust
/// Get the configuration file path (~/.esp-csi/config.toml)
pub fn get_config_path() -> Result<PathBuf>

/// Load configuration from file, with fallback defaults
pub fn load_config() -> Result<Config>

/// Save configuration to file
pub fn save_config(config: &Config) -> Result<()>
```

---

## csi/mod.rs - CSI Module

### Exports

```rust
pub mod parser;
pub mod processor;
pub use parser::CSIParser;
```

---

## csi/parser.rs - CSI Packet Parsing

### Struct: CSIParser
Regex-based CSI packet parser.

```rust
pub struct CSIParser {
    mac_regex: Regex,
    rssi_regex: Regex,
    channel_regex: Regex,
    csi_data_regex: Regex,
}
```

### Methods

```rust
impl CSIParser {
    /// Create a new parser instance
    pub fn new() -> Self
    
    /// Parse a single packet from raw output
    pub fn parse_packet(&self, output: &str) -> Option<CSIPacket>
    
    /// Extract field using regex
    fn extract_field(&self, regex: &Regex, text: &str) -> Option<String>
    
    /// Extract CSI data array
    fn extract_csi_array(&self, output: &str) -> Option<Vec<i32>>
}
```

---

## csi/processor.rs - CSI Data Pipeline

### Struct: CSIProcessor
Bridges parser and application data flow.

```rust
pub struct CSIProcessor {
    parser: CSIParser,
}
```

### Methods

```rust
impl CSIProcessor {
    /// Create a new processor with embedded parser
    pub fn new() -> Self
    
    /// Process raw serial output into structured packets
    pub fn process_output(&self, output: &str) -> Vec<CSIPacket>
}
```

### Data Flow

```
Raw Serial Data (String)
        ↓
CSIProcessor::process_output()
        ↓
Vec<CSIPacket> (Structured)
        ↓
app.state.packet_buffer (Storage)
```

---

## models.rs - Data Structures

### Struct: CSIPacket
Represents a single CSI measurement.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CSIPacket {
    pub mac: String,              // Source MAC address
    pub rssi: i32,                // Signal strength (dBm)
    pub channel: u8,              // WiFi channel
    pub timestamp: u64,           // Unix timestamp
    pub rate: u8,                 // Data rate (Mbps)
    pub noise_floor: i32,         // Noise level
    pub sig_len: u16,             // Packet length
    pub rx_state: u8,             // RX state code
    pub real_addr: u32,           // Address pointer
    pub len: u32,                 // CSI length
    pub first_word_invalid: bool, // Validity flag
    pub csi_len: u32,             // CSI array length
    pub data: Vec<u8>,            // Raw CSI data
}
```

### Methods

```rust
impl CSIPacket {
    /// Extract amplitude values from CSI data
    pub fn get_amplitude(&self) -> Vec<f32>
    
    /// Extract phase values from CSI data
    pub fn get_phase(&self) -> Vec<f32>
    
    /// Get number of subcarriers
    pub fn subcarrier_count(&self) -> usize
}
```

### Struct: AppState
Application state container.

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppState {
    pub packet_buffer: Vec<CSIPacket>,
    pub stats: CollectionStats,
    pub current_config: DeviceConfig,
    pub latest_packet: Option<CSIPacket>,
}
```

### Struct: CollectionStats
Statistics about current collection.

```rust
pub struct CollectionStats {
    pub packets_received: u64,
    pub rssi_min: i32,
    pub rssi_max: i32,
    pub rssi_avg: i32,
    pub start_time: u64,
    pub data_rate: f32,
}
```

---

## storage/mod.rs - Storage Module

### Exports

```rust
pub mod csv;
```

---

## storage/csv.rs - CSV Export

### Struct: CSIRecord
Serde-serializable packet record for CSV export.

```rust
#[derive(Serialize)]
pub struct CSIRecord {
    pub timestamp_us: u64,        // Packet timestamp
    pub mac_address: String,      // Source MAC
    pub rssi_dbm: i32,            // Signal strength
    pub channel: u8,              // WiFi channel
    pub rate_mbps: u8,            // Data rate
    pub signal_length: u16,       // Packet length
    pub subcarrier_count: usize,  // Subcarrier count
    pub amplitude_peak: f32,      // Peak amplitude
    pub amplitude_mean: f32,      // Mean amplitude
}
```

### Trait Implementations

```rust
impl From<&CSIPacket> for CSIRecord {
    /// Convert CSIPacket to CSV record automatically
    fn from(packet: &CSIPacket) -> Self
}
```

### Struct: CSVExporter
Static CSV export methods.

```rust
pub struct CSVExporter;
```

### Methods

```rust
impl CSVExporter {
    /// Export packets to CSV file
    pub fn export_packets(
        packets: &[CSIPacket],
        output_path: &Path,
    ) -> Result<()>
    
    /// Export packets with metadata comment headers
    pub fn export_with_metadata(
        packets: &[CSIPacket],
        output_path: &Path,
        metadata: &[(String, String)],
    ) -> Result<()>
}
```

### Metadata Format

```csv
# Export Time: 2024-12-02 17:45:30.123456 +00:00
# Packet Count: 1250
# Collection Status: Active

timestamp_us,mac_address,rssi_dbm,channel,rate_mbps,...
1000000,00:11:22:33:44:55,-50,6,65,...
1001000,00:11:22:33:44:55,-49,6,65,...
```

---

## visualization/mod.rs - Visualization Module

### Exports

```rust
pub mod plots;
```

---

## visualization/plots.rs - ASCII Plot Generation

### Struct: ASCIIPlotter
Unicode-based ASCII plot generator.

```rust
pub struct ASCIIPlotter {
    width: usize,   // Plot width in characters
    height: usize,  // Plot height in characters
}
```

### Methods

```rust
impl ASCIIPlotter {
    /// Create a new plotter with dimensions
    pub fn new(width: usize, height: usize) -> Self
    
    /// Generate amplitude plot from packets
    pub fn plot_amplitude(&self, packets: &[CSIPacket]) -> String
    
    /// Generate bar chart from values
    pub fn plot_bar_chart(&self, values: &[f32], label: &str) -> String
}
```

### Visual Elements

```
█  - Full block (high amplitude)
▄  - Lower half block (medium amplitude)
   - Space (low/no amplitude)
```

### Example Output

```
█ █ █ █ █ ▄ ▄ ▄
█ █ █ █ ▄ ▄ ▄ 
█ █ █ ▄ ▄ ▄  
█ █ ▄ ▄ ▄   
█ ▄ ▄ ▄    
```

---

## serial/mod.rs - Serial Module

### Exports

```rust
pub mod handler;
```

---

## serial/handler.rs - Serial Communication

### Struct: SerialHandler
Low-level serial port management.

```rust
pub struct SerialHandler {
    port: Box<dyn SerialPort>,
    timeout: Duration,
}
```

### Methods

```rust
impl SerialHandler {
    /// Open a serial connection
    pub fn new(port: &str, baud_rate: u32) -> Result<Self>
    
    /// Write data to serial port
    pub fn write(&mut self, data: &[u8]) -> Result<usize>
    
    /// Read data from serial port
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize>
    
    /// Read until timeout
    pub fn read_with_timeout(&mut self) -> Result<Vec<u8>>
}
```

---

## Data Flow Diagram

```
┌─────────────────┐
│  ESP32 Device   │
│  (Serial UART)  │
└────────┬────────┘
         │
         ↓ raw bytes
┌─────────────────────────────┐
│   SerialHandler::read()     │
│   (serial/handler.rs)       │
└────────┬────────────────────┘
         │
         ↓ raw string
┌─────────────────────────────┐
│   CSIProcessor              │
│   ::process_output()        │
│   (csi/processor.rs)        │
└────────┬────────────────────┘
         │
         ↓ Vec<CSIPacket>
┌─────────────────────────────┐
│   app.state.packet_buffer   │
│   (models.rs)               │
└────────┬────────────────────┘
         │
    ┌────┴────────────────┐
    │                     │
    ↓                     ↓
┌───────────────┐  ┌─────────────────┐
│ ASCIIPlotter  │  │ CSVExporter     │
│ ::plot_*()    │  │ ::export_*()    │
│ (visualiz/)   │  │ (storage/)      │
└───────────────┘  └─────────────────┘
    │                    │
    ↓ Plot String        ↓ CSV File
┌───────────────┐  ┌─────────────────┐
│ UI Rendering  │  │ ~/.esp-csi/     │
│ (main.rs)     │  │ exports/*.csv   │
└───────────────┘  └─────────────────┘
```

---

## Integration Example

```rust
// main.rs - Complete integration

// 1. Load configuration
let config = config::load_config()?;

// 2. Initialize app with config
let mut app = app::App::new(
    &config.serial.port,
    config.serial.baud_rate
)?;

// 3. Initialize visualization
let plotter = visualization::ASCIIPlotter::new(60, 8);

// 4. Read data from serial
app.read_data()?;  // Populates app.state.packet_buffer

// 5. Generate visualization
let plot = plotter.plot_amplitude(&app.state.packet_buffer);

// 6. Export to CSV
let metadata = vec![
    ("Export Time".to_string(), chrono::Local::now().to_string()),
    ("Packet Count".to_string(), app.state.packet_buffer.len().to_string()),
];
storage::csv::CSVExporter::export_with_metadata(
    &app.state.packet_buffer,
    &Path::new("export.csv"),
    &metadata
)?;
```

---

## Error Handling

All public functions return `Result<T>` for proper error propagation:

```rust
// Successful operation
match app.read_data() {
    Ok(()) => { /* process data */ }
    Err(e) => { /* handle error */ }
}

// Using ? operator
let config = config::load_config()?;  // Propagates error
```

Common errors:
- `SerialPortError` - Serial connection issues
- `IoError` - File I/O problems
- `ParseError` - Data parsing failures
- `ConfigError` - Configuration issues

---

**Last Updated:** 2024-12-02  
**Version:** 1.0 (Refactored & Stable)
