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
            // 21e8 POW commands
            "pow.mine" => self.parse_pow_mine(&parts[1..]),
            "pow.pool.clear" => Ok(Transaction::EntropyPoolClear),
            // Tuning commands
            "tuning.set" => self.parse_tuning_set(&parts[1..]),
            "tuning.random" => self.parse_tuning_random(&parts[1..]),
            "tuning.clear" => Ok(Transaction::TuningClear),
            "tuning.show" => Ok(Transaction::TuningShow),
            // Quick setups
            "setup.basic" => Ok(Transaction::SetupBasic),
            "setup.fm" => Ok(Transaction::SetupFM),
            "setup.pad" => Ok(Transaction::SetupPad),
            "setup.drums" => Ok(Transaction::SetupDrums),
            // Pathology commands
            "path" | "pathology" => self.parse_pathology(&parts[1..]),
            "path.list" => Ok(Transaction::PathologyList),
            "help" => Err(anyhow!("Commands: node.add, connect, param.set, setup.*, pow.mine, tuning.*, path.*")),
            _ => Err(anyhow!("Unknown command: {}", parts[0])),
        }
    }

    fn parse_pathology(&self, args: &[&str]) -> Result<Transaction> {
        if args.is_empty() {
            return Err(anyhow!("Usage: path <name> [param=value]\n\
                Available: irrational-meter, golden-polyrhythm, event-horizon, pitch-drift,\n\
                anchorfree, subliminal, chaos-groove, saboteur, phantom-voices, trap, unmeasurable"));
        }

        let name = args[0].to_string();

        // Parse optional parameters
        let mut params: Vec<(String, f64)> = Vec::new();
        for arg in &args[1..] {
            if let Some(eq_pos) = arg.find('=') {
                let key = arg[..eq_pos].to_string();
                if let Ok(val) = arg[eq_pos + 1..].parse::<f64>() {
                    params.push((key, val));
                }
            }
        }

        Ok(Transaction::PathologyApply { name, params })
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

    fn parse_pow_mine(&self, args: &[&str]) -> Result<Transaction> {
        if args.is_empty() {
            return Err(anyhow!("Usage: pow.mine <nonce_seed>"));
        }

        let nonce_seed = args.join(" ");
        Ok(Transaction::PowMine { nonce_seed })
    }

    fn parse_tuning_set(&self, args: &[&str]) -> Result<Transaction> {
        if args.is_empty() {
            return Err(anyhow!("Usage: tuning.set <hash_hex>"));
        }

        let hash_hex = args[0].to_string();
        Ok(Transaction::TuningSetFromHash { hash_hex })
    }

    fn parse_tuning_random(&self, args: &[&str]) -> Result<Transaction> {
        let seed = if args.is_empty() {
            // Use current time as seed if none provided
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(42)
        } else {
            args[0].parse::<u64>().unwrap_or(42)
        };
        Ok(Transaction::TuningSetRandom { seed })
    }
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}
