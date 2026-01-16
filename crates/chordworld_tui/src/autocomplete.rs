//! Auto-complete engine with fuzzy matching

/// All available node types organized by category
pub const NODE_CATEGORIES: &[(&str, &[&str])] = &[
    ("Oscillators", &[
        "OscSine", "OscSaw", "OscSquare", "OscTriangle",
        "OscNoise", "OscWavetable", "OscFM",
    ]),
    ("Filters", &[
        "FilterSVF", "FilterMoog", "FilterLP", "FilterHP",
        "FilterBP", "FilterNotch",
    ]),
    ("Envelopes", &[
        "EnvADSR", "EnvAR", "EnvMulti",
    ]),
    ("Effects", &[
        "FxDelay", "FxReverb", "FxDistortion", "FxChorus",
        "FxPhaser", "FxFlanger", "FxCompressor", "FxEQ",
    ]),
    ("Modulators", &[
        "ModLFO", "ModSeq", "ModSampleHold", "ModEnvFollower", "ModSlew",
    ]),
    ("Utilities", &[
        "Gain", "Out", "UtilMixer", "UtilVCA", "UtilSplit",
        "UtilMerge", "UtilQuantizer", "UtilScope",
    ]),
];

/// All node types as a flat list
pub const ALL_NODE_TYPES: &[&str] = &[
    // Oscillators
    "OscSine", "OscSaw", "OscSquare", "OscTriangle",
    "OscNoise", "OscWavetable", "OscFM",
    // Filters
    "FilterSVF", "FilterMoog", "FilterLP", "FilterHP",
    "FilterBP", "FilterNotch",
    // Envelopes
    "EnvADSR", "EnvAR", "EnvMulti",
    // Effects
    "FxDelay", "FxReverb", "FxDistortion", "FxChorus",
    "FxPhaser", "FxFlanger", "FxCompressor", "FxEQ",
    // Modulators
    "ModLFO", "ModSeq", "ModSampleHold", "ModEnvFollower", "ModSlew",
    // Utilities
    "Gain", "Out", "UtilMixer", "UtilVCA", "UtilSplit",
    "UtilMerge", "UtilQuantizer", "UtilScope",
];

/// All available commands
pub const COMMANDS: &[&str] = &[
    "node.add",
    "node.rm",
    "connect",
    "param.set",
    "play",
    "stop",
    "tempo",
    "help",
    // Quick setups
    "setup.basic",
    "setup.fm",
    "setup.pad",
    "setup.drums",
    // 21e8 POW
    "pow.mine",
    "pow.pool.stats",
    "pow.pool.clear",
    // Microtuning
    "tuning.set",
    "tuning.random",
    "tuning.clear",
    "tuning.show",
    // Pathologies - hostile edge cases
    "path",
    "path.list",
    "path.irrational-meter",
    "path.golden-polyrhythm",
    "path.event-horizon",
    "path.pitch-drift",
    "path.anchorfree",
    "path.subliminal",
    "path.chaos-groove",
    "path.saboteur",
    "path.phantom-voices",
    "path.trap",
    "path.unmeasurable",
    "path.time-travel",
];

/// Pathology presets for autocomplete
pub const PATHOLOGY_NAMES: &[&str] = &[
    "irrational-meter",
    "golden-polyrhythm",
    "event-horizon",
    "pitch-drift",
    "anchorfree",
    "subliminal",
    "chaos-groove",
    "saboteur",
    "phantom-voices",
    "trap",
    "unmeasurable",
    "time-travel",
];

/// Auto-completion engine
pub struct Autocompleter {
    /// Current completions matching the input
    pub completions: Vec<String>,
    /// Index of currently selected completion
    pub index: usize,
    /// Whether we've started cycling
    pub active: bool,
}

impl Autocompleter {
    pub fn new() -> Self {
        Self {
            completions: Vec::new(),
            index: 0,
            active: false,
        }
    }

    /// Reset completions state
    pub fn reset(&mut self) {
        self.completions.clear();
        self.index = 0;
        self.active = false;
    }

    /// Generate completions for the current input
    pub fn update(&mut self, input: &str) {
        self.completions.clear();
        self.index = 0;

        if input.is_empty() {
            return;
        }

        // Split input to see if we're completing a command or an argument
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            return;
        }

        // Check if we're typing a command or have completed one
        let completing_arg = parts.len() > 1 || input.ends_with(' ');

