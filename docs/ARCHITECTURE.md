# CHORDWORLD Architecture

## Overview

CHORDWORLD is a terminal user interface (TUI) application that combines tracker-style sequencing with modular patch-graph audio processing, built in Rust with deterministic replay and strict operational constraints.

## Thread Model

CHORDWORLD uses a **three-thread architecture** designed for real-time audio processing with deterministic state management:

### 1. Audio Thread (Real-Time)
- **Purpose**: Process audio blocks at consistent intervals
- **Hard Constraints**:
  - No heap allocations
  - No locks (reads atomic snapshot pointer)
  - No I/O operations
  - No logging from callback
- **Data Access**: Reads immutable `GraphSnapshot` via lock-free atomic swap
- **Block Size**: Configurable (default 256 frames @ 48kHz)

### 2. World Thread (Simulation)
- **Purpose**: Mutable state management and graph compilation
- **Responsibilities**:
  - Apply transactions to world state
  - Compile `GraphModel` → `GraphSnapshot`
  - Maintain event log for replay
  - Handle tracker playback scheduling
- **Data Flow**:
  - Receives `Transaction` commands
  - Publishes new `GraphSnapshot` to audio thread
  - Logs all state changes

### 3. UI Thread (TUI Rendering)
- **Purpose**: Terminal UI rendering and input handling
- **Responsibilities**:
  - Render panes (Tracker, Patch, Mix, Inspect, Log)
  - Handle keyboard input
  - Parse and send commands
  - Display status messages
- **Framework**: ratatui + crossterm

## Snapshot Swap Mechanism

The **snapshot swap** is the core synchronization primitive:

```
World Thread                    Audio Thread
-----------                     ------------
GraphModel (mutable)
    ↓
  compile()
    ↓
GraphSnapshot (immutable) ----→ reads at block boundary
    ↓                           processes nodes
  publish via Arc<>                 ↓
                                outputs audio
```

- World thread creates snapshots and publishes via `Arc<GraphSnapshot>`
- Audio thread swaps to latest snapshot at block boundaries
- No locks in audio callback - only atomic pointer reads

## Core Data Structures

### IDs (Stable References)
All objects have stable, unique IDs:
- `NodeId`, `PortId`, `ParamId`, `ConnectionId`
- `PatternId`, `InstrumentId`, `BufferId`, `RuleId`

### GraphModel (World Thread)
```rust
pub struct GraphModel {
    nodes: HashMap<NodeId, GraphNode>,
    connections: HashMap<ConnectionId, Connection>,
    // ... mutable state
}
```

### GraphSnapshot (Audio Thread)
```rust
pub struct GraphSnapshot {
    dispatch_order: Vec<SnapshotNode>,  // Topologically sorted
    buffer_count: usize,
    sample_rate: f32,
}
```

### Transaction Log
```rust
pub struct TransactionRecord {
    id: TransactionId,
    timestamp: u64,          // Logical world time
    transaction: Transaction,
    checksum: u32,
}
```

## Transaction System

All mutations go through the transaction system:

```rust
pub enum Transaction {
    NodeAdd { node_type, name },
    Connect { src, dst, map },
    ParamSet { node, param, value },
    TransportSet { state, apply },
    // ... more
}
```

**Apply Points**:
- `Now`: Immediate (param changes only)
- `NextBlock`: Next audio block boundary
- `NextLine`: Next tracker line
- `NextBeat/NextBar`: Musical quantization

**Determinism**:
1. Initial snapshot
2. Event log (ordered transactions)
3. RNG seeds (versioned)

→ Replay produces identical audio

## Node System

### Node Traits
```rust
pub trait DspNode: Send {
    fn reset(&mut self, sample_rate: f32);
    fn process(&mut self, ctx: &ProcessCtx,
               ins: &[AudioBusRef],
               outs: &mut [AudioBusMut]);
    fn set_param(&mut self, param: ParamIndex, value: ParamValue);
    fn handle_events(&mut self, ctx: &ProcessCtx, events: &mut EventQueue);
}

pub trait NodeDescriptor: Send + Sync {
    fn type_name(&self) -> &'static str;
    fn ports(&self) -> &'static [PortSpec];
    fn params(&self) -> &'static [ParamSpec];
}

pub trait Node: DspNode + NodeDescriptor + Send + Sync {}
```

### Built-in Nodes (v0.1)
- **OscSine**: Sine wave oscillator
- **Gain**: Amplitude control
- **Out**: Audio output endpoint

## Tracker System

### Pattern Structure
```rust
pub struct Pattern {
    id: PatternId,
    tracks: Vec<Track>,       // Columns
    row_count: usize,         // Rows (lines)
}

pub struct PatternCell {
    note: Option<Note>,
    instrument: Option<InstrumentId>,
    effects: Vec<Effect>,
}
```

### Time Model
- **Sample Time**: Absolute frame index
- **Musical Time**: Bar:Beat:Line (LPB lines-per-beat)
- **World Time**: Event-step counter

## Persistence Format

### Project Directory
```
project.chordworld/
├── snapshot.cbor        # Full state baseline
├── events.log           # Transaction log (append-only)
├── assets/              # Samples, IRs (content-addressed)
└── renders/             # Exported WAV files
```

### Snapshot Schema
- Version header
- Doctrine config
- Graph model (nodes, connections, params)
- Tracker data (songs, patterns, instruments)
- RNG seed + versioning

### Transaction Log Format
Each record:
- `tx_id` (monotonic u64)
- `timestamp` (logical time)
- `transaction` (CBOR/binary payload)
- `checksum` (CRC32/Blake3)

## Module Structure

```
chordworld_core/       # IDs, transactions, events, time
chordworld_dsp/        # Nodes, buffers, audio processing
chordworld_world/      # Graph model, tracker, world state
chordworld_engine/     # Audio device, CPAL integration
chordworld_tui/        # Terminal UI, commands, rendering
chordworld_app/        # Main binary
```

## Communication Channels

```
TUI Thread  ←→  Main Loop  ←→  World State
    ↓              ↓              ↓
Commands       Transactions  GraphSnapshot
    ↓              ↓              ↓
Parser         Apply          Compiler
                               ↓
                           Audio Thread
```

## Future Expansions (Chorum)

CHORDWORLD is the first component of the larger **CHORUM** system:

1. **Chordworld** (current): TUI tracker + patch graph
2. **Synthesis & Microtuning**: Advanced synthesis layer with custom tuning
3. **Ontology & Grammar**: Unified command language and object model

The architecture is designed to be extensible for these future layers.

## Performance Targets

- Audio: Glitch-free @ 48kHz, 256-sample blocks
- Graph: Support 100+ nodes with modest CPU
- UI: 60 FPS terminal rendering
- Determinism: Bit-exact offline render matches realtime

## Validation & Safety

### Graph Validation
- Cycle detection with delay requirements
- Port type compatibility checking
- Node/connection count limits (doctrine)

### Doctrine Enforcement
- Real-time safety (no alloc/lock/I/O in audio)
- UI constraints (16-color, keyboard-only, menu depth)
- Explicit provenance for all parameter values
- Deterministic replay via event log

---

**Version**: 0.1.0
**Date**: 2026-01-15
