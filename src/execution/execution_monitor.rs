// src/execution/execution_monitor.rs
// Phase 8 — Execution
// Zone C — External

use crate::kernel_core::event::EventPayload;

pub struct ExecutionMonitor;

impl ExecutionMonitor {
    /// Checks if an execution has exceeded its resource budget.
    pub fn check_budget(used: u64, budget: u64) -> bool {
        used <= budget
    }

    /// Proposes alerting on budget overrun.
    pub fn propose_budget_overrun(execution_id: String, used: u64, budget: u64) -> EventPayload {
        EventPayload::Custom {
            event_type: "ExecutionBudgetOverrun".into(),
            data: format!("{}|{}|{}", execution_id, used, budget).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn under_budget_passes() {
        assert!(ExecutionMonitor::check_budget(50, 100));
    }

    #[test]
    fn over_budget_fails() {
        assert!(!ExecutionMonitor::check_budget(150, 100));
    }
}
