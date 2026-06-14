// src/memory/procedural/workflow.rs
// Phase 3 — Memory
// Zone B — Cognitive
//
// Purpose:
//   Workflow is a higher-level orchestration unit that chains
//   multiple procedures and skills together to accomplish a
//   complex goal. It supports sequential and parallel execution,
//   error handling, and resource-aware scheduling. All operations
//   read from MemoryState and produce Events.
//
// Constitution Compliance:
//   - المادة الرابعة عشرة (Memory Constitution) — Procedural Memory
//   - Zone B: reads State, produces Events

use crate::kernel_core::event::EventPayload;

/// A workflow step — either a procedure or a skill.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowStep {
    Procedure { procedure_id: String },
    Skill { skill_id: String },
    Parallel { steps: Vec<WorkflowStep> },
    Conditional { condition: String, then_step: Box<WorkflowStep>, else_step: Option<Box<WorkflowStep>> },
}

/// A workflow definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workflow {
    pub workflow_id: String,
    pub name: String,
    pub description: String,
    pub steps: Vec<WorkflowStep>,
    pub on_error: ErrorBehavior,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorBehavior {
    Abort,
    Retry { max_retries: u32 },
    SkipAndContinue,
    Fallback { fallback_workflow_id: String },
}

pub struct WorkflowManager;

impl WorkflowManager {
    /// Proposes defining a new workflow.
    pub fn propose_define(
        workflow_id: String,
        name: String,
        description: String,
        steps: Vec<WorkflowStep>,
        on_error: ErrorBehavior,
    ) -> EventPayload {
        let steps_str = steps
            .iter()
            .enumerate()
            .map(|(i, s)| format!("{}:{:?}", i, s))
            .collect::<Vec<_>>()
            .join(";");
        EventPayload::Custom {
            event_type: "ProceduralWorkflowDefined".into(),
            data: format!(
                "{}|{}|{}|{}|{:?}",
                workflow_id, name, description, steps_str, on_error
            )
            .into_bytes(),
        }
    }

    /// Proposes executing a workflow.
    pub fn propose_execute(workflow_id: String, context: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "ProceduralWorkflowExecute".into(),
            data: format!("{}|{}", workflow_id, context).into_bytes(),
        }
    }

    /// Proposes cancelling a running workflow.
    pub fn propose_cancel(workflow_id: String, reason: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "ProceduralWorkflowCancelled".into(),
            data: format!("{}|{}", workflow_id, reason).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_define_creates_event() {
        let steps = vec![
            WorkflowStep::Procedure { procedure_id: "proc.load".into() },
            WorkflowStep::Skill { skill_id: "skill.parse".into() },
        ];
        let payload = WorkflowManager::propose_define(
            "wf.1".into(),
            "Load and Parse".into(),
            "Loads a file and parses it".into(),
            steps,
            ErrorBehavior::Retry { max_retries: 3 },
        );
        match payload {
            EventPayload::Custom { event_type, data } => {
                assert!(event_type.contains("WorkflowDefined"));
                let s = String::from_utf8(data).unwrap();
                assert!(s.contains("Load and Parse"));
            }
            _ => panic!("Wrong payload"),
        }
    }

    #[test]
    fn propose_execute_creates_event() {
        let payload = WorkflowManager::propose_execute("wf.1".into(), "file.txt".into());
        match payload {
            EventPayload::Custom { event_type, .. } => {
                assert!(event_type.contains("WorkflowExecute"));
            }
            _ => panic!("Wrong payload"),
        }
    }
}
