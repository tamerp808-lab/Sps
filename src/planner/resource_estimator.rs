// src/planner/resource_estimator.rs
// Phase 7 — Planner
// Zone B — Cognitive

use crate::kernel_core::event::EventPayload;

pub struct ResourceEstimator;

impl ResourceEstimator {
    /// Estimates total resources needed for a plan based on steps.
    pub fn estimate(step_count: usize, base_per_step: u64) -> u64 {
        step_count as u64 * base_per_step
    }

    pub fn propose_estimate(plan_id: String, total_cost: u64) -> EventPayload {
        EventPayload::Custom {
            event_type: "ResourceEstimate".into(),
            data: format!("{}|{}", plan_id, total_cost).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estimates_correctly() {
        assert_eq!(ResourceEstimator::estimate(5, 10), 50);
    }
}
