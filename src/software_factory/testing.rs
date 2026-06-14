// src/software_factory/testing.rs
// Phase 11 — Software Factory

use crate::kernel_core::event::EventPayload;

pub struct TestGenerator;

impl TestGenerator {
    pub fn propose_generate_tests(project_id: String, target_module: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "TestsGenerated".into(),
            data: format!("{}|{}", project_id, target_module).into_bytes(),
        }
    }
}

pub struct TestRunner;

impl TestRunner {
    pub fn run(passed: bool) -> EventPayload {
        EventPayload::Custom {
            event_type: "TestsRun".into(),
            data: format!("{}", passed).into_bytes(),
        }
    }
}

pub struct CoverageAnalyzer;

impl CoverageAnalyzer {
    pub fn analyze(coverage_pct: f64) -> EventPayload {
        EventPayload::Custom {
            event_type: "CoverageAnalyzed".into(),
            data: format!("{}", coverage_pct).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_event() {
        let p = TestRunner::run(true);
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
