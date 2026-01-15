//! Node traits for DSP processing

use crate::{AudioBusMut, AudioBusRef, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

/// DSP node trait for audio processing
///
/// CRITICAL: Implementations must not allocate, lock, or perform I/O
pub trait DspNode: Send {
    /// Reset node state (called when sample rate changes)
    fn reset(&mut self, sample_rate: f32);

    /// Process one block of audio
    ///
    /// # Safety
    /// Must not allocate, lock, or perform I/O
    fn process(&mut self, ctx: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]);

    /// Set parameter value
    fn set_param(&mut self, param: ParamIndex, value: ParamValue);

    /// Handle incoming events
    fn handle_events(&mut self, ctx: &ProcessCtx, events: &mut EventQueue);

    /// Called after processing (for updating meters, etc.)
    fn post_process(&mut self) {}
}

/// Node metadata descriptor
pub trait NodeDescriptor: Send + Sync {
    /// Node type name (must be unique)
    fn type_name(&self) -> &'static str;

    /// Port specifications
    fn ports(&self) -> &'static [PortSpec];

    /// Parameter specifications
    fn params(&self) -> &'static [ParamSpec];

    /// Display name
    fn display_name(&self) -> &'static str {
        self.type_name()
    }

    /// Category for organization
    fn category(&self) -> &'static str {
        "Uncategorized"
    }
}

/// Combined trait for nodes with metadata
pub trait Node: DspNode + NodeDescriptor + Send + Sync {}

impl<T: DspNode + NodeDescriptor + Send + Sync> Node for T {}

/// Node factory for creating node instances
pub type NodeFactory = fn() -> Box<dyn Node>;

/// Node registry for looking up node types
pub struct NodeRegistry {
    factories: Vec<(String, NodeFactory)>,
}

impl NodeRegistry {
    pub fn new() -> Self {
        Self {
            factories: Vec::new(),
        }
    }

    pub fn register(&mut self, type_name: impl Into<String>, factory: NodeFactory) {
        self.factories.push((type_name.into(), factory));
    }

    pub fn create(&self, type_name: &str) -> Option<Box<dyn Node>> {
        self.factories
            .iter()
            .find(|(name, _)| name == type_name)
            .map(|(_, factory)| factory())
    }

    pub fn types(&self) -> impl Iterator<Item = &str> {
        self.factories.iter().map(|(name, _)| name.as_str())
    }

    pub fn has_type(&self, type_name: &str) -> bool {
        self.factories.iter().any(|(name, _)| name == type_name)
    }
}

impl Default for NodeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper macro to implement NodeDescriptor
#[macro_export]
macro_rules! impl_node_descriptor {
    ($type:ty, $name:expr, $ports:expr, $params:expr) => {
        impl $crate::NodeDescriptor for $type {
            fn type_name(&self) -> &'static str {
                $name
            }

            fn ports(&self) -> &'static [PortSpec] {
                $ports
            }

            fn params(&self) -> &'static [ParamSpec] {
                $params
            }
        }
    };
}
