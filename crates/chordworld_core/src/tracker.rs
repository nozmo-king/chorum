//! Tracker Model - Renoise-like pattern sequencer
//!
//! Patterns contain rows × tracks, each cell has:
//! - NOTE: C-0 to B-9, OFF, or empty
//! - INSTR: 00-FF instrument index
//! - VOL: 00-7F volume
//! - FX1: Effect command + param
//! - FX2: Second effect slot (optional)

use serde::{Deserialize, Serialize};

/// Note representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Note {
    /// No note in this cell
    Empty,
    /// Note off (release)
    Off,
    /// MIDI note number 0-127
    On(u8),
}

impl Note {
    pub fn from_name(name: &str) -> Option<Self> {
        let name = name.trim().to_uppercase();
        if name == "---" || name.is_empty() {
            return Some(Note::Empty);
        }
        if name == "OFF" || name == "===" {
            return Some(Note::Off);
        }

        // Parse note like "C-4", "D#3", "Eb5"
        let chars: Vec<char> = name.chars().collect();
        if chars.len() < 2 {
            return None;
        }

        let note_char = chars[0];
        let base_note = match note_char {
            'C' => 0,
            'D' => 2,
            'E' => 4,
            'F' => 5,
            'G' => 7,
            'A' => 9,
            'B' => 11,
            _ => return None,
        };

        let (sharp_flat, octave_start) = if chars.len() > 2 {
            match chars[1] {
                '#' => (1, 2),
                'B' if chars[0] != 'B' => (-1, 2), // Flat (careful: B note vs b flat)
                '-' => (0, 2),
                _ => (0, 1),
            }
        } else {
            (0, 1)
        };

        let octave_str: String = chars[octave_start..].iter().collect();
        let octave: i32 = octave_str.parse().ok()?;

        let midi_note = (octave + 1) * 12 + base_note + sharp_flat;
        if midi_note >= 0 && midi_note <= 127 {
            Some(Note::On(midi_note as u8))
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Note::Empty => "---".to_string(),
            Note::Off => "OFF".to_string(),
            Note::On(n) => {
                let octave = (*n as i32 / 12) - 1;
                let note_in_octave = *n % 12;
                let note_name = match note_in_octave {
                    0 => "C-",
                    1 => "C#",
                    2 => "D-",
                    3 => "D#",
                    4 => "E-",
                    5 => "F-",
                    6 => "F#",
                    7 => "G-",
                    8 => "G#",
                    9 => "A-",
                    10 => "A#",
                    11 => "B-",
                    _ => "??",
                };
                format!("{}{}", note_name, octave)
            }
        }
    }

    pub fn to_freq(&self) -> Option<f32> {
        match self {
            Note::On(n) => {
                // A4 = 440 Hz = MIDI note 69
                let freq = 440.0 * 2.0_f32.powf((*n as f32 - 69.0) / 12.0);
                Some(freq)
            }
            _ => None,
        }
    }
}

impl Default for Note {
    fn default() -> Self {
        Note::Empty
    }
}

/// Effect command (tracker FX column)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Effect {
    pub cmd: u8,   // Command byte (00 = none)
    pub param: u8, // Parameter byte
}

impl Effect {
    pub fn none() -> Self {
        Self { cmd: 0, param: 0 }
    }

    pub fn new(cmd: u8, param: u8) -> Self {
        Self { cmd, param }
    }

    pub fn is_empty(&self) -> bool {
        self.cmd == 0
    }

    pub fn to_string(&self) -> String {
        if self.cmd == 0 {
            "....".to_string()
        } else {
            format!("{:02X}{:02X}", self.cmd, self.param)
        }
    }
}

/// A single cell in the tracker grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TrackerCell {
    pub note: Note,
    pub instrument: Option<u8>, // None = no change
    pub volume: Option<u8>,     // None = no change, 0-127
    pub fx1: Effect,
    pub fx2: Effect,
}

impl TrackerCell {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn note(note: Note) -> Self {
        Self {
            note,
            ..Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.note == Note::Empty
            && self.instrument.is_none()
            && self.volume.is_none()
            && self.fx1.is_empty()
            && self.fx2.is_empty()
    }
}

/// A track column in a pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: u64,
    pub name: String,
    pub cells: Vec<TrackerCell>,
    pub muted: bool,
    pub solo: bool,
}

impl Track {
    pub fn new(id: u64, name: String, length: usize) -> Self {
        Self {
            id,
            name,
            cells: vec![TrackerCell::empty(); length],
            muted: false,
            solo: false,
        }
    }

    pub fn get_cell(&self, row: usize) -> Option<&TrackerCell> {
        self.cells.get(row)
    }

