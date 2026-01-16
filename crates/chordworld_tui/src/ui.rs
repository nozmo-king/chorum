//! UI rendering

use crate::app::TuiMode;
use crate::autocomplete::NODE_CATEGORIES;
use crate::palette::Palette;
use crate::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Top bar with mode tabs
            Constraint::Min(0),     // Main area
            Constraint::Length(3),  // Bottom bar / command line
        ])
        .split(f.area());

    render_top_bar_with_tabs(f, app, chunks[0]);
    render_main_workspace(f, app, chunks[1]);
    render_bottom_bar(f, app, chunks[2]);

    // Render completions popup if in command mode with completions
    if app.command_mode && !app.autocompleter.completions.is_empty() {
        render_completions(f, app, chunks[2]);
    }

    // Render node menu overlay if open
    if app.menu_open {
        render_node_menu(f, app);
    }
}

fn render_top_bar_with_tabs(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(14),  // Title
            Constraint::Length(30),  // Mode tabs
            Constraint::Min(0),      // Status info
        ])
        .split(area);

    // Title
    let title = Paragraph::new(" CHURCHKEY ")
        .style(Style::default()
            .fg(Palette::BRIGHT_CYAN)
            .add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Palette::BORDER_NORMAL)));
    f.render_widget(title, chunks[0]);

    // Mode tabs
    let mode_names = vec!["F1:Track", "F2:Patch", "F3:Mix"];
    let selected = match app.mode {
        TuiMode::Tracker => 0,
        TuiMode::Patch => 1,
        TuiMode::Mix => 2,
        _ => 0,
    };
    let tabs = Tabs::new(mode_names)
        .select(selected)
        .style(Style::default().fg(Palette::FG_DIM))
        .highlight_style(Style::default().fg(Palette::BRIGHT_CYAN).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Palette::BORDER_NORMAL)));
    f.render_widget(tabs, chunks[1]);

    // Status info (tuning, entropy, node count)
    let mut status_spans = Vec::new();

    // Tuning
    if let Some(ref name) = app.graph_state.tuning_name {
        status_spans.push(Span::styled(format!(" ♪{} ", name), Style::default().fg(Palette::BRIGHT_MAGENTA)));
    } else {
        status_spans.push(Span::styled(" 12-TET ", Style::default().fg(Palette::FG_DIM)));
    }

    // Entropy
    if app.graph_state.entropy_count > 0 {
        status_spans.push(Span::styled(
            format!("⬡{} ", app.graph_state.entropy_count),
            Style::default().fg(Palette::YELLOW),
        ));
    }

    // Pattern info (if in tracker mode)
    if app.mode == TuiMode::Tracker {
        if let Some(p) = app.tracker_state.current_pattern() {
            status_spans.push(Span::styled(
                format!("Pat:{} ", p.name),
                Style::default().fg(Palette::FG_PRIMARY),
            ));
            status_spans.push(Span::styled(
                format!("Row:{:02X}/{:02X} ", app.tracker_view.cursor.row, p.length),
                Style::default().fg(Palette::FG_DIM),
            ));
        }
    }

    // Node count
    status_spans.push(Span::styled(
        format!("Nodes:{} ", app.graph_state.nodes.len()),
        Style::default().fg(Palette::FG_DIM),
    ));

    // Scope indicator
    if app.scope_visible {
        status_spans.push(Span::styled("[~]", Style::default().fg(Palette::BRIGHT_GREEN)));
    }

    let status = Paragraph::new(Line::from(status_spans))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Palette::BORDER_NORMAL)));
    f.render_widget(status, chunks[2]);
}

fn render_main_workspace(f: &mut Frame, app: &App, area: Rect) {
    // Three-pane layout: Tree | Workspace | Inspector
    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20),  // Tree/Browser
            Constraint::Min(40),     // Main workspace
            Constraint::Length(24),  // Inspector
        ])
        .split(area);

    render_tree_pane(f, app, h_chunks[0]);
    render_workspace_pane(f, app, h_chunks[1]);
    render_inspector_pane(f, app, h_chunks[2]);
}

