# CHORDWORLD MAXIMALIST ROADMAP

## Vision

Transform CHORDWORLD from a minimal tracker into a **MAXIMALIST MODULAR SYNTHESIS POWERHOUSE** with real-time visual graph editing, comprehensive DSP library, and immediate playability.

## Phase 1: Synthesis Library Expansion ✅ (In Progress)

### Oscillators (7+ types)
- [x] OscSine - Pure sine wave
- [ ] OscSaw - Sawtooth (bright, rich harmonics)
- [ ] OscSquare - Square wave (hollow, clarinet-like)
- [ ] OscTriangle - Triangle wave (flute-like, soft)
- [ ] OscNoise - White/pink/brown noise generator
- [ ] OscWavetable - Wavetable synthesis with morphing
- [ ] OscFM - 2-operator FM synthesis

### Filters (6+ types)
- [ ] FilterSVF - State Variable Filter (LP/HP/BP/Notch modes)
- [ ] FilterMoog - Moog ladder (classic analog sound)
- [ ] FilterLP - Simple lowpass
- [ ] FilterHP - Simple highpass
- [ ] FilterBP - Bandpass
- [ ] FilterNotch - Notch/reject

### Envelopes (3+ types)
- [ ] EnvADSR - Attack-Decay-Sustain-Release
- [ ] EnvAR - Attack-Release (percussion)
- [ ] EnvMulti - Multi-stage envelope

### Effects (8+ types)
- [ ] FxDelay - Stereo delay with feedback
- [ ] FxReverb - Algorithmic reverb (Freeverb-style)
- [ ] FxDistortion - Waveshaping distortion
- [ ] FxChorus - Chorus effect
- [ ] FxPhaser - Phaser effect
- [ ] FxFlanger - Flanger effect
- [ ] FxCompressor - Dynamics compression
- [ ] FxEQ - Parametric EQ

### Modulators (5+ types)
- [ ] ModLFO - Low-frequency oscillator (sine/saw/square/triangle/S&H)
- [ ] ModSeq - Step sequencer
- [ ] ModSampleHold - Sample and hold
- [ ] ModEnvFollower - Envelope follower
- [ ] ModSlew - Slew rate limiter

### Utilities (8+ types)
- [x] Gain - Amplitude control
- [x] Out - Audio output
- [ ] Mixer - Multi-channel mixer
- [ ] VCA - Voltage-controlled amplifier
- [ ] Split - Signal splitter
- [ ] Merge - Signal merger
- [ ] Quantizer - Pitch quantization
- [ ] Scope - Waveform visualizer

## Phase 2: Visual Graph TUI 🎨

### Real-Time Graph Rendering
- [ ] ASCII box-drawing for nodes
- [ ] Connection lines with routing
- [ ] Color-coding by node type
- [ ] Real-time parameter value display
- [ ] CPU usage per node visualization
- [ ] Audio level meters

### Graph Layout Algorithm
- [ ] Automatic node placement (force-directed)
- [ ] Manual drag-and-drop positioning
- [ ] Grid snapping
- [ ] Zoom and pan
- [ ] Minimap view

### Enhanced UI Modes
- [ ] **GRAPH Mode**: Visual patch editing
- [ ] **PARAM Mode**: Parameter editing with knobs/sliders
- [ ] **SCOPE Mode**: Waveform/spectrum analysis
- [ ] **PRESET Mode**: Browse and load patches
- [ ] **MINE Mode**: 21e8 hash mining interface

### Visual Elements
```
┌─────────────────────────────────────────────────────┐
│ CHORDWORLD - Visual Modular Synthesizer           │
├─────────────────────────────────────────────────────┤
│                                                     │
│   ┌─[OscSaw]─┐      ┌─[FilterMoog]─┐             │
│   │ Freq: 440├─────>│ Cutoff: 2000 ├──┐          │
│   │ Amp:  0.8│      │ Res:    0.7   │  │          │
│   └──────────┘      └───────────────┘  │          │
│                                         │          │
│   ┌─[ModLFO]──┐                       │          │
│   │ Rate: 2Hz├──────────────────────>│          │
│   │ Depth:0.5│                       │          │
│   └──────────┘                       v          │
│                            ┌─[Out]────┐          │
│                            │ Vol: 0.7 │          │
│                            └──────────┘          │
│                                                     │
├─────────────────────────────────────────────────────┤
│ [G]raph [P]arams [S]cope [R]esources [M]ine  : ▌│
└─────────────────────────────────────────────────────┘
```

