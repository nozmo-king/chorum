//! Application state for TUI

use crate::autocomplete::{Autocompleter, NODE_CATEGORIES};
use crate::command::CommandParser;
use crate::tracker_view::TrackerView;
use chordworld_core::{TrackerState, Transaction, TransactionResult};
use crossbeam_channel::{Receiver, Sender};

/// TUI mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiMode {
    Tracker,
    Patch,
    Mix,
    Inspect,
    Log,
}

/// Message from TUI to engine/world
pub enum TuiMessage {
    Transaction(Transaction),
    Quit,
}

/// Message from engine/world to TUI
pub enum EngineMessage {
    TransactionResult(TransactionResult),
    AudioData(AudioScopeData),
    GraphState(GraphState),
    Error(String),
}

/// Graph state for display
#[derive(Clone, Default)]
pub struct GraphState {
    pub nodes: Vec<NodeInfo>,
    pub connections: Vec<ConnectionInfo>,
    pub tuning_name: Option<String>,
    pub entropy_count: usize,
}

/// Node info for display
#[derive(Clone)]
pub struct NodeInfo {
    pub id: u64,
    pub name: String,
    pub node_type: String,
    pub bypassed: bool,
}

/// Connection info for display
#[derive(Clone)]
pub struct ConnectionInfo {
    pub id: u64,
    pub src_node: u64,
    pub src_port: String,
    pub dst_node: u64,
    pub dst_port: String,
}

/// Audio scope data for visualization
#[derive(Clone)]
pub struct AudioScopeData {
    pub buffers: Vec<Vec<f32>>,
    pub peak_levels: Vec<f32>,
}

impl Default for AudioScopeData {
    fn default() -> Self {
        Self {
            buffers: Vec::new(),
            peak_levels: Vec::new(),
        }
    }
}

/// Application state
pub struct App {
    pub mode: TuiMode,
    pub command_input: String,
    pub command_mode: bool,
    pub status_message: String,
    pub should_quit: bool,

    // Auto-complete
    pub autocompleter: Autocompleter,
    pub completion_base: String, // Original input before cycling

    // Node menu
    pub menu_open: bool,
    pub menu_category: usize,
    pub menu_item: usize,

    // Graph state for display
    pub graph_state: GraphState,
    pub selected_node: Option<u64>,
    pub auto_connect: bool, // Auto-connect new nodes

    // Audio visualization
    pub audio_data: AudioScopeData,
    pub scope_visible: bool,

    // 21e8 entropy display
    pub entropy_display: Option<(String, u32)>, // (hash_hex, aesthetic_score)

    // Tracker state
    pub tracker_state: TrackerState,
    pub tracker_view: TrackerView,

    // Communication
    pub tx: Sender<TuiMessage>,
    rx: Receiver<EngineMessage>,

    // Command parser
    command_parser: CommandParser,
}

impl App {
    pub fn new(tx: Sender<TuiMessage>, rx: Receiver<EngineMessage>) -> Self {
        Self {
            mode: TuiMode::Tracker,
            command_input: String::new(),
            command_mode: false,
            status_message: "CHURCHKEY | F1:Track F2:Patch | : cmd | Tab: cycle | Space: edit".to_string(),
            should_quit: false,
            autocompleter: Autocompleter::new(),
            completion_base: String::new(),
            menu_open: false,
            menu_category: 0,
            menu_item: 0,
            graph_state: GraphState::default(),
            selected_node: None,
            auto_connect: true,
            audio_data: AudioScopeData::default(),
            scope_visible: false, // Off by default in tracker mode
            entropy_display: None,
            tracker_state: TrackerState::new(),
            tracker_view: TrackerView::new(),
            tx,
            rx,
            command_parser: CommandParser::new(),
        }
    }

    pub fn cycle_mode(&mut self) {
        self.mode = match self.mode {
            TuiMode::Tracker => TuiMode::Patch,
            TuiMode::Patch => TuiMode::Mix,
            TuiMode::Mix => TuiMode::Tracker,
            _ => TuiMode::Tracker,
        };
        self.update_status_for_mode();
    }

    pub fn set_mode(&mut self, mode: TuiMode) {
        self.mode = mode;
        self.update_status_for_mode();
    }

    fn update_status_for_mode(&mut self) {
        self.status_message = match self.mode {
            TuiMode::Tracker => "TRACKER | ↑↓←→ navigate | Space: edit | Tab: column | Enter: confirm".to_string(),
            TuiMode::Patch => "PATCH | : cmd | Ctrl+N: nodes | Tab: complete".to_string(),
            TuiMode::Mix => "MIX | ↑↓ track | ←→ adjust volume".to_string(),
            _ => "CHURCHKEY".to_string(),
        };
    }

