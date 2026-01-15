//! World state (mutable simulation thread)

use crate::{GraphModel, GraphSnapshot, Pattern, SnapshotCompiler, Song};
use chordworld_core::{
    DoctrineConfig, IdGenerator, MusicalTime, PatternId, TimingConfig,
    Transaction, TransactionRecord, TransactionResult, TransportState, WorldTime,
};
use chordworld_dsp::NodeRegistry;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorldError {
    #[error("Graph error: {0}")]
    Graph(#[from] crate::GraphError),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("Doctrine violation: {0}")]
    Doctrine(String),

    #[error("Pattern not found: {0:?}")]
    PatternNotFound(PatternId),
}

/// Transport state
pub struct Transport {
    pub state: TransportState,
    pub position: MusicalTime,
    pub playing: bool,
}

impl Transport {
    pub fn new() -> Self {
        Self {
            state: TransportState::Stopped,
            position: MusicalTime::zero(),
            playing: false,
        }
    }

    pub fn play(&mut self) {
        self.state = TransportState::Playing;
        self.playing = true;
    }

    pub fn stop(&mut self) {
        self.state = TransportState::Stopped;
        self.playing = false;
        self.position = MusicalTime::zero();
    }

    pub fn pause(&mut self) {
        self.state = TransportState::Paused;
        self.playing = false;
    }
}

impl Default for Transport {
    fn default() -> Self {
        Self::new()
    }
}

/// World state (mutable, world thread)
pub struct WorldState {
    pub graph: GraphModel,
    pub song: Song,
    pub transport: Transport,
    pub timing: TimingConfig,
    pub doctrine: DoctrineConfig,
    pub world_time: WorldTime,
    pub id_gen: IdGenerator,

    // Transaction log
    transaction_log: Vec<TransactionRecord>,

    // Snapshot compiler
    compiler: SnapshotCompiler,
}

impl WorldState {
    pub fn new(registry: NodeRegistry, doctrine: DoctrineConfig) -> Self {
        let timing = TimingConfig::default_config();
        let compiler = SnapshotCompiler::new(timing.sample_rate);

        Self {
            graph: GraphModel::new(registry),
            song: Song::new("Untitled".to_string()),
            transport: Transport::new(),
            timing,
            doctrine,
            world_time: WorldTime::zero(),
            id_gen: IdGenerator::new(),
            transaction_log: Vec::new(),
            compiler,
        }
    }

    /// Apply a transaction to the world state
    pub fn apply_transaction(&mut self, transaction: Transaction) -> Result<TransactionResult, WorldError> {
        // Validate doctrine constraints
        if self.doctrine.mode != chordworld_core::DoctrineMode::Off {
            if let Err(e) = self.validate_doctrine(&transaction) {
                return Ok(TransactionResult::error(e));
            }
        }

        // Apply the transaction
        let result = self.apply_transaction_impl(transaction.clone())?;

        // Log the transaction if successful
        if result.is_success() {
            let tx_id = self.id_gen.next_transaction();
            let record = TransactionRecord::new(tx_id, self.world_time.as_u64(), transaction);
            self.transaction_log.push(record);
            self.world_time.increment();
        }

        Ok(result)
    }

    fn validate_doctrine(&self, transaction: &Transaction) -> Result<(), String> {
        match transaction {
            Transaction::NodeAdd { .. } => {
                self.doctrine.validate_node_count(self.graph.node_count() + 1)?;
            }
            Transaction::Connect { .. } => {
                self.doctrine.validate_connection_count(self.graph.connection_count() + 1)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn apply_transaction_impl(&mut self, transaction: Transaction) -> Result<TransactionResult, WorldError> {
        match transaction {
            Transaction::NodeAdd { node_type, name } => {
                let id = self.graph.add_node(&node_type, name)?;
                Ok(TransactionResult::success_with(format!("Created node {}", id)))
            }

            Transaction::NodeRemove { node } => {
                self.graph.remove_node(node)?;
                Ok(TransactionResult::success_with(format!("Removed node {}", node)))
            }

            Transaction::Connect { src, dst, map: _ } => {
                let conn_id = self.graph.connect(
                    src.node,
                    &src.port_name,
                    dst.node,
                    &dst.port_name,
                )?;
                Ok(TransactionResult::success_with(format!("Created connection {}", conn_id)))
            }

            Transaction::Disconnect { connection } => {
                self.graph.disconnect(connection)?;
                Ok(TransactionResult::success_with("Disconnected"))
            }

            Transaction::ParamSet { node, param, value } => {
                self.graph.set_param(node, param, value)?;
                Ok(TransactionResult::success())
            }

            Transaction::TransportSet { state, apply: _ } => {
                match state {
                    TransportState::Playing => self.transport.play(),
                    TransportState::Stopped => self.transport.stop(),
                    TransportState::Paused => self.transport.pause(),
                }
                Ok(TransactionResult::success_with(format!("Transport: {:?}", state)))
            }

            Transaction::SetTempo { bpm } => {
                self.timing.bpm = bpm.max(20.0).min(999.0);
                Ok(TransactionResult::success_with(format!("BPM set to {}", bpm)))
            }

            Transaction::PatternCreate { name, rows, tracks } => {
                let id = self.id_gen.next_pattern();
                let pattern = Pattern::new(id, name, rows as usize, tracks as usize);
                self.song.add_pattern(pattern);
                Ok(TransactionResult::success_with(format!("Created pattern {}", id)))
            }

            _ => Ok(TransactionResult::error("Transaction type not yet implemented")),
        }
    }

    /// Compile current graph to snapshot
    pub fn compile_snapshot(&self) -> Result<GraphSnapshot, WorldError> {
        self.compiler.compile(&self.graph).map_err(WorldError::Graph)
    }

    /// Get transaction log
    pub fn transaction_log(&self) -> &[TransactionRecord] {
        &self.transaction_log
    }

    /// Get current world time
    pub fn time(&self) -> WorldTime {
        self.world_time
    }

    /// Validate the current world state
    pub fn validate(&self) -> Result<Vec<String>, WorldError> {
        self.graph.validate().map_err(WorldError::Graph)
    }
}
