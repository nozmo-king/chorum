//! CHURCHKEY 16-Color EGA Palette
//!
//! Strict constraint: only these 16 colors.
//! No gradients, no transparency, no anti-aliasing.

use ratatui::style::Color;

/// The canonical 16-color EGA palette
pub struct Palette;

impl Palette {
    // Dark colors (0-7)
    pub const BLACK: Color = Color::Indexed(0);
    pub const BLUE: Color = Color::Indexed(4);
    pub const GREEN: Color = Color::Indexed(2);
    pub const CYAN: Color = Color::Indexed(6);
    pub const RED: Color = Color::Indexed(1);
    pub const MAGENTA: Color = Color::Indexed(5);
    pub const BROWN: Color = Color::Indexed(3);
    pub const LIGHT_GRAY: Color = Color::Indexed(7);

    // Bright colors (8-15)
    pub const DARK_GRAY: Color = Color::Indexed(8);
    pub const BRIGHT_BLUE: Color = Color::Indexed(12);
    pub const BRIGHT_GREEN: Color = Color::Indexed(10);
    pub const BRIGHT_CYAN: Color = Color::Indexed(14);
    pub const BRIGHT_RED: Color = Color::Indexed(9);
    pub const BRIGHT_MAGENTA: Color = Color::Indexed(13);
    pub const YELLOW: Color = Color::Indexed(11);
    pub const WHITE: Color = Color::Indexed(15);

    // Semantic aliases for UI elements
    pub const BG_PRIMARY: Color = Self::BLACK;
    pub const BG_SECONDARY: Color = Self::BLUE;
    pub const BG_SELECTED: Color = Self::CYAN;
    pub const BG_CURSOR: Color = Self::YELLOW;

    pub const FG_PRIMARY: Color = Self::LIGHT_GRAY;
    pub const FG_BRIGHT: Color = Self::WHITE;
    pub const FG_DIM: Color = Self::DARK_GRAY;

    pub const BORDER_NORMAL: Color = Self::BLUE;
    pub const BORDER_FOCUSED: Color = Self::BRIGHT_CYAN;

    // Tracker-specific
    pub const NOTE: Color = Self::WHITE;
    pub const NOTE_OFF: Color = Self::RED;
    pub const INSTRUMENT: Color = Self::YELLOW;
    pub const VOLUME: Color = Self::GREEN;
    pub const EFFECT: Color = Self::BRIGHT_MAGENTA;
    pub const CURSOR_ROW: Color = Self::CYAN;

    // Graph-specific
    pub const NODE_OSC: Color = Self::BRIGHT_CYAN;
    pub const NODE_FILTER: Color = Self::YELLOW;
    pub const NODE_EFFECT: Color = Self::BRIGHT_MAGENTA;
    pub const NODE_UTIL: Color = Self::BRIGHT_GREEN;
    pub const NODE_ENV: Color = Self::BRIGHT_RED;
    pub const CONNECTION: Color = Self::DARK_GRAY;

    // Status
    pub const STATUS_OK: Color = Self::BRIGHT_GREEN;
    pub const STATUS_WARN: Color = Self::YELLOW;
    pub const STATUS_ERROR: Color = Self::BRIGHT_RED;
    pub const STATUS_INFO: Color = Self::BRIGHT_CYAN;
}

/// Box drawing characters for UI
pub struct BoxChars;

impl BoxChars {
    // Single line
    pub const H: char = '─';
    pub const V: char = '│';
    pub const TL: char = '┌';
    pub const TR: char = '┐';
    pub const BL: char = '└';
    pub const BR: char = '┘';
    pub const T_DOWN: char = '┬';
    pub const T_UP: char = '┴';
    pub const T_RIGHT: char = '├';
    pub const T_LEFT: char = '┤';
    pub const CROSS: char = '┼';

    // Double line
    pub const DH: char = '═';
    pub const DV: char = '║';
    pub const DTL: char = '╔';
    pub const DTR: char = '╗';
    pub const DBL: char = '╚';
    pub const DBR: char = '╝';

    // Blocks for meters
    pub const BLOCK_FULL: char = '█';
    pub const BLOCK_HALF: char = '▄';
    pub const BLOCK_LIGHT: char = '░';
    pub const BLOCK_MED: char = '▒';
    pub const BLOCK_DARK: char = '▓';

    // Arrows
    pub const ARROW_RIGHT: char = '▶';
    pub const ARROW_LEFT: char = '◀';
    pub const ARROW_UP: char = '▲';
    pub const ARROW_DOWN: char = '▼';

    // Tracker specific
    pub const NOTE_MARKER: char = '♪';
    pub const PLAY_HEAD: char = '▶';
    pub const STOP: char = '■';
}
