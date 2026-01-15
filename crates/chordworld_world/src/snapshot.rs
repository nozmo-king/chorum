//! Immutable graph snapshot for audio thread

use chordworld_core::{NodeId, SampleTime};
use chordworld_dsp::{AudioBusMut, AudioBusRef, BufferPool, Node, ProcessCtx};
use std::collections::HashMap;

/// Buffer assignment for a node's port
#[derive(Debug, Clone, Copy)]
pub struct BufferAssignment {
    pub buffer_index: usize,
    pub channel: u32,
}

/// Node in the dispatch plan
pub struct SnapshotNode {
    pub id: NodeId,
    pub node: Box<dyn Node>,
    pub node_type: String,
    pub bypassed: bool,
    pub input_buffers: Vec<usize>,  // Buffer index per input port
    pub output_buffers: Vec<usize>, // Buffer index per output port
}

/// Immutable graph snapshot for audio processing
pub struct GraphSnapshot {
    pub nodes: Vec<SnapshotNode>,
    pub buffer_count: usize,
    pub sample_rate: f32,
    pub output_buffer: Option<usize>, // Buffer index that goes to audio output
}

impl GraphSnapshot {
    /// Create empty snapshot
    pub fn empty(sample_rate: f32) -> Self {
        Self {
            nodes: Vec::new(),
            buffer_count: 1,
            sample_rate,
            output_buffer: None,
        }
    }

    /// Process one block of audio
    pub fn process(&self, buffers: &mut BufferPool, block_size: u32, start_time: SampleTime) {
        let ctx = ProcessCtx::new(self.sample_rate, block_size, start_time);
        let block_size = block_size as usize;

        // Clear all buffers first
        buffers.clear_all();

        for snapshot_node in &self.nodes {
            if snapshot_node.bypassed {
                continue;
            }

            // Copy input data to temp storage to avoid borrow issues
            let input_buf_idx = snapshot_node.input_buffers.first().copied().unwrap_or(0);
            let out_buf_idx = snapshot_node.output_buffers.first().copied().unwrap_or(0);

            // Get input data (copy to avoid borrow conflicts)
            let input_data: Vec<f32> = if let Some(buf) = buffers.get(input_buf_idx) {
                buf[..block_size.min(buf.len())].to_vec()
            } else {
                vec![0.0; block_size]
            };

            // Create input bus from copied data
            let input_bus = AudioBusRef::from_slice(&input_data);
            let input_buses = [input_bus];

            // Get output buffer and process
            if let Some(out_buf) = buffers.get_mut(out_buf_idx) {
                let len = block_size.min(out_buf.len());
                let out_slice = &mut out_buf[..len];
                let mut output_bus = AudioBusMut::from_slice(out_slice);

                // SAFETY: We need mutable access to node for processing
                // This is safe because we're in the audio thread and have exclusive access
                let node_ptr = &snapshot_node.node as *const Box<dyn Node> as *mut Box<dyn Node>;
                unsafe {
                    (*node_ptr).process(&ctx, &input_buses, std::slice::from_mut(&mut output_bus));
                }
            }
        }

        // Copy output buffer to buffer 0 if it's different
        if let Some(out_idx) = self.output_buffer {
            if out_idx != 0 && out_idx < buffers.buffer_count() {
                // Copy via temp to avoid borrow issues
                let data: Vec<f32> = if let Some(src) = buffers.get(out_idx) {
                    src[..block_size.min(src.len())].to_vec()
                } else {
                    return;
                };
                if let Some(dst) = buffers.get_mut(0) {
                    let len = block_size.min(dst.len()).min(data.len());
                    dst[..len].copy_from_slice(&data[..len]);
                }
            }
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
        if graph.node_count() == 0 {
            return Ok(GraphSnapshot::empty(self.sample_rate));
        }

        // Build adjacency list for topological sort
        let mut in_degree: HashMap<NodeId, usize> = HashMap::new();
        let mut adjacency: HashMap<NodeId, Vec<NodeId>> = HashMap::new();

        for node in graph.nodes() {
            in_degree.insert(node.id, 0);
            adjacency.insert(node.id, Vec::new());
        }

        for conn in graph.connections() {
            *in_degree.get_mut(&conn.dst_node).unwrap() += 1;
            adjacency.get_mut(&conn.src_node).unwrap().push(conn.dst_node);
        }

        // Kahn's algorithm for topological sort
        let mut queue: Vec<NodeId> = in_degree
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(&id, _)| id)
            .collect();

        let mut sorted_ids: Vec<NodeId> = Vec::new();

        while let Some(node_id) = queue.pop() {
            sorted_ids.push(node_id);

            if let Some(neighbors) = adjacency.get(&node_id) {
                for &neighbor in neighbors {
                    let deg = in_degree.get_mut(&neighbor).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push(neighbor);
                    }
                }
            }
        }

        // Assign buffer indices
        // Buffer 0 is reserved for final output
        let mut next_buffer = 1usize;
        let mut node_output_buffers: HashMap<NodeId, usize> = HashMap::new();

        // Create snapshot nodes in sorted order
        let mut snapshot_nodes: Vec<SnapshotNode> = Vec::new();
        let mut output_buffer_idx: Option<usize> = None;

        for node_id in &sorted_ids {
            let graph_node = graph.get_node(*node_id).unwrap();

            // Clone the node instance
            // For now, we create a new instance and copy params
            // This is a simplification - ideally we'd have Clone or serialize
            let mut instance = graph.registry_create(&graph_node.node_type)
                .ok_or_else(|| crate::GraphError::NodeTypeNotFound(graph_node.node_type.clone()))?;

            // Apply stored params
            for (&param, &value) in &graph_node.params {
                instance.set_param(param, value);
            }

            // Reset with sample rate
            instance.reset(self.sample_rate);

            // Determine input buffers from connections
            let mut input_buffers: Vec<usize> = Vec::new();
            for conn in graph.connections() {
                if conn.dst_node == *node_id {
                    if let Some(&src_buf) = node_output_buffers.get(&conn.src_node) {
                        input_buffers.push(src_buf);
                    }
                }
            }
            // If no inputs, use a dummy buffer
            if input_buffers.is_empty() {
                input_buffers.push(next_buffer);
                next_buffer += 1;
            }

            // Assign output buffer
            let output_buffer = if graph_node.node_type == "Out" {
                // Out node writes to buffer 0
                output_buffer_idx = Some(0);
                0
            } else {
                let buf = next_buffer;
                next_buffer += 1;
                buf
            };

            node_output_buffers.insert(*node_id, output_buffer);

            snapshot_nodes.push(SnapshotNode {
                id: *node_id,
                node: instance,
                node_type: graph_node.node_type.clone(),
                bypassed: graph_node.bypassed,
                input_buffers,
                output_buffers: vec![output_buffer],
            });
        }

        Ok(GraphSnapshot {
            nodes: snapshot_nodes,
            buffer_count: next_buffer.max(64), // Ensure minimum buffer count
            sample_rate: self.sample_rate,
            output_buffer: output_buffer_idx,
        })
    }
}
