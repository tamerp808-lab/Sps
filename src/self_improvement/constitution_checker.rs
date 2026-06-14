// src/self_improvement/constitution_checker.rs
// Phase 10 — Self-Improvement

use crate::kernel_core::event::EventPayload;

pub struct SelfImprovementConstitutionChecker;

impl SelfImprovementConstitutionChecker {
    pub fn is_allowed(proposal: &str) -> bool {
        !proposal.contains("ZoneA")
    }

    pub fn propose_check(proposal_id: String, allowed: bool) -> EventPayload {
        EventPayload::Custom {
            event_type: "ConstitutionCheck".into(),
            data: format!("{}|{}", proposal_id, allowed).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zone_a_change() {
        assert!(!SelfImprovementConstitutionChecker::is_allowed("Modify ZoneA reducer"));
    }

    #[test]
    fn allows_zone_b_change() {
        assert!(SelfImprovementConstitutionChecker::is_allowed("Add memory heuristic"));
    }
}
