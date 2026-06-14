// src/world_model/project_model.rs
// Phase 4 — World Model
// Zone B — Cognitive
//
// Purpose:
//   ProjectModel represents software projects SPS is working on.
//   It tracks project metadata, structure, dependencies, and status.
//   It reads from WorldState and produces Events — no direct mutation.

use crate::canonical_state::world_state::EntityId;
use crate::kernel_core::event::EventPayload;

/// Status of a project under SPS management.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectStatus {
    Draft,
    Planned,
    InProgress,
    Review,
    Completed,
    Abandoned,
}

/// A software component within a project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectComponent {
    pub name: String,
    pub component_type: String,
    pub status: String,
}

/// Model of a project in the world.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectModel {
    pub project_id: EntityId,
    pub name: String,
    pub description: String,
    pub status: ProjectStatus,
    pub components: Vec<ProjectComponent>,
    pub dependencies: Vec<String>,
}

pub struct ProjectModelManager;

impl ProjectModelManager {
    /// Proposes creating a new project.
    pub fn propose_create(
        project_id: EntityId,
        name: String,
        description: String,
    ) -> EventPayload {
        EventPayload::Custom {
            event_type: "ProjectModelCreated".into(),
            data: format!("{}|{}|{}", project_id.0, name, description).into_bytes(),
        }
    }

    /// Proposes updating project status.
    pub fn propose_update_status(
        project_id: EntityId,
        status: ProjectStatus,
    ) -> EventPayload {
        EventPayload::Custom {
            event_type: "ProjectModelStatusUpdated".into(),
            data: format!("{}|{:?}", project_id.0, status).into_bytes(),
        }
    }

    /// Proposes adding a component to a project.
    pub fn propose_add_component(
        project_id: EntityId,
        component: ProjectComponent,
    ) -> EventPayload {
        EventPayload::Custom {
            event_type: "ProjectModelComponentAdded".into(),
            data: format!(
                "{}|{}|{}|{}",
                project_id.0, component.name, component.component_type, component.status
            )
            .into_bytes(),
        }
    }

    /// Proposes recording a dependency between projects.
    pub fn propose_add_dependency(
        project_id: EntityId,
        dependency_project_id: String,
    ) -> EventPayload {
        EventPayload::Custom {
            event_type: "ProjectModelDependencyAdded".into(),
            data: format!("{}|{}", project_id.0, dependency_project_id).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_create_creates_event() {
        let payload = ProjectModelManager::propose_create(
            EntityId("project.1".into()),
            "sps".into(),
            "Sovereign Processing System".into(),
        );
        match payload {
            EventPayload::Custom { event_type, data } => {
                assert!(event_type.contains("ProjectModelCreated"));
                let s = String::from_utf8(data).unwrap();
                assert!(s.contains("Sovereign"));
            }
            _ => panic!("Wrong payload"),
        }
    }

    #[test]
    fn propose_update_status_creates_event() {
        let payload = ProjectModelManager::propose_update_status(
            EntityId("project.1".into()),
            ProjectStatus::InProgress,
        );
        match payload {
            EventPayload::Custom { event_type, data } => {
                assert!(event_type.contains("ProjectModelStatusUpdated"));
                let s = String::from_utf8(data).unwrap();
                assert!(s.contains("InProgress"));
            }
            _ => panic!("Wrong payload"),
        }
    }
}
