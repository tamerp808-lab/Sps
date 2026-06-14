// src/memory/forgetting/pruning_policy.rs
// Phase 3 — Memory
// Zone B — Cognitive
//
// Purpose:
//   PruningPolicy decides which memories to remove permanently
//   when storage limits are reached. It uses importance, recency,
//   and access frequency to select victims. Produces Events.

use crate::kernel_core::event::EventPayload;

pub struct PruningPolicy;

impl PruningPolicy {
    /// Proposes pruning a specific memory item.
    pub fn propose_prune(item_id: String, reason: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "MemoryPruned".into(),
            data: format!("{}|{}", item_id, reason).into_bytes(),
        }
    }

    /// Selects which items to prune based on relevance threshold.
    pub fn select_prune_candidates(
        items: &[(String, f64, u64)], // (id, relevance, last_access_tick)
        current_tick: u64,
        min_relevance: f64,
        max_age: u64,
    ) -> Vec<String> {
        items
            .iter()
            .filter(|(_, rel, last_access)| {
                *rel < min_relevance || (current_tick - last_access) > max_age
            })
            .map(|(id, _, _)| id.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prune_low_relevance_items() {
        let items = vec![
            ("a".into(), 0.2, 100),
            ("b".into(), 0.9, 100),
        ];
        let candidates = PruningPolicy::select_prune_candidates(&items, 200, 0.5, 500);
        assert!(candidates.contains(&"a".into()));
        assert!(!candidates.contains(&"b".into()));
    }

    #[test]
    fn prune_old_items() {
        let items = vec![("a".into(), 0.8, 10)];
        let candidates = PruningPolicy::select_prune_candidates(&items, 200, 0.5, 50);
        assert!(candidates.contains(&"a".into()));
    }
}
