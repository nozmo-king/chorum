//! TUI Test - Run the tracker view without audio

use chordworld_tui::{
    handle_input, render, App, EngineMessage, TuiMessage,
};
use crossbeam_channel::bounded;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    // Create channels (we won't use the engine side much)
    let (tui_tx, _tui_rx) = bounded::<TuiMessage>(32);
    let (_engine_tx, engine_rx) = bounded::<EngineMessage>(32);

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(tui_tx.clone(), engine_rx);

    // Add some test data to the tracker
    {
        use chordworld_core::{Note, Effect};

        if let Some(pattern) = app.tracker_state.current_pattern_mut() {
            // Add some notes to make it interesting
            pattern.tracks[0].cells[0].note = Note::On(60); // C-4
            pattern.tracks[0].cells[0].instrument = Some(0);
            pattern.tracks[0].cells[4].note = Note::On(64); // E-4
            pattern.tracks[0].cells[8].note = Note::On(67); // G-4
            pattern.tracks[0].cells[12].note = Note::Off;

            pattern.tracks[1].cells[0].note = Note::On(48); // C-3
            pattern.tracks[1].cells[0].volume = Some(64);
            pattern.tracks[1].cells[8].note = Note::On(55); // G-3
            pattern.tracks[1].cells[8].fx1 = Effect::new(0x0A, 0x10); // Volume slide

            pattern.tracks[2].cells[2].note = Note::On(72); // C-5
            pattern.tracks[2].cells[6].note = Note::On(74); // D-5
            pattern.tracks[2].cells[10].note = Note::On(76); // E-5
            pattern.tracks[2].cells[14].note = Note::On(79); // G-5
        }
    }

    // Main loop
    loop {
        terminal.draw(|f| render(f, &app))?;

        handle_input(&mut app)?;

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

    println!("TUI Test shutdown complete.");

    Ok(())
}
