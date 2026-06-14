// src/software_factory/maintenance.rs
// Phase 11 — Software Factory

use crate::kernel_core::event::EventPayload;

pub struct BugTracker;

impl BugTracker {
    pub fn propose_bug(project_id: String, description: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "BugReported".into(),
            data: format!("{}|{}", project_id, description).into_bytes(),
        }
    }
}

pub struct PerformanceMonitor;

impl PerformanceMonitor {
    pub fn propose_metric(project_id: String, metric: String, value: f64) -> EventPayload {
        EventPayload::Custom {
            event_type: "PerformanceMetric".into(),
            data: format!("{}|{}|{}", project_id, metric, value).into_bytes(),
        }
    }
}

pub struct RefactoringSuggester;

impl RefactoringSuggester {
    pub fn propose_refactor(project_id: String, target: String, reason: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "RefactoringSuggested".into(),
            data: format!("{}|{}|{}", project_id, target, reason).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bug_event() {
        let p = BugTracker::propose_bug("p1".into(), "null pointer".into());
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
