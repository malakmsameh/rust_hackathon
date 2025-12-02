# Architecture Documentation

## System Design

### Data Flow

```
ESP32 Device
    ↓ (Serial/UART)
SerialHandler
    ↓
EspClient (Parse CSI data)
    ↓
CsiMeasurement (Models)
    ├→ AppState (Store)
    ├→ Visualization (Process)
    ├→ Storage (Save)
    └→ Streaming (Send)
```

### Component Responsibilities

#### Device Layer (`src/device/`)
- **SerialHandler**: Low-level serial communication
  - Connect/disconnect
  - Write/read operations
  - Error handling
  
- **EspClient**: Protocol-level communication
  - AT command translation
  - CSI data parsing
  - Device configuration

#### Data Layer (`src/models/`)
- **CsiMeasurement**: CSI data point (timestamp, channel, subcarriers)
- **ComplexNumber**: Magnitude & phase calculations
- **DeviceConfig**: Device settings (channel, bandwidth, interval)
- **AppState**: Application state management

#### UI Layer (`src/ui/`)
- **App**: Main application logic
  - Event handling
  - State updates
  - Rendering coordination
  
- **Components**:
  - Layout: Screen subdivisions
  - Plots: Data visualization
  - Status: Header/footer info

#### Processing Layer (`src/visualization/`)
- **DataProcessor**: Signal processing
  - Magnitude/phase extraction
  - Heatmap generation
  - Statistics calculation
  
- **Renderer**: Plot generation with Plotly

#### Storage Layer (`src/storage/`)
- **CsvStorage**: CSV export
- **RrdStorage**: Rerun.io format export

#### Streaming Layer (`src/streaming/`)
- **RerunClient**: Rerun.io integration
- Protocol implementation for live data streaming

## State Management

### AppState Structure
```rust
pub struct AppState {
    pub measurements: Vec<CsiMeasurement>,  // Collected data
    pub device_config: DeviceConfig,        // Device settings
    pub is_connected: bool,                 // Connection status
    pub is_collecting: bool,                // Collection status
    pub current_tab: usize,                 // Active visualization
}
```

## Concurrency Model

### Single-threaded with Async I/O
- Tokio runtime for async operations
- Terminal UI updates on main thread
- Serial I/O non-blocking
- Allows responsive UI during data collection

### Event Processing
```
Event Loop:
  1. Poll for keyboard input (non-blocking)
  2. Read serial data (non-blocking)
  3. Update AppState
  4. Render frame
  5. Sleep briefly (event polling interval)
```

## Error Handling

### Strategy
- **Propagate early**: Use `Result<T>` for recoverable errors
- **Log appropriately**: Different levels for user vs. developer info
- **Graceful degradation**: UI remains responsive on errors
- **Panic recovery**: Cleanup terminal on panic

### Error Types
- Serial communication errors → Retry or reconnect
- Data parsing errors → Skip measurement, log warning
- Storage errors → Alert user, allow retry
- UI errors → Log, continue operation

## Performance Considerations

### Memory Usage
- Measurements stored in Vec (growing data)
- Future: Ring buffer for fixed memory footprint
- Efficient data structures for visualization

### CPU Usage
- Async I/O prevents blocking
- Selective redrawing when state changes
- Efficient terminal updates (Ratatui)

### Latency
- Serial buffering: 100-200ms typical
- TUI refresh: 16-33ms (60 Hz display)
- Processing: < 1ms for visualization

## Extension Points

### Adding New Plot Types
1. Define PlotType variant
2. Implement in Renderer
3. Add to UI component selection

### Adding Storage Formats
1. Create new storage module
2. Implement Storage trait
3. Register in storage selection

### Adding Serial Protocols
1. Create protocol module
2. Implement Device trait
3. Plugin in device factory

## Design Patterns Used

- **Trait Objects**: Device abstraction
- **Builder Pattern**: Configuration building
- **Observer Pattern**: Event handling in TUI
- **Strategy Pattern**: Multiple storage/visualization strategies
- **Factory Pattern**: Storage format selection

## Testing Strategy

### Unit Tests
- Data models (serialization, calculations)
- Data processing (spectrum, statistics)

### Integration Tests
- Serial communication mock
- Full pipeline with test data

### Manual Testing
- UI responsiveness
- Data collection workflow
- Export functionality

## Security Considerations

- Serial port validation
- Input sanitization for AT commands
- File path validation for storage
- Network URL validation for streaming
