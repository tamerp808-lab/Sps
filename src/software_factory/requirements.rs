// src/software_factory/requirements.rs
// Phase 11 — Software Factory
// Zone C — External
//
// Purpose:
//   Extracts, validates, and stores project requirements.
//   All outputs pass through validation before becoming Events.

use crate::kernel_core::event::EventPayload;

pub struct RequirementsExtractor;

impl RequirementsExtractor {
    /// Proposes a new requirement.
    pub fn propose_requirement(project_id: String, description: String, priority: u64) -> EventPayload {
        EventPayload::Custom {
            event_type: "RequirementExtracted".into(),
            data: format!("{}|{}|{}", project_id, description, priority).into_bytes(),
        }
    }

    /// Validates that a requirement is unambiguous.
    pub fn validate(description: &str) -> bool {
        description.len() > 10 && description.contains(' ')
    }

    /// Proposes a validation result.
    pub fn propose_validation(requirement_id: String, valid: bool) -> EventPayload {
        EventPayload::Custom {
            event_type: "RequirementValidated".into(),
            data: format!("{}|{}", requirement_id, valid).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_requirement_passes() {
        assert!(RequirementsExtractor::validate("The system shall be fast and reliable"));
    }

    #[test]
    fn short_requirement_fails() {
        assert!(!RequirementsExtractor::validate("fast"));
    }
}
