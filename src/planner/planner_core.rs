use crate::canonical_state::goal_state::{Goal, ValuePriority};
use crate::goal_system::goal_evaluator::GoalEvaluator;
use crate::kernel_core::event::EventPayload;
use crate::planner::plan::{Plan, PlanStep, StepStatus, PlanStatus};

pub struct PlannerCore;

impl PlannerCore {
    pub fn create_plan(plan_id: String, goal: &Goal, capabilities: &[(String, String)], values: &ValuePriority) -> Option<Plan> {
        if !GoalEvaluator::is_safe(goal, values) { return None; }
        let steps: Vec<PlanStep> = capabilities.iter().enumerate().map(|(i, (cap_id, desc))| {
            PlanStep {
                step_id: format!("{}.{}", plan_id, i + 1),
                description: desc.clone(),
                capability_id: Some(cap_id.clone()),
                estimated_cost: 10,
                dependencies: if i > 0 { vec![format!("{}.{}", plan_id, i)] } else { vec![] },
                status: StepStatus::Pending,
            }
        }).collect();
        let total = steps.iter().map(|s| s.estimated_cost).sum();
        Some(Plan { plan_id, goal_id: goal.goal_id.clone(), steps, total_estimated_cost: total, plan_status: PlanStatus::Draft })
    }

    pub fn propose_generate_plan(goal_id: String, capabilities: Vec<(String, String)>) -> EventPayload {
        let caps_str = capabilities.iter().map(|(id, desc)| format!("{}:{}", id, desc)).collect::<Vec<_>>().join(",");
        EventPayload::Custom { event_type: "PlanGenerated".into(), data: format!("{}|{}", goal_id, caps_str).into_bytes() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical_state::goal_state::GoalStatus;

    #[test]
    fn safe_plan_is_created() {
        let values = ValuePriority::default();
        let goal = Goal { goal_id: "g1".into(), description: "test".into(), priority: 5, supporting_values: vec!["Truth".into()], status: GoalStatus::Active, parent_goal_id: None };
        let caps = vec![("read".into(), "read file".into())];
        let plan = PlannerCore::create_plan("p1".into(), &goal, &caps, &values);
        assert!(plan.is_some());
    }

    #[test]
    fn unsafe_plan_is_rejected() {
        let values = ValuePriority::default();
        let goal = Goal { goal_id: "g2".into(), description: "bad".into(), priority: 5, supporting_values: vec!["not Safety".into()], status: GoalStatus::Active, parent_goal_id: None };
        let caps = vec![("delete".into(), "delete files".into())];
        let plan = PlannerCore::create_plan("p2".into(), &goal, &caps, &values);
        assert!(plan.is_none());
    }
}
