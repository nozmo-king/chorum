//! CHORDWORLD Main Application

use chordworld_core::{DoctrineConfig, Transaction, PortEndpoint, ConnectionMap};
use chordworld_dsp::{register_builtin_nodes, NodeRegistry};
use chordworld_engine::{AudioDeviceConfig, ChordworldEngine};
use chordworld_tui::{
    handle_input, render, App, ConnectionInfo, EngineMessage, GraphState, NodeInfo, TuiMessage,
};
use crossbeam_channel::bounded;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

/// Build graph state from engine for TUI display
fn build_graph_state(engine: &ChordworldEngine) -> GraphState {
    let mut nodes = Vec::new();
    let mut connections = Vec::new();

    for node in engine.world.graph.nodes() {
        nodes.push(NodeInfo {
            id: node.id.0,
            name: node.name.clone(),
            node_type: node.node_type.clone(),
            bypassed: node.bypassed,
        });
    }

    for conn in engine.world.graph.connections() {
        connections.push(ConnectionInfo {
            id: conn.id.0,
            src_node: conn.src_node.0,
            src_port: conn.src_port.clone(),
            dst_node: conn.dst_node.0,
            dst_port: conn.dst_port.clone(),
        });
    }

    let tuning_name = engine.world.active_tuning.as_ref().map(|t| t.name.clone());
    let entropy_count = engine.world.entropy_pool.len();

    GraphState {
        nodes,
        connections,
        tuning_name,
        entropy_count,
    }
}

/// Find Out node ID if it exists
fn find_out_node(engine: &ChordworldEngine) -> Option<u64> {
    engine.world.graph.nodes()
        .find(|n| n.node_type == "Out")
        .map(|n| n.id.0)
}

/// Find last non-Out node for auto-connect
fn find_last_source_node(engine: &ChordworldEngine) -> Option<u64> {
    engine.world.graph.nodes()
        .filter(|n| n.node_type != "Out")
        .last()
        .map(|n| n.id.0)
}

fn main() -> anyhow::Result<()> {
    // Initialize node registry
    let mut registry = NodeRegistry::new();
    register_builtin_nodes(&mut registry);

    // Create world state with strict doctrine
    let doctrine = DoctrineConfig::strict();
    let world = chordworld_world::WorldState::new(registry, doctrine);

    // Create audio config
    let audio_config = AudioDeviceConfig::default_config();

    // Create engine
    let mut engine = ChordworldEngine::new(world, audio_config)?;

    // Start audio engine
    engine.start()?;

    println!("Audio engine started at {} Hz", engine.audio.sample_rate());

    // Create channels for TUI <-> Engine communication
    let (tui_tx, tui_rx) = bounded::<TuiMessage>(32);
    let (engine_tx, engine_rx) = bounded::<EngineMessage>(32);

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(tui_tx.clone(), engine_rx);

    // Main loop
    loop {
        terminal.draw(|f| render(f, &app))?;

        handle_input(&mut app)?;

        // Process messages from TUI
        while let Ok(msg) = tui_rx.try_recv() {
            match msg {
                TuiMessage::Transaction(transaction) => {
                    // Check if this is a NodeAdd for auto-connect
                    let is_node_add = matches!(transaction, Transaction::NodeAdd { .. });
                    let added_node_type = if let Transaction::NodeAdd { ref node_type, .. } = transaction {
                        Some(node_type.clone())
                    } else {
                        None
                    };

                    match engine.world.apply_transaction(transaction) {
                        Ok(result) => {
                            engine_tx.send(EngineMessage::TransactionResult(result.clone())).ok();

                            // Auto-connect logic for new nodes
                            if is_node_add && result.is_success() && app.auto_connect {
                                if let Some(node_type) = added_node_type {
                                    // Find the newly added node (it's the last one)
                                    if let Some(new_node) = engine.world.graph.nodes().last() {
                                        let new_id = new_node.id;

                                        // If it's an Out node, connect last source to it
                                        if node_type == "Out" {
                                            if let Some(src_id) = find_last_source_node(&engine) {
                                                let _ = engine.world.apply_transaction(Transaction::Connect {
                                                    src: PortEndpoint::new(chordworld_core::NodeId(src_id), "out"),
                                                    dst: PortEndpoint::new(new_id, "in"),
                                                    map: ConnectionMap::Direct,
                                                });
                                            }
                                        } else {
                                            // For non-Out nodes, ensure Out exists and connect to it
                                            let out_id = if let Some(id) = find_out_node(&engine) {
                                                Some(id)
                                            } else {
                                                // Auto-create Out node if it doesn't exist
                                                if let Ok(out_result) = engine.world.apply_transaction(Transaction::NodeAdd {
                                                    node_type: "Out".to_string(),
                                                    name: None,
                                                }) {
                                                    if out_result.is_success() {
                                                        find_out_node(&engine)
                                                    } else {
                                                        None
                                                    }
                                                } else {
                                                    None
                                                }
                                            };

                                            // Connect new node to Out
                                            if let Some(out_id) = out_id {
                                                let _ = engine.world.apply_transaction(Transaction::Connect {
                                                    src: PortEndpoint::new(new_id, "out"),
                                                    dst: PortEndpoint::new(chordworld_core::NodeId(out_id), "in"),
                                                    map: ConnectionMap::Direct,
                                                });
                                            }
                                        }
                                    }
                                }
                            }

                            // Recompile snapshot if transaction succeeded
                            if let Err(e) = engine.update_and_compile() {
                                engine_tx
                                    .send(EngineMessage::Error(format!("Compile error: {}", e)))
                                    .ok();
                            }

                            // Send updated graph state
                            engine_tx.send(EngineMessage::GraphState(build_graph_state(&engine))).ok();
                        }
                        Err(e) => {
                            engine_tx
                                .send(EngineMessage::Error(format!("Transaction failed: {}", e)))
                                .ok();
                        }
                    }
                }
                TuiMessage::Quit => {
                    app.should_quit = true;
                }
            }
        }

        app.handle_engine_messages();

        if app.should_quit {
            break;
        }

        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Stop engine
    engine.stop();

    println!("CHORDWORLD shutdown complete.");

    Ok(())
}
