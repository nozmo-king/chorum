//! Input handling

use crate::app::TuiMode;
use crate::App;
use chordworld_core::Note;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use anyhow::Result;

pub fn handle_input(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            handle_key_event(app, key)?;
        }
    }
    Ok(())
}

fn handle_key_event(app: &mut App, key: KeyEvent) -> Result<()> {
    // Menu mode takes priority
    if app.menu_open {
        return handle_menu_input(app, key);
    }

    if app.command_mode {
        handle_command_mode_input(app, key)
    } else {
        match app.mode {
            TuiMode::Tracker => handle_tracker_mode_input(app, key),
            _ => handle_normal_mode_input(app, key),
        }
    }
}

fn handle_command_mode_input(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Enter => {
            app.execute_command();
        }
        KeyCode::Esc => {
            app.exit_command_mode();
        }
        KeyCode::Tab => {
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                app.cycle_completion_prev();
            } else {
                app.cycle_completion();
            }
        }
        KeyCode::Char(c) => {
            app.command_input.push(c);
            app.on_input_changed();
        }
        KeyCode::Backspace => {
            app.command_input.pop();
            app.on_input_changed();
        }
        _ => {}
    }
    Ok(())
}

fn handle_normal_mode_input(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.quit();
        }
        KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.open_node_menu();
        }
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.toggle_scope();
        }
        KeyCode::Char(':') => {
            app.enter_command_mode();
        }
        KeyCode::Char(' ') => {
            // Toggle play/stop (simplified)
            // In full version, check transport state
        }
        KeyCode::F(1) => {
            app.status_message = "F1: Help | : command | Tab: autocomplete | Ctrl+N: nodes | Ctrl+S: scope | Ctrl+Q: quit".to_string();
        }
        _ => {}
    }
    Ok(())
}

fn handle_menu_input(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.close_node_menu();
        }
        KeyCode::Enter => {
            app.menu_select();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.menu_up();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.menu_down();
        }
        KeyCode::Left | KeyCode::Char('h') => {
            app.menu_left();
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.menu_right();
        }
        KeyCode::Tab => {
            // Tab cycles through categories
            app.menu_right();
        }
        KeyCode::BackTab => {
            app.menu_left();
        }
        _ => {}
    }
    Ok(())
}

/// Tracker mode: keyboard-driven note entry and navigation
fn handle_tracker_mode_input(app: &mut App, key: KeyEvent) -> Result<()> {
    // Global shortcuts work in all modes
    match key.code {
        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.quit();
            return Ok(());
        }
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.toggle_scope();
            return Ok(());
        }
        KeyCode::Char(':') => {
            app.enter_command_mode();
            return Ok(());
        }
        KeyCode::F(1) => {
            app.set_mode(TuiMode::Tracker);
            return Ok(());
        }
        KeyCode::F(2) => {
            app.set_mode(TuiMode::Patch);
            return Ok(());
        }
        KeyCode::F(3) => {
            app.set_mode(TuiMode::Mix);
            return Ok(());
        }
        _ => {}
    }

    // Get pattern info for navigation bounds
    let pattern_len = app.tracker_state.current_pattern()
        .map(|p| p.length)
        .unwrap_or(64);
    let num_tracks = app.tracker_state.current_pattern()
        .map(|p| p.tracks.len())
        .unwrap_or(4);

    if app.tracker_view.edit_mode {
        // Edit mode: direct note entry
        handle_tracker_edit_input(app, key, pattern_len, num_tracks)
    } else {
        // Navigation mode
        handle_tracker_nav_input(app, key, pattern_len, num_tracks)
    }
}

fn handle_tracker_nav_input(app: &mut App, key: KeyEvent, pattern_len: usize, num_tracks: usize) -> Result<()> {
    match key.code {
        // Navigation
        KeyCode::Up | KeyCode::Char('k') => {
            app.tracker_view.move_up(pattern_len);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.tracker_view.move_down(pattern_len);
        }
        KeyCode::Left | KeyCode::Char('h') => {
            app.tracker_view.move_left(num_tracks);
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.tracker_view.move_right(num_tracks);
        }
        KeyCode::PageUp => {
            app.tracker_view.page_up(pattern_len, 16);
        }
        KeyCode::PageDown => {
            app.tracker_view.page_down(pattern_len, 16);
        }
        KeyCode::Home => {
            app.tracker_view.cursor.row = 0;
        }
        KeyCode::End => {
            app.tracker_view.cursor.row = pattern_len.saturating_sub(1);
        }
        KeyCode::Tab => {
            // Tab cycles columns within a track
            app.tracker_view.cursor.column = app.tracker_view.cursor.column.next();
        }
        KeyCode::BackTab => {
            app.tracker_view.cursor.column = app.tracker_view.cursor.column.prev();
        }

        // Enter edit mode
        KeyCode::Enter | KeyCode::Char(' ') => {
            app.tracker_view.edit_mode = true;
            app.status_message = "EDIT | Type note (C-4, D#3, OFF) | Esc: cancel | Enter: confirm".to_string();
        }

        // Quick note entry without explicit edit mode
        KeyCode::Char(c) if is_note_char(c) => {
            // Start note entry
            app.tracker_view.edit_mode = true;
            app.status_message = format!("EDIT | {}_ | Esc: cancel", c.to_uppercase());
        }

        // Delete current cell
        KeyCode::Delete | KeyCode::Backspace => {
            delete_current_cell(app);
        }

        // Toggle play
        KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.tracker_view.playing = !app.tracker_view.playing;
            app.status_message = if app.tracker_view.playing {
                "▶ Playing".to_string()
            } else {
                "■ Stopped".to_string()
            };
        }

        _ => {}
    }
    Ok(())
}

