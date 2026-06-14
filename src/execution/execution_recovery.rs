// src/execution/execution_recovery.rs
// Phase 8 — Execution
// Zone C — External

use crate::kernel_core::event::EventPayload;

pub struct ExecutionRecovery;

impl ExecutionRecovery {
    /// Decides whether to retry a failed step.
    pub fn should_retry(failure_count: u32, max_retries: u32) -> bool {
        failure_count < max_retries
    }

    /// Proposes retrying a failed step.
    pub fn propose_retry(execution_id: String, step_id: String, attempt: u32) -> EventPayload {
        EventPayload::Custom {
            event_type: "ExecutionRetry".into(),
            data: format!("{}|{}|{}", execution_id, step_id, attempt).into_bytes(),
        }
    }

    /// Proposes rolling back an execution.
    pub fn propose_rollback(execution_id: String, reason: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "ExecutionRollback".into(),
            data: format!("{}|{}", execution_id, reason).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_below_max() {
        assert!(ExecutionRecovery::should_retry(2, 3));
    }

    #[test]
    fn no_retry_at_max() {
        assert!(!ExecutionRecovery::should_retry(3, 3));
    }
}
