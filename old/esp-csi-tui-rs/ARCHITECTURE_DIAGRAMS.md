# Architecture & Data Flow Diagrams

## Current Architecture (Problem)

```
┌─────────────────────────────────────────────────────────────────────┐
│                          main.rs (300 lines)                         │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │ App Struct   │  │ Event Loop   │  │ UI Rendering │              │
│  │ Management   │  │ & Commands   │  │ & Layout     │              │
│  └──────────────┘  └──────────────┘  └──────────────┘              │
│                                                                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │ Data          │  │ File I/O     │  │ Status Bar   │              │
│  │ Processing    │  │ & Export     │  │ & Displays   │              │
│  └──────────────┘  └──────────────┘  └──────────────┘              │
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ Everything scattered, hard to find, impossible to test       │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
         ↓           ↓              ↓              ↓
    Serial    Config Data    Visualization    Error Handling
    (Works)  (Doesn't exist) (Missing)        (Basic only)

Issues:
❌ 90% of code in one file
❌ Cannot unit test
❌ Hard to add features
❌ Code reuse impossible
```

---

## Proposed Architecture (Solution)

```
┌────────────────────────────────────────────────────────────────┐
│                        Application Layer                        │
├────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐       │
│  │ main.rs  │  │ app.rs   │  │commands. │  │ config.rs│       │
│  │ (thin)   │→ │(state)   │→ │rs(logic) │→ │(settings)│       │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘       │
│                                                                  │
└────────────────────────────────────────────────────────────────┘
        ↓                ↓                ↓
┌───────────────┐  ┌────────────┐  ┌──────────────┐
│   UI Module   │  │  Serial    │  │ CSI Module   │
├───────────────┤  │  Module    │  ├──────────────┤
│ layout.rs     │  ├────────────┤  │ parser.rs    │
│ handlers.rs   │  │ handler.rs │  │ processor.rs │
│ components.rs │  │ queue.rs   │  └──────────────┘
│ styles.rs     │  └────────────┘       ↓
└───────────────┘       ↓          ┌──────────────┐
       ↓         ┌─────────────┐   │ Storage      │
       │         │ Device/Port │   ├──────────────┤
       │         └─────────────┘   │ csv.rs       │
       │                           │ rrd.rs       │
       └───────────────────────────┼ persistence  │
                                   └──────────────┘
                                          ↓
                                   ┌──────────────┐
                                   │  Files       │
                                   └──────────────┘

    ┌──────────────────────────────────────────────┐
    │     Visualization & Streaming (Future)       │
    ├──────────────────────────────────────────────┤
    │  plots.rs  │ heatmap.rs │ 3d.rs │ rerun.rs │
    └──────────────────────────────────────────────┘
           ↓           ↓          ↓         ↓
        ASCII       Heatmap     3D      Remote
        Chart       Display    Render   Stream

Benefits:
✅ Each module has single responsibility
✅ Easy to unit test
✅ Features added independently
✅ Code reusable and maintainable
✅ Parallel development possible
```

---

## Data Flow Diagram

### Current Data Flow (Problematic)

```
Serial Port
    ↓
SerialHandler → String Buffer
    ↓
main.rs read_data()
    ↓
String Processing (mixed concerns)
    ↓
App State Update (sometimes)
    ↓
UI Rendering (always)
    ↓
Screen

Issues:
- String parsing scattered in main.rs
- No structured data extraction
- Parser exists but not used
- Hard to debug
- Data loss possible
```

### Proposed Data Flow (Clean)