fn handle_tracker_edit_input(app: &mut App, key: KeyEvent, _pattern_len: usize, _num_tracks: usize) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.tracker_view.edit_mode = false;
            app.status_message = "TRACKER | ↑↓←→ navigate | Space: edit | Tab: column".to_string();
        }
        KeyCode::Enter => {
            app.tracker_view.edit_mode = false;
            app.status_message = "TRACKER | ↑↓←→ navigate | Space: edit | Tab: column".to_string();
        }

        // Note entry: QWERTY keyboard as piano layout
        // Lower row: Z=C, S=C#, X=D, D=D#, C=E, V=F, G=F#, B=G, H=G#, N=A, J=A#, M=B
        // Upper row: Q=C+1, 2=C#+1, W=D+1, etc.
        KeyCode::Char(c) => {
            if let Some(note) = key_to_note(c, 4) { // Base octave 4
                enter_note(app, note);
                app.tracker_view.edit_mode = false;
                app.status_message = format!("Entered: {}", note.to_string());
            }
        }
        _ => {}
    }
    Ok(())
}

fn is_note_char(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a'..='g' | 'z' | 'x' | 'c' | 'v' | 'b' | 'n' | 'm' | 'q' | 'w' | 'e' | 'r' | 't' | 'y' | 'u' | 'i' | 'o' | 'p')
}

/// Map keyboard key to MIDI note (piano keyboard layout)
fn key_to_note(c: char, base_octave: u8) -> Option<Note> {
    let c = c.to_ascii_lowercase();

    // Lower row: C to B in base octave
    let (semitone, octave_offset) = match c {
        'z' => (0, 0),   // C
        's' => (1, 0),   // C#
        'x' => (2, 0),   // D
        'd' => (3, 0),   // D#
        'c' => (4, 0),   // E
        'v' => (5, 0),   // F
        'g' => (6, 0),   // F#
        'b' => (7, 0),   // G
        'h' => (8, 0),   // G#
        'n' => (9, 0),   // A
        'j' => (10, 0),  // A#
        'm' => (11, 0),  // B

        // Upper row: C to B in base+1 octave
        'q' => (0, 1),   // C
        '2' => (1, 1),   // C#
        'w' => (2, 1),   // D
        '3' => (3, 1),   // D#
        'e' => (4, 1),   // E
        'r' => (5, 1),   // F
        '5' => (6, 1),   // F#
        't' => (7, 1),   // G
        '6' => (8, 1),   // G#
        'y' => (9, 1),   // A
        '7' => (10, 1),  // A#
        'u' => (11, 1),  // B

        // Even higher
        'i' => (0, 2),   // C+2
        '9' => (1, 2),   // C#+2
        'o' => (2, 2),   // D+2
        '0' => (3, 2),   // D#+2
        'p' => (4, 2),   // E+2

        // Special
        '1' => return Some(Note::Off), // Note off
        '`' => return Some(Note::Empty), // Clear

        _ => return None,
    };

    let midi = (base_octave as i32 + octave_offset + 1) * 12 + semitone;
    if midi >= 0 && midi <= 127 {
        Some(Note::On(midi as u8))
    } else {
        None
    }
}

fn enter_note(app: &mut App, note: Note) {
    let row = app.tracker_view.cursor.row;
    let track = app.tracker_view.cursor.track;

    if let Some(pattern) = app.tracker_state.current_pattern_mut() {
        if let Some(cell) = pattern.tracks.get_mut(track).and_then(|t| t.cells.get_mut(row)) {
            cell.note = note;
        }
    }
}

fn delete_current_cell(app: &mut App) {
    let row = app.tracker_view.cursor.row;
    let track = app.tracker_view.cursor.track;

    if let Some(pattern) = app.tracker_state.current_pattern_mut() {
        if let Some(cell) = pattern.tracks.get_mut(track).and_then(|t| t.cells.get_mut(row)) {
            use crate::tracker_view::TrackerColumn;
            match app.tracker_view.cursor.column {
                TrackerColumn::Note => cell.note = Note::Empty,
                TrackerColumn::Instrument => cell.instrument = None,
                TrackerColumn::Volume => cell.volume = None,
                TrackerColumn::Fx1 => cell.fx1 = chordworld_core::Effect::none(),
                TrackerColumn::Fx2 => cell.fx2 = chordworld_core::Effect::none(),
            }
        }
    }
}
