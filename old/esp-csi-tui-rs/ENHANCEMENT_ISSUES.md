# ESP32 CSI TUI - Enhancement Issues & Implementation Tickets

This document breaks down the enhancement roadmap into actionable GitHub/GitLab issues.

---

## Phase 1: Foundation & Organization

### Issue #1: Refactor main.rs - Extract App State Management
**Priority**: 🔴 CRITICAL  
**Effort**: 2-3 hours  
**Impact**: Enables all other refactoring

**Description**:
Extract the `App` struct and its methods from `main.rs` into a new `app.rs` module.

**Acceptance Criteria**:
- [ ] New file `src/app.rs` created with App struct
- [ ] All App methods moved to app.rs
- [ ] main.rs imports and uses App from app module
- [ ] Compiles without warnings
- [ ] No functionality changed (same behavior)

**Files to Modify**:
- [x] Create `src/app.rs`
- [x] Update `src/main.rs`

**Subtasks**:
1. Create app.rs with App struct definition
2. Move `App::new()` method
3. Move `App::send_command()` method
4. Move `App::read_data()` method
5. Move `App::save_to_csv()` method
6. Move `App::get_status()` method
7. Update imports in main.rs

---

### Issue #2: Extract Event Handling
**Priority**: 🔴 CRITICAL  
**Effort**: 2 hours  
**Depends On**: #1

**Description**:
Move event handling logic from `run_app()` to `ui/handlers.rs`.

**Acceptance Criteria**:
- [ ] New file `src/ui/handlers.rs` created
- [ ] `handle_key_event()` function implemented
- [ ] All key bindings moved to handlers
- [ ] Event handling decoupled from rendering
- [ ] Tests for key handling

**Files to Modify**:
- [x] Create `src/ui/handlers.rs`
- [x] Create `src/ui/mod.rs`
- [x] Update `src/main.rs`

---

### Issue #3: Extract UI Rendering Logic
**Priority**: 🔴 CRITICAL  
**Effort**: 2-3 hours  
**Depends On**: #1, #2

**Description**:
Move all rendering code to `ui/layout.rs` with reusable components.

**Acceptance Criteria**:
- [ ] `src/ui/layout.rs` created with rendering functions
- [ ] `render_header()`, `render_status()`, etc. functions
- [ ] UI state isolated from app state
- [ ] Themes/colors moved to `ui/styles.rs`
- [ ] Reusable widget components created

**Files to Modify**:
- [x] Create `src/ui/layout.rs`
- [x] Create `src/ui/styles.rs`
- [x] Create `src/ui/components.rs`

---

### Issue #4: Create Custom Error Types
**Priority**: 🟠 HIGH  
**Effort**: 1.5 hours  
**Depends On**: #1

**Description**:
Replace generic anyhow errors with custom error types using thiserror.

**Acceptance Criteria**:
- [ ] `src/errors.rs` created
- [ ] `AppError` enum with all error variants
- [ ] Proper error messages for each type
- [ ] All error conversion impls
- [ ] Updated main.rs and other files to use AppError

**Error Types Needed**:
```rust
SerialError(String)
ParseError(String)
ConfigError(String)
StorageError(io::Error)
UIError(String)
DeviceError(String)
```

---

### Issue #5: Configuration Management System
**Priority**: 🟠 HIGH  
**Effort**: 2 hours

**Description**:
Implement config file loading and management.

**Acceptance Criteria**:
- [ ] `src/config.rs` created
- [ ] TOML config file format defined
- [ ] Load from `~/.esp-csi/config.toml`
- [ ] Fallback to sensible defaults
- [ ] CLI override support
- [ ] Config save/update functionality

**Config Structure**:
```toml
[serial]
port = "/dev/ttyUSB0"
baud_rate = 115200
timeout_ms = 10

[collection]
default_channel = 6
wifi_mode = "sniffer"

[storage]
export_format = "csv"
data_directory = "~/esp-csi-data"

[visualization]
plot_type = "2d"
refresh_rate = 30
```

---

### Issue #6: Create Command System
**Priority**: 🟠 HIGH  
**Effort**: 1.5 hours

**Description**:
Formalize command handling with enum-based command system.

**Acceptance Criteria**:
- [ ] `src/commands.rs` created
- [ ] `Command` enum defined
- [ ] `ExportFormat` enum defined
- [ ] `CommandHandler` trait
- [ ] Command parsing and execution
- [ ] Command validation

**Command Variants**:
```rust
SetWifiSniffer { channel: u8 }
StartCollection { duration: u32 }
StopCollection
SetTrafficFrequency { hz: u32 }
SetCSIConfig { ... }
ShowConfig
ExportData { format: ExportFormat }
Help
```

---

## Phase 2: Core Features

### Issue #7: Implement ASCII Plotting
**Priority**: 🔴 CRITICAL  
**Effort**: 3-4 hours

