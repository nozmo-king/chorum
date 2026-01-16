```
    ██████╗  █████╗ ████████╗ ██████╗██╗  ██╗
    ██╔══██╗██╔══██╗╚══██╔══╝██╔════╝██║  ██║
    ██████╔╝███████║   ██║   ██║     ███████║
    ██╔═══╝ ██╔══██║   ██║   ██║     ██╔══██║
    ██║     ██║  ██║   ██║   ╚██████╗██║  ██║
    ╚═╝     ╚═╝  ╚═╝   ╚═╝    ╚═════╝╚═╝  ╚═╝

     ██████╗ ██████╗  █████╗ ██████╗ ██╗  ██╗
    ██╔════╝ ██╔══██╗██╔══██╗██╔══██╗██║  ██║
    ██║  ███╗██████╔╝███████║██████╔╝███████║
    ██║   ██║██╔══██╗██╔══██║██╔═══╝ ██╔══██║
    ╚██████╔╝██║  ██║██║  ██║██║     ██║  ██║
     ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚═╝  ╚═╝

    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    ▓  MODULAR ROUTING · PURE DATA STYLE · DSP  ▓
    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
```

# Patch Graph

The patch graph is a modular signal routing system inspired by Pure Data,
Max/MSP, and Eurorack. Audio and control signals flow through nodes connected
by virtual patch cables.

---

## Signal Flow

```
                    ┌─────────────────────────────────────────────┐
                    │              EXAMPLE SIGNAL CHAIN           │
                    └─────────────────────────────────────────────┘

    ┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐
    │ OscSine  │────▶│FilterSVF │────▶│ FxReverb │────▶│   Out    │
    │          │     │          │     │          │     │          │
    │ freq:440 │     │ cutoff:  │     │ mix: 0.3 │     │ gain:0.8 │
    │ gain:0.8 │     │   2000   │     │ decay:2s │     │          │
    └──────────┘     └──────────┘     └──────────┘     └──────────┘
         │                ▲
         │                │
         │          ┌─────┴────┐
         │          │  ModLFO  │
         └─────────▶│          │
                    │ rate:0.5 │
                    │ depth:   │
                    │   500    │
                    └──────────┘
```

---

## Node Categories

```
╔═══════════════════════════════════════════════════════════════════╗
║                         OSCILLATORS                               ║
╠═══════════════════════════════════════════════════════════════════╣
║  OscSine      │ Pure sine wave                                    ║
║  OscSaw       │ Sawtooth wave (rich harmonics)                    ║
║  OscSquare    │ Square/pulse wave (hollow sound)                  ║
║  OscTriangle  │ Triangle wave (mellow)                            ║
║  OscNoise     │ White noise generator                             ║
║  OscWavetable │ Wavetable oscillator                              ║
║  OscFM        │ FM synthesis oscillator                           ║
╠═══════════════════════════════════════════════════════════════════╣
║                          FILTERS                                  ║
╠═══════════════════════════════════════════════════════════════════╣
║  FilterSVF    │ State variable filter (LP/HP/BP/Notch)            ║
║  FilterMoog   │ Moog ladder filter (24dB/oct)                     ║
║  FilterLP     │ Low-pass filter                                   ║
║  FilterHP     │ High-pass filter                                  ║
║  FilterBP     │ Band-pass filter                                  ║
║  FilterNotch  │ Notch/band-reject filter                          ║
╠═══════════════════════════════════════════════════════════════════╣
║                         ENVELOPES                                 ║
╠═══════════════════════════════════════════════════════════════════╣
║  EnvADSR      │ Attack-Decay-Sustain-Release                      ║
║  EnvAR        │ Attack-Release (simpler)                          ║
║  EnvMulti     │ Multi-stage envelope                              ║
╠═══════════════════════════════════════════════════════════════════╣
║                          EFFECTS                                  ║
╠═══════════════════════════════════════════════════════════════════╣
║  FxDelay      │ Delay line with feedback                          ║
║  FxReverb     │ Reverb (room simulation)                          ║
║  FxDistortion │ Waveshaping distortion                            ║
║  FxChorus     │ Chorus effect                                     ║
║  FxPhaser     │ Phaser effect                                     ║
║  FxFlanger    │ Flanger effect                                    ║
║  FxCompressor │ Dynamics compressor                               ║
║  FxEQ         │ Parametric equalizer                              ║
╠═══════════════════════════════════════════════════════════════════╣
║                        MODULATORS                                 ║
╠═══════════════════════════════════════════════════════════════════╣
║  ModLFO       │ Low frequency oscillator                          ║
║  ModSeq       │ Step sequencer                                    ║
║  ModSampleHold│ Sample and hold                                   ║
║  ModEnvFollow │ Envelope follower                                 ║
║  ModSlew      │ Slew limiter / portamento                         ║
╠═══════════════════════════════════════════════════════════════════╣
║                        UTILITIES                                  ║
╠═══════════════════════════════════════════════════════════════════╣
║  Gain         │ Volume/gain control                               ║
║  Out          │ Audio output (to speakers)                        ║
║  UtilMixer    │ Multi-channel mixer                               ║
║  UtilVCA      │ Voltage controlled amplifier                      ║
║  UtilSplit    │ Signal splitter                                   ║
║  UtilMerge    │ Signal merger                                     ║
║  UtilQuantizer│ Pitch quantizer                                   ║
║  UtilScope    │ Oscilloscope display                              ║
╚═══════════════════════════════════════════════════════════════════╝
```

