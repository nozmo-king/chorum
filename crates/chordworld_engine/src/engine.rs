//! Audio engine with threaded architecture

use crate::{AudioDevice, AudioDeviceConfig, DeviceError};
use chordworld_core::SampleTime;
use chordworld_dsp::BufferPool;
use chordworld_world::{GraphSnapshot, WorldState};
use crossbeam_channel::{Sender, bounded};
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Device error: {0}")]
    Device(#[from] DeviceError),

    #[error("Engine not running")]
    NotRunning,
}

/// Message from World thread to Engine
pub enum WorldMessage {
    UpdateSnapshot(Arc<GraphSnapshot>),
    Stop,
}

/// Audio engine state
pub struct AudioEngine {
    device: AudioDevice,
    stream: Option<cpal::Stream>,
    snapshot_tx: Sender<Arc<GraphSnapshot>>,
    current_time: Arc<Mutex<SampleTime>>,
}

impl AudioEngine {
    pub fn new(config: AudioDeviceConfig) -> Result<Self, EngineError> {
        let device = AudioDevice::new(config)?;
        let (snapshot_tx, _snapshot_rx) = bounded(2);

        Ok(Self {
            device,
            stream: None,
            snapshot_tx,
            current_time: Arc::new(Mutex::new(SampleTime::zero())),
        })
    }

    pub fn start(&mut self, initial_snapshot: Arc<GraphSnapshot>) -> Result<(), EngineError> {
        let buffer_frames = self.device.buffer_frames();
        let channels = self.device.channels() as usize;
        let _sample_rate = self.device.sample_rate();

        // Create channel for snapshot updates
        let (snapshot_tx, snapshot_rx) = bounded::<Arc<GraphSnapshot>>(2);
        self.snapshot_tx = snapshot_tx;

        // Initialize with the initial snapshot
        let current_snapshot = Arc::new(Mutex::new(initial_snapshot));
        let current_snapshot_clone = current_snapshot.clone();

        // Audio thread state - use larger buffer pool for variable buffer sizes on macOS
        let max_buffer_size = buffer_frames.max(2048) as usize;
        let mut buffer_pool = BufferPool::new(64, max_buffer_size);
        let mut sample_time = SampleTime::zero();
        let time_counter = self.current_time.clone();

        // Build the audio stream
        let stream = self.device.build_output_stream(
            move |output: &mut [f32]| {
                // Calculate actual block size from output buffer
                let actual_block_size = (output.len() / channels) as u32;

                // Check for snapshot updates (non-blocking)
                if let Ok(new_snapshot) = snapshot_rx.try_recv() {
                    *current_snapshot_clone.lock().unwrap() = new_snapshot;
                }

                // Clear output
                output.fill(0.0);

                // Get current snapshot
                let snapshot = current_snapshot_clone.lock().unwrap();

                // Process the graph with actual block size
                buffer_pool.clear_all();
                snapshot.process(&mut buffer_pool, actual_block_size, sample_time);

                // Copy to output (simplified - assumes buffer 0 is output)
                if let Some(buffer) = buffer_pool.get(0) {
                    for (i, sample) in output.iter_mut().enumerate() {
                        let frame = i / channels;
                        if frame < buffer.len() {
                            *sample = buffer[frame];
                        }
                    }
                }

                // Advance time
                sample_time = sample_time.add(actual_block_size as u64);
                *time_counter.lock().unwrap() = sample_time;
            },
            |err| {
                eprintln!("Audio stream error: {}", err);
            },
        )?;

        use cpal::traits::StreamTrait;
        stream.play().map_err(DeviceError::PlayStream)?;

        self.stream = Some(stream);

        Ok(())
    }

    pub fn stop(&mut self) {
        self.stream = None;
    }

    pub fn update_snapshot(&self, snapshot: Arc<GraphSnapshot>) -> Result<(), EngineError> {
        self.snapshot_tx
            .send(snapshot)
            .map_err(|_| EngineError::NotRunning)?;
        Ok(())
    }

    pub fn current_time(&self) -> SampleTime {
        *self.current_time.lock().unwrap()
    }

    pub fn sample_rate(&self) -> f32 {
        self.device.sample_rate()
    }

    pub fn is_running(&self) -> bool {
        self.stream.is_some()
    }
}

/// Combined engine + world wrapper
pub struct ChordworldEngine {
    pub audio: AudioEngine,
    pub world: WorldState,
}

impl ChordworldEngine {
    pub fn new(
        world: WorldState,
        audio_config: AudioDeviceConfig,
    ) -> Result<Self, EngineError> {
        let audio = AudioEngine::new(audio_config)?;

        Ok(Self { audio, world })
    }

    pub fn start(&mut self) -> Result<(), EngineError> {
        // Compile initial snapshot
        let snapshot = self.world.compile_snapshot()
            .map_err(|e| EngineError::Device(DeviceError::Device(e.to_string())))?;

        self.audio.start(Arc::new(snapshot))?;

        Ok(())
    }

    pub fn update_and_compile(&mut self) -> Result<(), EngineError> {
        let snapshot = self.world.compile_snapshot()
            .map_err(|e| EngineError::Device(DeviceError::Device(e.to_string())))?;

        self.audio.update_snapshot(Arc::new(snapshot))?;

        Ok(())
    }

    pub fn stop(&mut self) {
        self.audio.stop();
    }
}
