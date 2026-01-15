# CHORDWORLD Doctrine

## Introduction

The **Doctrine** is a set of operational constraints inspired by TempleOS that enforce simplicity, determinism, legibility, anti-bloat principles, and real-time safety. It is not a theological system but a **concrete engineering ruleset** that can be enabled, relaxed, or disabled.

## Doctrine Modes

### STRICT (Default)
Maximum constraints for clarity and determinism.

### RELAXED
Eases some UI constraints while maintaining core determinism and safety.

### OFF
No doctrine enforcement (not recommended for production).

---

## Core Principles

### 1. Real-Time Safety
**Audio thread must never**:
- Allocate heap memory
- Acquire locks
- Perform I/O (file, network, logging)
- Call blocking functions
- Panic (must handle errors gracefully)

**Enforcement**:
- Static analysis (clippy lints)
- Runtime panic guard (mutes output, posts error)
- Allocator instrumentation (optional, for testing)

### 2. Determinism
**Every audio render must be bit-exact**:
- State = f(initial_snapshot, event_log, rng_seeds)
- Offline render == realtime playback (same inputs)
- All randomness uses explicit, versioned seeds

**Enforcement**:
- Transaction log records all mutations
- Snapshot versioning (major.minor.patch)
- Automated determinism hash tests

### 3. Explicitness
**No silent magic**:
- Every action logged in event log
- All automation/modulation sources visible
- No hidden state changes
- Provenance view for every parameter

**Enforcement**:
- Doctrine config: `explicit_logging: true`
- Provenance table tracks all param sources
- Validation reports auto-fixes explicitly

### 4. No Ambiguity
**Commands must**:
- Succeed with clear confirmation, OR
- Fail with precise error message
- Never "partially work" silently

**Enforcement**:
- Transaction results: Success | Error | Deferred
- No warnings that can be ignored
- All errors must be actionable

---

## UI Constraints (Strict Mode)

### Keyboard-Only Operation
- No mouse required or supported
- All actions accessible via keyboard
- No hidden gestures or multi-touch

### 16-Color Palette
```rust
pub enum ColorPalette {
    Sixteen,    // 16 named colors (strict)
    TwoFiftySix, // 256 colors (relaxed)
    Full,       // No restriction (off)
}
```

**Rationale**: Forces clarity through constraint; reduces visual noise.

### Fixed Grid Layout
- No proportional fonts (monospace only)
- No dynamic reflow beyond simple panes
- Text-based layout, not pixel-based

### Maximum Menu Depth
**Strict**: 7 levels maximum
**Relaxed**: 15 levels

**Escape hatches**:
- Jump search (`/`): Fuzzy search over objects/commands
- Go-to path (`g`): Direct path navigation (`graph/node:42/param:cutoff`)
- Command palette (`:`): Verb-first structured commands

---

## Graph Constraints (Strict Mode)

### Node Limits
- **Max nodes**: 1024 (strict), 4096 (relaxed)
- **Max connections**: 4096 (strict), 16384 (relaxed)

**Rationale**: Prevents runaway graph complexity; forces modular design.

### Feedback Guard
Cycles must have:
- At least one `Delay` node in the loop, AND
- Effective delay ≥ `min_feedback_delay_samples` (default: 256, = block size)

**Enforcement**:
- Graph validation rejects invalid cycles
- Auto-fix can insert delays (logged explicitly)
- Cycle detection runs on every graph mutation

---

## Behavioral Constraints

### No Background Services
- No telemetry collection
- No automatic update checks
- No hidden network calls
- No background threads (except audio + world)

**Rationale**: User control and transparency; no surprise behavior.

### No Silent Failures
- All errors surfaced to user
- No "best effort" that hides problems
- Overflow in event queues = error counter (logged)

### Simple File Formats
**Strict mode prefers**:
- Binary CBOR for snapshots (compact, typed)
- Append-only log for transactions
- No complex embedded databases

**Rationale**: Auditable, debuggable, version-controllable.

---

## Doctrine Configuration

