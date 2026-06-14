// src/self_improvement/evolution_sandbox.rs
// Phase 10 — Self-Improvement

use crate::kernel_core::event::EventPayload;

pub struct EvolutionSandbox;

impl EvolutionSandbox {
    pub fn run_test(proposal_id: &str) -> bool {
        proposal_id.len() % 2 == 0 // dummy
    }

    pub fn propose_sandbox_result(proposal_id: String, passed: bool) -> EventPayload {
        EventPayload::Custom {
            event_type: "SandboxTestResult".into(),
            data: format!("{}|{}", proposal_id, passed).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sandbox_runs() {
        let result = EvolutionSandbox::run_test("prop1");
        assert!(!result); // length 5 -> odd -> false
    }
}
