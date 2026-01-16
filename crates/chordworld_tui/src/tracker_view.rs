//! Tracker Pattern Grid View
//!
//! Renders the pattern editor with note/instrument/volume/fx columns.
//! Keyboard-first, EGA palette compliant.

use crate::palette::{BoxChars, Palette};
use chordworld_core::{Note, Pattern, TrackerCell};
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Tracker cursor position
#[derive(Debug, Clone, Copy, Default)]
pub struct TrackerCursor {
    pub row: usize,
    pub track: usize,
    pub column: TrackerColumn, // Which sub-column within the track
}

/// Sub-columns within a track
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TrackerColumn {
    #[default]
    Note,
    Instrument,
    Volume,
    Fx1,
    Fx2,
}

impl TrackerColumn {
    pub fn next(self) -> Self {
        match self {
            TrackerColumn::Note => TrackerColumn::Instrument,
            TrackerColumn::Instrument => TrackerColumn::Volume,
            TrackerColumn::Volume => TrackerColumn::Fx1,
            TrackerColumn::Fx1 => TrackerColumn::Fx2,
            TrackerColumn::Fx2 => TrackerColumn::Note,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            TrackerColumn::Note => TrackerColumn::Fx2,
            TrackerColumn::Instrument => TrackerColumn::Note,
            TrackerColumn::Volume => TrackerColumn::Instrument,
            TrackerColumn::Fx1 => TrackerColumn::Volume,
            TrackerColumn::Fx2 => TrackerColumn::Fx1,
        }
    }
}

/// Tracker view state
pub struct TrackerView {
    pub cursor: TrackerCursor,
    pub scroll_row: usize,
    pub scroll_track: usize,
    pub edit_mode: bool,
    pub playing: bool,
    pub play_row: usize,
}

impl Default for TrackerView {
    fn default() -> Self {
        Self {
            cursor: TrackerCursor::default(),
            scroll_row: 0,
            scroll_track: 0,
            edit_mode: false,
            playing: false,
            play_row: 0,
        }
    }
}

impl TrackerView {
    pub fn new() -> Self {
        Self::default()
    }