fn render_tree_pane(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Browser ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Palette::BORDER_NORMAL));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();

    match app.mode {
        TuiMode::Tracker => {
            // Show patterns and songs
            lines.push(Line::from(Span::styled("▼ Patterns", Style::default().fg(Palette::FG_BRIGHT))));
            for (i, p) in app.tracker_state.patterns.iter().enumerate() {
                let marker = if i == app.tracker_state.current_pattern { "▶" } else { " " };
                lines.push(Line::from(Span::styled(
                    format!(" {}{}", marker, p.name),
                    Style::default().fg(if i == app.tracker_state.current_pattern { Palette::BRIGHT_CYAN } else { Palette::FG_PRIMARY }),
                )));
            }
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled("▼ Songs", Style::default().fg(Palette::FG_BRIGHT))));
            for (i, s) in app.tracker_state.songs.iter().enumerate() {
                let marker = if i == app.tracker_state.current_song { "▶" } else { " " };
                lines.push(Line::from(Span::styled(
                    format!(" {}{}", marker, s.name),
                    Style::default().fg(if i == app.tracker_state.current_song { Palette::BRIGHT_CYAN } else { Palette::FG_PRIMARY }),
                )));
            }
        }
        TuiMode::Patch => {
            // Show node categories
            for (cat_name, nodes) in NODE_CATEGORIES.iter() {
                lines.push(Line::from(Span::styled(
                    format!("▼ {}", cat_name),
                    Style::default().fg(Palette::FG_BRIGHT),
                )));
                for node in nodes.iter().take(4) {
                    lines.push(Line::from(Span::styled(
                        format!("   {}", node),
                        Style::default().fg(Palette::FG_DIM),
                    )));
                }
                if nodes.len() > 4 {
                    lines.push(Line::from(Span::styled(
                        format!("   +{} more", nodes.len() - 4),
                        Style::default().fg(Palette::FG_DIM),
                    )));
                }
            }
        }
        TuiMode::Mix => {
            // Show tracks
            lines.push(Line::from(Span::styled("▼ Tracks", Style::default().fg(Palette::FG_BRIGHT))));
            if let Some(p) = app.tracker_state.current_pattern() {
                for (i, t) in p.tracks.iter().enumerate() {
                    let mute = if t.muted { "M" } else { " " };
                    let solo = if t.solo { "S" } else { " " };
                    lines.push(Line::from(Span::styled(
                        format!(" [{}{}] {}", mute, solo, t.name),
                        Style::default().fg(Palette::FG_PRIMARY),
                    )));
                }
            }
        }
        _ => {}
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}

fn render_workspace_pane(f: &mut Frame, app: &App, area: Rect) {
    // Split for scope if visible
    let (content_area, scope_area) = if app.scope_visible {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(8)])
            .split(area);
        (chunks[0], Some(chunks[1]))
    } else {
        (area, None)
    };

    // Main content based on mode
    match app.mode {
        TuiMode::Tracker => {
            render_tracker_view(f, app, content_area);
        }
        TuiMode::Patch => {
            if app.graph_state.nodes.is_empty() {
                render_welcome(f, content_area);
            } else {
                render_graph_view(f, app, content_area);
            }
        }
        TuiMode::Mix => {
            render_mix_view(f, app, content_area);
        }
        _ => {
            render_welcome(f, content_area);
        }
    }

    // Scope
    if let Some(scope_rect) = scope_area {
        render_oscilloscope(f, app, scope_rect);
    }
}

