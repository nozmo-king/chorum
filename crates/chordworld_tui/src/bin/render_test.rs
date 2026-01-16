//! Render test - output full TUI to stdout (non-interactive)

use chordworld_core::{Effect, Note};
use chordworld_tui::{App, EngineMessage, TuiMessage, render, NodeInfo, ConnectionInfo, GraphState};
use crossbeam_channel::bounded;
use ratatui::{
    backend::TestBackend,
    Terminal,
};

fn main() {
    // Create channels (mock)
    let (tui_tx, _tui_rx) = bounded::<TuiMessage>(32);
    let (_engine_tx, engine_rx) = bounded::<EngineMessage>(32);

    // Create app
    let mut app = App::new(tui_tx, engine_rx);

    // Add test notes to tracker
    if let Some(pattern) = app.tracker_state.current_pattern_mut() {
        // Track 1: Chord progression
        pattern.tracks[0].cells[0].note = Note::On(60); // C-4
        pattern.tracks[0].cells[0].instrument = Some(0);
        pattern.tracks[0].cells[0].volume = Some(100);
        pattern.tracks[0].cells[4].note = Note::On(64); // E-4
        pattern.tracks[0].cells[8].note = Note::On(67); // G-4
        pattern.tracks[0].cells[12].note = Note::Off;

        // Track 2: Bass line
        pattern.tracks[1].cells[0].note = Note::On(36); // C-2
        pattern.tracks[1].cells[0].volume = Some(80);
        pattern.tracks[1].cells[4].note = Note::On(43); // G-2
        pattern.tracks[1].cells[8].note = Note::On(36); // C-2
        pattern.tracks[1].cells[8].fx1 = Effect::new(0x0A, 0x10); // Volume slide

        // Track 3: High melody
        pattern.tracks[2].cells[2].note = Note::On(72); // C-5
        pattern.tracks[2].cells[6].note = Note::On(74); // D-5
        pattern.tracks[2].cells[10].note = Note::On(76); // E-5
        pattern.tracks[2].cells[14].note = Note::On(79); // G-5
        pattern.tracks[2].cells[14].fx1 = Effect::new(0x03, 0x20); // Tone portamento

        // Track 4: Percussion
        pattern.tracks[3].cells[0].note = Note::On(36);
        pattern.tracks[3].cells[4].note = Note::On(38);
        pattern.tracks[3].cells[8].note = Note::On(36);
        pattern.tracks[3].cells[12].note = Note::On(38);
        pattern.tracks[3].cells[14].note = Note::On(42);
    }

    // Add mock nodes to graph state
    app.graph_state = GraphState {
        nodes: vec![
            NodeInfo { id: 0, name: "Sine".to_string(), node_type: "OscSine".to_string(), bypassed: false },
            NodeInfo { id: 1, name: "Filter".to_string(), node_type: "FilterSVF".to_string(), bypassed: false },
            NodeInfo { id: 2, name: "Reverb".to_string(), node_type: "FxReverb".to_string(), bypassed: false },
            NodeInfo { id: 3, name: "Master".to_string(), node_type: "Out".to_string(), bypassed: false },
        ],
        connections: vec![
            ConnectionInfo { id: 0, src_node: 0, src_port: "out".to_string(), dst_node: 1, dst_port: "in".to_string() },
            ConnectionInfo { id: 1, src_node: 1, src_port: "out".to_string(), dst_node: 2, dst_port: "in".to_string() },
            ConnectionInfo { id: 2, src_node: 2, src_port: "out".to_string(), dst_node: 3, dst_port: "in".to_string() },
        ],
        tuning_name: Some("21e8-Heptatonic".to_string()),
        entropy_count: 3,
    };

    // Set cursor position
    app.tracker_view.cursor.row = 4;
    app.tracker_view.cursor.track = 1;

    // Test with larger terminal
    let backend = TestBackend::new(120, 35);
    let mut terminal = Terminal::new(backend).unwrap();

    // Render Tracker mode
    println!("\n╔════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                                            CHURCHKEY - TRACKER MODE (F1)                                           ║");
    println!("╠════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╣");

    terminal.draw(|f| render(f, &app)).unwrap();
    print_buffer(&terminal);

    // Switch to Patch mode and render
    app.set_mode(chordworld_tui::TuiMode::Patch);
    app.selected_node = Some(1); // Select the filter

    println!("\n╔════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                                             CHURCHKEY - PATCH MODE (F2)                                            ║");
    println!("╠════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╣");

    terminal.draw(|f| render(f, &app)).unwrap();
    print_buffer(&terminal);

    // Switch to Mix mode and render
    app.set_mode(chordworld_tui::TuiMode::Mix);

    println!("\n╔════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                                              CHURCHKEY - MIX MODE (F3)                                             ║");
    println!("╠════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╣");

    terminal.draw(|f| render(f, &app)).unwrap();
    print_buffer(&terminal);

    println!("╚════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╝");
    println!();
    println!("┌─ CONTROLS ─────────────────────────────────────────────────────────────────────┐");
    println!("│ F1/F2/F3      Switch modes (Tracker/Patch/Mix)                                 │");
    println!("│ Arrow/HJKL    Navigate                                                         │");
    println!("│ Tab           Cycle columns (Tracker) / Next node (Patch)                      │");
    println!("│ Space/Enter   Edit mode (Tracker) / Select (Patch)                             │");
    println!("│ ZXCVBNM       Notes C-D-E-F-G-A-B (octave 4)                                   │");
    println!("│ QWERTY        Notes C-D-E-F-G-A-B (octave 5)                                   │");
    println!("│ :             Command mode                                                     │");
    println!("│ Ctrl+N        Node browser                                                     │");
    println!("│ Ctrl+S        Toggle oscilloscope                                              │");
    println!("│ Ctrl+Q        Quit                                                             │");
    println!("└────────────────────────────────────────────────────────────────────────────────┘");
    println!();
    println!("┌─ COMMANDS ─────────────────────────────────────────────────────────────────────┐");
    println!("│ :setup.basic      Create basic synth (OscSine → Out)                           │");
    println!("│ :setup.fm         Create FM synth                                              │");
    println!("│ :setup.pad        Create ambient pad                                           │");
    println!("│ :node.add <type>  Add node (e.g., :node.add OscSaw)                            │");
    println!("│ :connect <a> <b>  Connect nodes (e.g., :connect 0:out 1:in)                    │");
    println!("│ :pow.mine <seed>  Mine aesthetic hash for microtuning                          │");
    println!("│ :tuning.random    Generate random microtonal scale                             │");
    println!("│ :path <name>      Apply pathology (e.g., :path saboteur entropy=0.9)          │");
    println!("│ :path.list        List available pathologies                                   │");
    println!("└────────────────────────────────────────────────────────────────────────────────┘");
}

fn print_buffer(terminal: &Terminal<TestBackend>) {
    let buffer = terminal.backend().buffer();
    for y in 0..buffer.area.height {
        print!("║");
        for x in 0..buffer.area.width {
            let cell = buffer.cell((x, y)).unwrap();
            print!("{}", cell.symbol());
        }
        println!("║");
    }
}
