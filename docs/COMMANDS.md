```
     ██████╗ ██████╗ ███╗   ███╗███╗   ███╗ █████╗ ███╗   ██╗██████╗ ███████╗
    ██╔════╝██╔═══██╗████╗ ████║████╗ ████║██╔══██╗████╗  ██║██╔══██╗██╔════╝
    ██║     ██║   ██║██╔████╔██║██╔████╔██║███████║██╔██╗ ██║██║  ██║███████╗
    ██║     ██║   ██║██║╚██╔╝██║██║╚██╔╝██║██╔══██║██║╚██╗██║██║  ██║╚════██║
    ╚██████╗╚██████╔╝██║ ╚═╝ ██║██║ ╚═╝ ██║██║  ██║██║ ╚████║██████╔╝███████║
     ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═════╝ ╚══════╝

    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
    ░  FULL COMMAND REFERENCE · TYPE : TO ENTER COMMAND MODE · TAB = COMPLETE ░
    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
```

# Command Reference

Press \`:` to enter command mode. Press \`Tab\` to autocomplete.
Press \`Enter\` to execute. Press \`Esc\` to cancel.

---

## Node Operations

| Command | Description |
|---------|-------------|
| \`:node.add <type>\` | Create node of given type |
| \`:node.add <type> <name>\` | Create node with custom name |
| \`:node.rm <id>\` | Remove node by ID |

### Examples
\`\`\`bash
:node.add OscSine              # Add sine oscillator
:node.add FilterSVF bass_filt  # Add filter named "bass_filt"
:node.add FxReverb             # Add reverb
:node.rm 3                     # Remove node with ID 3
\`\`\`

---

## Connection Operations

| Command | Description |
|---------|-------------|
| \`:connect <src>:<port> <dst>:<port>\` | Connect two nodes |

### Examples
\`\`\`bash
:connect 0:out 1:in            # Node 0 output → Node 1 input
:connect 2:out 3:in            # Node 2 output → Node 3 input
\`\`\`

---

## Parameter Operations

| Command | Description |
|---------|-------------|
| \`:param.set <node>.<param> <val>\` | Set parameter value |

### Examples
\`\`\`bash
:param.set 0.0 440             # Set node 0, param 0 (freq) to 440 Hz
:param.set 1.1 2000            # Set node 1, param 1 (cutoff) to 2000 Hz
\`\`\`

---

## Transport

| Command | Description |
|---------|-------------|
| \`:play\` | Start playback |
| \`:stop\` | Stop playback |
| \`:tempo <bpm>\` | Set tempo in BPM |

---

## Quick Setups

| Command | Creates |
|---------|---------|
| \`:setup.basic\` | OscSine → Out |
| \`:setup.fm\` | FM synthesis chain |
| \`:setup.pad\` | OscSine + OscSaw → FilterSVF → FxReverb |
| \`:setup.drums\` | Drum synthesis setup |

---

## 21e8 Entropy / Microtuning

| Command | Description |
|---------|-------------|
| \`:pow.mine <seed>\` | Mine aesthetic hash from seed text |
| \`:pow.pool.clear\` | Clear entropy pool |
| \`:tuning.set <hash>\` | Set tuning from hex hash |
| \`:tuning.random\` | Generate random microtonal scale |
| \`:tuning.clear\` | Return to 12-TET |
| \`:tuning.show\` | Display current tuning info |

---

## Pathologies

| Command | Description |
|---------|-------------|
| \`:path <name>\` | Apply pathology |
| \`:path <name> <key>=<value>\` | Apply with custom parameters |
| \`:path.list\` | List available pathologies |

### Available Pathologies
- \`irrational-meter\` - Continuously irrational meter (√2/π)
- \`golden-polyrhythm\` - Incommensurable periods (φ ratio)
- \`event-horizon\` - Temporal singularity approach
- \`pitch-drift\` - Continuous pitch manifold
- \`saboteur\` - Self-adversarial pattern generator
- \`phantom-voices\` - Psychoacoustic combination tones
- \`trap\` - False predictive model violation
- \`unmeasurable\` - Non-integrable tempo function

---

```
            ╭─────────────────────────────────────────────╮
            │   "The command line is not an interface.   │
            │    It is a conversation."                  │
            ╰─────────────────────────────────────────────╯
```
