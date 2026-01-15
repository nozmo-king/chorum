//! Direct audio test - bypasses node graph

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32::consts::TAU;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device");

    println!("Using device: {}", device.name()?);

    let config = device.default_output_config()?;
    println!("Sample rate: {}", config.sample_rate().0);
    println!("Channels: {}", config.channels());

    let sample_rate = config.sample_rate().0 as f32;
    let channels = config.channels() as usize;

    let mut phase = 0.0f32;
    let freq = 440.0; // A4

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                let sample = (phase * TAU).sin() * 0.3;
                phase += freq / sample_rate;
                if phase >= 1.0 { phase -= 1.0; }

                for s in frame.iter_mut() {
                    *s = sample;
                }
            }
        },
        |err| eprintln!("Stream error: {}", err),
        None,
    )?;

    stream.play()?;
    println!("\nPlaying 440 Hz sine wave for 3 seconds...");
    std::thread::sleep(Duration::from_secs(3));

    println!("Done!");
    Ok(())
}
