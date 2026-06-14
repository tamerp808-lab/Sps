// src/autonomy/proactive_planner.rs
// Phase 12 — Autonomy

use crate::kernel_core::event::EventPayload;

pub struct ProactivePlanner;

impl ProactivePlanner {
    /// Proposes generating a plan proactively (not in response to a user).
    pub fn propose_proactive_plan(agent_id: String, description: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "ProactivePlanGenerated".into(),
            data: format!("{}|{}", agent_id, description).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proactive_plan_event() {
        let p = ProactivePlanner::propose_proactive_plan("agent.1".into(), "optimize memory usage".into());
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
