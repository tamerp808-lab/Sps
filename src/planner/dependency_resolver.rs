// src/planner/dependency_resolver.rs
// Phase 7 — Planner
// Zone B — Cognitive

use crate::kernel_core::event::EventPayload;
use crate::planner::plan::{Plan, PlanStep};

pub struct DependencyResolver;

impl DependencyResolver {
    /// Topologically sorts steps by dependencies.
    pub fn resolve(plan: &Plan) -> Vec<String> {
        let mut sorted = Vec::new();
        let mut remaining: Vec<&PlanStep> = plan.steps.iter().collect();
        while !remaining.is_empty() {
            let ready: Vec<usize> = remaining.iter().enumerate().filter(|(_, s)| s.dependencies.iter().all(|d| sorted.contains(d))).map(|(i, _)| i).collect();
            if ready.is_empty() { break; } // cycle
            for i in ready.into_iter().rev() {
                sorted.push(remaining[i].step_id.clone());
                remaining.remove(i);
            }
        }
        sorted
    }

    pub fn propose_resolution(plan_id: String, order: Vec<String>) -> EventPayload {
        EventPayload::Custom {
            event_type: "DependencyResolved".into(),
            data: format!("{}|{}", plan_id, order.join(",")).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planner::plan::{PlanStep, StepStatus, PlanStatus};

    #[test]
    fn resolves_order() {
        let steps = vec![
            PlanStep { step_id: "s1".into(), description: "a".into(), capability_id: None, estimated_cost: 1, dependencies: vec![], status: StepStatus::Pending },
            PlanStep { step_id: "s2".into(), description: "b".into(), capability_id: None, estimated_cost: 1, dependencies: vec!["s1".into()], status: StepStatus::Pending },
        ];
        let plan = Plan { plan_id: "p".into(), goal_id: "g".into(), steps, total_estimated_cost: 2, plan_status: PlanStatus::Draft };
        let order = DependencyResolver::resolve(&plan);
        assert_eq!(order, vec!["s1", "s2"]);
    }
}
