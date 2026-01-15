//! Stable IDs for all objects in the CHORDWORLD system

use serde::{Deserialize, Serialize};
use std::fmt;

/// Macro to define ID types with common operations
macro_rules! define_id {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub struct $name(pub u64);

        impl $name {
            pub fn new(id: u64) -> Self {
                Self(id)
            }

            pub fn as_u64(self) -> u64 {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }
    };
}

define_id!(NodeId, "Unique identifier for a node in the patch graph");
define_id!(PortId, "Unique identifier for a port on a node");
define_id!(ParamId, "Unique identifier for a parameter");
define_id!(ConnectionId, "Unique identifier for a connection between ports");
define_id!(PatternId, "Unique identifier for a tracker pattern");
define_id!(InstrumentId, "Unique identifier for an instrument");
define_id!(BufferId, "Unique identifier for an audio/control buffer");
define_id!(RuleId, "Unique identifier for a rule");
define_id!(MacroId, "Unique identifier for a macro/subgraph");
define_id!(TransactionId, "Unique identifier for a transaction");

/// Parameter index within a node (not globally unique)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ParamIndex(pub u32);

impl ParamIndex {
    pub fn new(idx: u32) -> Self {
        Self(idx)
    }
}

/// ID generator for creating unique IDs
#[derive(Debug, Default)]
pub struct IdGenerator {
    next_id: u64,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self { next_id: 1 }
    }

    pub fn next(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn next_node(&mut self) -> NodeId {
        NodeId(self.next())
    }

    pub fn next_port(&mut self) -> PortId {
        PortId(self.next())
    }

    pub fn next_param(&mut self) -> ParamId {
        ParamId(self.next())
    }

    pub fn next_connection(&mut self) -> ConnectionId {
        ConnectionId(self.next())
    }

    pub fn next_pattern(&mut self) -> PatternId {
        PatternId(self.next())
    }

    pub fn next_instrument(&mut self) -> InstrumentId {
        InstrumentId(self.next())
    }

    pub fn next_buffer(&mut self) -> BufferId {
        BufferId(self.next())
    }

    pub fn next_rule(&mut self) -> RuleId {
        RuleId(self.next())
    }

    pub fn next_macro(&mut self) -> MacroId {
        MacroId(self.next())
    }

    pub fn next_transaction(&mut self) -> TransactionId {
        TransactionId(self.next())
    }
}