## Phase 3: Preset System 🎵

### Factory Presets (20+ sounds)
- [ ] **Bass**: Sub bass, acid bass, wobble bass
- [ ] **Lead**: Saw lead, square lead, FM bell
- [ ] **Pad**: Warm pad, string pad, choir pad
- [ ] **Arp**: Classic arp, sequenced arp
- [ ] **Drum**: Kick, snare, hi-hat, clap
- [ ] **FX**: Noise sweep, riser, impact
- [ ] **Experimental**: Glitch, granular, 21e8-tuned

### Preset Format
```toml
[preset]
name = "Acid Bass"
category = "Bass"
author = "CHORDWORLD"

[nodes]
osc = { type = "OscSaw", params = { freq = 55.0, detune = 0.02 } }
filter = { type = "FilterMoog", params = { cutoff = 500.0, res = 0.8 } }
env = { type = "EnvADSR", params = { a = 0.01, d = 0.3, s = 0.0, r = 0.1 } }
out = { type = "Out", params = { gain = 0.7 } }

[connections]
[[connections]]
from = "osc:out"
to = "filter:in"

[[connections]]
from = "filter:out"
to = "out:in"

[[connections]]
from = "env:out"
to = "filter:cutoff"
```

### Quick-Start Commands
```
:preset.load acid_bass
:preset.save my_patch
:preset.random [category]
:preset.export <file>
```

## Phase 4: macOS Support 🍎

### Build Configuration
- [ ] Update Cargo.toml with macOS targets
- [ ] Test CPAL on macOS (should work out of box)
- [ ] Add CoreAudio-specific optimizations
- [ ] Document macOS build process

### Platform-Specific Features
- [ ] Native audio device selection
- [ ] MIDI integration (via midir)
- [ ] Metal acceleration (future)

## Phase 5: Performance & Polish ⚡

### Optimization
- [ ] SIMD vectorization for DSP
- [ ] Multi-threaded graph compilation
- [ ] Lock-free parameter updates
- [ ] Audio buffer pool recycling

### Quality of Life
- [ ] Undo/redo for graph operations
- [ ] Copy/paste nodes and connections
- [ ] Graph templates
- [ ] MIDI learn for parameters
- [ ] Keyboard shortcuts for everything

## Phase 6: Integration Features 🔗

### 21e8 Integration
- [ ] Mine hashes directly from TUI
- [ ] Visual hash rarity indicator
- [ ] Auto-apply hash-derived scales to oscillators
- [ ] Entropy pool browser in TUI

### Tracker Integration
- [ ] Pattern editor with visual feedback
- [ ] Piano roll view
- [ ] Automation lanes
- [ ] Note probability/humanization

## Timeline

### Sprint 1 (Current)
- Complete oscillator library (7 types)
- Basic filter implementations (SVF, Moog)
- ADSR envelope
- Gain/Mixer/VCA utilities

### Sprint 2
- Visual graph renderer foundation
- Box-drawing node representation
- Connection routing
- Basic graph layout

### Sprint 3
- Effect library (delay, reverb, distortion)
- Modulator library (LFO, sequencer)
- Preset system with 10 factory sounds

### Sprint 4
- Enhanced TUI with multiple modes
- Parameter visualization
- Scope/meter displays
- macOS build testing

### Sprint 5
- Performance optimization
- SIMD implementation
- Multi-threading
- Polish and documentation

## Success Metrics

- [ ] **30+ node types** available
- [ ] **20+ factory presets** ready to play
- [ ] **Visual graph** renders in real-time
- [ ] **<5ms latency** on modern hardware
- [ ] **Cross-platform** (Linux + macOS)
- [ ] **Zero-setup** - works immediately after build

## Philosophy

**MAXIMALISM** means:
- Every feature you expect from a modular synth
- Beautiful visual feedback everywhere
- Immediate gratification (load preset, hear sound)
- No compromises on quality or features
- Real-time everything
- Make it FEEL alive

**But Also**:
- Maintain determinism (doctrine compliance)
- Keep real-time safety (no alloc in audio thread)
- Transaction log for undo/replay
- 21e8 integration for xenharmonic exploration

---

**Status**: 🚧 Phase 1 in progress
**Target**: Transform CHORDWORLD into the maximalist modular synth it deserves to be
**Vibe**: GO ABSOLUTELY NUTS 🎸🔥
