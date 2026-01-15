//! Graph model for the world thread

use chordworld_core::{ConnectionId, IdGenerator, NodeId, ParamIndex, ParamValue};
use chordworld_dsp::{Node, NodeRegistry};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GraphError {
    #[error("Node not found: {0}")]
    NodeNotFound(NodeId),

    #[error("Node type not found: {0}")]
    NodeTypeNotFound(String),

    #[error("Port not found: {node}:{port}")]
    PortNotFound { node: NodeId, port: String },

    #[error("Invalid connection: {0}")]
    InvalidConnection(String),

    #[error("Cycle detected without sufficient delay")]
    CycleDetected,

    #[error("Node limit exceeded")]
    NodeLimitExceeded,

    #[error("Connection limit exceeded")]
    ConnectionLimitExceeded,
}

/// Connection between ports
#[derive(Debug, Clone)]
pub struct Connection {
    pub id: ConnectionId,
    pub src_node: NodeId,
    pub src_port: String,
    pub dst_node: NodeId,
    pub dst_port: String,
    pub src_channel: Option<u32>,
    pub dst_channel: Option<u32>,
}

/// Node instance in the graph
pub struct GraphNode {
    pub id: NodeId,
    pub name: String,
    pub node_type: String,
    pub instance: Box<dyn Node>,
    pub bypassed: bool,
    pub params: HashMap<ParamIndex, ParamValue>,
}

impl GraphNode {
    pub fn new(id: NodeId, name: String, node_type: String, instance: Box<dyn Node>) -> Self {
        Self {
            id,
            name,
            node_type,
            instance,
            bypassed: false,
            params: HashMap::new(),
        }
    }

    pub fn set_param(&mut self, param: ParamIndex, value: ParamValue) {
        self.params.insert(param, value);
        self.instance.set_param(param, value);
    }
}

/// Graph model (world thread)
pub struct GraphModel {
    nodes: HashMap<NodeId, GraphNode>,
    connections: HashMap<ConnectionId, Connection>,
    node_names: HashMap<String, NodeId>,
    id_gen: IdGenerator,
    registry: NodeRegistry,
}

impl GraphModel {
    pub fn new(registry: NodeRegistry) -> Self {
        Self {
            nodes: HashMap::new(),
            connections: HashMap::new(),
            node_names: HashMap::new(),
            id_gen: IdGenerator::new(),
            registry,
        }
    }

    pub fn add_node(&mut self, node_type: &str, name: Option<String>) -> Result<NodeId, GraphError> {
        let instance = self
            .registry
            .create(node_type)
            .ok_or_else(|| GraphError::NodeTypeNotFound(node_type.to_string()))?;

        let id = self.id_gen.next_node();
        let name = name.unwrap_or_else(|| format!("{}_{}", node_type, id.0));

        let node = GraphNode::new(id, name.clone(), node_type.to_string(), instance);

        self.node_names.insert(name, id);
        self.nodes.insert(id, node);

        Ok(id)
    }

    pub fn remove_node(&mut self, id: NodeId) -> Result<(), GraphError> {
        let node = self
            .nodes
            .remove(&id)
            .ok_or(GraphError::NodeNotFound(id))?;

        self.node_names.remove(&node.name);

        // Remove all connections involving this node
        self.connections
            .retain(|_, conn| conn.src_node != id && conn.dst_node != id);

        Ok(())
    }

    pub fn connect(
        &mut self,
        src_node: NodeId,
        src_port: &str,
        dst_node: NodeId,
        dst_port: &str,
    ) -> Result<ConnectionId, GraphError> {
        // Validate nodes exist
        if !self.nodes.contains_key(&src_node) {
            return Err(GraphError::NodeNotFound(src_node));
        }
        if !self.nodes.contains_key(&dst_node) {
            return Err(GraphError::NodeNotFound(dst_node));
        }

        let id = self.id_gen.next_connection();
        let connection = Connection {
            id,
            src_node,
            src_port: src_port.to_string(),
            dst_node,
            dst_port: dst_port.to_string(),
            src_channel: None,
            dst_channel: None,
        };

        self.connections.insert(id, connection);

        Ok(id)
    }

    pub fn disconnect(&mut self, id: ConnectionId) -> Result<(), GraphError> {
        self.connections.remove(&id);
        Ok(())
    }

    pub fn set_param(
        &mut self,
        node_id: NodeId,
        param: ParamIndex,
        value: ParamValue,
    ) -> Result<(), GraphError> {
        let node = self
            .nodes
            .get_mut(&node_id)
            .ok_or(GraphError::NodeNotFound(node_id))?;

        node.set_param(param, value);
        Ok(())
    }

    pub fn get_node(&self, id: NodeId) -> Option<&GraphNode> {
        self.nodes.get(&id)
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut GraphNode> {
        self.nodes.get_mut(&id)
    }

    pub fn get_node_by_name(&self, name: &str) -> Option<&GraphNode> {
        self.node_names.get(name).and_then(|id| self.nodes.get(id))
    }

    pub fn find_node_id(&self, name_or_id: &str) -> Option<NodeId> {
        // Try parsing as ID first
        if let Ok(id) = name_or_id.parse::<u64>() {
            let node_id = NodeId(id);
            if self.nodes.contains_key(&node_id) {
                return Some(node_id);
            }
        }

        // Try as name
        self.node_names.get(name_or_id).copied()
    }

    pub fn nodes(&self) -> impl Iterator<Item = &GraphNode> {
        self.nodes.values()
    }

    pub fn connections(&self) -> impl Iterator<Item = &Connection> {
        self.connections.values()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    pub fn validate(&self) -> Result<Vec<String>, GraphError> {
        let mut warnings = Vec::new();

        // Check for cycles (simplified - just warn for now)
        // Full cycle detection with delay checking would go here

        // Check for disconnected nodes
        let mut connected_nodes = std::collections::HashSet::new();
        for conn in self.connections.values() {
            connected_nodes.insert(conn.src_node);
            connected_nodes.insert(conn.dst_node);
        }

        for node in self.nodes.values() {
            if !connected_nodes.contains(&node.id) && self.nodes.len() > 1 {
                warnings.push(format!("Node {} ({}) is disconnected", node.name, node.id));
            }
        }

        Ok(warnings)
    }
}
