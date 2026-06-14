// src/world_model/tool_model.rs
// Phase 4 — World Model
// Zone B — Cognitive
//
// Purpose:
//   ToolModel represents external tools and utilities that SPS
//   can invoke. Tools are capability providers that operate in
//   Zone C. This model tracks tool identity, availability, and
//   invocation interface. It reads from WorldState and produces Events.

use crate::canonical_state::world_state::EntityId;
use crate::kernel_core::event::EventPayload;

/// The type of interface a tool exposes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolInterface {
    CLI { command: String },
    HTTP { endpoint: String },
    Library { function: String },
    Syscall { number: u64 },
}

/// Model of an external tool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolModel {
    pub tool_id: EntityId,
    pub name: String,
    pub description: String,
    pub interface: ToolInterface,
    pub available: bool,
}

pub struct ToolModelManager;

impl ToolModelManager {
    /// Proposes registering a new tool.
    pub fn propose_register(
        tool_id: EntityId,
        name: String,
        description: String,
        interface: ToolInterface,
    ) -> EventPayload {
        let iface_str = match &interface {
            ToolInterface::CLI { command } => format!("CLI:{}", command),
            ToolInterface::HTTP { endpoint } => format!("HTTP:{}", endpoint),
            ToolInterface::Library { function } => format!("Lib:{}", function),
            ToolInterface::Syscall { number } => format!("Syscall:{}", number),
        };
        EventPayload::Custom {
            event_type: "ToolModelRegistered".into(),
            data: format!("{}|{}|{}|{}", tool_id.0, name, description, iface_str)
                .into_bytes(),
        }
    }

    /// Proposes updating tool availability.
    pub fn propose_update_availability(
        tool_id: EntityId,
        available: bool,
    ) -> EventPayload {
        EventPayload::Custom {
            event_type: "ToolModelAvailabilityUpdated".into(),
            data: format!("{}|{}", tool_id.0, available).into_bytes(),
        }
    }

    /// Proposes removing a tool from the world model.
    pub fn propose_remove(tool_id: EntityId) -> EventPayload {
        EventPayload::Custom {
            event_type: "ToolModelRemoved".into(),
            data: tool_id.0.into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_register_creates_event() {
        let payload = ToolModelManager::propose_register(
            EntityId("tool.1".into()),
            "git".into(),
            "Version control".into(),
            ToolInterface::CLI { command: "git".into() },
        );
        match payload {
            EventPayload::Custom { event_type, data } => {
                assert!(event_type.contains("ToolModelRegistered"));
                let s = String::from_utf8(data).unwrap();
                assert!(s.contains("git"));
            }
            _ => panic!("Wrong payload"),
        }
    }
}