    /// Render the tracker pattern grid
    pub fn render(&self, f: &mut Frame, area: Rect, pattern: Option<&Pattern>) {
        let block = Block::default()
            .title(format!(
                " Pattern {} ",
                pattern.map(|p| p.name.as_str()).unwrap_or("--")
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Palette::BORDER_FOCUSED))
            .style(Style::default().bg(Palette::BG_PRIMARY));

        let inner = block.inner(area);
        f.render_widget(block, area);

        let Some(pattern) = pattern else {
            let empty = Paragraph::new("No pattern loaded")
                .style(Style::default().fg(Palette::FG_DIM));
            f.render_widget(empty, inner);
            return;
        };

        // Calculate visible rows
        let visible_rows = (inner.height as usize).saturating_sub(1); // -1 for header
        let visible_tracks = self.calc_visible_tracks(inner.width as usize, pattern.tracks.len());

        let mut lines: Vec<Line> = Vec::new();

        // Header row
        lines.push(self.render_header(pattern, visible_tracks));

        // Pattern rows
        for row_offset in 0..visible_rows {
            let row = self.scroll_row + row_offset;
            if row >= pattern.length {
                break;
            }

            let is_cursor_row = row == self.cursor.row;
            let is_play_row = self.playing && row == self.play_row;

            lines.push(self.render_row(pattern, row, visible_tracks, is_cursor_row, is_play_row));
        }

        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, inner);
    }

    fn calc_visible_tracks(&self, width: usize, total_tracks: usize) -> usize {
        // Each track is: NOTE(3) + INSTR(2) + VOL(2) + FX1(4) + FX2(4) + separator(1) = 16 chars
        // Plus row number column: 3 chars
        let track_width = 16;
        let row_col_width = 4;
        let available = width.saturating_sub(row_col_width);
        let max_tracks = available / track_width;
        max_tracks.min(total_tracks - self.scroll_track)
    }

    fn render_header(&self, pattern: &Pattern, visible_tracks: usize) -> Line<'static> {
        let mut spans: Vec<Span> = Vec::new();

        // Row number column
        spans.push(Span::styled(
            "ROW ",
            Style::default().fg(Palette::FG_DIM),
        ));

        // Track headers
        for i in 0..visible_tracks {
            let track_idx = self.scroll_track + i;
            if let Some(track) = pattern.tracks.get(track_idx) {
                let name = if track.name.len() > 12 {
                    format!("{}...", &track.name[..9])
                } else {
                    format!("{:^12}", track.name)
                };
                spans.push(Span::styled(
                    format!("{}{}", name, BoxChars::V),
                    Style::default().fg(Palette::FG_BRIGHT),
                ));
            }
        }

        Line::from(spans)
    }

    fn render_row(
        &self,
        pattern: &Pattern,
        row: usize,
        visible_tracks: usize,
        is_cursor_row: bool,
        is_play_row: bool,
    ) -> Line<'static> {
        let mut spans: Vec<Span> = Vec::new();

        // Row number
        let row_style = if is_play_row {
            Style::default().fg(Palette::BG_CURSOR).add_modifier(Modifier::BOLD)
        } else if row % pattern.lpb as usize == 0 {
            Style::default().fg(Palette::FG_BRIGHT)
        } else {
            Style::default().fg(Palette::FG_DIM)
        };

        let row_marker = if is_play_row {
            format!("{}{:02X}", BoxChars::PLAY_HEAD, row)
        } else {
            format!(" {:02X}", row)
        };
        spans.push(Span::styled(format!("{} ", row_marker), row_style));

        // Track cells
        for i in 0..visible_tracks {
            let track_idx = self.scroll_track + i;
            let is_cursor_track = track_idx == self.cursor.track;

            if let Some(cell) = pattern.get_cell(track_idx, row) {
                spans.extend(self.render_cell(
                    cell,
                    is_cursor_row && is_cursor_track,
                    is_cursor_row,
                ));
            }
            spans.push(Span::styled(
                BoxChars::V.to_string(),
                Style::default().fg(Palette::FG_DIM),
            ));
        }

        Line::from(spans)
    }

    fn render_cell(
        &self,
        cell: &TrackerCell,
        is_cursor: bool,
        is_cursor_row: bool,
    ) -> Vec<Span<'static>> {
        let mut spans = Vec::new();

        let row_bg = if is_cursor_row {
            Some(Palette::CURSOR_ROW)
        } else {
            None
        };

        // NOTE column
        let note_str = cell.note.to_string();
        let note_style = if is_cursor && self.cursor.column == TrackerColumn::Note {
            Style::default()
                .fg(Palette::BG_PRIMARY)
                .bg(Palette::BG_CURSOR)
                .add_modifier(Modifier::BOLD)
        } else {
            let fg = match cell.note {
                Note::Empty => Palette::FG_DIM,
                Note::Off => Palette::NOTE_OFF,
                Note::On(_) => Palette::NOTE,
            };
            let mut s = Style::default().fg(fg);
            if let Some(bg) = row_bg {
                s = s.bg(bg);
            }
            s
        };
        spans.push(Span::styled(note_str, note_style));

        // INSTRUMENT column
        let instr_str = cell
            .instrument
            .map(|i| format!("{:02X}", i))
            .unwrap_or_else(|| "--".to_string());
        let instr_style = if is_cursor && self.cursor.column == TrackerColumn::Instrument {
            Style::default()
                .fg(Palette::BG_PRIMARY)
                .bg(Palette::BG_CURSOR)
        } else {
            let mut s = Style::default().fg(if cell.instrument.is_some() {
                Palette::INSTRUMENT
            } else {
                Palette::FG_DIM
            });
            if let Some(bg) = row_bg {
                s = s.bg(bg);
            }
            s
        };
        spans.push(Span::styled(instr_str, instr_style));

        // VOLUME column
        let vol_str = cell
            .volume
            .map(|v| format!("{:02X}", v))
            .unwrap_or_else(|| "--".to_string());
        let vol_style = if is_cursor && self.cursor.column == TrackerColumn::Volume {
            Style::default()
                .fg(Palette::BG_PRIMARY)
                .bg(Palette::BG_CURSOR)
        } else {
            let mut s = Style::default().fg(if cell.volume.is_some() {
                Palette::VOLUME
            } else {
                Palette::FG_DIM
            });
            if let Some(bg) = row_bg {
                s = s.bg(bg);
            }
            s
        };
        spans.push(Span::styled(vol_str, vol_style));

        // FX1 column
        let fx1_str = cell.fx1.to_string();
        let fx1_style = if is_cursor && self.cursor.column == TrackerColumn::Fx1 {
            Style::default()
                .fg(Palette::BG_PRIMARY)
                .bg(Palette::BG_CURSOR)
        } else {
            let mut s = Style::default().fg(if cell.fx1.is_empty() {
                Palette::FG_DIM
            } else {
                Palette::EFFECT
            });
            if let Some(bg) = row_bg {
                s = s.bg(bg);
            }
            s
        };
        spans.push(Span::styled(fx1_str, fx1_style));

        // FX2 column
        let fx2_str = cell.fx2.to_string();
        let fx2_style = if is_cursor && self.cursor.column == TrackerColumn::Fx2 {
            Style::default()
                .fg(Palette::BG_PRIMARY)
                .bg(Palette::BG_CURSOR)
        } else {
            let mut s = Style::default().fg(if cell.fx2.is_empty() {
                Palette::FG_DIM
            } else {
                Palette::EFFECT
            });
            if let Some(bg) = row_bg {
                s = s.bg(bg);
            }
            s
        };
        spans.push(Span::styled(fx2_str, fx2_style));

        spans
    }

    // Navigation
    pub fn move_up(&mut self, pattern_len: usize) {
        if self.cursor.row > 0 {
            self.cursor.row -= 1;
            self.ensure_row_visible(pattern_len);
        }
    }

    pub fn move_down(&mut self, pattern_len: usize) {
        if self.cursor.row + 1 < pattern_len {
            self.cursor.row += 1;
            self.ensure_row_visible(pattern_len);
        }
    }

    pub fn move_left(&mut self, num_tracks: usize) {
        if self.cursor.column == TrackerColumn::Note {
            if self.cursor.track > 0 {
                self.cursor.track -= 1;
                self.cursor.column = TrackerColumn::Fx2;
            }
        } else {
            self.cursor.column = self.cursor.column.prev();
        }
    }

    pub fn move_right(&mut self, num_tracks: usize) {
        if self.cursor.column == TrackerColumn::Fx2 {
            if self.cursor.track + 1 < num_tracks {
                self.cursor.track += 1;
                self.cursor.column = TrackerColumn::Note;
            }
        } else {
            self.cursor.column = self.cursor.column.next();
        }
    }

    pub fn page_up(&mut self, pattern_len: usize, page_size: usize) {
        self.cursor.row = self.cursor.row.saturating_sub(page_size);
        self.ensure_row_visible(pattern_len);
    }

    pub fn page_down(&mut self, pattern_len: usize, page_size: usize) {
        self.cursor.row = (self.cursor.row + page_size).min(pattern_len.saturating_sub(1));
        self.ensure_row_visible(pattern_len);
    }

    fn ensure_row_visible(&mut self, _pattern_len: usize) {
        // Scroll to keep cursor visible
        if self.cursor.row < self.scroll_row {
            self.scroll_row = self.cursor.row;
        }
        // We'd need visible height here for proper scrolling
    }
}
