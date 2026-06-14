use crate::kernel_core::event::EventPayload;

pub struct ConsistencyChecker;
impl ConsistencyChecker {
    pub fn is_consistent(new: &str, existing: &[String]) -> bool {
        !existing.iter().any(|e| (e.contains("not") && new.contains(&e.replace("not ", ""))) || (new.contains("not") && e.contains(&new.replace("not ", ""))))
    }
    pub fn propose_check(new_fact: String, consistent: bool) -> EventPayload {
        EventPayload::Custom { event_type: "ConsistencyChecked".into(), data: format!("{}|{}", new_fact, consistent).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn consistent() { assert!(ConsistencyChecker::is_consistent("X is Y", &["A is B".into()])); }
    #[test] fn contradiction() { assert!(!ConsistencyChecker::is_consistent("X is Y", &["X is not Y".into()])); }
}
