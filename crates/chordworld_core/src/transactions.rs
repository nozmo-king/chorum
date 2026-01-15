//! Transaction types for deterministic state mutations

use crate::{ConnectionId, InstrumentId, MacroId, NodeId, ParamIndex, ParamValue, PatternId, TransactionId, TransportState};
use serde::{Deserialize, Serialize};

/// Apply point for transactions (when they take effect)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApplyPoint {
    Now,        // Immediate (only if safe)
    NextBlock,  // Next audio block boundary
    NextLine,   // Next tracker line
    NextBeat,   // Next beat boundary
    NextBar,    // Next bar boundary
}

/// Port endpoint specification for connections
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortEndpoint {
    pub node: NodeId,
    pub port_name: String,
    pub channel: Option<u32>, // For audio connections
}

impl PortEndpoint {
    pub fn new(node: NodeId, port_name: impl Into<String>) -> Self {
        Self {
            node,
            port_name: port_name.into(),
            channel: None,
        }
    }

    pub fn with_channel(mut self, channel: u32) -> Self {
        self.channel = Some(channel);
        self
    }
}

/// Connection mapping (for audio channel routing or control scaling)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConnectionMap {
    /// Direct connection (default)
    Direct,

    /// Audio channel mapping
    AudioChannels {
        map: Vec<(u32, u32)>, // (src_channel, dst_channel) pairs
    },

    /// Control scaling/bias
    ControlScale {
        scale: f32,
        bias: f32,
    },
}

impl Default for ConnectionMap {
    fn default() -> Self {
        ConnectionMap::Direct
    }
}

/// Core transaction types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Transaction {
    // Node operations
    NodeAdd {
        node_type: String,
        name: Option<String>,
    },

    NodeRemove {
        node: NodeId,
    },

    NodeRename {
        node: NodeId,
        new_name: String,
    },

    NodeClone {
        node: NodeId,
        new_name: Option<String>,
    },

    NodeBypass {
        node: NodeId,
        bypassed: bool,
    },

    // Connection operations
    Connect {
        src: PortEndpoint,
        dst: PortEndpoint,
        map: ConnectionMap,
    },

    Disconnect {
        connection: ConnectionId,
    },

    DisconnectNode {
        node: NodeId,
    },

    // Parameter operations
    ParamSet {
        node: NodeId,
        param: ParamIndex,
        value: ParamValue,
    },

    ParamNudge {
        node: NodeId,
        param: ParamIndex,
        delta: f32,
    },

    ParamBind {
        node: NodeId,
        param: ParamIndex,
        ctrl_src: NodeId,
        ctrl_port: String,
    },

    ParamUnbind {
        node: NodeId,
        param: ParamIndex,
        ctrl_src: NodeId,
    },

    // Macro operations
    MacroCreate {
        name: String,
        nodes: Vec<NodeId>,
    },

    MacroInstantiate {
        macro_id: MacroId,
        instance_name: Option<String>,
    },

    MacroExpand {
        node: NodeId,
    },

    // Pattern/Tracker operations
    PatternCreate {
        name: String,
        rows: u32,
        tracks: u32,
    },

    PatternSetNote {
        pattern: PatternId,
        row: u32,
        track: u32,
        note: Option<u8>,
        velocity: Option<u8>,
        instrument: Option<InstrumentId>,
    },

    // Transport operations
    TransportSet {
        state: TransportState,
        apply: ApplyPoint,
    },

    TransportSeek {
        position: u64, // In samples or lines, TBD
    },

    // Project-level operations
    SetTempo {
        bpm: f32,
    },

    SetLPB {
        lpb: u32, // Lines per beat
    },
}

/// Transaction record for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRecord {
    pub id: TransactionId,
    pub timestamp: u64,          // Logical timestamp (world step counter)
    pub transaction: Transaction,
    pub checksum: u32,           // CRC32 of serialized transaction
}

impl TransactionRecord {
    pub fn new(id: TransactionId, timestamp: u64, transaction: Transaction) -> Self {
        let checksum = Self::compute_checksum(&transaction);
        Self {
            id,
            timestamp,
            transaction,
            checksum,
        }
    }

    fn compute_checksum(transaction: &Transaction) -> u32 {
        // Simplified checksum for now; use proper CRC32 in production
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        // Serialize transaction to bytes and hash
        // For now, just hash the discriminant
        std::mem::discriminant(transaction).hash(&mut hasher);
        hasher.finish() as u32
    }

    pub fn verify_checksum(&self) -> bool {
        self.checksum == Self::compute_checksum(&self.transaction)
    }
}

/// Result of applying a transaction
#[derive(Debug, Clone)]
pub enum TransactionResult {
    Success {
        message: Option<String>,
    },
    Error {
        message: String,
    },
    Deferred {
        apply_point: ApplyPoint,
    },
}

impl TransactionResult {
    pub fn success() -> Self {
        TransactionResult::Success { message: None }
    }

    pub fn success_with(message: impl Into<String>) -> Self {
        TransactionResult::Success {
            message: Some(message.into()),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        TransactionResult::Error {
            message: message.into(),
        }
    }

    pub fn deferred(apply_point: ApplyPoint) -> Self {
        TransactionResult::Deferred { apply_point }
    }

    pub fn is_success(&self) -> bool {
        matches!(self, TransactionResult::Success { .. })
    }

    pub fn is_error(&self) -> bool {
        matches!(self, TransactionResult::Error { .. })
    }
}
