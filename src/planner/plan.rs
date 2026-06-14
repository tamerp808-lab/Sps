// src/planner/plan.rs
// Phase 7 — Planner
// Zone B — Cognitive

use crate::kernel_core::event::EventPayload;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanStatus { Draft, Validated, Executing, Completed, Failed }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepStatus { Pending, Ready, InProgress, Completed, Failed }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanStep {
    pub step_id: String,
    pub description: String,
    pub capability_id: Option<String>,
    pub estimated_cost: u64,
    pub dependencies: Vec<String>,
    pub status: StepStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    pub plan_id: String,
    pub goal_id: String,
    pub steps: Vec<PlanStep>,
    pub total_estimated_cost: u64,
    pub plan_status: PlanStatus,
}

pub struct PlanManager;

impl PlanManager {
    pub fn propose_create(
        plan_id: String, goal_id: String, steps: Vec<PlanStep>,
    ) -> EventPayload {
        let steps_str = steps.iter().map(|s| format!("{}:{}:{}:{}:{:?}", s.step_id, s.description, s.capability_id.as_deref().unwrap_or(""), s.estimated_cost, s.status)).collect::<Vec<_>>().join(";");
        EventPayload::Custom {
            event_type: "PlanCreated".into(),
            data: format!("{}|{}|{}", plan_id, goal_id, steps_str).into_bytes(),
        }
    }

    pub fn propose_update_status(plan_id: String, status: PlanStatus) -> EventPayload {
        EventPayload::Custom {
            event_type: "PlanStatusUpdated".into(),
            data: format!("{}|{:?}", plan_id, status).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_create_ok() {
        let steps = vec![PlanStep { step_id: "s1".into(), description: "test".into(), capability_id: None, estimated_cost: 5, dependencies: vec![], status: StepStatus::Pending }];
        let p = PlanManager::propose_create("p1".into(), "g1".into(), steps);
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
