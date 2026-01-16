//! CHURCHKEY TUI
//!
//! Deterministic, keyboard-first audio workstation TUI
//! - Tracker workflow (Renoise-like)
//! - Patch graph (Pure Data/Max-like)
//! - 16-color EGA palette (TempleOS-inspired constraints)

pub mod app;
pub mod autocomplete;
pub mod command;
pub mod input;
pub mod palette;
pub mod panes;
pub mod tracker_view;
pub mod ui;

pub use app::*;
pub use autocomplete::*;
pub use command::*;
pub use input::*;
pub use palette::*;
pub use ui::*;
