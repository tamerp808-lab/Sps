// src/self_improvement/proposal_generator.rs
// Phase 10 — Self-Improvement
// Zone B — Cognitive

use crate::kernel_core::event::EventPayload;

pub struct ProposalGenerator;

impl ProposalGenerator {
    /// Generates an improvement proposal from a lesson or insight.
    pub fn generate(description: String, target: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "SelfImprovementProposal".into(),
            data: format!("{}|{}", description, target).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_proposal_event() {
        let p = ProposalGenerator::generate("optimize memory".into(), "memory".into());
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
