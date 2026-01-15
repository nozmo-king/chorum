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
- **21e8 Protocol**: Aesthetic proof-of-work generating microtonal tuning systems
- **Microtonal Synthesis**: Hash-derived exotic scales and deterministic entropy
- **Extensible**: Designed for future synthesis and microtuning layers

---

## Quick Start

### Prerequisites

- Rust toolchain (1.70+)
- Platform-specific audio libraries:
  - **Linux**: ALSA development libraries
    ```bash
    sudo apt-get install libasound2-dev
    ```
  - **macOS**: No additional dependencies (uses CoreAudio via CPAL)
  - **Windows**: Not yet tested (should work via WASAPI)

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
│   ├── chordworld_pow/       # 21e8 proof-of-work + microtonality
│   └── chordworld_app/       # Main binary
├── docs/
│   ├── ARCHITECTURE.md       # System architecture
│   ├── DOCTRINE.md          # Operational constraints
│   ├── COMMANDS.md          # Command reference
│   └── 21E8_PROTOCOL.md     # Aesthetic PoW + microtonality
└── README.md                # This file
```

---

## The 21e8 Protocol

**Aesthetic Proof-of-Work + Microtonal Entropy**

CHORDWORLD implements the **21e8 paradigm** - where beautiful cryptographic hashes generate exotic musical tuning systems. Not all hashes are equal: some possess **aesthetic supremacy**.

### How It Works

1. **Mine Aesthetic Hashes**: Find hashes with rare patterns (leading zeros, palindromes, magic sequences like "21e8")
2. **Derive Tuning Systems**: Each hash generates a unique microtonal scale (19-72 EDO with micro-detuning)
3. **Deterministic Entropy**: Use hash pools as reproducible randomness for composition
4. **Rarity Classification**: Common → Uncommon → Rare → Epic → Legendary → Supreme

### Example

```bash
# Mine a hash with aesthetic criteria
:pow.mine "my-song" 20           # Find hash with 20+ leading zeros

# Hash generates a microtonal scale
# e.g., 31-EDO with ±2.5 cent detuning

# Use for composition
:tuning.set <hash>                # Set active tuning from hash
:node.add MicrotonalOsc osc      # Use hash-derived scale
```

### Philosophy

Inspired by Bitcoin block #528249 (`00000000000000000021e800...`), the 21e8 protocol recognizes that:
- **Compute can create art**: Beautiful hashes are aesthetic artifacts
- **Proof-of-work transcends security**: It becomes a medium of expression
- **Musical xenharmony from cryptographic chaos**: Hash entropy → exotic scales

See [docs/21E8_PROTOCOL.md](docs/21E8_PROTOCOL.md) for the complete specification.

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
- [21E8_PROTOCOL.md](docs/21E8_PROTOCOL.md) - Aesthetic PoW and microtonality

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
