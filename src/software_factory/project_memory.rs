// src/software_factory/project_memory.rs
// Phase 11 — Software Factory

use crate::kernel_core::event::EventPayload;

pub struct DecisionLog;

impl DecisionLog {
    pub fn propose_decision(project_id: String, decision: String, rationale: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "DecisionLogged".into(),
            data: format!("{}|{}|{}", project_id, decision, rationale).into_bytes(),
        }
    }
}

pub struct LessonLedger;

impl LessonLedger {
    pub fn propose_lesson(project_id: String, lesson: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "LessonLearned".into(),
            data: format!("{}|{}", project_id, lesson).into_bytes(),
        }
    }
}

pub struct ProjectSnapshot;

impl ProjectSnapshot {
    pub fn propose_snapshot(project_id: String, status: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "ProjectSnapshot".into(),
            data: format!("{}|{}", project_id, status).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decision_log_event() {
        let p = DecisionLog::propose_decision("p1".into(), "use rust".into(), "safety".into());
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
