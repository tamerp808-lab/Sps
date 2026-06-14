use crate::kernel_core::event::EventPayload;

pub struct BugReasoner;

impl BugReasoner {
    /// Simple heuristic: a bug may be caused by a recent change.
    pub fn suspect_recent_change(bug_description: &str, recent_changes: &[String]) -> Vec<String> {
        recent_changes.iter()
            .filter(|change| bug_description.contains(change.as_str()))
            .cloned()
            .collect()
    }

    pub fn propose_suspect(bug: String, suspects: Vec<String>) -> EventPayload {
        let suspects_str = suspects.join(",");
        EventPayload::Custom {
            event_type: "BugSuspectsFound".into(),
            data: format!("{}|{}", bug, suspects_str).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_suspect_change() {
        let changes = vec!["refactor memory".into(), "add feature X".into()];
        let suspects = BugReasoner::suspect_recent_change("memory leak in refactor", &changes);
        assert!(suspects.contains(&"refactor memory".into()));
    }
}
