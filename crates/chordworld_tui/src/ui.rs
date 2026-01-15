//! UI rendering

use crate::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Top bar
            Constraint::Min(0),     // Main area
            Constraint::Length(3),  // Bottom bar
        ])
        .split(f.area());

    render_top_bar(f, app, chunks[0]);
    render_main_area(f, app, chunks[1]);
    render_bottom_bar(f, app, chunks[2]);
}

fn render_top_bar(f: &mut Frame, app: &App, area: Rect) {
    let title = Span::styled(
        " CHORDWORLD ",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );

    let mode = Span::styled(
        format!(" Mode: {:?} ", app.mode),
        Style::default().fg(Color::Yellow),
    );

    let text = Line::from(vec![title, mode]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(text).block(block);

    f.render_widget(paragraph, area);
}

fn render_main_area(f: &mut Frame, _app: &App, area: Rect) {
    let block = Block::default()
        .title("Main View")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    let welcome_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Welcome to CHORDWORLD",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("A Rust TUI Tracker + Patch-Graph Audio OS"),
        Line::from(""),
        Line::from(Span::styled(
            "Quick Start:",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from("  : - Enter command mode"),
        Line::from("  F1 - Help"),
        Line::from("  Ctrl+Q - Quit"),
        Line::from(""),
        Line::from(Span::styled(
            "Commands:",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from("  node.add <type> [name] - Add a node (OscSine, Gain, Out)"),
        Line::from("  connect <src> <dst> - Connect nodes (e.g., 1:out 2:in)"),
        Line::from("  param.set <node>.<param> <value> - Set parameter"),
        Line::from("  play / stop - Control transport"),
        Line::from("  tempo <bpm> - Set tempo"),
        Line::from(""),
        Line::from(Span::styled(
            "Doctrine Mode: STRICT",
            Style::default().fg(Color::Yellow),
        )),
        Line::from("  Keyboard-only, 16-color palette, deterministic"),
    ];

    let paragraph = Paragraph::new(welcome_text).block(block);

    f.render_widget(paragraph, area);
}

fn render_bottom_bar(f: &mut Frame, app: &App, area: Rect) {
    let style = if app.command_mode {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let text = if app.command_mode {
        Line::from(vec![
            Span::styled(":", Style::default().fg(Color::Yellow)),
            Span::raw(&app.command_input),
        ])
    } else {
        Line::from(app.status_message.as_str())
    };

    let block = Block::default().borders(Borders::ALL).border_style(style);

    let paragraph = Paragraph::new(text).block(block);

    f.render_widget(paragraph, area);
}
