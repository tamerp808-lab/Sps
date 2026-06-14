// src/software_factory/security_review.rs
// Phase 11 — Software Factory

use crate::kernel_core::event::EventPayload;

pub struct VulnerabilityScanner;

impl VulnerabilityScanner {
    pub fn scan(project_id: String, vulnerabilities_found: u64) -> EventPayload {
        EventPayload::Custom {
            event_type: "VulnerabilityScanCompleted".into(),
            data: format!("{}|{}", project_id, vulnerabilities_found).into_bytes(),
        }
    }
}

pub struct DependencyAuditor;

impl DependencyAuditor {
    pub fn audit(project_id: String, outdated_deps: Vec<String>) -> EventPayload {
        let deps = outdated_deps.join(",");
        EventPayload::Custom {
            event_type: "DependencyAuditCompleted".into(),
            data: format!("{}|{}", project_id, deps).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_event() {
        let p = VulnerabilityScanner::scan("proj".into(), 0);
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