    pub fn enter_command_mode(&mut self) {
        self.command_mode = true;
        self.command_input.clear();
        self.autocompleter.reset();
        self.completion_base.clear();
    }

    pub fn exit_command_mode(&mut self) {
        self.command_mode = false;
        self.command_input.clear();
        self.autocompleter.reset();
        self.completion_base.clear();
    }

    /// Update autocomplete suggestions for current input
    pub fn update_completions(&mut self) {
        self.autocompleter.update(&self.command_input);
        self.completion_base = self.command_input.clone();
    }

    /// Cycle to next completion
    pub fn cycle_completion(&mut self) {
        if !self.autocompleter.active {
            self.autocompleter.update(&self.command_input);
            self.completion_base = self.command_input.clone();
        }
        if let Some(completed) = self.autocompleter.cycle_next(&self.completion_base) {
            self.command_input = completed;
        }
    }

    /// Cycle to previous completion (Shift+Tab)
    pub fn cycle_completion_prev(&mut self) {
        if !self.autocompleter.active {
            self.autocompleter.update(&self.command_input);
            self.completion_base = self.command_input.clone();
        }
        if let Some(completed) = self.autocompleter.cycle_prev(&self.completion_base) {
            self.command_input = completed;
        }
    }

    /// Reset completion cycling when input changes
    pub fn on_input_changed(&mut self) {
        self.autocompleter.reset();
        self.update_completions();
    }

    // === Node Menu ===

    pub fn open_node_menu(&mut self) {
        self.menu_open = true;
        self.menu_category = 0;
        self.menu_item = 0;
    }

    pub fn close_node_menu(&mut self) {
        self.menu_open = false;
    }

    pub fn menu_up(&mut self) {
        if self.menu_item > 0 {
            self.menu_item -= 1;
        }
    }

    pub fn menu_down(&mut self) {
        let items_in_category = NODE_CATEGORIES
            .get(self.menu_category)
            .map(|(_, items)| items.len())
            .unwrap_or(0);
        if self.menu_item + 1 < items_in_category {
            self.menu_item += 1;
        }
    }

    pub fn menu_left(&mut self) {
        if self.menu_category > 0 {
            self.menu_category -= 1;
            self.menu_item = 0;
        }
    }

    pub fn menu_right(&mut self) {
        if self.menu_category + 1 < NODE_CATEGORIES.len() {
            self.menu_category += 1;
            self.menu_item = 0;
        }
    }

    pub fn menu_select(&mut self) {
        if let Some((_, items)) = NODE_CATEGORIES.get(self.menu_category) {
            if let Some(&node_type) = items.get(self.menu_item) {
                let transaction = Transaction::NodeAdd {
                    node_type: node_type.to_string(),
                    name: None,
                };
                self.tx.send(TuiMessage::Transaction(transaction)).ok();
                self.status_message = format!("Created {}", node_type);
            }
        }
        self.close_node_menu();
    }

    pub fn toggle_scope(&mut self) {
        self.scope_visible = !self.scope_visible;
    }

    pub fn execute_command(&mut self) {
        let input = self.command_input.trim();

        if input.is_empty() {
            self.exit_command_mode();
            return;
        }

        match self.command_parser.parse(input) {
            Ok(transaction) => {
                self.tx.send(TuiMessage::Transaction(transaction)).ok();
                self.status_message = format!("Executing: {}", input);
            }
            Err(e) => {
                self.status_message = format!("Error: {}", e);
            }
        }

        self.exit_command_mode();
    }

    pub fn handle_engine_messages(&mut self) {
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                EngineMessage::TransactionResult(result) => {
                    match result {
                        TransactionResult::Success { message } => {
                            if let Some(msg) = message {
                                self.status_message = msg;
                            }
                        }
                        TransactionResult::Error { message } => {
                            self.status_message = format!("Error: {}", message);
                        }
                        TransactionResult::Deferred { apply_point } => {
                            self.status_message = format!("Deferred to {:?}", apply_point);
                        }
                    }
                }
                EngineMessage::AudioData(data) => {
                    self.audio_data = data;
                }
                EngineMessage::GraphState(state) => {
                    self.graph_state = state;
                }
                EngineMessage::Error(e) => {
                    self.status_message = format!("Engine error: {}", e);
                }
            }
        }
    }

    pub fn quit(&mut self) {
        self.tx.send(TuiMessage::Quit).ok();
        self.should_quit = true;
    }
}
