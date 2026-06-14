// src/reasoning/causal/causal_reasoner.rs
// Phase 5 — Reasoning
// Zone B — Cognitive

use crate::kernel_core::event::EventPayload;

pub struct CausalReasoner;

impl CausalReasoner {
    /// Infers a possible cause if effect matches a known pattern.
    pub fn infer_cause(effect: &str, known_causes: &[(String, String)]) -> Option<String> {
        known_causes.iter()
            .find(|(_, eff)| eff == effect)
            .map(|(cause, _)| cause.clone())
    }

    pub fn propose_causal_link(cause: String, effect: String, confidence: f64) -> EventPayload {
        EventPayload::Custom {
            event_type: "CausalLinkInferred".into(),
            data: format!("{}|{}|{}", cause, effect, confidence).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infers_cause_from_known_pair() {
        let pairs = vec![("rain".into(), "wet ground".into())];
        assert_eq!(CausalReasoner::infer_cause("wet ground", &pairs), Some("rain".into()));
    }
}
