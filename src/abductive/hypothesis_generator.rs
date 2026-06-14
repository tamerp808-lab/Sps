// src/reasoning/abductive/hypothesis_generator.rs
// Phase 5 — Reasoning
// Zone B — Cognitive

use crate::kernel_core::event::EventPayload;

pub struct HypothesisGenerator;

impl HypothesisGenerator {
    /// Generates possible explanations for an observation.
    pub fn generate(observation: &str, knowledge_base: &[String]) -> Vec<String> {
        knowledge_base.iter()
            .filter(|fact| observation.contains(fact.as_str()))
            .cloned()
            .collect()
    }

    pub fn propose_hypothesis(hypothesis: String, confidence: f64) -> EventPayload {
        EventPayload::Custom {
            event_type: "AbductiveHypothesisGenerated".into(),
            data: format!("{}|{}", hypothesis, confidence).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_matching_hypotheses() {
        let kb = vec!["sps runs on linux".into(), "rust is safe".into()];
        let obs = "sps runs on linux today";
        let hyps = HypothesisGenerator::generate(obs, &kb);
        assert_eq!(hyps.len(), 1);
    }
}
