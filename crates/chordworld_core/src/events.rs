//! Event types for CHORDWORLD
//!
//! All events are small POD structs (copyable, no heap allocations)

use crate::{InstrumentId, NodeId, ParamIndex};
use serde::{Deserialize, Serialize};

/// Parameter value (extensible but starts with f32)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ParamValue {
    Float(f32),
    Int(i32),
}

impl ParamValue {
    pub fn as_float(&self) -> f32 {
        match self {
            ParamValue::Float(v) => *v,
            ParamValue::Int(v) => *v as f32,
        }
    }

    pub fn as_int(&self) -> i32 {
        match self {
            ParamValue::Float(v) => *v as i32,
            ParamValue::Int(v) => *v,
        }
    }
}

impl From<f32> for ParamValue {
    fn from(v: f32) -> Self {
        ParamValue::Float(v)
    }
}

impl From<i32> for ParamValue {
    fn from(v: i32) -> Self {
        ParamValue::Int(v)
    }
}

/// Transport state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportState {
    Stopped,
    Playing,
    Paused,
}

/// Clock domain for synchronization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClockDomain {
    Audio,      // Sample-accurate audio clock
    Musical,    // Bar/beat/line grid
    World,      // World-thread event steps
}

/// Core event types
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Event {
    /// Note on event
    NoteOn {
        note: u8,
        velocity: u8,
        instrument: InstrumentId,
        channel: u8,
    },

    /// Note off event
    NoteOff {
        note: u8,
        instrument: InstrumentId,
        channel: u8,
    },

    /// Set parameter value
    ParamSet {
        node: NodeId,
        param: ParamIndex,
        value: ParamValue,
    },

    /// Transport state change
    Transport {
        state: TransportState,
    },

    /// Clock tick for synchronization
    ClockTick {
        domain: ClockDomain,
        step: u64,
    },

    /// User-defined message (escape hatch)
    UserMsg {
        tag: u32,
        a: i32,
        b: i32,
    },
}

impl Event {
    pub fn note_on(note: u8, velocity: u8, instrument: InstrumentId) -> Self {
        Event::NoteOn {
            note,
            velocity,
            instrument,
            channel: 0,
        }
    }

    pub fn note_off(note: u8, instrument: InstrumentId) -> Self {
        Event::NoteOff {
            note,
            instrument,
            channel: 0,
        }
    }

    pub fn param_set(node: NodeId, param: ParamIndex, value: impl Into<ParamValue>) -> Self {
        Event::ParamSet {
            node,
            param,
            value: value.into(),
        }
    }

    pub fn transport(state: TransportState) -> Self {
        Event::Transport { state }
    }
}

/// Bounded event queue for real-time processing
pub const EVENTQ_CAP: usize = 1024;

#[derive(Debug, Clone)]
pub struct EventQueue {
    events: Vec<Event>,
    capacity: usize,
    overflow_count: usize,
}

impl EventQueue {
    pub fn new(capacity: usize) -> Self {
        Self {
            events: Vec::with_capacity(capacity),
            capacity,
            overflow_count: 0,
        }
    }

    pub fn push(&mut self, event: Event) -> bool {
        if self.events.len() < self.capacity {
            self.events.push(event);
            true
        } else {
            self.overflow_count += 1;
            false
        }
    }

    pub fn clear(&mut self) {
        self.events.clear();
        self.overflow_count = 0;
    }

    pub fn iter(&self) -> impl Iterator<Item = &Event> {
        self.events.iter()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn overflow_count(&self) -> usize {
        self.overflow_count
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new(EVENTQ_CAP)
    }
}
