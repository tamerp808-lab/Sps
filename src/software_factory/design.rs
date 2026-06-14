// src/software_factory/design.rs
// Phase 11 — Software Factory

use crate::kernel_core::event::EventPayload;

pub struct ApiDesigner;

impl ApiDesigner {
    pub fn propose_api(project_id: String, endpoint: String, method: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "ApiDesigned".into(),
            data: format!("{}|{}|{}", project_id, endpoint, method).into_bytes(),
        }
    }
}

pub struct DatabaseDesigner;

impl DatabaseDesigner {
    pub fn propose_schema(project_id: String, table: String, columns: Vec<String>) -> EventPayload {
        let cols = columns.join(",");
        EventPayload::Custom {
            event_type: "DatabaseSchemaDesigned".into(),
            data: format!("{}|{}|{}", project_id, table, cols).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_api_ok() {
        let p = ApiDesigner::propose_api("p1".into(), "/users".into(), "GET".into());
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
