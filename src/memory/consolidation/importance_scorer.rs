// src/memory/consolidation/importance_scorer.rs
// Phase 3 — Memory
// Zone B — Cognitive
//
// Purpose:
//   ImportanceScorer evaluates how important a memory is for
//   long-term retention. It considers factors like emotional
//   weight, repetition, and relevance to active goals. Produces
//   Events — pure read on state.

use crate::kernel_core::event::EventPayload;

pub struct ImportanceScorer;

impl ImportanceScorer {
    /// Scores an item based on repetition count and base relevance.
    pub fn score(repetition_count: u64, base_relevance: f64, is_goal_related: bool) -> f64 {
        let rep_bonus = (repetition_count as f64 * 0.1).min(0.5);
        let goal_bonus = if is_goal_related { 0.3 } else { 0.0 };
        (base_relevance + rep_bonus + goal_bonus).min(1.0)
    }

    /// Proposes recording an importance score event.
    pub fn propose_record(item_id: String, score: f64) -> EventPayload {
        EventPayload::Custom {
            event_type: "MemoryImportanceScored".into(),
            data: format!("{}|{}", item_id, score).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn goal_related_gets_bonus() {
        let s = ImportanceScorer::score(5, 0.4, true);
        assert!(s > 0.8);
    }

    #[test]
    fn max_score_capped_at_one() {
        let s = ImportanceScorer::score(10, 0.9, true);
        assert_eq!(s, 1.0);
    }
}
