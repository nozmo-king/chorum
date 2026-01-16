```
    ██████╗  █████╗ ████████╗██╗  ██╗ ██████╗ ██╗      ██████╗  ██████╗ ██╗███████╗███████╗
    ██╔══██╗██╔══██╗╚══██╔══╝██║  ██║██╔═══██╗██║     ██╔═══██╗██╔════╝ ██║██╔════╝██╔════╝
    ██████╔╝███████║   ██║   ███████║██║   ██║██║     ██║   ██║██║  ███╗██║█████╗  ███████╗
    ██╔═══╝ ██╔══██║   ██║   ██╔══██║██║   ██║██║     ██║   ██║██║   ██║██║██╔══╝  ╚════██║
    ██║     ██║  ██║   ██║   ██║  ██║╚██████╔╝███████╗╚██████╔╝╚██████╔╝██║███████╗███████║
    ╚═╝     ╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝ ╚═════╝ ╚══════╝ ╚═════╝  ╚═════╝ ╚═╝╚══════╝╚══════╝

    ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
    █  TEMPORAL & RHYTHMIC HOSTILE CONSTRUCTS · MATHEMATICALLY DEFINABLE · UNPERFORMABLE █
    ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
```

# Temporal Pathologies

> *"These are not jokes; they are hostile edge cases."*

Pathologies are mathematically definable but humanly unperformable musical
constructs. They exist at the boundary between computation and cognition,
exploiting the gaps between what can be specified and what can be executed.

---

## Available Pathologies

### `irrational-meter`
**Continuously Irrational Meter**

Meter defined as √2 / π beats per bar (~0.4502). The bar length is an
irrational number, meaning subdivisions never align perfectly.

```bash
:path irrational-meter base_bpm=120
```

```
    The beat exists, but you cannot count it.
    The measure exists, but you cannot subdivide it.
    The rhythm exists, but you cannot notate it.
```

---

### `golden-polyrhythm`
**Unresolvable Polyrhythm**

Two rhythmic streams with periods in golden ratio (φ = 1.618033...).
They never phase-lock, not even asymptotically. The phase relationship
drifts eternally.

```bash
:path golden-polyrhythm
```

```
    Layer A: ●───●───●───●───●───●───●───●───
    Layer B: ●────●────●────●────●────●────●──
                    (never aligns)
```

---

### `event-horizon`
**Temporal Event Horizon**

