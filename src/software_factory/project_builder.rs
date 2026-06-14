// src/software_factory/project_builder.rs
// Phase 11 — Software Factory

use crate::kernel_core::event::EventPayload;

pub struct ProjectBuilder;

impl ProjectBuilder {
    /// Proposes starting the full software factory pipeline.
    pub fn propose_build(project_id: String, requirements: Vec<String>) -> EventPayload {
        let reqs = requirements.join(";");
        EventPayload::Custom {
            event_type: "ProjectBuildStarted".into(),
            data: format!("{}|{}", project_id, reqs).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_event() {
        let p = ProjectBuilder::propose_build("p1".into(), vec!["fast".into(), "reliable".into()]);
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