    pub fn set_cell(&mut self, row: usize, cell: TrackerCell) {
        if row < self.cells.len() {
            self.cells[row] = cell;
        }
    }
}

/// A pattern (grid of tracks × rows)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub id: u64,
    pub name: String,
    pub tracks: Vec<Track>,
    pub length: usize, // Number of rows
    pub lpb: u32,      // Lines per beat
}

impl Pattern {
    pub fn new(id: u64, name: String, num_tracks: usize, length: usize) -> Self {
        let tracks: Vec<Track> = (0..num_tracks)
            .map(|i| Track::new(i as u64, format!("Track {}", i + 1), length))
            .collect();

        Self {
            id,
            name,
            tracks,
            length,
            lpb: 4,
        }
    }

    pub fn get_cell(&self, track: usize, row: usize) -> Option<&TrackerCell> {
        self.tracks.get(track)?.get_cell(row)
    }

    pub fn set_cell(&mut self, track: usize, row: usize, cell: TrackerCell) {
        if let Some(t) = self.tracks.get_mut(track) {
            t.set_cell(row, cell);
        }
    }

    pub fn resize(&mut self, new_length: usize) {
        self.length = new_length;
        for track in &mut self.tracks {
            track.cells.resize(new_length, TrackerCell::empty());
        }
    }

    pub fn add_track(&mut self) -> u64 {
        let id = self.tracks.len() as u64;
        self.tracks.push(Track::new(id, format!("Track {}", id + 1), self.length));
        id
    }
}

/// Song order entry
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SongOrderEntry {
    pub pattern_id: u64,
    pub loop_count: u8, // 0 = play once, 1+ = repeat
}

/// Song (sequence of patterns)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub id: u64,
    pub name: String,
    pub bpm: f32,
    pub order: Vec<SongOrderEntry>,
    pub loop_start: Option<usize>, // Index in order list
    pub loop_end: Option<usize>,
}

impl Song {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            bpm: 120.0,
            order: Vec::new(),
            loop_start: None,
            loop_end: None,
        }
    }

    pub fn add_pattern(&mut self, pattern_id: u64) {
        self.order.push(SongOrderEntry {
            pattern_id,
            loop_count: 0,
        });
    }
}

/// Tracker state (all patterns and songs)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackerState {
    pub patterns: Vec<Pattern>,
    pub songs: Vec<Song>,
    pub current_pattern: usize,
    pub current_song: usize,
}

impl TrackerState {
    pub fn new() -> Self {
        // Create default pattern
        let mut patterns = Vec::new();
        patterns.push(Pattern::new(0, "Pattern 00".to_string(), 4, 64));

        // Create default song
        let mut songs = Vec::new();
        let mut song = Song::new(0, "Song 1".to_string());
        song.add_pattern(0);
        songs.push(song);

        Self {
            patterns,
            songs,
            current_pattern: 0,
            current_song: 0,
        }
    }

    pub fn get_pattern(&self, id: u64) -> Option<&Pattern> {
        self.patterns.iter().find(|p| p.id == id)
    }

    pub fn get_pattern_mut(&mut self, id: u64) -> Option<&mut Pattern> {
        self.patterns.iter_mut().find(|p| p.id == id)
    }

    pub fn current_pattern(&self) -> Option<&Pattern> {
        self.patterns.get(self.current_pattern)
    }

    pub fn current_pattern_mut(&mut self) -> Option<&mut Pattern> {
        self.patterns.get_mut(self.current_pattern)
    }

    pub fn add_pattern(&mut self, name: String, tracks: usize, length: usize) -> u64 {
        let id = self.patterns.len() as u64;
        self.patterns.push(Pattern::new(id, name, tracks, length));
        id
    }
}

impl Default for TrackerState {
    fn default() -> Self {
        Self::new()
    }
}

/// Effect commands (subset for MVP)
pub mod effects {
    pub const NONE: u8 = 0x00;
    pub const ARPEGGIO: u8 = 0x00; // 0xy - arpeggio
    pub const PORTAMENTO_UP: u8 = 0x01;
    pub const PORTAMENTO_DOWN: u8 = 0x02;
    pub const TONE_PORTAMENTO: u8 = 0x03;
    pub const VIBRATO: u8 = 0x04;
    pub const VOLUME_SLIDE: u8 = 0x0A;
    pub const JUMP_TO_ORDER: u8 = 0x0B;
    pub const SET_VOLUME: u8 = 0x0C;
    pub const PATTERN_BREAK: u8 = 0x0D;
    pub const SET_SPEED: u8 = 0x0F;
    pub const SET_GLOBAL_VOLUME: u8 = 0x10;
    pub const NOTE_CUT: u8 = 0xEC;
    pub const NOTE_DELAY: u8 = 0xED;
}