```
Serial Port (115200 baud)
    ↓
┌─────────────────────────┐
│ SerialHandler           │
│ - Connection management │
│ - Buffering             │
│ - Error recovery        │
└─────────────────────────┘
    ↓ (raw string data)
┌─────────────────────────┐
│ CSIProcessor            │
│ - Route to parser       │
│ - Error handling        │
│ - Packet extraction     │
└─────────────────────────┘
    ↓ (CSIPacket structs)
┌─────────────────────────┐
│ AppState                │
│ - Store packets         │
│ - Update statistics     │
│ - Buffer management     │
└─────────────────────────┘
    ↓ (structured data)
┌─────────────────────────────────┐
│ Parallel Processing             │
├─────────────────────────────────┤
│ ┌──────────┐ ┌──────────────┐  │
│ │UI Render │ │Export/Storage│  │
│ │          │ │              │  │
│ │ Plots    │ │CSV Export    │  │
│ │ Tables   │ │RRD Export    │  │
│ │ Stats    │ │Persistence   │  │
│ └──────────┘ └──────────────┘  │
│                                  │
│ ┌──────────────────────────┐    │
│ │Streaming                 │    │
│ │- Rerun.io               │    │
│ │- Camera (future)        │    │
│ └──────────────────────────┘    │
└─────────────────────────────────┘

Benefits:
✅ Clear data flow
✅ Modular processing
✅ Easy to add transformations
✅ Testable at each stage
✅ Parallel exports
```

---

## Event Handling Flow

### Current State (Monolithic)

```
Terminal Event
    ↓
match in run_app()
    ↓
Send command to device
    ↓
Update app state
    ↓
Trigger re-render
    ↓
Screen update

Issues:
- All logic in one function
- Hard to test key handling
- No command validation
- Mixed concerns
```

### Proposed State (Modular)

```
Terminal Event
    ↓
┌──────────────────────┐
│ ui/handlers.rs       │
│ handle_key_event()   │
└──────────────────────┘
    ↓
Validate Input
    ↓
┌──────────────────────┐
│ commands.rs          │
│ Parse → Command Enum │
└──────────────────────┘
    ↓
Validate Command
    ↓
Execute Command:
  ├─ Set WiFi
  ├─ Start Collection
  ├─ Export Data
  ├─ Configure Device
  └─ Show Help
    ↓
Update State:
  ├─ Device Config
  ├─ Collection State
  ├─ UI Panels
  └─ Message Buffer
    ↓
UI rerenders automatically (observer pattern)

Benefits:
✅ Testable key handling
✅ Command validation
✅ Reusable command system
✅ Easy to add new commands
✅ Separation of concerns
```

---

## State Management Architecture

### Current (Messy)

```
App struct contains:
├── SerialHandler (mutable)
├── CSIParser (immutable)
├── AppState (mutable)
├── output_buffer: Vec (mutable)
├── max_lines: usize
├── last_command: String (mutable)
├── connection_status: String (mutable)
├── data_display: String (mutable)
├── captured_data: Vec (mutable)
└── is_collecting: bool (mutable)

Problem: State scattered everywhere, hard to track
```

### Proposed (Organized)

```
AppState {
  // Device State
  connection: ConnectionState {
    is_connected: bool,
    status: String,
    last_error: Option<String>,
  }
  
  // Collection State
  collection: CollectionState {
    is_collecting: bool,
    packets_received: u64,
    start_time: SystemTime,
  }
  
  // Data State
  data: DataState {
    packet_buffer: Vec<CSIPacket>,
    latest_packet: Option<CSIPacket>,
    statistics: CollectionStats,
  }
  
  // Configuration State
  config: DeviceConfig {
    wifi_mode: String,
    channel: u8,
    frequency: u32,
  }
  
  // UI State (separate from data)
  ui: UiState {
    output_buffer: Vec<String>,
    last_command: String,
    current_view: ViewMode,
  }
}

Benefits:
✅ Clear grouping
✅ Easy to serialize
✅ Predictable mutations
✅ Testable state transitions
```

---

## Module Dependencies

### Current (Tangled)

```
main.rs
├── depends on everything
└── everything depends on main.rs

Result: Circular dependencies, hard to extract
```

### Proposed (DAG - Directed Acyclic Graph)

