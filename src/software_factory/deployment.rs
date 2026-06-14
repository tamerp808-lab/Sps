// src/software_factory/deployment.rs
// Phase 11 — Software Factory

use crate::kernel_core::event::EventPayload;

pub struct DeploymentGenerator;

impl DeploymentGenerator {
    pub fn propose_deploy(project_id: String, target: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "DeploymentGenerated".into(),
            data: format!("{}|{}", project_id, target).into_bytes(),
        }
    }
}

pub struct EnvironmentConfigurator;

impl EnvironmentConfigurator {
    pub fn propose_configure(project_id: String, env: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "EnvironmentConfigured".into(),
            data: format!("{}|{}", project_id, env).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deploy_event() {
        let p = DeploymentGenerator::propose_deploy("p1".into(), "linux".into());
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
