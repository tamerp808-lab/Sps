// src/memory/consolidation/consolidator.rs
// Phase 3 — Memory
// Zone B — Cognitive
//
// Purpose:
//   Consolidator moves memories from working/episodic into
//   long-term storage based on importance. It is the bridge
//   between short-lived and permanent memory. Produces Events
//   for each consolidation action.

use crate::kernel_core::event::EventPayload;

pub struct Consolidator;

impl Consolidator {
    /// Proposes consolidating an episode into long-term memory.
    pub fn propose_consolidate_episode(episode_id: String, importance: f64) -> EventPayload {
        EventPayload::Custom {
            event_type: "MemoryConsolidateEpisode".into(),
            data: format!("{}|{}", episode_id, importance).into_bytes(),
        }
    }

    /// Proposes consolidating a fact into semantic memory.
    pub fn propose_consolidate_fact(fact_id: String, confidence: f64) -> EventPayload {
        EventPayload::Custom {
            event_type: "MemoryConsolidateFact".into(),
            data: format!("{}|{}", fact_id, confidence).into_bytes(),
        }
    }

    /// Decides whether an episode is important enough to consolidate.
    pub fn should_consolidate(importance: f64, age_ticks: u64) -> bool {
        // Consolidate if importance is high or memory is recent and moderate
        importance > 0.7 || (importance > 0.4 && age_ticks < 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_importance_consolidates() {
        assert!(Consolidator::should_consolidate(0.8, 500));
    }

    #[test]
    fn low_importance_old_does_not() {
        assert!(!Consolidator::should_consolidate(0.3, 500));
    }
}
