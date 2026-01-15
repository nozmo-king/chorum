//! Time domain representations

use serde::{Deserialize, Serialize};
use std::fmt;

/// Sample time (absolute frame index)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SampleTime(pub u64);

impl SampleTime {
    pub fn zero() -> Self {
        Self(0)
    }

    pub fn from_samples(samples: u64) -> Self {
        Self(samples)
    }

    pub fn as_samples(&self) -> u64 {
        self.0
    }

    pub fn add(&self, samples: u64) -> Self {
        Self(self.0 + samples)
    }
}

/// Block time (block index + in-block offset)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockTime {
    pub block: u64,
    pub offset: u32,
}

impl BlockTime {
    pub fn new(block: u64, offset: u32) -> Self {
        Self { block, offset }
    }

    pub fn to_sample_time(&self, block_size: u32) -> SampleTime {
        SampleTime((self.block * block_size as u64) + self.offset as u64)
    }
}

/// Musical time (bar:beat:line)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MusicalTime {
    pub bar: u32,
    pub beat: u32,
    pub line: u32,
    pub tick: u32, // Sub-line tick (optional subdivision)
}

impl MusicalTime {
    pub fn new(bar: u32, beat: u32, line: u32) -> Self {
        Self {
            bar,
            beat,
            line,
            tick: 0,
        }
    }

    pub fn zero() -> Self {
        Self {
            bar: 0,
            beat: 0,
            line: 0,
            tick: 0,
        }
    }

    pub fn with_tick(mut self, tick: u32) -> Self {
        self.tick = tick;
        self
    }

    /// Convert to total line count (for comparison)
    pub fn to_lines(&self, beats_per_bar: u32, lines_per_beat: u32) -> u64 {
        let lines_per_bar = beats_per_bar * lines_per_beat;
        (self.bar as u64 * lines_per_bar as u64)
            + (self.beat as u64 * lines_per_beat as u64)
            + self.line as u64
    }
}

impl fmt::Display for MusicalTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.tick > 0 {
            write!(
                f,
                "{}:{:02}:{:02}.{}",
                self.bar, self.beat, self.line, self.tick
            )
        } else {
            write!(f, "{}:{:02}:{:02}", self.bar, self.beat, self.line)
        }
    }
}

/// World time (event-step counter)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct WorldTime(pub u64);

impl WorldTime {
    pub fn zero() -> Self {
        Self(0)
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

/// Timing configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimingConfig {
    pub sample_rate: f32,
    pub block_size: u32,
    pub bpm: f32,
    pub beats_per_bar: u32,
    pub lines_per_beat: u32,
}

impl TimingConfig {
    pub fn default_config() -> Self {
        Self {
            sample_rate: 48000.0,
            block_size: 256,
            bpm: 120.0,
            beats_per_bar: 4,
            lines_per_beat: 4,
        }
    }

    /// Convert sample time to musical time
    pub fn samples_to_musical(&self, samples: SampleTime) -> MusicalTime {
        let samples = samples.as_samples() as f32;
        let samples_per_beat = (60.0 / self.bpm) * self.sample_rate;
        let samples_per_line = samples_per_beat / self.lines_per_beat as f32;

        let total_lines = (samples / samples_per_line) as u32;
        let lines_per_bar = self.beats_per_bar * self.lines_per_beat;

        let bar = total_lines / lines_per_bar;
        let remaining_lines = total_lines % lines_per_bar;
        let beat = remaining_lines / self.lines_per_beat;
        let line = remaining_lines % self.lines_per_beat;

        MusicalTime::new(bar, beat, line)
    }

    /// Convert musical time to sample time
    pub fn musical_to_samples(&self, musical: MusicalTime) -> SampleTime {
        let total_lines = musical.to_lines(self.beats_per_bar, self.lines_per_beat);
        let samples_per_beat = (60.0 / self.bpm) * self.sample_rate;
        let samples_per_line = samples_per_beat / self.lines_per_beat as f32;
        let samples = (total_lines as f32 * samples_per_line) as u64;
        SampleTime(samples)
    }
}
