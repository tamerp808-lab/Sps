// src/reasoning/abductive/explanation_ranker.rs
// Phase 5 — Reasoning
// Zone B — Cognitive

use crate::kernel_core::event::EventPayload;

pub struct ExplanationRanker;

impl ExplanationRanker {
    /// Ranks explanations by length (shorter = simpler = better).
    pub fn rank(explanations: &[String]) -> Vec<String> {
        let mut sorted = explanations.to_vec();
        sorted.sort_by_key(|e| e.len());
        sorted
    }

    pub fn propose_ranking(ranked: Vec<String>) -> EventPayload {
        let ranked_str = ranked.join(";");
        EventPayload::Custom {
            event_type: "AbductiveExplanationsRanked".into(),
            data: ranked_str.into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks_shortest_first() {
        let exps = vec!["a long explanation".into(), "short".into()];
        let ranked = ExplanationRanker::rank(&exps);
        assert_eq!(ranked[0], "short");
    }
}