**Description**:
Add real-time ASCII/Unicode plotting for CSI amplitude and phase.

**Acceptance Criteria**:
- [ ] `src/visualization/plots.rs` created
- [ ] `Plotter` trait defined
- [ ] `ASCIIPlotter` implementation
- [ ] Plots amplitude over time
- [ ] Plots phase over time
- [ ] Shows in separate TUI panel
- [ ] Updates in real-time

**Dependencies to Add**:
```toml
plotters = "0.3"
unicode-plot = "0.1"
```

---

### Issue #8: Implement Heatmap Visualization
**Priority**: 🟠 HIGH  
**Effort**: 2-3 hours  
**Depends On**: #7

**Description**:
Create heatmap visualization for CSI subcarriers over time.

**Acceptance Criteria**:
- [ ] `src/visualization/heatmap.rs` created
- [ ] Heatmap renders CSI magnitude by subcarrier
- [ ] Time axis for sequential packets
- [ ] Color gradient from cold→hot
- [ ] Integrated into TUI layout

---

### Issue #9: Enhanced CSV Export
**Priority**: 🟠 HIGH  
**Effort**: 2 hours

**Description**:
Improve CSV export with proper structure and headers.

**Acceptance Criteria**:
- [ ] `src/storage/csv.rs` created
- [ ] `CSIRecord` struct defined
- [ ] Serde serialization
- [ ] Proper headers
- [ ] Separate amplitude/phase columns
- [ ] Metadata section in export file
- [ ] Timestamps in ISO format

**CSV Format**:
```csv
# ESP32 CSI Data Export
# Collection Start: 2024-12-02T10:30:00Z
# Collection Duration: 60s
# Device: ESP32-C3

timestamp_us,mac_address,rssi_dbm,channel,frequency_mhz,subcarrier_count,amplitude_json,phase_json
1000000,40:E1:E4:1F:81:C6,-61,6,2437,[1.2,1.3,1.4],[-0.5,-0.4,-0.3]
1001000,40:E1:E4:1F:81:C6,-61,6,2437,[1.3,1.4,1.5],[-0.4,-0.3,-0.2]
```

---

### Issue #10: Implement RRD Format Support
**Priority**: 🔴 CRITICAL (for Rerun)  
**Effort**: 3-4 hours

**Description**:
Create RRD format exporter for Rerun.io compatibility.

**Acceptance Criteria**:
- [ ] `src/storage/rrd.rs` created
- [ ] RRD file format specification research
- [ ] RRD exporter implementation
- [ ] Includes time series data
- [ ] Includes metadata
- [ ] Validates against Rerun schema

**Research Needed**:
- Rerun .rrd file format specification
- Required fields and structure
- Timestamp format requirements

---

### Issue #11: Data Persistence Layer
**Priority**: 🟠 HIGH  
**Effort**: 2 hours

**Description**:
Implement data persistence for collected packets.

**Acceptance Criteria**:
- [ ] `src/storage/persistence.rs` created
- [ ] Store packets between sessions
- [ ] SQLite backing (optional) or JSON lines
- [ ] Query/filter collected data
- [ ] Data cleanup/archival policy

---

### Issue #12: Interactive Configuration UI
**Priority**: 🟠 HIGH  
**Effort**: 3-4 hours

**Description**:
Create interactive TUI screen for device configuration.

**Acceptance Criteria**:
- [ ] New config screen in TUI
- [ ] WiFi mode selection (station/sniffer)
- [ ] Channel selection (1-13)
- [ ] SSID input field (for station mode)
- [ ] Password input (masked)
- [ ] Traffic frequency slider
- [ ] CSI feature toggles
- [ ] Live validation feedback

---

### Issue #13: Parser Performance Optimization
**Priority**: 🟠 MEDIUM  
**Effort**: 2 hours

**Description**:
Optimize CSI parser for lower latency and memory usage.

**Acceptance Criteria**:
- [ ] Pre-compile regex patterns (lazy_static)
- [ ] Reduce allocations
- [ ] Parse benchmark created
- [ ] < 5ms parse latency achieved
- [ ] Memory usage profiled

**Optimizations**:
```rust
lazy_static::lazy_static! {
    static ref MAC_REGEX: Regex = Regex::new(r"mac:\s+([0-9a-fA-F:]+)").unwrap();
    // ... other patterns
}
```

---

### Issue #14: Proper Serial I/O with Reconnection
**Priority**: 🟠 MEDIUM  
**Effort**: 2-3 hours

**Description**:
Enhance serial handler with reconnection logic and better error handling.

**Acceptance Criteria**:
- [ ] Automatic reconnection on disconnect
- [ ] Connection health checks
- [ ] Command queue for reliable delivery
- [ ] Flow control and backpressure handling
- [ ] Better timeout management

---