```rust
pub struct DoctrineConfig {
    pub mode: DoctrineMode,

    // UI
    pub keyboard_only: bool,
    pub color_palette: ColorPalette,
    pub fixed_grid_layout: bool,
    pub max_menu_depth: Option<u32>,

    // Behavior
    pub explicit_logging: bool,
    pub require_provenance: bool,
    pub no_background_services: bool,
    pub no_silent_magic: bool,
    pub no_ambiguity: bool,

    // Graph
    pub max_node_count: Option<usize>,
    pub max_connection_count: Option<usize>,
    pub min_feedback_delay_samples: usize,

    // Files
    pub simple_file_formats: bool,
}
```

### Creating Configs

```rust
// Strict mode (default)
let config = DoctrineConfig::strict();

// Relaxed mode
let config = DoctrineConfig::relaxed();

// Custom
let mut config = DoctrineConfig::strict();
config.max_menu_depth = Some(10);
config.color_palette = ColorPalette::TwoFiftySix;
```

---

## Validation & Enforcement

### Compile-Time
- Trait bounds (`DspNode: Send`, no `&mut` in audio thread)
- Type system prevents unsafe sharing

### Load-Time
- Project version compatibility check
- Node type version migration or error

### Runtime
- Transaction validation before apply
- Graph validation on mutations
- Doctrine limit checks

### Testing
**Automated Acceptance Tests**:
1. **Determinism Render Hash**: Replay must match golden hash
2. **No Allocation in Audio**: Instrumented allocator detects violations
3. **Feedback Guard**: Invalid cycles rejected
4. **Transaction Replay Integrity**: Log replay produces canonical state
5. **UI Smoke Test**: Headless command execution (no panics)

---

## Why These Rules?

### Inspiration: TempleOS
TempleOS enforced radical simplicity:
- 16-color graphics
- Fixed 640×480 resolution
- No networking
- No privilege separation
- Divine guidance aesthetics

**CHORDWORLD adapts these ideas as engineering constraints**:
- Simplicity through limitation
- Determinism through explicitness
- Legibility through anti-bloat
- User sovereignty through transparency

### Not Ideology, Engineering
The Doctrine is **pragmatic**:
- Improves testability (determinism)
- Reduces cognitive load (simplicity)
- Enables inspection (provenance)
- Prevents entire classes of bugs (real-time safety)

**You can turn it off** if these constraints don't serve you.

---

## Doctrine Violations

### How Violations Are Handled

**Strict Mode**:
- Transaction rejected with error message
- Graph mutation blocked
- User prompted to fix or relax doctrine

**Relaxed Mode**:
- Warnings logged
- Most limits raised 4x
- Still enforces core safety (no alloc in audio)

**Off Mode**:
- No enforcement
- User responsible for safety
- Not recommended for production

### Example Violations

```
// Node limit exceeded
Error: Node count 1025 exceeds doctrine limit 1024.
Hint: Use 'doctrine relax' or refactor into macros.

// Menu depth exceeded
Error: Menu depth 8 exceeds doctrine limit 7.
Hint: Use jump search (/) or command palette (:).

// Cycle without delay
Error: Cycle detected without sufficient delay.
Auto-fix: Insert Delay node (min 256 samples)? [y/N]
```

---

## Future Extensions

As CHORDWORLD evolves into **CHORUM**, the Doctrine will expand:

### Synthesis & Microtuning
- Tuning table constraints
- Modulation depth limits
- Anti-aliasing requirements

### Ontology & Grammar
- Path grammar restrictions
- Command verb whitelisting
- Object naming conventions

---

## Summary

The Doctrine is a **tool for correctness**, not a dogma. It makes CHORDWORLD:
- **Safe**: No audio thread allocations or crashes
- **Deterministic**: Bit-exact replay from event log
- **Legible**: All state changes explicit and inspectable
- **Simple**: Constraints force minimal, modular design

You control the strictness. Strict mode is the default because it catches bugs early and enforces good practices. Relaxed mode eases constraints for power users. Off mode gives you the raw system with no guardrails.

**The Doctrine serves you. You do not serve the Doctrine.**

---

**Version**: 0.1.0
**Date**: 2026-01-15
