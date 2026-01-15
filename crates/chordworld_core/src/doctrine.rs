//! Doctrine mode configuration (TempleOS-inspired constraints)

use serde::{Deserialize, Serialize};

/// Doctrine mode preset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DoctrineMode {
    Strict,
    Relaxed,
    Off,
}

/// Color palette constraint
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorPalette {
    Sixteen,    // 16-color palette (strict)
    TwoFiftySix, // 256 colors (relaxed)
    Full,       // No restriction (off)
}

/// Doctrine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctrineConfig {
    pub mode: DoctrineMode,

    // UI constraints
    pub keyboard_only: bool,
    pub color_palette: ColorPalette,
    pub fixed_grid_layout: bool,
    pub max_menu_depth: Option<u32>,

    // Behavior constraints
    pub explicit_logging: bool,
    pub require_provenance: bool,
    pub no_background_services: bool,
    pub no_silent_magic: bool,
    pub no_ambiguity: bool,

    // Graph constraints
    pub max_node_count: Option<usize>,
    pub max_connection_count: Option<usize>,
    pub min_feedback_delay_samples: usize,

    // File format
    pub simple_file_formats: bool,
}

impl DoctrineConfig {
    pub fn strict() -> Self {
        Self {
            mode: DoctrineMode::Strict,
            keyboard_only: true,
            color_palette: ColorPalette::Sixteen,
            fixed_grid_layout: true,
            max_menu_depth: Some(7),
            explicit_logging: true,
            require_provenance: true,
            no_background_services: true,
            no_silent_magic: true,
            no_ambiguity: true,
            max_node_count: Some(1024),
            max_connection_count: Some(4096),
            min_feedback_delay_samples: 256,
            simple_file_formats: true,
        }
    }

    pub fn relaxed() -> Self {
        Self {
            mode: DoctrineMode::Relaxed,
            keyboard_only: true,
            color_palette: ColorPalette::TwoFiftySix,
            fixed_grid_layout: true,
            max_menu_depth: Some(15),
            explicit_logging: true,
            require_provenance: true,
            no_background_services: true,
            no_silent_magic: false,
            no_ambiguity: true,
            max_node_count: Some(4096),
            max_connection_count: Some(16384),
            min_feedback_delay_samples: 64,
            simple_file_formats: true,
        }
    }

    pub fn off() -> Self {
        Self {
            mode: DoctrineMode::Off,
            keyboard_only: false,
            color_palette: ColorPalette::Full,
            fixed_grid_layout: false,
            max_menu_depth: None,
            explicit_logging: false,
            require_provenance: false,
            no_background_services: true,
            no_silent_magic: false,
            no_ambiguity: false,
            max_node_count: None,
            max_connection_count: None,
            min_feedback_delay_samples: 0,
            simple_file_formats: false,
        }
    }

    pub fn validate_node_count(&self, count: usize) -> Result<(), String> {
        if let Some(max) = self.max_node_count {
            if count >= max {
                return Err(format!(
                    "Node count {} exceeds doctrine limit {}",
                    count, max
                ));
            }
        }
        Ok(())
    }

    pub fn validate_connection_count(&self, count: usize) -> Result<(), String> {
        if let Some(max) = self.max_connection_count {
            if count >= max {
                return Err(format!(
                    "Connection count {} exceeds doctrine limit {}",
                    count, max
                ));
            }
        }
        Ok(())
    }

    pub fn validate_menu_depth(&self, depth: u32) -> Result<(), String> {
        if let Some(max) = self.max_menu_depth {
            if depth > max {
                return Err(format!(
                    "Menu depth {} exceeds doctrine limit {}. Use jump search or command palette.",
                    depth, max
                ));
            }
        }
        Ok(())
    }
}

impl Default for DoctrineConfig {
    fn default() -> Self {
        Self::strict()
    }
}
