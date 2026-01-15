//! Tracker/sequencer data structures

use chordworld_core::{InstrumentId, PatternId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Note value in a tracker pattern
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Note {
    pub note: u8,      // MIDI note number
    pub octave: u8,    // Octave (for display)
    pub velocity: u8,  // Velocity (1-127, 0 = use default)
}

impl Note {
    pub fn new(note: u8, octave: u8) -> Self {
        Self {
            note,
            octave,
            velocity: 0,
        }
    }

    pub fn with_velocity(mut self, velocity: u8) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn midi_note(&self) -> u8 {
        self.note
    }
}

/// Effect command in a pattern cell
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Effect {
    pub command: u8, // Effect type
    pub value: u8,   // Effect parameter
}

/// Cell in a pattern (one row, one track)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatternCell {
    pub note: Option<Note>,
    pub instrument: Option<InstrumentId>,
    pub effects: Vec<Effect>, // Can have multiple effects
}

impl PatternCell {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn with_note(note: Note) -> Self {
        Self {
            note: Some(note),
            instrument: None,
            effects: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.note.is_none() && self.instrument.is_none() && self.effects.is_empty()
    }
}

/// A track in a pattern (one column)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub cells: Vec<PatternCell>,
}

impl Track {
    pub fn new(rows: usize) -> Self {
        Self {
            cells: vec![PatternCell::empty(); rows],
        }
    }

    pub fn get_cell(&self, row: usize) -> Option<&PatternCell> {
        self.cells.get(row)
    }

    pub fn get_cell_mut(&mut self, row: usize) -> Option<&mut PatternCell> {
        self.cells.get_mut(row)
    }

    pub fn set_cell(&mut self, row: usize, cell: PatternCell) {
        if row < self.cells.len() {
            self.cells[row] = cell;
        }
    }
}

/// A pattern (grid of tracks x rows)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub id: PatternId,
    pub name: String,
    pub tracks: Vec<Track>,
    pub row_count: usize,
}

impl Pattern {
    pub fn new(id: PatternId, name: String, rows: usize, track_count: usize) -> Self {
        let tracks = (0..track_count).map(|_| Track::new(rows)).collect();

        Self {
            id,
            name,
            tracks,
            row_count: rows,
        }
    }

    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }

    pub fn get_cell(&self, row: usize, track: usize) -> Option<&PatternCell> {
        self.tracks.get(track)?.get_cell(row)
    }

    pub fn get_cell_mut(&mut self, row: usize, track: usize) -> Option<&mut PatternCell> {
        self.tracks.get_mut(track)?.get_cell_mut(row)
    }

    pub fn set_note(
        &mut self,
        row: usize,
        track: usize,
        note: Option<Note>,
        instrument: Option<InstrumentId>,
    ) {
        if let Some(cell) = self.get_cell_mut(row, track) {
            cell.note = note;
            if instrument.is_some() {
                cell.instrument = instrument;
            }
        }
    }
}

/// Song structure (arrangement of patterns)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub name: String,
    pub sequence: Vec<PatternId>, // Order list
    pub patterns: HashMap<PatternId, Pattern>,
    pub bpm: f32,
    pub lpb: u32, // Lines per beat
}

impl Song {
    pub fn new(name: String) -> Self {
        Self {
            name,
            sequence: Vec::new(),
            patterns: HashMap::new(),
            bpm: 120.0,
            lpb: 4,
        }
    }

    pub fn add_pattern(&mut self, pattern: Pattern) {
        let id = pattern.id;
        self.patterns.insert(id, pattern);
    }

    pub fn get_pattern(&self, id: PatternId) -> Option<&Pattern> {
        self.patterns.get(&id)
    }

    pub fn get_pattern_mut(&mut self, id: PatternId) -> Option<&mut Pattern> {
        self.patterns.get_mut(&id)
    }
}

/// Instrument definition (simplified for now)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    pub id: InstrumentId,
    pub name: String,
    // Future: reference to node graph macro, sample data, etc.
}

impl Instrument {
    pub fn new(id: InstrumentId, name: String) -> Self {
        Self { id, name }
    }
}
