//! Application state for TUI

use crate::command::CommandParser;
use chordworld_core::{Transaction, TransactionResult};
use chordworld_world::WorldState;
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
    Error(String),
}

/// Application state
pub struct App {
    pub mode: TuiMode,
    pub command_input: String,
    pub command_mode: bool,
    pub status_message: String,
    pub should_quit: bool,

    // Communication
    tx: Sender<TuiMessage>,
    rx: Receiver<EngineMessage>,

    // Command parser
    command_parser: CommandParser,
}

impl App {
    pub fn new(tx: Sender<TuiMessage>, rx: Receiver<EngineMessage>) -> Self {
        Self {
            mode: TuiMode::Patch,
            command_input: String::new(),
            command_mode: false,
            status_message: "Welcome to CHORDWORLD".to_string(),
            should_quit: false,
            tx,
            rx,
            command_parser: CommandParser::new(),
        }
    }

    pub fn enter_command_mode(&mut self) {
        self.command_mode = true;
        self.command_input.clear();
    }

    pub fn exit_command_mode(&mut self) {
        self.command_mode = false;
        self.command_input.clear();
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
