// src/software_factory/architecture.rs
// Phase 11 — Software Factory

use crate::kernel_core::event::EventPayload;

pub struct ArchitectureGenerator;

impl ArchitectureGenerator {
    /// Proposes an architecture pattern for a project.
    pub fn propose_pattern(project_id: String, pattern: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "ArchitecturePatternSelected".into(),
            data: format!("{}|{}", project_id, pattern).into_bytes(),
        }
    }

    /// Proposes technology choices.
    pub fn propose_technology(project_id: String, tech: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "ArchitectureTechnologySelected".into(),
            data: format!("{}|{}", project_id, tech).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_pattern_creates_event() {
        let p = ArchitectureGenerator::propose_pattern("proj.1".into(), "microservices".into());
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