Notes accelerate toward infinite density at a finite time. As the target
approaches, inter-onset intervals shrink toward zero (Zeno's paradox).

```bash
:path event-horizon target_time=60 initial_rate=1.0
```

```
    t=0    ●
    t=10   ●
    t=30   ● ●
    t=50   ●●●●
    t=58   ●●●●●●●●
    t=59   ●●●●●●●●●●●●●●●●●●●●●●●●●●●●●●●●
    t=60   ∞ (singularity)
```

---

### `pitch-drift`
**Continuum Pitch Lattice**

Pitch exists on a continuous manifold. Every frequency is valid; there are
no discrete steps. Fixed-tuning instruments cannot participate.

```bash
:path pitch-drift drift_rate=5.0
```

---

### `anchorfree`
**Microtonal Drift Without Reference**

Every pitch is defined relative to the previous pitch. There is no absolute
anchor. Over time, the tonal center wanders unpredictably.

```bash
:path anchorfree cents_per_step=7.0
```

```
    Start: A4 = 440 Hz
    Step 1: +7 cents → 441.78 Hz
    Step 2: -3 cents → 441.02 Hz
    Step 3: +12 cents → 444.09 Hz
    ...
    Step 1000: ???
```

---

### `subliminal`
**Sub-Perceptual Modulation**

Pitch changes occur below the threshold of conscious detection (~5 cents)
but still affect beating patterns and combination tones. You don't hear
the change, but you feel it.

```bash
:path subliminal modulation_hz=0.5 depth_cents=5.0
```

---

### `chaos-groove`
**Uncomputable Groove**

Rhythm defined by a non-computable sequence approximation. The next beat
is unpredictable not by randomness but by fundamental algorithmic limits.

```bash
:path chaos-groove chaos_seed=21e8
```

---

### `saboteur`
**Self-Adversarial Generator**

The algorithm detects emerging patterns and actively sabotages them.
Expectation is trained and then violated at precisely the moment of
recognition.

```bash
:path saboteur entropy=0.7
```

```
    Pattern detected: ascending...
    Expected next: G4
    Sabotage: C#2 (maximally different)

    "The machine hates your predictions."
```

---

### `phantom-voices`
**Phantom Polyphony**

Exploits psychoacoustic combination tones. Two frequencies f₁ and f₂
produce phantom tones at f₂-f₁, 2f₁-f₂, and higher orders. The listener
hears voices that are not in the signal.

```bash
:path phantom-voices fundamental=440 interval_ratio=1.5
```

```
    f₁ = 440 Hz (A4)
    f₂ = 660 Hz (E5)
    ─────────────────
    Phantom: 220 Hz (A3)     [difference tone]
    Phantom: 220 Hz (A3)     [cubic difference]
    Phantom: 1100 Hz (C#6)   [sum tone]
```

---

### `trap`
**False Predictive Model Trap**

Establishes a clear pattern, trains the listener's expectation system,
then violates at sub-reaction times. The surprise arrives before the
prediction can be cancelled.

```bash
:path trap pattern_length=4 violation_prob=0.3
```

---

### `unmeasurable`
**Non-Integrable Tempo**

Tempo follows a function with no closed-form integral:
`tempo(t) = base × (1 + 0.3 × sin(πφt) × sin(et/2) × sin(√2t))`

Bar lengths cannot be precomputed. You can only experience time locally.

```bash
:path unmeasurable complexity=0.8
```

---

### `time-travel`
**Self-Editing Score**

Earlier measures are retroactively altered by later ones. The past changes
based on the future. The score is unstable under observation.

```bash
:path time-travel lookback=4
```

---

## Mathematical Foundations

### Irrational Meter Ratio
```
    ratio = √2 / π ≈ 0.4502...

    This is transcendental × algebraic irrational.
    No rational approximation is exact.
```

### Event Horizon IOI (Inter-Onset Interval)
```
    IOI(t) = initial_rate × (target_time - t)² / target_time

    As t → target_time:
        IOI → 0
        Events → ∞
```

### Golden Polyrhythm
```
    φ = (1 + √5) / 2 ≈ 1.618033988749895

    Period A: T
    Period B: T × φ

    Phase difference after n cycles:
        Δφ = n × (1 - 1/φ) mod 1
        = n × (φ - 1) mod 1
        = n × (1/φ) mod 1

    Since 1/φ is irrational, Δφ never repeats.
```

---

## Listening: Music at the Edge

*Compositions that approach pathological territory:*

- [Conlon Nancarrow - Study No. 37](https://www.youtube.com/watch?v=LFz2lCEkjFk) (impossible tempo canons)
- [Iannis Xenakis - Metastaseis](https://www.youtube.com/watch?v=SZazYFchLRI) (stochastic masses)
- [Autechre - Gantz Graf](https://www.youtube.com/watch?v=ev3vENli7wQ) (algorithmic complexity)
- [Florian Hecker - Chimerization](https://www.youtube.com/watch?v=8p2i8wZP9u8) (psychoacoustic weapons)
- [Ryoji Ikeda - Test Pattern](https://www.youtube.com/watch?v=XwjlYpJCBgk) (perceptual limits)
- [Pan Sonic - Kesto](https://www.youtube.com/watch?v=91VnxZIhvxc) (brutalist electronics)
- [Maryanne Amacher - Sound Characters](https://www.youtube.com/watch?v=P79QxIgRgXc) (otoacoustic emissions)
- [Eliane Radigue - Trilogie de la Mort](https://www.youtube.com/watch?v=QtS9rEt6w24) (drone time dissolution)

---

```
    ╔════════════════════════════════════════════════════════════════════╗
    ║                                                                    ║
    ║   "The only honest music is the music that cannot be played."     ║
    ║                                                                    ║
    ║   "The only true rhythm is the rhythm that cannot be counted."    ║
    ║                                                                    ║
    ║   "The only real pitch is the pitch that cannot be tuned."        ║
    ║                                                                    ║
    ║                        — Pathological Manifesto                   ║
    ║                                                                    ║
    ╚════════════════════════════════════════════════════════════════════╝
```