---

## Commands

```bash
# Add nodes
:node.add OscSine           # Create sine oscillator
:node.add FilterSVF myfilter # Create filter with custom name
:node.add FxReverb          # Create reverb

# Connect nodes
:connect 0:out 1:in         # Connect node 0's output to node 1's input
:connect 2:out 3:in         # Chain node 2 to node 3

# Set parameters
:param.set 0.0 440          # Set node 0, param 0 (freq) to 440
:param.set 1.1 2000         # Set node 1, param 1 (cutoff) to 2000

# Remove nodes
:node.rm 2                  # Remove node with ID 2

# Quick setups
:setup.basic                # OscSine → Out
:setup.fm                   # FM synthesis chain
:setup.pad                  # Ambient pad (sine+saw+reverb)
:setup.drums                # Drum synthesis setup
```

---

## Auto-Connect

When creating nodes, CHURCHKEY automatically:

1. Creates an `Out` node if none exists
2. Connects new nodes to `Out` automatically
3. Maintains the signal chain

This means you can just add nodes and they'll be routed:

```bash
:node.add OscSine     # → Automatically creates Out and connects
:node.add FilterSVF   # → Inserts between OscSine and Out
:node.add FxReverb    # → Inserts into chain
```

---

## Listening: Modular Synthesis Masters

*Knobs, cables, and chaos:*

- [Keith Fullerton Whitman - Generators](https://www.youtube.com/watch?v=5M9Zf5Kj-xI) (modular drone)
- [Surgeon - Dynamic Tension](https://www.youtube.com/watch?v=Z4M8bA-eHGY) (modular techno)
- [Alessandro Cortini - Scuro Chiaro](https://www.youtube.com/watch?v=gKqCk9RZGOA) (Buchla beauty)
- [Kaitlyn Aurelia Smith - An Intention](https://www.youtube.com/watch?v=f0GzXPVZfqI) (Buchla west coast)
- [Caterina Barbieri - Fantas](https://www.youtube.com/watch?v=sDpGKqk0QU8) (sequenced minimalism)
- [Merzbow - Pulse Demon](https://www.youtube.com/watch?v=AguPH0XBxdw) (noise modular destruction)

---

```
         ╔══════════════════════════════════════════════════╗
         ║                                                  ║
         ║   "A patch cable is a question.                 ║
         ║    The answer is whatever sound emerges."       ║
         ║                                                  ║
         ║              — Serge Modular Koan               ║
         ║                                                  ║
         ╚══════════════════════════════════════════════════╝
```
