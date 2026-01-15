//! CHORDWORLD Main Application

use chordworld_core::DoctrineConfig;
use chordworld_dsp::{register_builtin_nodes, NodeRegistry};
use chordworld_engine::{AudioDeviceConfig, ChordworldEngine};
use chordworld_tui::{handle_input, render, App, EngineMessage, TuiMessage};
use crossbeam_channel::bounded;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

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
                    match engine.world.apply_transaction(transaction) {
                        Ok(result) => {
                            engine_tx.send(EngineMessage::TransactionResult(result)).ok();

                            // Recompile snapshot if transaction succeeded
                            if let Err(e) = engine.update_and_compile() {
                                engine_tx
                                    .send(EngineMessage::Error(format!("Compile error: {}", e)))
                                    .ok();
                            }
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