fn render_inspector_pane(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Inspector ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Palette::BORDER_NORMAL));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();

    match app.mode {
        TuiMode::Tracker => {
            // Show current cell info
            lines.push(Line::from(Span::styled("Cell Info", Style::default().fg(Palette::FG_BRIGHT).add_modifier(Modifier::BOLD))));
            lines.push(Line::from(""));

            let cursor = &app.tracker_view.cursor;
            lines.push(Line::from(vec![
                Span::styled("Row: ", Style::default().fg(Palette::FG_DIM)),
                Span::styled(format!("{:02X}", cursor.row), Style::default().fg(Palette::FG_PRIMARY)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("Track: ", Style::default().fg(Palette::FG_DIM)),
                Span::styled(format!("{}", cursor.track + 1), Style::default().fg(Palette::FG_PRIMARY)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("Column: ", Style::default().fg(Palette::FG_DIM)),
                Span::styled(format!("{:?}", cursor.column), Style::default().fg(Palette::FG_PRIMARY)),
            ]));

            if let Some(pattern) = app.tracker_state.current_pattern() {
                if let Some(cell) = pattern.get_cell(cursor.track, cursor.row) {
                    lines.push(Line::from(""));
                    lines.push(Line::from(vec![
                        Span::styled("Note: ", Style::default().fg(Palette::FG_DIM)),
                        Span::styled(cell.note.to_string(), Style::default().fg(Palette::NOTE)),
                    ]));
                    if let Some(inst) = cell.instrument {
                        lines.push(Line::from(vec![
                            Span::styled("Inst: ", Style::default().fg(Palette::FG_DIM)),
                            Span::styled(format!("{:02X}", inst), Style::default().fg(Palette::INSTRUMENT)),
                        ]));
                    }
                    if let Some(vol) = cell.volume {
                        lines.push(Line::from(vec![
                            Span::styled("Vol: ", Style::default().fg(Palette::FG_DIM)),
                            Span::styled(format!("{:02X}", vol), Style::default().fg(Palette::VOLUME)),
                        ]));
                    }
                    if !cell.fx1.is_empty() {
                        lines.push(Line::from(vec![
                            Span::styled("FX1: ", Style::default().fg(Palette::FG_DIM)),
                            Span::styled(cell.fx1.to_string(), Style::default().fg(Palette::EFFECT)),
                        ]));
                    }
                    if !cell.fx2.is_empty() {
                        lines.push(Line::from(vec![
                            Span::styled("FX2: ", Style::default().fg(Palette::FG_DIM)),
                            Span::styled(cell.fx2.to_string(), Style::default().fg(Palette::EFFECT)),
                        ]));
                    }
                }
            }

            // Edit mode indicator
            lines.push(Line::from(""));
            if app.tracker_view.edit_mode {
                lines.push(Line::from(Span::styled("● EDIT MODE", Style::default().fg(Palette::BRIGHT_RED).add_modifier(Modifier::BOLD))));
            } else {
                lines.push(Line::from(Span::styled("○ NAV MODE", Style::default().fg(Palette::FG_DIM))));
            }
        }
        TuiMode::Patch => {
            // Show selected node info
            lines.push(Line::from(Span::styled("Node Info", Style::default().fg(Palette::FG_BRIGHT).add_modifier(Modifier::BOLD))));
            lines.push(Line::from(""));

            if let Some(node_id) = app.selected_node {
                if let Some(node) = app.graph_state.nodes.iter().find(|n| n.id == node_id) {
                    lines.push(Line::from(vec![
                        Span::styled("ID: ", Style::default().fg(Palette::FG_DIM)),
                        Span::styled(format!("{}", node.id), Style::default().fg(Palette::FG_PRIMARY)),
                    ]));
                    lines.push(Line::from(vec![
                        Span::styled("Type: ", Style::default().fg(Palette::FG_DIM)),
                        Span::styled(&node.node_type, Style::default().fg(Palette::BRIGHT_CYAN)),
                    ]));
                    lines.push(Line::from(vec![
                        Span::styled("Name: ", Style::default().fg(Palette::FG_DIM)),
                        Span::styled(&node.name, Style::default().fg(Palette::FG_PRIMARY)),
                    ]));
                    if node.bypassed {
                        lines.push(Line::from(Span::styled("BYPASSED", Style::default().fg(Palette::BRIGHT_RED))));
                    }
                }
            } else {
                lines.push(Line::from(Span::styled("No selection", Style::default().fg(Palette::FG_DIM))));
            }

            // Connection summary
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled("Connections", Style::default().fg(Palette::FG_BRIGHT))));
            lines.push(Line::from(Span::styled(
                format!("{} total", app.graph_state.connections.len()),
                Style::default().fg(Palette::FG_DIM),
            )));
        }
        TuiMode::Mix => {
            lines.push(Line::from(Span::styled("Mix Info", Style::default().fg(Palette::FG_BRIGHT).add_modifier(Modifier::BOLD))));
            lines.push(Line::from(""));
            if let Some(p) = app.tracker_state.current_pattern() {
                lines.push(Line::from(vec![
                    Span::styled("Tracks: ", Style::default().fg(Palette::FG_DIM)),
                    Span::styled(format!("{}", p.tracks.len()), Style::default().fg(Palette::FG_PRIMARY)),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("BPM: ", Style::default().fg(Palette::FG_DIM)),
                    Span::styled("120", Style::default().fg(Palette::FG_PRIMARY)),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("LPB: ", Style::default().fg(Palette::FG_DIM)),
                    Span::styled(format!("{}", p.lpb), Style::default().fg(Palette::FG_PRIMARY)),
                ]));
            }
        }
        _ => {}
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}

