//! Audio device management with CPAL

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Host, Stream, StreamConfig};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("No audio device available")]
    NoDevice,

    #[error("Device error: {0}")]
    Device(String),

    #[error("Stream build error: {0}")]
    BuildStream(#[from] cpal::BuildStreamError),

    #[error("Stream play error: {0}")]
    PlayStream(#[from] cpal::PlayStreamError),
}

/// Represents the actual block size being used
#[derive(Debug, Clone, Copy)]
pub enum ActualBlockSize {
    /// Fixed block size (requested and confirmed)
    Fixed(u32),
    /// Default/variable block size (device decides)
    Default,
}

/// Audio device configuration (requested)
pub struct AudioDeviceConfig {
    pub sample_rate: f32,
    pub channels: u16,
    pub block_size: u32,
}

impl AudioDeviceConfig {
    pub fn default_config() -> Self {
        Self {
            sample_rate: 48000.0,
            channels: 2,
            block_size: 512, // Use 512 for better macOS compatibility
        }
    }
}

/// Actual device configuration (what the device is using)
pub struct ActualDeviceConfig {
    pub sample_rate: f32,
    pub channels: u16,
    pub block_size: ActualBlockSize,
    /// Fallback block size for buffer allocation when using Default
    pub buffer_frames: u32,
}

/// Audio device manager
pub struct AudioDevice {
    _host: Host,
    device: Device,
    config: StreamConfig,
    pub actual_config: ActualDeviceConfig,
}

impl AudioDevice {
    pub fn new(requested_config: AudioDeviceConfig) -> Result<Self, DeviceError> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or(DeviceError::NoDevice)?;

        let supported_config = device
            .default_output_config()
            .map_err(|e| DeviceError::Device(e.to_string()))?;

        let sample_rate = supported_config.sample_rate().0;
        let channels = supported_config.channels();

        // On macOS, use Default buffer size for better compatibility
        // CoreAudio handles buffer sizing automatically
        #[cfg(target_os = "macos")]
        let (buffer_size, block_size_type) = {
            (cpal::BufferSize::Default, ActualBlockSize::Default)
        };

        #[cfg(not(target_os = "macos"))]
        let (buffer_size, block_size_type) = {
            (
                cpal::BufferSize::Fixed(requested_config.block_size),
                ActualBlockSize::Fixed(requested_config.block_size),
            )
        };

        let config = StreamConfig {
            channels,
            sample_rate: cpal::SampleRate(sample_rate),
            buffer_size,
        };

        let actual_config = ActualDeviceConfig {
            sample_rate: sample_rate as f32,
            channels,
            block_size: block_size_type,
            buffer_frames: requested_config.block_size, // Use as fallback for buffer allocation
        };

        Ok(Self {
            _host: host,
            device,
            config,
            actual_config,
        })
    }

    pub fn build_output_stream<D, E>(&self, mut data_callback: D, error_callback: E) -> Result<Stream, DeviceError>
    where
        D: FnMut(&mut [f32]) + Send + 'static,
        E: FnMut(cpal::StreamError) + Send + 'static,
    {
        let stream = self.device.build_output_stream(
            &self.config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                data_callback(data);
            },
            error_callback,
            None,
        )?;

        Ok(stream)
    }

    pub fn sample_rate(&self) -> f32 {
        self.actual_config.sample_rate
    }

    pub fn channels(&self) -> u16 {
        self.actual_config.channels
    }

    /// Returns the buffer frame count for allocation purposes
    pub fn buffer_frames(&self) -> u32 {
        self.actual_config.buffer_frames
    }

    /// Returns whether we're using a fixed or default buffer size
    pub fn block_size_mode(&self) -> ActualBlockSize {
        self.actual_config.block_size
    }
}
