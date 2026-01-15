//! Processing context and types

use chordworld_core::SampleTime;

/// Processing context passed to nodes during process()
pub struct ProcessCtx {
    pub sample_rate: f32,
    pub block_size: u32,
    pub start_time: SampleTime,
}

impl ProcessCtx {
    pub fn new(sample_rate: f32, block_size: u32, start_time: SampleTime) -> Self {
        Self {
            sample_rate,
            block_size,
            start_time,
        }
    }

    pub fn samples_per_second(&self) -> f32 {
        self.sample_rate
    }

    pub fn block_duration_secs(&self) -> f32 {
        self.block_size as f32 / self.sample_rate
    }
}

/// Port type specification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortType {
    AudioIn,
    AudioOut,
    CtrlIn,
    CtrlOut,
    EventIn,
    EventOut,
}

impl PortType {
    pub fn is_input(&self) -> bool {
        matches!(self, PortType::AudioIn | PortType::CtrlIn | PortType::EventIn)
    }

    pub fn is_output(&self) -> bool {
        !self.is_input()
    }

    pub fn is_audio(&self) -> bool {
        matches!(self, PortType::AudioIn | PortType::AudioOut)
    }

    pub fn is_control(&self) -> bool {
        matches!(self, PortType::CtrlIn | PortType::CtrlOut)
    }

    pub fn is_event(&self) -> bool {
        matches!(self, PortType::EventIn | PortType::EventOut)
    }

    pub fn is_compatible(&self, other: PortType) -> bool {
        match (self, other) {
            (PortType::AudioOut, PortType::AudioIn) => true,
            (PortType::CtrlOut, PortType::CtrlIn) => true,
            (PortType::EventOut, PortType::EventIn) => true,
            _ => false,
        }
    }
}

/// Port specification
#[derive(Debug, Clone)]
pub struct PortSpec {
    pub name: &'static str,
    pub port_type: PortType,
    pub channels: u32, // For audio ports
}

impl PortSpec {
    pub const fn audio_in(name: &'static str, channels: u32) -> Self {
        Self {
            name,
            port_type: PortType::AudioIn,
            channels,
        }
    }

    pub const fn audio_out(name: &'static str, channels: u32) -> Self {
        Self {
            name,
            port_type: PortType::AudioOut,
            channels,
        }
    }

    pub const fn ctrl_in(name: &'static str) -> Self {
        Self {
            name,
            port_type: PortType::CtrlIn,
            channels: 1,
        }
    }

    pub const fn ctrl_out(name: &'static str) -> Self {
        Self {
            name,
            port_type: PortType::CtrlOut,
            channels: 1,
        }
    }

    pub const fn event_in(name: &'static str) -> Self {
        Self {
            name,
            port_type: PortType::EventIn,
            channels: 1,
        }
    }

    pub const fn event_out(name: &'static str) -> Self {
        Self {
            name,
            port_type: PortType::EventOut,
            channels: 1,
        }
    }
}

/// Parameter specification
#[derive(Debug, Clone)]
pub struct ParamSpec {
    pub name: &'static str,
    pub default: f32,
    pub min: f32,
    pub max: f32,
    pub unit: &'static str,
}

impl ParamSpec {
    pub const fn new(name: &'static str, default: f32, min: f32, max: f32, unit: &'static str) -> Self {
        Self {
            name,
            default,
            min,
            max,
            unit,
        }
    }

    pub fn clamp(&self, value: f32) -> f32 {
        value.clamp(self.min, self.max)
    }

    pub fn normalize(&self, value: f32) -> f32 {
        (value - self.min) / (self.max - self.min)
    }

    pub fn denormalize(&self, normalized: f32) -> f32 {
        self.min + normalized * (self.max - self.min)
    }
}

/// Control value (block-rate or per-sample in future)
#[derive(Debug, Clone, Copy)]
pub struct CtrlValue {
    pub value: f32,
}

impl CtrlValue {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn as_f32(&self) -> f32 {
        self.value
    }
}
