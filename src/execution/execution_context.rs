// src/execution/execution_context.rs
// Phase 8 — Execution
// Zone C — External
//
// Purpose:
//   ExecutionContext holds the transient state of a running execution:
//   which plan, which step, current status, and resource usage.
//   It is NOT part of CanonicalState — it's Zone C scratch space
//   that produces Events to update the canonical ExecutionState.

use crate::kernel_core::event::EventPayload;

#[derive(Debug, Clone)]
pub enum ExecutionStatus { Running, Waiting, Completed, Failed }

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub execution_id: String,
    pub plan_id: String,
    pub current_step_id: String,
    pub status: ExecutionStatus,
    pub resources_used: u64,
}

pub struct ExecutionContextManager;

impl ExecutionContextManager {
    /// Proposes creating a new execution context.
    pub fn propose_create(plan_id: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "ExecutionContextCreated".into(),
            data: plan_id.into_bytes(),
        }
    }

    /// Proposes advancing to the next step.
    pub fn propose_advance(execution_id: String, next_step_id: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "ExecutionContextAdvanced".into(),
            data: format!("{}|{}", execution_id, next_step_id).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_create_ok() {
        let p = ExecutionContextManager::propose_create("p1".into());
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
