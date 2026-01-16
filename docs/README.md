```

     ██████╗██╗  ██╗██╗   ██╗██████╗  ██████╗██╗  ██╗██╗  ██╗███████╗██╗   ██╗
    ██╔════╝██║  ██║██║   ██║██╔══██╗██╔════╝██║  ██║██║ ██╔╝██╔════╝╚██╗ ██╔╝
    ██║     ███████║██║   ██║██████╔╝██║     ███████║█████╔╝ █████╗   ╚████╔╝
    ██║     ██╔══██║██║   ██║██╔══██╗██║     ██╔══██║██╔═██╗ ██╔══╝    ╚██╔╝
    ╚██████╗██║  ██║╚██████╔╝██║  ██║╚██████╗██║  ██║██║  ██╗███████╗   ██║
     ╚═════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝   ╚═╝

              ┌─────────────────────────────────────────────────┐
              │  DETERMINISTIC KEYBOARD-FIRST AUDIO WORKSTATION │
              │     16-Color EGA Palette · Tracker Workflow     │
              │      Patch Graph · Temporal Pathologies         │
              └─────────────────────────────────────────────────┘

```

# CHURCHKEY

> *"The future of music is not in the cloud. It is in the terminal."*

A deterministic, keyboard-first audio workstation combining:
- **Tracker workflow** (Renoise-like pattern sequencer)
- **Patch graph** (Pure Data/Max-style modular routing)
- **16-color EGA palette** (TempleOS-inspired visual constraints)
- **Temporal Pathologies** (mathematically-defined hostile edge cases)
- **21e8 Entropy** (proof-of-work aesthetic hash microtuning)

---

## Documentation

| Document | Description |
|----------|-------------|
| [TRACKER.md](./TRACKER.md) | Pattern sequencer & note entry |
| [PATCH.md](./PATCH.md) | Modular node graph system |
| [PATHOLOGIES.md](./PATHOLOGIES.md) | Temporal & rhythmic hostile constructs |
| [COMMANDS.md](./COMMANDS.md) | Full command reference |
| [PHILOSOPHY.md](./PHILOSOPHY.md) | Design principles & influences |

---

## Quick Start

```bash
# Build
cargo build --release

# Run (requires audio device)
cargo run --release --bin chordworld

# Test TUI without audio
cargo run --release --bin tui_test

# Render test (non-interactive)
cargo run --release --bin render_test
```

---

## Controls

```
┌──────────────────────────────────────────────────────────────┐
│                        NAVIGATION                            │
├──────────────────────────────────────────────────────────────┤
│  F1 / F2 / F3      Switch modes (Tracker / Patch / Mix)     │
│  Arrow Keys        Navigate grid / graph                     │
│  H J K L           Vim-style navigation                      │
│  Tab               Cycle columns (Tracker) / nodes (Patch)  │
│  Page Up/Down      Jump 16 rows                              │
│  Home / End        Jump to start / end of pattern            │
├──────────────────────────────────────────────────────────────┤
│                       NOTE ENTRY                             │
├──────────────────────────────────────────────────────────────┤
│  Z X C V B N M     Notes C D E F G A B (octave 4)           │
│  S D   G H J       Sharps C# D#   F# G# A# (octave 4)       │
│  Q W E R T Y U     Notes C D E F G A B (octave 5)           │
│  2 3   5 6 7       Sharps C# D#   F# G# A# (octave 5)       │
│  1                 Note OFF                                  │
│  `                 Clear cell                                │
├──────────────────────────────────────────────────────────────┤
│                        COMMANDS                              │
├──────────────────────────────────────────────────────────────┤
│  :                 Enter command mode                        │
│  Tab               Autocomplete                              │
│  Ctrl+N            Node browser overlay                      │
│  Ctrl+S            Toggle oscilloscope                       │
│  Ctrl+Q            Quit                                      │
└──────────────────────────────────────────────────────────────┘
```

---

## Listening

*Music that shaped this software:*

### Early Dubstep & Garage
- [Horsepower Productions - Gorgon Sound](https://www.youtube.com/watch?v=VnWDGZG6Bzk)
- [El-B - Buck & Bury](https://www.youtube.com/watch?v=89O_4O6PtcA)
- [Zed Bias - Neighbourhood](https://www.youtube.com/watch?v=6GkbR7i5cWc)
- [Artwork - Red](https://www.youtube.com/watch?v=FArHtvPS6lY)
- [Skream - Midnight Request Line](https://www.youtube.com/watch?v=gGhvaAi09VE)

### Underground Techno
- [Surgeon - Magneze](https://www.youtube.com/watch?v=Z4M8bA-eHGY)
- [Regis - Blood Witness](https://www.youtube.com/watch?v=EQJTYTjPxZI)
- [Female - Male](https://www.youtube.com/watch?v=_uJKqIGbYp4)
- [Orphx - Radiotherapy](https://www.youtube.com/watch?v=YnKf6GhMGIA)
- [Ancient Methods - The Jericho Records](https://www.youtube.com/watch?v=sMXcJLqjmfk)

### Jungle / Early DnB
- [Source Direct - The Cult](https://www.youtube.com/watch?v=Ci0ERyX3pBE)
- [Photek - Ni Ten Ichi Ryu](https://www.youtube.com/watch?v=_WjeWsiujmU)
- [Peshay - Piano Tune](https://www.youtube.com/watch?v=KNz0kqJPk-o)
- [Doc Scott - Shadow Boxing](https://www.youtube.com/watch?v=h3LlVUHmPo0)
- [Dillinja - The Angels Fell](https://www.youtube.com/watch?v=3_LT4sDRJps)

### Experimental / IDM
- [Autechre - Clipper](https://www.youtube.com/watch?v=Ht3kcY0kxFE)
- [Aphex Twin - Ventolin](https://www.youtube.com/watch?v=6Wr-ZSXtrKE)
- [Squarepusher - My Red Hot Car](https://www.youtube.com/watch?v=d8LPgA14Pow)
- [Luke Vibert - I Love Acid](https://www.youtube.com/watch?v=7sXWnmNLZF0)

---

```
                    ╔═══════════════════════════════════════╗
                    ║                                       ║
                    ║   "Every loop is a prayer.            ║
                    ║    Every break is a breath.           ║
                    ║    Every 808 is a heartbeat."         ║
                    ║                                       ║
                    ║              — Unknown                ║
                    ║                                       ║
                    ╚═══════════════════════════════════════╝
```

---

## License

MIT

