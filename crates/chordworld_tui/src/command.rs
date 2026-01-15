//! Command parser for TUI commands

use chordworld_core::{ApplyPoint, NodeId, ParamIndex, ParamValue, PortEndpoint, Transaction, TransportState};
use anyhow::{anyhow, Result};

pub struct CommandParser;

impl CommandParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, input: &str) -> Result<Transaction> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Err(anyhow!("Empty command"));
        }

        match parts[0] {
            "node.add" => self.parse_node_add(&parts[1..]),
            "node.rm" | "node.remove" => self.parse_node_remove(&parts[1..]),
            "connect" => self.parse_connect(&parts[1..]),
            "param.set" => self.parse_param_set(&parts[1..]),
            "play" => Ok(Transaction::TransportSet {
                state: TransportState::Playing,
                apply: ApplyPoint::Now,
            }),
            "stop" => Ok(Transaction::TransportSet {
                state: TransportState::Stopped,
                apply: ApplyPoint::Now,
            }),
            "tempo" => self.parse_tempo(&parts[1..]),
            "help" => Err(anyhow!("Help: node.add <type> [name], connect <src> <dst>, param.set <node>.<param> <value>, play, stop, tempo <bpm>")),
            _ => Err(anyhow!("Unknown command: {}", parts[0])),
        }
    }

    fn parse_node_add(&self, args: &[&str]) -> Result<Transaction> {
        if args.is_empty() {
            return Err(anyhow!("Usage: node.add <type> [name]"));
        }

        let node_type = args[0].to_string();
        let name = args.get(1).map(|s| s.to_string());

        Ok(Transaction::NodeAdd { node_type, name })
    }

    fn parse_node_remove(&self, args: &[&str]) -> Result<Transaction> {
        if args.is_empty() {
            return Err(anyhow!("Usage: node.rm <node_id>"));
        }

        let node_id = args[0].parse::<u64>()?;
        Ok(Transaction::NodeRemove {
            node: NodeId(node_id),
        })
    }

    fn parse_connect(&self, args: &[&str]) -> Result<Transaction> {
        if args.len() < 2 {
            return Err(anyhow!("Usage: connect <src_node>:<src_port> <dst_node>:<dst_port>"));
        }

        let src = self.parse_endpoint(args[0])?;
        let dst = self.parse_endpoint(args[1])?;

        Ok(Transaction::Connect {
            src,
            dst,
            map: chordworld_core::ConnectionMap::Direct,
        })
    }

    fn parse_endpoint(&self, s: &str) -> Result<PortEndpoint> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid endpoint format: {}", s));
        }

        let node_id = parts[0].parse::<u64>()?;
        let port_name = parts[1].to_string();

        Ok(PortEndpoint::new(NodeId(node_id), port_name))
    }

    fn parse_param_set(&self, args: &[&str]) -> Result<Transaction> {
        if args.len() < 2 {
            return Err(anyhow!("Usage: param.set <node>.<param> <value>"));
        }

        let target = args[0];
        let value_str = args[1];

        let parts: Vec<&str> = target.split('.').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid target format: {}", target));
        }

        let node_id = parts[0].parse::<u64>()?;
        let param_idx = parts[1].parse::<u32>()?;
        let value = value_str.parse::<f32>()?;

        Ok(Transaction::ParamSet {
            node: NodeId(node_id),
            param: ParamIndex(param_idx),
            value: ParamValue::Float(value),
        })
    }

    fn parse_tempo(&self, args: &[&str]) -> Result<Transaction> {
        if args.is_empty() {
            return Err(anyhow!("Usage: tempo <bpm>"));
        }

        let bpm = args[0].parse::<f32>()?;
        Ok(Transaction::SetTempo { bpm })
    }
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}
