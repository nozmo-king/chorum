```
    ████████╗██████╗  █████╗  ██████╗██╗  ██╗███████╗██████╗
    ╚══██╔══╝██╔══██╗██╔══██╗██╔════╝██║ ██╔╝██╔════╝██╔══██╗
       ██║   ██████╔╝███████║██║     █████╔╝ █████╗  ██████╔╝
       ██║   ██╔══██╗██╔══██║██║     ██╔═██╗ ██╔══╝  ██╔══██╗
       ██║   ██║  ██║██║  ██║╚██████╗██║  ██╗███████╗██║  ██║
       ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝

    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
    ░  PATTERN SEQUENCER · RENOISE-STYLE · KEYBOARD-FIRST  ░
    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
```

# Tracker Mode

The tracker is a vertical pattern sequencer inspired by Renoise, ProTracker,
and FastTracker II. Patterns contain rows × tracks, where each cell holds:

```
┌─────────────────────────────────────────────────────────────┐
│                      CELL STRUCTURE                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│     C-4  00  64  0A10  ....                                 │
│     ───  ──  ──  ────  ────                                 │
│      │    │   │    │     │                                  │
│      │    │   │    │     └── FX2: Second effect slot        │
│      │    │   │    └──────── FX1: Effect command + param    │
│      │    │   └───────────── VOL: Volume (00-7F)            │
│      │    └───────────────── INST: Instrument index (00-FF) │
│      └────────────────────── NOTE: C-0 to B-9, OFF, or ---  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Grid Layout

```
ROW    Track 1       Track 2       Track 3       Track 4
────────────────────────────────────────────────────────────
 00   C-4 00 64 ....│ --- -- -- ....│ C-2 01 50 ....│ --- -- -- ....
 01   --- -- -- ....│ --- -- -- ....│ --- -- -- ....│ --- -- -- ....
 02   --- -- -- ....│ D-4 00 -- ....│ --- -- -- ....│ C-5 02 -- ....
 03   --- -- -- ....│ --- -- -- ....│ --- -- -- ....│ --- -- -- ....
 04   E-4 00 64 ....│ --- -- -- ....│ G-2 01 50 ....│ --- -- -- ....
▶05   --- -- -- ....│ --- -- -- ....│ --- -- -- ....│ --- -- -- ....  ← CURSOR
 06   --- -- -- ....│ F-4 00 -- ....│ --- -- -- ....│ D-5 02 -- ....
 07   --- -- -- ....│ --- -- -- ....│ --- -- -- ....│ --- -- -- ....
 08   G-4 00 64 ....│ --- -- -- ....│ C-2 01 50 0A10│ --- -- -- ....
```

---

## Note Entry

The keyboard acts as a piano. Lower row is octave 4, upper row is octave 5:

```
    ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
    │ 2 │ 3 │   │ 5 │ 6 │ 7 │   │ 9 │ 0 │   │   │   │   │  ← Sharps (oct 5)
    │C#5│D#5│   │F#5│G#5│A#5│   │C#6│D#6│   │   │   │   │
    ├───┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴───┴───┤
    │  Q  │ W │ E │ R │ T │ Y │ U │ I │ O │ P │         │  ← Notes (oct 5)
    │ C-5 │D-5│E-5│F-5│G-5│A-5│B-5│C-6│D-6│E-6│         │
    ├─────┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬────────┤
    │  S   │ D │   │ G │ H │ J │   │   │   │   │        │  ← Sharps (oct 4)
    │ C#4  │D#4│   │F#4│G#4│A#4│   │   │   │   │        │
    ├──────┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴────────┤
    │   Z    │ X │ C │ V │ B │ N │ M │   │   │          │  ← Notes (oct 4)
    │  C-4   │D-4│E-4│F-4│G-4│A-4│B-4│   │   │          │
    └────────┴───┴───┴───┴───┴───┴───┴───┴───┴──────────┘

    Special keys:
      1 = Note OFF
      ` = Clear cell (empty)
```

---

## Navigation

```
┌────────────────┬─────────────────────────────────────────┐
│ Key            │ Action                                  │
├────────────────┼─────────────────────────────────────────┤
│ ↑ / k          │ Move up one row                         │
│ ↓ / j          │ Move down one row                       │
│ ← / h          │ Move left (prev column or prev track)   │
│ → / l          │ Move right (next column or next track)  │
│ Tab            │ Next column within track                │
│ Shift+Tab      │ Previous column within track            │
│ Page Up        │ Move up 16 rows                         │
│ Page Down      │ Move down 16 rows                       │
│ Home           │ Jump to row 0                           │
│ End            │ Jump to last row                        │
├────────────────┼─────────────────────────────────────────┤
│ Space / Enter  │ Enter EDIT mode                         │
│ Esc            │ Exit EDIT mode                          │
│ Delete         │ Clear current cell column               │
│ Backspace      │ Clear current cell column               │
└────────────────┴─────────────────────────────────────────┘
```

---

## Effects

Classic tracker effects (subset for MVP):

```
┌────────┬──────────────────────────────────────────────────┐
│ Effect │ Description                                      │
├────────┼──────────────────────────────────────────────────┤
│ 00xy   │ Arpeggio (cycle note, note+x, note+y semitones) │
│ 01xx   │ Portamento up (pitch slide up by xx)            │
│ 02xx   │ Portamento down (pitch slide down by xx)        │
│ 03xx   │ Tone portamento (slide to note at speed xx)     │
│ 04xy   │ Vibrato (speed x, depth y)                      │
│ 0Axy   │ Volume slide (x=up, y=down per tick)            │
│ 0Bxx   │ Jump to order position xx                       │
│ 0Cxx   │ Set volume to xx                                │
│ 0Dxx   │ Pattern break (jump to row xx of next pattern)  │
│ 0Fxx   │ Set speed/tempo (xx < 20 = speed, else tempo)   │
│ ECxx   │ Note cut after xx ticks                         │
│ EDxx   │ Note delay for xx ticks                         │
└────────┴──────────────────────────────────────────────────┘
```

---

## Listening: Tracker Music Legends

*The sound of pixels and patterns:*

- [Venetian Snares - Hajnal](https://www.youtube.com/watch?v=FbJ63spk48s) (Renoise)
- [Squarepusher - Vic Acid](https://www.youtube.com/watch?v=PUB-M7vjdqQ) (hardware tracker vibes)
- [Aphex Twin - Fingerbib](https://www.youtube.com/watch?v=KMvCEqYJnJw) (tracker soul)
- [Ceephax Acid Crew - Probey's Poker](https://www.youtube.com/watch?v=fINXGPu8_-I) (pure acid)
- [Bogdan Raczynski - Samurai Math Beats](https://www.youtube.com/watch?v=qXWiTQK_Wd4) (unhinged)

---

```
           ╭──────────────────────────────────────────────╮
           │                                              │
           │   "The pattern is the meditation.           │
           │    The grid is the garden.                  │
           │    The cursor is the breath."               │
           │                                              │
           │           — Tracker Monk Proverb            │
           │                                              │
           ╰──────────────────────────────────────────────╯
```
