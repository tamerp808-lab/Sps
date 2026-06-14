// src/autonomy/autonomy_manager.rs
// Phase 12 — Autonomy
// Zone C — External

use crate::kernel_core::event::EventPayload;

pub struct AutonomyManager;

impl AutonomyManager {
    /// Proposes granting autonomy to an agent for a specific domain.
    pub fn propose_grant(agent_id: String, domain: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "AutonomyGranted".into(),
            data: format!("{}|{}", agent_id, domain).into_bytes(),
        }
    }

    /// Proposes revoking autonomy.
    pub fn propose_revoke(agent_id: String, reason: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "AutonomyRevoked".into(),
            data: format!("{}|{}", agent_id, reason).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grant_creates_event() {
        let p = AutonomyManager::propose_grant("agent.1".into(), "memory".into());
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