        if completing_arg {
            // Complete arguments based on command
            let cmd = parts[0];
            let arg_prefix = if input.ends_with(' ') {
                ""
            } else {
                parts.last().unwrap_or(&"")
            };

            match cmd {
                "node.add" => {
                    // Complete with node types
                    self.completions = fuzzy_match(arg_prefix, ALL_NODE_TYPES);
                }
                "path" | "pathology" => {
                    // Complete with pathology names
                    self.completions = fuzzy_match(arg_prefix, PATHOLOGY_NAMES);
                }
                _ => {}
            }
        } else {
            // Complete command
            self.completions = fuzzy_match(parts[0], COMMANDS);
        }
    }

    /// Cycle to next completion, returns the completed text
    pub fn cycle_next(&mut self, original_input: &str) -> Option<String> {
        if self.completions.is_empty() {
            return None;
        }

        if !self.active {
            self.active = true;
        } else {
            self.index = (self.index + 1) % self.completions.len();
        }

        Some(self.apply_completion(original_input))
    }

    /// Cycle to previous completion
    pub fn cycle_prev(&mut self, original_input: &str) -> Option<String> {
        if self.completions.is_empty() {
            return None;
        }

        if !self.active {
            self.active = true;
        } else {
            self.index = if self.index == 0 {
                self.completions.len() - 1
            } else {
                self.index - 1
            };
        }

        Some(self.apply_completion(original_input))
    }

    /// Apply the current completion to the input
    fn apply_completion(&self, original_input: &str) -> String {
        if self.completions.is_empty() {
            return original_input.to_string();
        }

        let completion = &self.completions[self.index];
        let parts: Vec<&str> = original_input.split_whitespace().collect();

        if parts.len() <= 1 && !original_input.ends_with(' ') {
            // Replacing command
            completion.clone()
        } else {
            // Replacing argument - keep the command part
            let cmd_part = parts[0];
            format!("{} {}", cmd_part, completion)
        }
    }

    /// Get the currently selected completion (if any)
    pub fn current(&self) -> Option<&str> {
        if self.active && !self.completions.is_empty() {
            Some(&self.completions[self.index])
        } else {
            None
        }
    }
}

impl Default for Autocompleter {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple fuzzy matching - matches if all characters appear in order
fn fuzzy_match(query: &str, candidates: &[&str]) -> Vec<String> {
    let query_lower = query.to_lowercase();
    let mut matches: Vec<(String, i32)> = candidates
        .iter()
        .filter_map(|&candidate| {
            let score = fuzzy_score(&query_lower, &candidate.to_lowercase());
            if score > 0 {
                Some((candidate.to_string(), score))
            } else {
                None
            }
        })
        .collect();

    // Sort by score (descending), then alphabetically
    matches.sort_by(|a, b| {
        b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))
    });

    matches.into_iter().map(|(s, _)| s).collect()
}

/// Calculate fuzzy match score (higher is better, 0 means no match)
fn fuzzy_score(query: &str, candidate: &str) -> i32 {
    if query.is_empty() {
        return 1; // Empty query matches everything
    }

    // Exact prefix match gets highest score
    if candidate.starts_with(query) {
        return 1000 + (100 - candidate.len() as i32);
    }

    // Check if all query chars appear in order
    let mut query_chars = query.chars().peekable();
    let mut score = 0;
    let mut prev_match_idx: i32 = -1;
    let mut consecutive_bonus = 0;

    for (idx, c) in candidate.chars().enumerate() {
        if let Some(&qc) = query_chars.peek() {
            if c == qc {
                query_chars.next();

                // Bonus for consecutive matches
                if idx as i32 == prev_match_idx + 1 {
                    consecutive_bonus += 10;
                }

                // Bonus for matching at word boundaries
                if idx == 0 || candidate.chars().nth(idx - 1).map_or(false, |p| !p.is_alphanumeric()) {
                    score += 20;
                }

                score += 10 + consecutive_bonus;
                prev_match_idx = idx as i32;
            } else {
                consecutive_bonus = 0;
            }
        }
    }

    // All query chars must be found
    if query_chars.peek().is_some() {
        0
    } else {
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_match_prefix() {
        let matches = fuzzy_match("Osc", ALL_NODE_TYPES);
        assert!(matches[0].starts_with("Osc"));
    }

    #[test]
    fn test_fuzzy_match_subsequence() {
        let matches = fuzzy_match("fs", ALL_NODE_TYPES);
        assert!(matches.iter().any(|m| m == "FilterSVF"));
    }

    #[test]
    fn test_command_completion() {
        let matches = fuzzy_match("node", COMMANDS);
        assert!(matches.contains(&"node.add".to_string()));
        assert!(matches.contains(&"node.rm".to_string()));
    }
}
