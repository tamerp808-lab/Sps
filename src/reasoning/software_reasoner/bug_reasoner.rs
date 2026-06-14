use crate::kernel_core::event::EventPayload;

pub struct BugReasoner;
impl BugReasoner {
    pub fn suspect_recent_change(bug: &str, changes: &[String]) -> Vec<String> {
        changes.iter().filter(|c| bug.contains(c.as_str())).cloned().collect()
    }
    pub fn propose_suspect(bug: String, suspects: Vec<String>) -> EventPayload {
        EventPayload::Custom { event_type: "BugSuspectsFound".into(), data: format!("{}|{}", bug, suspects.join(",")).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn finds() {
        // تغيير موجود كجزء من نص الخطأ
        let suspects = BugReasoner::suspect_recent_change("memory leak in refactor", &["refactor".into(), "ui".into()]);
        assert!(suspects.contains(&"refactor".into()));
    }
}
