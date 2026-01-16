//! World state (mutable simulation thread)

use crate::{GraphModel, GraphSnapshot, Pattern, SnapshotCompiler, Song};
use chordworld_core::{
    DoctrineConfig, IdGenerator, MusicalTime, PatternId, TimingConfig,
    Transaction, TransactionRecord, TransactionResult, TransportState, WorldTime,
};
use chordworld_dsp::NodeRegistry;
use chordworld_pow::{
    AestheticCriteria, AestheticScore, EntropyEntry, EntropyPool,
    Hash256, Miner, MicrotonalScale, MiningConfig,
};
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

    // 21e8 Entropy system
    pub entropy_pool: EntropyPool,
    pub active_tuning: Option<MicrotonalScale>,
    pub last_mined: Option<(Hash256, AestheticScore)>,

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
            entropy_pool: EntropyPool::new(),
            active_tuning: None,
            last_mined: None,
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

            // 21e8 POW Mining
            Transaction::PowMine { nonce_seed } => {
                // Configure mining with moderate difficulty for interactive use
                let mut config = MiningConfig::default();
                config.criteria = AestheticCriteria::standard();
                config.criteria.min_difficulty = 12; // Moderate difficulty
                config.max_iterations = Some(1_000_000); // Limit for responsiveness

                let miner = Miner::new(config);
                match miner.mine(nonce_seed.as_bytes()) {
                    Ok(result) => {
                        let entry = EntropyEntry::new(result.hash, result.aesthetic_score.clone())
                            .with_label(&nonce_seed);
                        self.entropy_pool.add(entry);
                        self.last_mined = Some((result.hash, result.aesthetic_score.clone()));

                        let hash_str = format!("{}", result.hash);
                        let rarity = result.aesthetic_score.rarity.as_str();
                        let score = result.aesthetic_score.total_score;
                        Ok(TransactionResult::success_with(format!(
                            "Mined {} hash: {}... [score:{}]",
                            rarity,
                            &hash_str[..16],
                            score
                        )))
                    }
                    Err(e) => Ok(TransactionResult::error(format!("Mining failed: {}", e))),
                }
            }

            Transaction::EntropyPoolClear => {
                self.entropy_pool = EntropyPool::new();
                Ok(TransactionResult::success_with("Entropy pool cleared"))
            }

            Transaction::TuningSetFromHash { hash_hex } => {
                // Parse hash from hex string
                if hash_hex.len() < 64 {
                    return Ok(TransactionResult::error("Hash must be 64 hex characters"));
                }

                let mut bytes = [0u8; 32];
                for (i, chunk) in hash_hex.as_bytes().chunks(2).take(32).enumerate() {
                    if let Ok(byte) = u8::from_str_radix(std::str::from_utf8(chunk).unwrap_or("00"), 16) {
                        bytes[i] = byte;
                    }
                }

                let hash = Hash256(bytes);
                let scale = MicrotonalScale::from_hash(&hash);
                self.active_tuning = Some(scale.clone());

                Ok(TransactionResult::success_with(format!(
                    "Tuning set: {} ({} notes)",
                    scale.name,
                    scale.pitches_cents.len()
                )))
            }

            Transaction::TuningSetRandom { seed } => {
                if let Some(entry) = self.entropy_pool.select_by_seed(seed) {
                    let scale = entry.scale.clone();
                    self.active_tuning = Some(scale.clone());
                    Ok(TransactionResult::success_with(format!(
                        "Tuning from pool: {} ({} notes)",
                        scale.name,
                        scale.pitches_cents.len()
                    )))
                } else {
                    // Generate a fresh hash if pool is empty
                    let mut hasher = blake3::Hasher::new();
                    hasher.update(&seed.to_le_bytes());
                    let hash = hasher.finalize();
                    let hash = Hash256::from(*hash.as_bytes());
                    let scale = MicrotonalScale::from_hash(&hash);
                    self.active_tuning = Some(scale.clone());
                    Ok(TransactionResult::success_with(format!(
                        "Tuning generated: {} ({} notes)",
                        scale.name,
                        scale.pitches_cents.len()
                    )))
                }
            }

            Transaction::TuningClear => {
                self.active_tuning = None;
                Ok(TransactionResult::success_with("Tuning cleared (12-TET)"))
            }

            Transaction::TuningShow => {
                if let Some(ref tuning) = self.active_tuning {
                    Ok(TransactionResult::success_with(format!(
                        "Tuning: {} | {} notes | divisions: {}",
                        tuning.name,
                        tuning.pitches_cents.len(),
                        tuning.divisions
                    )))
                } else {
                    Ok(TransactionResult::success_with("Tuning: 12-TET (standard)"))
                }
            }

            // Quick setups
            Transaction::SetupBasic => {
                // Create: OscSine -> Out
                self.graph.add_node("Out", Some("Main".to_string()))?;
                let out_id = self.graph.nodes().find(|n| n.name == "Main").map(|n| n.id);

                self.graph.add_node("OscSine", Some("Osc1".to_string()))?;
                let osc_id = self.graph.nodes().find(|n| n.name == "Osc1").map(|n| n.id);

                if let (Some(osc), Some(out)) = (osc_id, out_id) {
                    self.graph.connect(osc, "out", out, "in")?;
                }

                Ok(TransactionResult::success_with("Setup: Basic sine oscillator"))
            }

            Transaction::SetupFM => {
                // Create: OscFM -> Out
                self.graph.add_node("Out", Some("Main".to_string()))?;
                let out_id = self.graph.nodes().find(|n| n.name == "Main").map(|n| n.id);

                self.graph.add_node("OscFM", Some("FM1".to_string()))?;
                let fm_id = self.graph.nodes().find(|n| n.name == "FM1").map(|n| n.id);

                if let (Some(fm), Some(out)) = (fm_id, out_id) {
                    self.graph.connect(fm, "out", out, "in")?;
                    // Set FM params: 220Hz, ratio 2.0, index 3.0
                    self.graph.set_param(fm, chordworld_core::ParamIndex(0), chordworld_core::ParamValue::Float(220.0))?;
                    self.graph.set_param(fm, chordworld_core::ParamIndex(1), chordworld_core::ParamValue::Float(2.0))?;
                    self.graph.set_param(fm, chordworld_core::ParamIndex(2), chordworld_core::ParamValue::Float(3.0))?;
                }

                Ok(TransactionResult::success_with("Setup: FM synthesis (220Hz, ratio:2, idx:3)"))
            }

            Transaction::SetupPad => {
                // Create: OscSine + OscSaw + UtilMixer -> FxReverb -> Out
                self.graph.add_node("Out", Some("Main".to_string()))?;
                let out_id = self.graph.nodes().find(|n| n.name == "Main").map(|n| n.id);

                self.graph.add_node("FxReverb", Some("Reverb".to_string()))?;
                let reverb_id = self.graph.nodes().find(|n| n.name == "Reverb").map(|n| n.id);

                self.graph.add_node("OscSine", Some("Pad1".to_string()))?;
                let sine_id = self.graph.nodes().find(|n| n.name == "Pad1").map(|n| n.id);

                self.graph.add_node("OscSaw", Some("Pad2".to_string()))?;
                let saw_id = self.graph.nodes().find(|n| n.name == "Pad2").map(|n| n.id);

                // Connect chain
                if let (Some(sine), Some(reverb)) = (sine_id, reverb_id) {
                    self.graph.connect(sine, "out", reverb, "in")?;
                    self.graph.set_param(sine, chordworld_core::ParamIndex(0), chordworld_core::ParamValue::Float(220.0))?;
                }
                if let (Some(saw), Some(reverb)) = (saw_id, reverb_id) {
                    self.graph.connect(saw, "out", reverb, "in")?;
                    self.graph.set_param(saw, chordworld_core::ParamIndex(0), chordworld_core::ParamValue::Float(221.0))?; // Slight detune
                }
                if let (Some(reverb), Some(out)) = (reverb_id, out_id) {
                    self.graph.connect(reverb, "out", out, "in")?;
                }

                Ok(TransactionResult::success_with("Setup: Pad (sine+saw -> reverb)"))
            }

            Transaction::SetupDrums => {
                // Create: OscNoise -> FxDistortion -> Out
                self.graph.add_node("Out", Some("Main".to_string()))?;
                let out_id = self.graph.nodes().find(|n| n.name == "Main").map(|n| n.id);

                self.graph.add_node("OscNoise", Some("Noise".to_string()))?;
                let noise_id = self.graph.nodes().find(|n| n.name == "Noise").map(|n| n.id);

                self.graph.add_node("EnvAR", Some("Env".to_string()))?;
                let env_id = self.graph.nodes().find(|n| n.name == "Env").map(|n| n.id);

                if let (Some(noise), Some(out)) = (noise_id, out_id) {
                    self.graph.connect(noise, "out", out, "in")?;
                }

                Ok(TransactionResult::success_with("Setup: Drums (noise + envelope)"))
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
