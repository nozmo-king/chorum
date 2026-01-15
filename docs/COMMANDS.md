# CHORDWORLD Command Reference

## Command System

All commands are typed into the command prompt (`:`) and follow a structured syntax.

## Global Commands

### Help
```
help
```
Shows command help (context-sensitive).

### Quit
```
Ctrl+Q
```
Quit CHORDWORLD.

---

## Patch Commands

### Node Operations

#### node.add
Add a new node to the graph.
```
node.add <type> [name]
```

**Examples**:
```
node.add OscSine
node.add OscSine osc1
node.add Gain
node.add Out output
```

**Built-in node types** (v0.1):
- `OscSine`: Sine wave oscillator
- `Gain`: Amplitude control
- `Out`: Audio output

#### node.rm / node.remove
Remove a node from the graph.
```
node.rm <node_id|name>
```

**Examples**:
```
node.rm 1
node.rm osc1
```

### Connection Operations

#### connect
Connect two node ports.
```
connect <src_node>:<src_port> <dst_node>:<dst_port>
```

**Examples**:
```
connect 1:out 2:in
connect osc1:out gain:in
connect gain:out output:in
```

#### disconnect
Disconnect a specific connection.
```
disconnect <connection_id>
```

#### disconnect.node
Disconnect all connections for a node.
```
disconnect.node <node_id|name>
```

### Parameter Operations

#### param.set
Set a parameter value.
```
param.set <node>.<param_index> <value>
```

**Examples**:
```
param.set 1.0 440.0    # Set freq (param 0) to 440 Hz
param.set 1.1 0.8      # Set amp (param 1) to 0.8
param.set osc1.0 880.0 # Set osc1 freq to 880 Hz
```

**Common parameter indices**:
- **OscSine**: 0=freq, 1=amp
- **Gain**: 0=gain

---

## Transport Commands

### play
Start playback.
```
play
```
Keyboard shortcut: `Space`

### stop
Stop playback and reset to beginning.
```
stop
```

### tempo
Set the tempo in BPM.
```
tempo <bpm>
```

**Example**:
```
tempo 140
```

---

## Mode Switching

### Mode Keys
- `F1`: Help
- `Tab`: Cycle through panes
- `:`: Command mode
- `/`: Jump search
- `g`: Go-to path
- `Esc`: Back/Cancel

---

## Quick Examples

### Create a simple patch
```
:node.add OscSine osc1
:node.add Gain gain
:node.add Out out
:connect osc1:out gain:in
:connect gain:out out:in
:param.set osc1.0 440.0
:param.set gain.0 0.5
:play
```

This creates:
```
OscSine (440 Hz) → Gain (0.5) → Out
```

### Adjust parameters while playing
```
:param.set osc1.0 880.0    # Double the frequency
:param.set gain.0 0.2      # Reduce volume
```

---

## Future Commands (Not Yet Implemented)

These commands are planned for future versions:

### Pattern Commands
```
pattern.create <name> <rows> <tracks>
pattern.set <pattern> <row> <track> <note> [velocity] [instrument]
```

### Macro Commands
```
macro.create <name> <node...>
macro.instantiate <name> [instance_name]
macro.expand <node>
```

### Validation Commands
```
graph.validate
graph.feedback_guard <on|off>
```

---

**Version**: 0.1.0