## Phase 3: Advanced Features

### Issue #15: Rerun.io Live Streaming Integration
**Priority**: 🔴 CRITICAL  
**Effort**: 4-5 hours

**Description**:
Implement real-time data streaming to Rerun.io viewer.

**Acceptance Criteria**:
- [ ] `src/streaming/rerun.rs` created
- [ ] `RerunStreamer` struct implemented
- [ ] Connect to local Rerun server
- [ ] Stream CSI amplitude data
- [ ] Stream phase information
- [ ] Remote URL support
- [ ] Enable/disable via config
- [ ] Error handling and recovery

**Implementation Steps**:
1. Research Rerun Python/Rust SDK
2. Implement data serialization to Rerun format
3. Create background streaming task
4. Handle network errors gracefully
5. Add UI indicator for streaming status

**Dependencies**:
```toml
rerun = "0.14"  # Check latest
tokio-util = "0.7"
```

---

### Issue #16: 3D Visualization
**Priority**: 🟡 MEDIUM  
**Effort**: 4-6 hours

**Description**:
Create 3D plot rendering for CSI data.

**Acceptance Criteria**:
- [ ] `src/visualization/3d.rs` created
- [ ] 3D amplitude plot over time/subcarriers
- [ ] 3D phase visualization
- [ ] Rotation/pan controls
- [ ] Export to image format

**Options for 3D**:
- ASCII 3D (simpler, no dependencies)
- Bevy 3D engine (complex but powerful)
- Plotters 3D support (if available)

---

### Issue #17: Color Domain Visualization
**Priority**: 🟡 MEDIUM  
**Effort**: 2-3 hours

**Description**:
Implement Color Domain (complex magnitude) visualization.

**Acceptance Criteria**:
- [ ] Complex number I/Q visualization
- [ ] Color mapped to magnitude
- [ ] Phase as hue
- [ ] Real-time updates
- [ ] Legend and scale

---

### Issue #18: Camera Video Streaming (Bonus)
**Priority**: 🟢 LOW  
**Effort**: 4-5 hours

**Description**:
Implement camera capture and live streaming in TUI.

**Acceptance Criteria**:
- [ ] `src/streaming/camera.rs` created
- [ ] Camera enumeration
- [ ] Video capture (OpenCV/GStreamer)
- [ ] ASCII art rendering in TUI
- [ ] FPS display

**Dependencies**:
```toml
opencv = "0.91"  # or
gstreamer = "0.18"  # or
rscam = "0.5"  # simpler option
```

---

## Phase 4: Polish & Testing

### Issue #19: Comprehensive Unit Testing
**Priority**: 🟠 HIGH  
**Effort**: 3-4 hours

**Description**:
Add unit tests for all modules.

**Test Coverage**:
- [ ] Parser: 5+ tests
- [ ] Storage: 4+ tests
- [ ] Commands: 3+ tests
- [ ] Config: 3+ tests
- [ ] Error handling: 2+ tests
- [ ] Overall coverage > 60%

**Test Files**:
```
tests/
├── parser_tests.rs
├── storage_tests.rs
├── command_tests.rs
├── config_tests.rs
└── integration_tests.rs
```

---

### Issue #20: Integration Testing
**Priority**: 🟠 MEDIUM  
**Effort**: 2-3 hours

**Description**:
Create integration tests for full workflows.

**Test Scenarios**:
- [ ] Full data collection workflow
- [ ] Export and reimport cycle
- [ ] Configuration persistence
- [ ] Error recovery
- [ ] Serial communication (mocked)

---

### Issue #21: Performance Profiling & Optimization
**Priority**: 🟡 MEDIUM  
**Effort**: 2-3 hours

**Description**:
Profile and optimize performance bottlenecks.

**Benchmarks to Create**:
- [ ] Parser latency (target: < 5ms)
- [ ] Memory per packet (target: < 2KB)
- [ ] UI frame time (target: < 33ms for 30fps)
- [ ] Serial I/O throughput
- [ ] Storage operations

---

### Issue #22: API Documentation
**Priority**: 🟡 MEDIUM  
**Effort**: 2-3 hours

**Description**:
Add comprehensive documentation to public APIs.

