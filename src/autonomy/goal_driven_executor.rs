// src/autonomy/goal_driven_executor.rs
// Phase 12 — Autonomy

use crate::kernel_core::event::EventPayload;

pub struct GoalDrivenExecutor;

impl GoalDrivenExecutor {
    /// Proposes selecting the next goal autonomously.
    pub fn propose_select_goal(agent_id: String, goal_id: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "AutonomousGoalSelected".into(),
            data: format!("{}|{}", agent_id, goal_id).into_bytes(),
        }
    }

    /// Proposes executing the current goal's plan.
    pub fn propose_execute_goal(agent_id: String, plan_id: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "AutonomousGoalExecution".into(),
            data: format!("{}|{}", agent_id, plan_id).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_goal_event() {
        let p = GoalDrivenExecutor::propose_select_goal("agent.1".into(), "goal.2".into());
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