fn render_top_bar(f: &mut Frame, app: &App, area: Rect) {
    let title = Span::styled(
        " CHORDWORLD ",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );

    // Tuning display
    let tuning = if let Some(ref name) = app.graph_state.tuning_name {
        Span::styled(
            format!(" ♪{} ", name),
            Style::default().fg(Color::Magenta),
        )
    } else {
        Span::styled(" 12-TET ", Style::default().fg(Color::DarkGray))
    };

    // Entropy pool count
    let entropy = if app.graph_state.entropy_count > 0 {
        Span::styled(
            format!(" ⬡{} ", app.graph_state.entropy_count),
            Style::default().fg(Color::Yellow),
        )
    } else {
        Span::raw("")
    };

    let scope_indicator = Span::styled(
        if app.scope_visible { " [~] " } else { "" },
        Style::default().fg(Color::Green),
    );

    let node_count = Span::styled(
        format!(" {}n ", app.graph_state.nodes.len()),
        Style::default().fg(Color::DarkGray),
    );

    let text = Line::from(vec![title, tuning, entropy, scope_indicator, node_count]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(text).block(block);

    f.render_widget(paragraph, area);
}

fn render_main_area(f: &mut Frame, app: &App, area: Rect) {
    // Split main area for scope if visible
    let (content_area, scope_area) = if app.scope_visible {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(10)])
            .split(area);
        (chunks[0], Some(chunks[1]))
    } else {
        (area, None)
    };

    // Render main content based on mode
    match app.mode {
        TuiMode::Tracker => {
            render_tracker_view(f, app, content_area);
        }
        TuiMode::Patch => {
            if app.graph_state.nodes.is_empty() {
                render_welcome(f, content_area);
            } else {
                render_graph_view(f, app, content_area);
            }
        }
        TuiMode::Mix => {
            render_mix_view(f, app, content_area);
        }
        _ => {
            render_welcome(f, content_area);
        }
    }

    // Render oscilloscope if visible
    if let Some(scope_rect) = scope_area {
        render_oscilloscope(f, app, scope_rect);
    }
}

fn render_tracker_view(f: &mut Frame, app: &App, area: Rect) {
    let pattern = app.tracker_state.current_pattern();
    app.tracker_view.render(f, area, pattern);
}

fn render_mix_view(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Mix ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Palette::BORDER_FOCUSED));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Show tracks with volume levels
    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));

    if let Some(pattern) = app.tracker_state.current_pattern() {
        for (i, track) in pattern.tracks.iter().enumerate() {
            let mute_indicator = if track.muted { "[M]" } else { "   " };
            let solo_indicator = if track.solo { "[S]" } else { "   " };

            // Simple volume bar
            let vol_bar = "████████████████";

            lines.push(Line::from(vec![
                Span::styled(
                    format!(" {:02} ", i),
                    Style::default().fg(Palette::FG_DIM),
                ),
                Span::styled(
                    format!("{:12}", track.name),
                    Style::default().fg(Palette::FG_BRIGHT),
                ),
                Span::styled(mute_indicator, Style::default().fg(Palette::NOTE_OFF)),
                Span::styled(solo_indicator, Style::default().fg(Palette::INSTRUMENT)),
                Span::raw(" "),
                Span::styled(vol_bar, Style::default().fg(Palette::VOLUME)),
            ]));
        }
    } else {
        lines.push(Line::from(Span::styled(
            "  No pattern loaded",
            Style::default().fg(Palette::FG_DIM),
        )));
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}

