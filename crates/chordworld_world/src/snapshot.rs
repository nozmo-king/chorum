//! Immutable graph snapshot for audio thread

use chordworld_core::{NodeId, SampleTime};
use chordworld_dsp::{BufferPool, Node, ProcessCtx};
use std::sync::Arc;

/// Buffer assignment for a node's port
#[derive(Debug, Clone, Copy)]
pub struct BufferAssignment {
    pub buffer_index: usize,
    pub channel: u32,
}

/// Node in the dispatch plan
pub struct SnapshotNode {
    pub id: NodeId,
    pub node: Arc<dyn Node>,
    pub bypassed: bool,
    pub input_buffers: Vec<Vec<BufferAssignment>>,  // Per input port
    pub output_buffers: Vec<Vec<BufferAssignment>>, // Per output port
}

/// Immutable graph snapshot for audio processing
pub struct GraphSnapshot {
    pub dispatch_order: Vec<SnapshotNode>,
    pub buffer_count: usize,
    pub sample_rate: f32,
}

impl GraphSnapshot {
    /// Create empty snapshot
    pub fn empty(sample_rate: f32) -> Self {
        Self {
            dispatch_order: Vec::new(),
            buffer_count: 0,
            sample_rate,
        }
    }

    /// Process one block of audio
    pub fn process(&self, _buffers: &mut BufferPool, block_size: u32, start_time: SampleTime) {
        let _ctx = ProcessCtx::new(self.sample_rate, block_size, start_time);

        for snapshot_node in &self.dispatch_order {
            if snapshot_node.bypassed {
                continue;
            }

            // For now, simplified processing without proper buffer routing
            // Full implementation would map input/output buffers correctly

            // This is a minimal implementation to get things compiling
            // Real version needs proper buffer routing from assignments
        }
    }
}

/// Compiler that builds GraphSnapshot from GraphModel
pub struct SnapshotCompiler {
    sample_rate: f32,
}

impl SnapshotCompiler {
    pub fn new(sample_rate: f32) -> Self {
        Self { sample_rate }
    }

    pub fn compile(&self, graph: &crate::GraphModel) -> Result<GraphSnapshot, crate::GraphError> {
        // Simplified compilation for now
        // Full version would:
        // 1. Compute topological sort
        // 2. Assign buffer indices
        // 3. Handle feedback loops
        // 4. Build dispatch plan

        let dispatch_order = Vec::new();
        let mut buffer_count = 0;

        // For now, just iterate nodes in arbitrary order
        for _node in graph.nodes() {
            // Clone the node instance (we'll need Arc or similar in practice)
            // For now, skip actual node instances
            buffer_count += 2; // Simplified: 2 buffers per node
        }

        Ok(GraphSnapshot {
            dispatch_order,
            buffer_count,
            sample_rate: self.sample_rate,
        })
    }
}