```
                    main.rs
                        ↓
                    app.rs
                    ↙   ↓   ↘
              ui/   serial   csi/
                ↓      ↓       ↓
           models  errors   storage
                ↓      ↓       ↓
              config ← ← ← ← ←
                        
          visualization/ ← app
          streaming/     ← app

Rules:
✅ Dependencies only go DOWN
✅ No circular references
✅ Each module independent
✅ Can test in isolation
```

---

## Feature Implementation Timeline

### Phase 1: Foundation (Days 1-2)

```
┌─────────────────────────────────────┐
│ Refactoring & Organization          │
├─────────────────────────────────────┤
│                                       │
│ main.rs → app.rs                     │
│   ↓                                   │
│ app.rs + handlers.rs                 │
│   ↓                                   │
│ ui/layout.rs                         │
│   ↓                                   │
│ config.rs + errors.rs                │
│   ↓                                   │
│ commands.rs                          │
│                                       │
│ ✓ Result: Clean architecture ready   │
│           for feature development    │
│                                       │
└─────────────────────────────────────┘
```

### Phase 2: Core Features (Days 3-4)

```
┌─────────────────────────────────────┐
│ Visualization & Storage             │
├─────────────────────────────────────┤
│                                       │
│ visualization/plots.rs              │
│   ↓                                   │
│ visualization/heatmap.rs            │
│   ↓                                   │
│ storage/csv.rs                      │
│   ↓                                   │
│ storage/rrd.rs                      │
│   ↓                                   │
│ ui/ configuration panel              │
│                                       │
│ ✓ Result: Working visualizations     │
│           and data export            │
│                                       │
└─────────────────────────────────────┘
```

### Phase 3: Advanced (Days 5+)

```
┌─────────────────────────────────────┐
│ Streaming & Advanced Visualization  │
├─────────────────────────────────────┤
│                                       │
│ streaming/rerun.rs                  │
│   ↓                                   │
│ visualization/3d.rs                 │
│   ↓                                   │
│ streaming/camera.rs (bonus)         │
│                                       │
│ ✓ Result: Complete feature set       │
│                                       │
└─────────────────────────────────────┘
```

---

## Performance Optimization Layers

```
User Input
    ↓
┌──────────────────────────────────┐
│ Layer 1: Event Handling (~1ms)   │
│ - Key press detection            │
│ - Command parsing                │
└──────────────────────────────────┘
    ↓
┌──────────────────────────────────┐
│ Layer 2: Serial I/O (~10ms)      │
│ - Read from device               │
│ - Non-blocking                   │
│ - Buffering                      │
└──────────────────────────────────┘
    ↓
┌──────────────────────────────────┐
│ Layer 3: Data Processing (~5ms)  │
│ - Parse packets                  │
│ - Extract features               │
│ - Update statistics              │
└──────────────────────────────────┘
    ↓
┌──────────────────────────────────┐
│ Layer 4: Storage (async)         │
│ - Export to CSV                  │
│ - Persist to disk                │
│ - Stream to Rerun                │
│ - (doesn't block UI)             │
└──────────────────────────────────┘
    ↓
┌──────────────────────────────────┐
│ Layer 5: UI Rendering (~16ms)    │
│ - Layout calculation             │
│ - Widget rendering               │
│ - Plot generation                │
│ - 30fps max                      │
└──────────────────────────────────┘
    ↓
Screen Update

Total Frame Time: ~40ms = 25fps
Target: 30fps = 33ms per frame
Headroom: 7ms for other tasks
```

---

## Test Strategy Pyramid

