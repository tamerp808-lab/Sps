use crate::kernel_core::event::EventPayload;

pub struct HypothesisGenerator;
impl HypothesisGenerator {
    pub fn generate(obs: &str, kb: &[String]) -> Vec<String> { kb.iter().filter(|f| obs.contains(f.as_str())).cloned().collect() }
    pub fn propose_hypothesis(hyp: String, confidence: f64) -> EventPayload {
        EventPayload::Custom { event_type: "AbductiveHypothesisGenerated".into(), data: format!("{}|{}", hyp, confidence).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn generates() { assert_eq!(HypothesisGenerator::generate("sky is blue", &["sky is blue".into()]).len(), 1); }
}
