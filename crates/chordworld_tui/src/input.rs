//! Input handling

use crate::App;
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
    if app.command_mode {
        handle_command_mode_input(app, key)
    } else {
        handle_normal_mode_input(app, key)
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
        KeyCode::Char(c) => {
            app.command_input.push(c);
        }
        KeyCode::Backspace => {
            app.command_input.pop();
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
        KeyCode::Char(':') => {
            app.enter_command_mode();
        }
        KeyCode::Char(' ') => {
            // Toggle play/stop (simplified)
            // In full version, check transport state
        }
        KeyCode::F(1) => {
            app.status_message = "F1: Help - Use : for commands. Ctrl+Q to quit.".to_string();
        }
        _ => {}
    }
    Ok(())
}
