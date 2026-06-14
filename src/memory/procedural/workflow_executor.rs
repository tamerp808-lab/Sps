// src/memory/procedural/workflow_executor.rs
// Phase 3 — Memory
// Zone B — Cognitive
//
// Purpose:
//   WorkflowExecutor is the engine that executes workflows, procedures,
//   and skills stored in procedural memory. It reads the current
//   MemoryState to retrieve the definition, then produces a sequence
//   of Events for each step that must be executed. It does NOT execute
//   steps directly — it delegates to the Execution layer (Phase 8)
//   by producing the appropriate events.
//
// Constitution Compliance:
//   - المادة الرابعة عشرة (Memory Constitution) — Procedural Memory
//   - Zone B: reads State, produces Events
//   - Execution delegation: produces events for Zone C

use crate::canonical_state::memory_state::MemoryState;
use crate::kernel_core::event::EventPayload;

/// The state of a running workflow execution.
#[derive(Debug, Clone)]
pub struct WorkflowExecutionState {
    pub execution_id: String,
    pub workflow_id: String,
    pub current_step_index: usize,
    pub total_steps: usize,
    pub status: ExecutionStatus,
    pub context: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionStatus {
    NotStarted,
    Running,
    Waiting,
    Completed,
    Failed { reason: String },
    Cancelled,
}

pub struct WorkflowExecutor;

impl WorkflowExecutor {
    /// Proposes starting the execution of a workflow.
    /// Returns events that the Execution layer will pick up.
    pub fn propose_start(
        execution_id: String,
        workflow_id: String,
        context: String,
    ) -> EventPayload {
        EventPayload::Custom {
            event_type: "WorkflowExecutionStarted".into(),
            data: format!("{}|{}|{}", execution_id, workflow_id, context).into_bytes(),
        }
    }

    /// Proposes advancing to the next step in a workflow.
    pub fn propose_next_step(
        execution_id: String,
        step_description: String,
        capability_id: String,
    ) -> EventPayload {
        EventPayload::Custom {
            event_type: "WorkflowStepExecutionRequested".into(),
            data: format!("{}|{}|{}", execution_id, step_description, capability_id)
                .into_bytes(),
        }
    }

    /// Proposes completing a workflow execution.
    pub fn propose_complete(execution_id: String, result: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "WorkflowExecutionCompleted".into(),
            data: format!("{}|{}", execution_id, result).into_bytes(),
        }
    }

    /// Proposes failing a workflow execution.
    pub fn propose_fail(execution_id: String, reason: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "WorkflowExecutionFailed".into(),
            data: format!("{}|{}", execution_id, reason).into_bytes(),
        }
    }

    /// Proposes retrying a failed step.
    pub fn propose_retry_step(
        execution_id: String,
        step_index: usize,
        attempt: u32,
    ) -> EventPayload {
        EventPayload::Custom {
            event_type: "WorkflowStepRetry".into(),
            data: format!("{}|{}|{}", execution_id, step_index, attempt).into_bytes(),
        }
    }

    /// Proposes skipping a failed step and continuing.
    pub fn propose_skip_step(execution_id: String, step_index: usize) -> EventPayload {
        EventPayload::Custom {
            event_type: "WorkflowStepSkipped".into(),
            data: format!("{}|{}", execution_id, step_index).into_bytes(),
        }
    }

    /// Returns the list of step descriptions for a given workflow.
    /// Pure read — no Events produced.
    pub fn get_workflow_steps(
        state: &MemoryState,
        workflow_name: &str,
    ) -> Vec<String> {
        // Search for a skill matching the workflow name
        for skill in state.procedural.values() {
            if skill.name.contains(workflow_name) {
                return skill.steps.clone();
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_start_creates_event() {
        let payload = WorkflowExecutor::propose_start(
            "exec.1".into(),
            "wf.load".into(),
            "file.txt".into(),
        );
        match payload {
            EventPayload::Custom { event_type, data } => {
                assert!(event_type.contains("WorkflowExecutionStarted"));
                let s = String::from_utf8(data).unwrap();
                assert!(s.contains("exec.1"));
            }
            _ => panic!("Wrong payload"),
        }
    }

    #[test]
    fn propose_next_step_creates_event() {
        let payload = WorkflowExecutor::propose_next_step(
            "exec.1".into(),
            "Read file".into(),
            "read_file".into(),
        );
        match payload {
            EventPayload::Custom { event_type, .. } => {
                assert!(event_type.contains("WorkflowStepExecutionRequested"));
            }
            _ => panic!("Wrong payload"),
        }
    }

    #[test]
    fn propose_complete_creates_event() {
        let payload = WorkflowExecutor::propose_complete("exec.1".into(), "success".into());
        match payload {
            EventPayload::Custom { event_type, .. } => {
                assert!(event_type.contains("WorkflowExecutionCompleted"));
            }
            _ => panic!("Wrong payload"),
        }
    }

    #[test]
    fn propose_fail_creates_event() {
        let payload = WorkflowExecutor::propose_fail("exec.1".into(), "timeout".into());
        match payload {
            EventPayload::Custom { event_type, .. } => {
                assert!(event_type.contains("WorkflowExecutionFailed"));
            }
            _ => panic!("Wrong payload"),
        }
    }

    #[test]
    fn get_workflow_steps_returns_steps() {
        let mut state = MemoryState::empty();
        state.procedural.insert(
            "s1".into(),
            crate::canonical_state::memory_state::Skill {
                skill_id: "s1".into(),
                name: "read file".into(),
                steps: vec!["open".into(), "read".into(), "close".into()],
            },
        );
        let steps = WorkflowExecutor::get_workflow_steps(&state, "read file");
        assert_eq!(steps.len(), 3);
        assert_eq!(steps[0], "open");
    }
}
