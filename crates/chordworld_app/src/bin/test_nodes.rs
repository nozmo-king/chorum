//! Test new DSP nodes

use chordworld_core::{ConnectionMap, NodeId, ParamIndex, ParamValue, PortEndpoint, Transaction};
use chordworld_dsp::{register_builtin_nodes, NodeRegistry};
use chordworld_engine::{AudioDeviceConfig, ChordworldEngine};
use std::thread;
use std::time::Duration;

/// Simple test helper that creates osc -> out and plays for 2 seconds
fn test_osc(
    engine: &mut ChordworldEngine,
    osc_type: &str,
    name: &str,
    params: &[(u32, f32)],
) -> anyhow::Result<()> {
    println!("=== {} ===", name);

    // Clear all nodes first by creating fresh world
    // Actually we can't do that easily, so let's just add fresh nodes each time
    // The graph handles disconnected nodes fine

    // Get starting node ID by checking current count
    let base_id = engine.world.graph.node_count() as u64 + engine.world.graph.connection_count() as u64 + 1;

    let osc_id = NodeId(base_id);
    let out_id = NodeId(base_id + 1);

    // Add osc and out
    engine.world.apply_transaction(Transaction::NodeAdd {
        node_type: osc_type.into(),
        name: None,
    })?;
    engine.world.apply_transaction(Transaction::NodeAdd {
        node_type: "Out".into(),
        name: None,
    })?;

    // Connect
    engine.world.apply_transaction(Transaction::Connect {
        src: PortEndpoint::new(osc_id, "out"),
        dst: PortEndpoint::new(out_id, "in"),
        map: ConnectionMap::Direct,
    })?;

    // Set params
    for &(idx, val) in params {
        engine.world.apply_transaction(Transaction::ParamSet {
            node: osc_id,
            param: ParamIndex(idx),
            value: ParamValue::Float(val),
        })?;
    }

    engine.update_and_compile()?;
    thread::sleep(Duration::from_secs(2));
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut registry = NodeRegistry::new();
    register_builtin_nodes(&mut registry);

    println!("CHORDWORLD DSP Node Test");
    println!("========================");
    println!("Registered {} node types\n", registry.types().count());

    let doctrine = chordworld_core::DoctrineConfig::strict();
    let world = chordworld_world::WorldState::new(registry, doctrine);
    let audio_config = AudioDeviceConfig::default_config();
    let mut engine = ChordworldEngine::new(world, audio_config)?;

    engine.start()?;
    println!("Audio engine: {} Hz\n", engine.audio.sample_rate());

    // Test basic oscillators
    test_osc(&mut engine, "OscSine", "Sine Wave (440 Hz)", &[(0, 440.0)])?;
    test_osc(&mut engine, "OscSaw", "Sawtooth Wave (220 Hz)", &[(0, 220.0)])?;
    test_osc(&mut engine, "OscSquare", "Square Wave (330 Hz, PW 0.3)", &[(0, 330.0), (1, 0.3)])?;
    test_osc(&mut engine, "OscTriangle", "Triangle Wave (262 Hz)", &[(0, 262.0)])?;
    test_osc(&mut engine, "OscNoise", "Pink Noise", &[(0, 1.0)])?;
    test_osc(&mut engine, "OscFM", "FM Synthesis (220 Hz, ratio 2, idx 4)", &[(0, 220.0), (1, 2.0), (2, 4.0)])?;

    println!("\n=== All tests complete! ===");
    engine.stop();
    Ok(())
}