```
                    ▲
                   ╱│╲
                  ╱ │ ╲     End-to-End Tests (10%)
                 ╱  │  ╲   - Full workflows
                ╱───┼───╲  - 2-3 tests
               ╱    │    ╲
              ╱     │     ╲
             ╱──────┼──────╲
            ╱ Integration   ╲  Integration Tests (25%)
           ╱   Tests        ╲ - Component interactions
          ╱────────┼────────╲ - 5-8 tests
         ╱         │         ╲
        ╱          │          ╲
       ╱───────────┼───────────╲
      ╱  Unit Tests - Each Module ╲  Unit Tests (65%)
     ╱        (Fast, Isolated)      ╲ - Parser tests
    ╱                                ╲ - Storage tests
   ╱──────────────────────────────────╲ - Command tests
  ╱                                    ╲ - Config tests
 ╱____________________________________ ╲- Error tests

Coverage Target: >60%
Execution Time: <1s for all unit tests
```

---

## Deployment & Release Process

```
Development Branch
    ↓
    ├─→ Feature branches (each issue)
    │       ↓
    │   Code + Tests
    │       ↓
    │   PR Review
    │       ↓
    │   Merge to dev
    ↓
dev branch (integration point)
    ↓
    ├─→ cargo test
    │   cargo clippy
    │   cargo fmt
    └─→ cargo build --release
    ↓
Release Branch
    ↓
    ├─→ Version bump
    │   Changelog
    │   Tag release
    └─→ Binary artifacts
    ↓
main/master branch
    ↓
Binary Distribution
    ├─→ GitHub Releases
    ├─→ AUR (if applicable)
    └─→ Documentation
```

---

## Success Metrics Dashboard

```
┌──────────────────────────────────────┐
│        Code Quality Metrics           │
├──────────────────────────────────────┤
│ Cyclomatic Complexity    [████░░] 60% │
│ Test Coverage            [███████░] 70% │
│ Documentation            [██████░░] 65% │
│ Technical Debt           [██░░░░░░] 20% │
└──────────────────────────────────────┘

┌──────────────────────────────────────┐
│      Performance Metrics              │
├──────────────────────────────────────┤
│ Parser Latency (target: <5ms)         │
│ ├─ Current: Unknown                   │
│ └─ Target:  ████░ 80%                 │
│                                        │
│ UI Frame Time (target: <33ms)         │
│ ├─ Current: Unknown                   │
│ └─ Target:  ██████░ 85%               │
│                                        │
│ Memory per Packet (target: <2KB)      │
│ ├─ Current: Unknown                   │
│ └─ Target:  ███░░░ 40%                │
└──────────────────────────────────────┘

┌──────────────────────────────────────┐
│      Feature Completion               │
├──────────────────────────────────────┤
│ Phase 1: Foundation        [░░░░░░░░░] 0% │
│ Phase 2: Core Features     [░░░░░░░░░] 0% │
│ Phase 3: Advanced          [░░░░░░░░░] 0% │
│ Phase 4: Polish            [░░░░░░░░░] 0% │
│                                        │
│ Overall Completion: 0%               │
└──────────────────────────────────────┘
```

---

## Risk Mitigation Matrix

```
Risk: Monolithic Code Hard to Refactor
├─ Probability: Medium
├─ Impact: High
└─ Mitigation:
    ├─ Start with Phase 1 IMMEDIATELY
    ├─ Do refactoring before new features
    ├─ Use git branches for parallel work
    └─ Regular commits for checkpoints

Risk: Visualization Complex
├─ Probability: Medium
├─ Impact: High
└─ Mitigation:
    ├─ Start simple (ASCII plots)
    ├─ Use Plotters crate (proven)
    ├─ Reference implementations
    └─ Incremental complexity increase

Risk: Performance Issues
├─ Probability: Low
├─ Impact: Medium
└─ Mitigation:
    ├─ Profile early and often
    ├─ Benchmark each component
    ├─ Optimization passes planned
    └─ Target metrics defined

Risk: Team Coordination
├─ Probability: Medium
├─ Impact: Medium
└─ Mitigation:
    ├─ Clear issue ownership
    ├─ Daily standups
    ├─ GitHub projects for tracking
    └─ Code review process
```

---

This visual architecture provides a clear roadmap for implementation and helps communicate the vision to the team!
