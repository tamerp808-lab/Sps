// src/execution/executor.rs
// Phase 8 — Execution
// Zone C — External
//
// Purpose:
//   Executor is the engine that carries out plan steps by invoking
//   capability providers. It reads from PlannerState and produces
//   Events for each step — never mutates state directly.

use crate::kernel_core::event::EventPayload;

pub struct Executor;

impl Executor {
    /// Proposes starting the execution of a plan.
    pub fn propose_start(plan_id: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "ExecutionStarted".into(),
            data: plan_id.into_bytes(),
        }
    }

    /// Proposes executing a single step.
    pub fn propose_execute_step(plan_id: String, step_id: String, capability_id: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "ExecutionStep".into(),
            data: format!("{}|{}|{}", plan_id, step_id, capability_id).into_bytes(),
        }
    }

    /// Proposes completing an execution.
    pub fn propose_complete(execution_id: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "ExecutionCompleted".into(),
            data: execution_id.into_bytes(),
        }
    }

    /// Proposes failing an execution.
    pub fn propose_fail(execution_id: String, reason: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "ExecutionFailed".into(),
            data: format!("{}|{}", execution_id, reason).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_start_creates_event() {
        let payload = Executor::propose_start("plan.1".into());
        match payload {
            EventPayload::Custom { event_type, .. } => assert!(event_type.contains("ExecutionStarted")),
            _ => panic!("Wrong payload"),
        }
    }
}