fn render_welcome(f: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("CHORDWORLD - Patch-Graph Audio")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let text = vec![
        Line::from(""),
        Line::from(Span::styled("  Quick Start - type : then command", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(vec![
            Span::styled("    :setup.basic", Style::default().fg(Color::Cyan)),
            Span::raw("  → Sine oscillator (hear sound immediately)"),
        ]),
        Line::from(vec![
            Span::styled("    :setup.fm", Style::default().fg(Color::Cyan)),
            Span::raw("     → FM synthesis"),
        ]),
        Line::from(vec![
            Span::styled("    :setup.pad", Style::default().fg(Color::Cyan)),
            Span::raw("    → Ambient pad (sine+saw+reverb)"),
        ]),
        Line::from(""),
        Line::from(Span::styled("  21e8 Microtuning", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))),
        Line::from(vec![
            Span::styled("    :pow.mine <text>", Style::default().fg(Color::Magenta)),
            Span::raw("  → Mine aesthetic hash"),
        ]),
        Line::from(vec![
            Span::styled("    :tuning.random", Style::default().fg(Color::Magenta)),
            Span::raw("    → Random microtonal scale"),
        ]),
        Line::from(""),
        Line::from(Span::styled("  Controls", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("    Ctrl+N   Node browser"),
        Line::from("    Ctrl+S   Toggle scope"),
        Line::from("    Tab      Autocomplete"),
        Line::from("    Ctrl+Q   Quit"),
    ];

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

fn render_graph_view(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(format!("Patch Graph [{} nodes, {} connections]",
            app.graph_state.nodes.len(),
            app.graph_state.connections.len()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Build node display lines
    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));

    // Group nodes by type category
    let mut oscillators: Vec<&crate::app::NodeInfo> = Vec::new();
    let mut filters: Vec<&crate::app::NodeInfo> = Vec::new();
    let mut effects: Vec<&crate::app::NodeInfo> = Vec::new();
    let mut utilities: Vec<&crate::app::NodeInfo> = Vec::new();
    let mut other: Vec<&crate::app::NodeInfo> = Vec::new();

    for node in &app.graph_state.nodes {
        if node.node_type.starts_with("Osc") {
            oscillators.push(node);
        } else if node.node_type.starts_with("Filter") {
            filters.push(node);
        } else if node.node_type.starts_with("Fx") {
            effects.push(node);
        } else if node.node_type.starts_with("Util") || node.node_type == "Out" || node.node_type == "Gain" {
            utilities.push(node);
        } else {
            other.push(node);
        }
    }

    // Render each category
    let categories = [
        ("Oscillators", &oscillators, Color::Cyan),
        ("Filters", &filters, Color::Yellow),
        ("Effects", &effects, Color::Magenta),
        ("Utilities", &utilities, Color::Green),
        ("Other", &other, Color::White),
    ];

    for (name, nodes, color) in categories {
        if !nodes.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("  {} ─────", name),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            )));

            for node in nodes.iter() {
                let bypass_marker = if node.bypassed { " [BYPASS]" } else { "" };

                // Find connections for this node
                let incoming: Vec<_> = app.graph_state.connections.iter()
                    .filter(|c| c.dst_node == node.id)
                    .collect();
                let outgoing: Vec<_> = app.graph_state.connections.iter()
                    .filter(|c| c.src_node == node.id)
                    .collect();

                let conn_info = if incoming.is_empty() && outgoing.is_empty() {
                    " (disconnected)".to_string()
                } else {
                    let mut parts = Vec::new();
                    for c in &incoming {
                        parts.push(format!("←{}", c.src_node));
                    }
                    for c in &outgoing {
                        parts.push(format!("→{}", c.dst_node));
                    }
                    format!(" [{}]", parts.join(" "))
                };

                lines.push(Line::from(vec![
                    Span::styled(
                        format!("    {} ", node.id),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::styled(
                        &node.node_type,
                        Style::default().fg(color),
                    ),
                    Span::styled(
                        format!(" \"{}\"", node.name),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::styled(conn_info, Style::default().fg(Color::DarkGray)),
                    Span::styled(bypass_marker, Style::default().fg(Color::Red)),
                ]));
            }
            lines.push(Line::from(""));
        }
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}

fn render_bottom_bar(f: &mut Frame, app: &App, area: Rect) {
    let style = if app.command_mode {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let cursor_text = if app.command_mode && app.autocompleter.active {
        format!(" [{}]", app.autocompleter.index + 1)
    } else {
        String::new()
    };

    let text = if app.command_mode {
        Line::from(vec![
            Span::styled(":", Style::default().fg(Color::Yellow)),
            Span::raw(&app.command_input),
            Span::styled(cursor_text, Style::default().fg(Color::DarkGray)),
        ])
    } else {
        Line::from(app.status_message.as_str())
    };

    let block = Block::default().borders(Borders::ALL).border_style(style);

    let paragraph = Paragraph::new(text).block(block);

    f.render_widget(paragraph, area);
}

fn render_completions(f: &mut Frame, app: &App, bottom_bar_area: Rect) {
    let completions = &app.autocompleter.completions;
    if completions.is_empty() {
        return;
    }

    // Show up to 6 completions
    let visible_count = completions.len().min(6);
    let height = visible_count as u16 + 2; // +2 for borders

    // Position popup above the command line
    let popup_area = Rect {
        x: bottom_bar_area.x + 1,
        y: bottom_bar_area.y.saturating_sub(height),
        width: 30.min(bottom_bar_area.width - 2),
        height,
    };

    // Clear the area first
    f.render_widget(Clear, popup_area);

    let items: Vec<ListItem> = completions
        .iter()
        .take(visible_count)
        .enumerate()
        .map(|(i, c)| {
            let style = if app.autocompleter.active && i == app.autocompleter.index {
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            ListItem::new(c.as_str()).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray))
                .title("Completions"),
        );

    f.render_widget(list, popup_area);
}

fn render_node_menu(f: &mut Frame, app: &App) {
    let area = f.area();

    // Center the menu
    let menu_width = 60.min(area.width.saturating_sub(4));
    let menu_height = 20.min(area.height.saturating_sub(4));
    let menu_x = (area.width.saturating_sub(menu_width)) / 2;
    let menu_y = (area.height.saturating_sub(menu_height)) / 2;

    let menu_area = Rect {
        x: menu_x,
        y: menu_y,
        width: menu_width,
        height: menu_height,
    };

    // Clear the area
    f.render_widget(Clear, menu_area);

    // Split into tabs and content
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(menu_area);

    // Category tabs
    let category_names: Vec<&str> = NODE_CATEGORIES.iter().map(|(name, _)| *name).collect();
    let tabs = Tabs::new(category_names.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Node Browser [←/→ category, ↑/↓ select, Enter create, Esc close]"),
        )
        .select(app.menu_category)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, chunks[0]);

    // Node list for selected category
    if let Some((_, items)) = NODE_CATEGORIES.get(app.menu_category) {
        let list_items: Vec<ListItem> = items
            .iter()
            .enumerate()
            .map(|(i, &node_type)| {
                let style = if i == app.menu_item {
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(format!("  {}", node_type)).style(style)
            })
            .collect();

        let list = List::new(list_items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White)),
        );

        f.render_widget(list, chunks[1]);
    }
}

fn render_oscilloscope(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("Oscilloscope [Ctrl+S toggle]")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Generate demo waveform if no audio data
    let buffer: Vec<f32> = if app.audio_data.buffers.is_empty() {
        // Generate a nice demo sine wave
        let samples = (inner.width as usize * 2).max(128);
        (0..samples)
            .map(|i| {
                let t = i as f32 / samples as f32;
                (t * std::f32::consts::TAU * 3.0).sin() * 0.7
            })
            .collect()
    } else {
        app.audio_data.buffers[0].clone()
    };

    if buffer.is_empty() {
        return;
    }

    // Render waveform using braille characters
    let waveform = render_waveform_braille(&buffer, inner.width as usize, inner.height as usize);
    let text: Vec<Line> = waveform
        .into_iter()
        .map(|s| Line::from(Span::styled(s, Style::default().fg(Color::Green))))
        .collect();

    let paragraph = Paragraph::new(text);
    f.render_widget(paragraph, inner);
}

/// Render waveform using braille characters
/// Each braille character is a 2x4 grid of dots
fn render_waveform_braille(samples: &[f32], width: usize, height: usize) -> Vec<String> {
    if samples.is_empty() || width == 0 || height == 0 {
        return vec![];
    }

    // Braille characters: each char is 2 columns × 4 rows of dots
    // Unicode braille starts at U+2800
    // Dot positions (bit index):
    //   0 3
    //   1 4
    //   2 5
    //   6 7

    let char_cols = width;
    let char_rows = height;
    let pixel_cols = char_cols * 2;
    let pixel_rows = char_rows * 4;

    // Create pixel grid
    let mut pixels = vec![vec![false; pixel_cols]; pixel_rows];

    // Sample the waveform across the width
    let samples_per_col = samples.len() as f32 / pixel_cols as f32;

    for col in 0..pixel_cols {
        let sample_start = (col as f32 * samples_per_col) as usize;
        let sample_end = ((col + 1) as f32 * samples_per_col) as usize;
        let sample_end = sample_end.min(samples.len());

        if sample_start >= samples.len() {
            continue;
        }

        // Find min and max in this column's samples
        let mut min_val = samples[sample_start];
        let mut max_val = samples[sample_start];
        for &s in &samples[sample_start..sample_end] {
            min_val = min_val.min(s);
            max_val = max_val.max(s);
        }

        // Map to pixel rows (center is 0, top is +1, bottom is -1)
        let center_row = pixel_rows / 2;
        let half_height = pixel_rows as f32 / 2.0;

        let min_row = (center_row as f32 - min_val * half_height * 0.9) as usize;
        let max_row = (center_row as f32 - max_val * half_height * 0.9) as usize;

        let min_row = min_row.min(pixel_rows - 1);
        let max_row = max_row.min(pixel_rows - 1);

        // Fill pixels between min and max
        let start_row = max_row.min(min_row);
        let end_row = max_row.max(min_row);
        for row in start_row..=end_row {
            pixels[row][col] = true;
        }
    }

    // Convert pixels to braille characters
    let mut result = Vec::with_capacity(char_rows);

    for char_row in 0..char_rows {
        let mut line = String::with_capacity(char_cols);

        for char_col in 0..char_cols {
            let px = char_col * 2;
            let py = char_row * 4;

            // Build braille character from 2x4 pixel block
            let mut braille: u8 = 0;

            // Check bounds and set bits
            if py < pixel_rows && px < pixel_cols {
                if pixels[py][px] { braille |= 0x01; }
                if px + 1 < pixel_cols && pixels[py][px + 1] { braille |= 0x08; }
            }
            if py + 1 < pixel_rows && px < pixel_cols {
                if pixels[py + 1][px] { braille |= 0x02; }
                if px + 1 < pixel_cols && pixels[py + 1][px + 1] { braille |= 0x10; }
            }
            if py + 2 < pixel_rows && px < pixel_cols {
                if pixels[py + 2][px] { braille |= 0x04; }
                if px + 1 < pixel_cols && pixels[py + 2][px + 1] { braille |= 0x20; }
            }
            if py + 3 < pixel_rows && px < pixel_cols {
                if pixels[py + 3][px] { braille |= 0x40; }
                if px + 1 < pixel_cols && pixels[py + 3][px + 1] { braille |= 0x80; }
            }

            // Convert to braille unicode character
            let braille_char = char::from_u32(0x2800 + braille as u32).unwrap_or(' ');
            line.push(braille_char);
        }

        result.push(line);
    }

    result
}
