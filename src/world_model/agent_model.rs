// src/world_model/agent_model.rs
// Phase 4 — World Model
// Zone B — Cognitive
//
// Purpose:
//   AgentModel represents SPS agents within the world model.
//   It tracks agent identity, capabilities (as a bundle), and
//   operational status. It reads from WorldState and produces Events
//   — never mutates state directly.
//
// Constitution Compliance:
//   - المادة الخامسة عشرة (World Model Constitution)
//   - المادة التاسعة (Agent Creation Law)
//   - Zone B: reads State, produces Events

use crate::canonical_state::world_state::EntityId;
use crate::kernel_core::event::EventPayload;

/// Operational status of an agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentStatus {
    Initializing,
    Running,
    Paused,
    Failed { reason: String },
    Terminated,
}

/// A capability that an agent provides.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentCapability {
    pub capability_id: String,
    pub description: String,
}

/// Model of an agent in the world.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentModel {
    pub agent_id: EntityId,
    pub name: String,
    pub status: AgentStatus,
    pub capabilities: Vec<AgentCapability>,
    pub zone: String, // "B" or "C" per Constitution
}

pub struct AgentModelManager;

impl AgentModelManager {
    /// Proposes registering a new agent.
    pub fn propose_register(
        agent_id: EntityId,
        name: String,
        capabilities: Vec<AgentCapability>,
        zone: String,
    ) -> EventPayload {
        let caps_str = capabilities
            .iter()
            .map(|c| format!("{}:{}", c.capability_id, c.description))
            .collect::<Vec<_>>()
            .join(",");
        EventPayload::Custom {
            event_type: "AgentModelRegistered".into(),
            data: format!("{}|{}|{}|{}", agent_id.0, name, caps_str, zone)
                .into_bytes(),
        }
    }

    /// Proposes updating agent status.
    pub fn propose_update_status(
        agent_id: EntityId,
        status: AgentStatus,
    ) -> EventPayload {
        EventPayload::Custom {
            event_type: "AgentModelStatusUpdated".into(),
            data: format!("{}|{:?}", agent_id.0, status).into_bytes(),
        }
    }

    /// Proposes adding a capability to an agent.
    pub fn propose_add_capability(
        agent_id: EntityId,
        capability: AgentCapability,
    ) -> EventPayload {
        EventPayload::Custom {
            event_type: "AgentModelCapabilityAdded".into(),
            data: format!(
                "{}|{}:{}",
                agent_id.0, capability.capability_id, capability.description
            )
            .into_bytes(),
        }
    }

    /// Proposes removing an agent from the world model.
    pub fn propose_remove(agent_id: EntityId) -> EventPayload {
        EventPayload::Custom {
            event_type: "AgentModelRemoved".into(),
            data: agent_id.0.into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_register_creates_event() {
        let payload = AgentModelManager::propose_register(
            EntityId("agent.1".into()),
            "builder".into(),
            vec![AgentCapability {
                capability_id: "cap.build".into(),
                description: "Builds software".into(),
            }],
            "B".into(),
        );
        match payload {
            EventPayload::Custom { event_type, data } => {
                assert!(event_type.contains("AgentModelRegistered"));
                let s = String::from_utf8(data).unwrap();
                assert!(s.contains("builder"));
            }
            _ => panic!("Wrong payload"),
        }
    }

    #[test]
    fn propose_update_status_creates_event() {
        let payload = AgentModelManager::propose_update_status(
            EntityId("agent.1".into()),
            AgentStatus::Failed {
                reason: "timeout".into(),
            },
        );
        match payload {
            EventPayload::Custom { event_type, data } => {
                assert!(event_type.contains("AgentModelStatusUpdated"));
                let s = String::from_utf8(data).unwrap();
                assert!(s.contains("timeout"));
            }
            _ => panic!("Wrong payload"),
        }
    }
}
