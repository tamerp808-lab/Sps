use crate::kernel_core::event::EventPayload;
use crate::planner::plan::Plan;

pub struct PlanValidator;

impl PlanValidator {
    pub fn validate(plan: &Plan) -> bool {
        let step_ids: Vec<&str> = plan.steps.iter().map(|s| s.step_id.as_str()).collect();
        for step in &plan.steps {
            for dep in &step.dependencies {
                if !step_ids.contains(&dep.as_str()) { return false; }
            }
        }
        true
    }

    pub fn propose_validation(plan_id: String, valid: bool) -> EventPayload {
        EventPayload::Custom { event_type: "PlanValidated".into(), data: format!("{}|{}", plan_id, valid).into_bytes() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planner::plan::{PlanStep, StepStatus, PlanStatus};

    #[test]
    fn valid_plan_passes() {
        let steps = vec![
            PlanStep { step_id: "1".into(), description: "a".into(), capability_id: None, estimated_cost: 1, dependencies: vec![], status: StepStatus::Pending },
            PlanStep { step_id: "2".into(), description: "b".into(), capability_id: None, estimated_cost: 1, dependencies: vec!["1".into()], status: StepStatus::Pending },
        ];
        let plan = Plan { plan_id: "p".into(), goal_id: "g".into(), steps, total_estimated_cost: 2, plan_status: PlanStatus::Draft };
        assert!(PlanValidator::validate(&plan));
    }
}
