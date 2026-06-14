// src/memory/procedural/procedure.rs
// Phase 3 — Memory
// Zone B — Cognitive
//
// Purpose:
//   Procedure represents a sequence of steps that form a workflow
//   or a compound skill. Unlike a single Skill, a Procedure may
//   branch, loop, or invoke sub-skills. It is stored in procedural
//   memory and executed by the workflow executor. All operations
//   produce Events — no direct state mutation.
//
// Constitution Compliance:
//   - المادة الرابعة عشرة (Memory Constitution) — Procedural Memory
//   - Zone B: reads State, produces Events

use crate::kernel_core::event::EventPayload;

/// A single step within a procedure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcedureStep {
    /// Execute a skill by ID.
    ExecuteSkill { skill_id: String },
    /// Execute a sub-procedure by ID.
    ExecuteProcedure { procedure_id: String },
    /// Wait for a condition to be met.
    Wait { condition: String, timeout_ticks: u64 },
    /// Conditional branch.
    Branch { condition: String, true_step: u32, false_step: u32 },
    /// Repeat a range of steps.
    Loop { start_step: u32, end_step: u32, max_iterations: u32 },
    /// End the procedure.
    End,
}

/// A procedure stored in procedural memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Procedure {
    pub procedure_id: String,
    pub name: String,
    pub description: String,
    pub steps: Vec<ProcedureStep>,
    pub version: u32,
}

pub struct ProcedureManager;

impl ProcedureManager {
    /// Proposes registering a new procedure.
    pub fn propose_register(
        procedure_id: String,
        name: String,
        description: String,
        steps: Vec<ProcedureStep>,
    ) -> EventPayload {
        let steps_str = steps
            .iter()
            .enumerate()
            .map(|(i, s)| format!("{}:{:?}", i, s))
            .collect::<Vec<_>>()
            .join(";");
        EventPayload::Custom {
            event_type: "ProceduralProcedureRegistered".into(),
            data: format!(
                "{}|{}|{}|{}|1",
                procedure_id, name, description, steps_str
            )
            .into_bytes(),
        }
    }

    /// Proposes updating a procedure (creates a new version).
    pub fn propose_update(
        procedure_id: String,
        new_steps: Vec<ProcedureStep>,
        new_version: u32,
    ) -> EventPayload {
        let steps_str = new_steps
            .iter()
            .enumerate()
            .map(|(i, s)| format!("{}:{:?}", i, s))
            .collect::<Vec<_>>()
            .join(";");
        EventPayload::Custom {
            event_type: "ProceduralProcedureUpdated".into(),
            data: format!("{}|{}|{}", procedure_id, steps_str, new_version).into_bytes(),
        }
    }

    /// Proposes removing a procedure.
    pub fn propose_remove(procedure_id: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "ProceduralProcedureRemoved".into(),
            data: procedure_id.into_bytes(),
        }
    }

    /// Proposes executing a procedure.
    pub fn propose_execute(procedure_id: String, context: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "ProceduralProcedureExecute".into(),
            data: format!("{}|{}", procedure_id, context).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_register_creates_event() {
        let steps = vec![
            ProcedureStep::ExecuteSkill { skill_id: "read_file".into() },
            ProcedureStep::Branch {
                condition: "file.exists".into(),
                true_step: 2,
                false_step: 4,
            },
            ProcedureStep::End,
        ];
        let payload = ProcedureManager::propose_register(
            "proc.1".into(),
            "Load Config".into(),
            "Loads configuration from file".into(),
            steps,
        );
        match payload {
            EventPayload::Custom { event_type, data } => {
                assert!(event_type.contains("ProcedureRegistered"));
                let s = String::from_utf8(data).unwrap();
                assert!(s.contains("Load Config"));
            }
            _ => panic!("Wrong payload"),
        }
    }

    #[test]
    fn propose_update_creates_event() {
        let new_steps = vec![ProcedureStep::End];
        let payload = ProcedureManager::propose_update("proc.1".into(), new_steps, 2);
        match payload {
            EventPayload::Custom { event_type, data } => {
                assert!(event_type.contains("ProcedureUpdated"));
                let s = String::from_utf8(data).unwrap();
                assert!(s.contains("2")); // version
            }
            _ => panic!("Wrong payload"),
        }
    }

    #[test]
    fn propose_execute_creates_event() {
        let payload = ProcedureManager::propose_execute("proc.1".into(), "ctx".into());
        match payload {
            EventPayload::Custom { event_type, .. } => {
                assert!(event_type.contains("ProcedureExecute"));
            }
            _ => panic!("Wrong payload"),
        }
    }
}
