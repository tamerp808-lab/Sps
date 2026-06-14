// src/software_factory/implementation.rs
// Phase 11 — Software Factory

use crate::kernel_core::event::EventPayload;

pub struct BackendGenerator;

impl BackendGenerator {
    pub fn propose_generate(project_id: String, language: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "BackendGenerated".into(),
            data: format!("{}|{}", project_id, language).into_bytes(),
        }
    }
}

pub struct CodeValidator;

impl CodeValidator {
    pub fn validate(code: &str) -> bool {
        code.contains("fn") || code.contains("class") || code.contains("def")
    }

    pub fn propose_validation(project_id: String, valid: bool) -> EventPayload {
        EventPayload::Custom {
            event_type: "CodeValidated".into(),
            data: format!("{}|{}", project_id, valid).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_code_passes() {
        assert!(CodeValidator::validate("fn main() {}"));
    }

    #[test]
    fn invalid_code_fails() {
        assert!(!CodeValidator::validate("gibberish"));
    }
}