**Documentation Needed**:
- [ ] Module-level docs (//!)
- [ ] Function/struct docs (///)
- [ ] Example usage in docs
- [ ] Generated docs: `cargo doc --open`

---

### Issue #23: User Documentation
**Priority**: 🟠 HIGH  
**Effort**: 2-3 hours

**Description**:
Create user-facing documentation and examples.

**Documentation**:
- [ ] User guide: Getting started
- [ ] Keyboard shortcuts reference
- [ ] Configuration guide
- [ ] Data export guide
- [ ] Troubleshooting section
- [ ] Video tutorial (optional)

---

### Issue #24: Code Quality & Cleanup
**Priority**: 🟡 MEDIUM  
**Effort**: 1-2 hours

**Description**:
Clean up code, remove unused imports, fix clippy warnings.

**Tasks**:
- [ ] Run `cargo clippy` - fix all warnings
- [ ] Run `cargo fmt` - format code
- [ ] Remove dead code
- [ ] Consolidate imports
- [ ] Add missing documentation

---

## Cross-Cutting Concerns

### Issue #25: Logging Infrastructure
**Priority**: 🟠 MEDIUM  
**Effort**: 1.5 hours

**Description**:
Implement comprehensive logging across all modules.

**Acceptance Criteria**:
- [ ] Structured logging with `tracing` or `log`
- [ ] Debug, Info, Warn, Error levels
- [ ] File logging support
- [ ] Configurable via config file
- [ ] Performance metrics logging

---

### Issue #26: Error Recovery & Graceful Degradation
**Priority**: 🟠 MEDIUM  
**Effort**: 2 hours

**Description**:
Implement graceful error recovery mechanisms.

**Recovery Strategies**:
- [ ] Serial connection drops → auto-reconnect
- [ ] Parsing errors → skip packet, continue
- [ ] Storage full → warning, pause collection
- [ ] UI crashes → fallback to simple mode
- [ ] Network errors → queue data for later

---

### Issue #27: Security Review
**Priority**: 🟡 LOW  
**Effort**: 1.5 hours

**Description**:
Security audit and hardening.

**Checks**:
- [ ] No unwrap() outside tests
- [ ] Input validation on serial data
- [ ] File path validation
- [ ] Network URL validation
- [ ] Permissions check
- [ ] Secrets not in logs

---

## Implementation Priority Order

### Week 1 (Critical Path)
1. Issue #1 - Refactor main.rs
2. Issue #2 - Extract event handling  
3. Issue #3 - Extract UI rendering
4. Issue #4 - Custom error types
5. Issue #13 - Parser optimization

### Week 2 (Core Features)
6. Issue #5 - Configuration management
7. Issue #7 - ASCII plotting
8. Issue #9 - Enhanced CSV export
9. Issue #10 - RRD format support
10. Issue #12 - Configuration UI

### Week 3 (Advanced)
11. Issue #15 - Rerun.io integration
12. Issue #16 - 3D visualization
13. Issue #8 - Heatmap visualization

### Week 4 (Polish)
14. Issue #19 - Unit testing
15. Issue #20 - Integration testing
16. Issue #22 - API documentation
17. Issue #23 - User documentation
18. Issue #24 - Code cleanup

---

## Effort Summary

| Phase | Issues | Estimated Hours | Team Impact |
|-------|--------|-----------------|-------------|
| Phase 1 | #1-6 | 12-14 | Foundation for everything |
| Phase 2 | #7-14 | 18-22 | Core hackathon features |
| Phase 3 | #15-18 | 16-20 | Advanced features |
| Phase 4 | #19-27 | 12-16 | Polish and quality |
| **Total** | **27** | **58-72 hours** | **Complete product** |

---

## Team Assignment Template

```markdown
## Team A: Core Infrastructure (Phase 1)
- Developer 1: #1-3 (Refactoring)
- Developer 2: #4-6 (Config + Commands)

## Team B: Visualization (Phase 2)  
- Developer 1: #7-8 (Plotting)
- Developer 2: #9-10 (Storage)

## Team C: Advanced Features (Phase 3)
- Developer 1: #15 (Rerun)
- Developer 2: #16-17 (3D + Color)

## Team D: Testing & Docs (Phase 4)
- Developer 1: #19-21 (Testing)
- Developer 2: #22-24 (Docs + Quality)
```

---

## Success Metrics

✅ **Code Quality**:
- Cyclomatic complexity < 10 per function
- No functions > 50 lines
- Test coverage > 60%
- 0 clippy warnings

✅ **Performance**:
- Parser latency < 5ms
- UI refresh < 33ms (30fps)
- Memory per packet < 2KB
- Startup time < 2s

✅ **Functionality**:
- All hackathon requirements met
- Smooth data collection
- Real-time visualization working
- Data export/import working
- Rerun integration working

✅ **User Experience**:
- Intuitive keybindings
- Clear error messages
- Responsive UI
- Complete documentation

---

## Issue Template

For each issue, create a GitHub issue with:

```markdown
## Description
[Clear problem statement]

## Acceptance Criteria
- [ ] Criterion 1
- [ ] Criterion 2
- [ ] Criterion 3

## Files to Create/Modify
- [ ] src/path/file.rs

## Dependencies
```toml
[deps]
```

## Testing
[Testing strategy]

## Effort Estimate
[Hours]

## Blocked By
[Related issues]

## Notes
[Additional context]
```

---

This structure ensures parallel work, clear ownership, and measurable progress!
