# CHORDWORLD

> A Rust TUI Tracker + Patch-Graph Audio OS

**CHORDWORLD** is a terminal user interface (TUI) application that combines tracker-style music sequencing with modular patch-graph audio processing. Built in Rust with strict determinism, real-time safety, and TempleOS-inspired operational constraints.

**Status**: Initial implementation (v0.1.0) - Milestone A-B complete

---

## Features

- **TUI Interface**: Terminal-based UI with keyboard-only operation
- **Patch Graph**: Modular audio processing with nodes and connections
- **Real-Time Audio**: CPAL-based audio engine with lock-free processing
- **Deterministic**: Bit-exact replay from transaction logs
- **Doctrine Mode**: TempleOS-inspired constraints for simplicity and safety
- **Extensible**: Designed for future synthesis and microtuning layers

---

## Quick Start

### Prerequisites

- Rust toolchain (1.70+)
- ALSA development libraries (Linux):
  ```bash
  sudo apt-get install libasound2-dev
  ```

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

---

## Usage

### Basic Commands

Start the application and use `:` to enter commands:

```
:node.add OscSine osc1        # Add sine oscillator
:node.add Gain gain           # Add gain node
:node.add Out out             # Add output node
:connect osc1:out gain:in     # Connect osc to gain
:connect gain:out out:in      # Connect gain to output
:param.set osc1.0 440.0       # Set frequency to 440 Hz
:param.set gain.0 0.5         # Set gain to 0.5
:play                         # Start playback
```

### Keyboard Shortcuts

- `:` - Command mode
- `F1` - Help
- `Space` - Play/Stop
- `Ctrl+Q` - Quit
- `Esc` - Cancel/Back

---

## Architecture

CHORDWORLD uses a three-thread architecture:

1. **Audio Thread**: Lock-free real-time processing
2. **World Thread**: Mutable state and graph compilation
3. **UI Thread**: Terminal rendering and input

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for details.

---

## Doctrine

The **Doctrine** enforces operational constraints:

- Real-time safety (no alloc/lock/I/O in audio)
- Deterministic replay
- Keyboard-only, 16-color UI
- No background services
- Explicit provenance for all parameters

See [docs/DOCTRINE.md](docs/DOCTRINE.md) for the full specification.

---

## Project Structure

```
chorum/
├── crates/
│   ├── chordworld_core/      # IDs, transactions, events
│   ├── chordworld_dsp/       # DSP nodes and processing
│   ├── chordworld_world/     # World state and graph
│   ├── chordworld_engine/    # Audio engine (CPAL)
│   ├── chordworld_tui/       # Terminal UI
│   └── chordworld_app/       # Main binary
├── docs/
│   ├── ARCHITECTURE.md       # System architecture
│   ├── DOCTRINE.md          # Operational constraints
│   └── COMMANDS.md          # Command reference
└── README.md                # This file
```

---

## Roadmap

### Milestone A ✓
Boot + Silence - TUI launches, audio opens

### Milestone B ✓
Minimal Graph + Sound - Basic nodes (OscSine, Gain, Out) with live parameter control

### Milestone C (Next)
Tracker MVP - Pattern editor with note entry and playback

### Milestone D
Persistence + Replay - Save/load with deterministic playback

### Milestone E
Expanded Node Set + Inspect - Filters, delays, meters, provenance

### Milestone F
Offline Render - Headless WAV export with determinism tests

---

## Future: CHORUM

CHORDWORLD is the first component of **CHORUM**, a larger system:

1. **Chordworld** (current): TUI tracker + patch graph
2. **Synthesis & Microtuning**: Advanced synthesis with custom tuning
3. **Ontology & Grammar**: Unified command language and object model

---

## Documentation

- [ARCHITECTURE.md](docs/ARCHITECTURE.md) - System design and thread model
- [DOCTRINE.md](docs/DOCTRINE.md) - Operational constraints and enforcement
- [COMMANDS.md](docs/COMMANDS.md) - Command reference

---

## Built With

- [Rust](https://www.rust-lang.org/) - Systems programming language
- [CPAL](https://github.com/RustAudio/cpal) - Cross-platform audio I/O
- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal control
- [serde](https://serde.rs/) + [ciborium](https://github.com/enarx/ciborium) - Serialization

---

## License

MIT

---

## Author

Built with Claude Code on the web.

**Version**: 0.1.0
**Date**: 2026-01-15
