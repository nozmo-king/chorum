//! Audio device management with CPAL

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
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

/// Audio device configuration
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
            block_size: 256,
        }
    }
}

/// Audio device manager
pub struct AudioDevice {
    _host: Host,
    device: Device,
    config: StreamConfig,
    pub actual_config: AudioDeviceConfig,
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

        let config = StreamConfig {
            channels,
            sample_rate: cpal::SampleRate(sample_rate),
            buffer_size: cpal::BufferSize::Fixed(requested_config.block_size),
        };

        let actual_config = AudioDeviceConfig {
            sample_rate: sample_rate as f32,
            channels,
            block_size: requested_config.block_size,
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

    pub fn block_size(&self) -> u32 {
        self.actual_config.block_size
    }
}
