//! Project versioning and compatibility

use serde::{Deserialize, Serialize};
use std::fmt;

/// Project version for snapshot/log compatibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ProjectVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ProjectVersion {
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Current version of CHORDWORLD
    pub const CURRENT: ProjectVersion = ProjectVersion::new(0, 1, 0);

    /// Check if this version is compatible with another version
    pub fn is_compatible_with(&self, other: &ProjectVersion) -> bool {
        // Major version must match exactly
        if self.major != other.major {
            return false;
        }

        // Minor version can be greater (backward compatible)
        if self.minor < other.minor {
            return false;
        }

        // Patch version is always compatible
        true
    }

    pub fn is_breaking_change(&self, other: &ProjectVersion) -> bool {
        self.major != other.major
    }
}

impl fmt::Display for ProjectVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Node type versioning
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeTypeVersion {
    pub type_name: String,
    pub version: u32,
}

impl NodeTypeVersion {
    pub fn new(type_name: impl Into<String>, version: u32) -> Self {
        Self {
            type_name: type_name.into(),
            version,
        }
    }
}

/// RNG seed versioning for determinism
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RngVersion {
    pub algorithm: String, // e.g., "xorshift64", "pcg32"
    pub seed: u64,
}

impl RngVersion {
    pub fn new(algorithm: impl Into<String>, seed: u64) -> Self {
        Self {
            algorithm: algorithm.into(),
            seed,
        }
    }
}
