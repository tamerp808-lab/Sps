use crate::canonical_state::goal_state::{Goal, ValuePriority};
use crate::kernel_core::event::EventPayload;

pub struct GoalEvaluator;

impl GoalEvaluator {
    pub fn score(goal: &Goal, values: &ValuePriority) -> f64 {
        let mut score = goal.priority as f64;
        for cv in &values.core_order {
            let cv_str = format!("{:?}", cv);
            if goal.supporting_values.contains(&cv_str) { score += 50.0; }
        }
        for ov in &values.operational_order {
            let ov_str = format!("{:?}", ov);
            if goal.supporting_values.contains(&ov_str) { score += 10.0; }
        }
        score
    }

    pub fn is_safe(goal: &Goal, _values: &ValuePriority) -> bool {
        let forbidden = ["not Truth", "not Safety", "not ConstitutionCompliance"];
        !goal.supporting_values.iter().any(|v| forbidden.contains(&v.as_str()))
    }

    pub fn propose_evaluation(goal_id: String, score: f64, safe: bool) -> EventPayload {
        EventPayload::Custom {
            event_type: "GoalEvaluated".into(),
            data: format!("{}|{}|{}", goal_id, score, safe).into_bytes(),
        }
    }
}
