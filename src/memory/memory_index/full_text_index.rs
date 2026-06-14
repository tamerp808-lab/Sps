// src/memory/memory_index/full_text_index.rs
// Phase 3 — Memory
// Zone B — Cognitive
//
// Purpose:
//   FullTextIndex provides keyword-based search across all memory
//   layers. It builds an inverted index from MemoryState and supports
//   fast lookups. All operations are pure reads or produce Events.

use crate::canonical_state::memory_state::MemoryState;
use crate::kernel_core::event::EventPayload;
use std::collections::BTreeMap;

pub struct FullTextIndex;

impl FullTextIndex {
    /// Builds an inverted index from memory state.
    pub fn build(state: &MemoryState) -> BTreeMap<String, Vec<String>> {
        let mut index: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (id, fact) in &state.semantic {
            for word in format!("{} {} {}", fact.subject, fact.predicate, fact.object).split_whitespace() {
                index.entry(word.to_lowercase()).or_default().push(id.clone());
            }
        }
        for ep in &state.episodic {
            for word in ep.description.split_whitespace() {
                index.entry(word.to_lowercase()).or_default().push(ep.episode_id.clone());
            }
        }
        index
    }

    /// Searches the index for a keyword.
    pub fn search(index: &BTreeMap<String, Vec<String>>, keyword: &str) -> Vec<String> {
        index.get(&keyword.to_lowercase()).cloned().unwrap_or_default()
    }

    /// Proposes rebuilding the index (event for index refresh).
    pub fn propose_rebuild() -> EventPayload {
        EventPayload::Custom {
            event_type: "FullTextIndexRebuilt".into(),
            data: vec![].into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical_state::memory_state::{SemanticFact, Episode};
    use ordered_float::OrderedFloat;

    #[test]
    fn build_and_search() {
        let mut state = MemoryState::empty();
        state.semantic.insert("f1".into(), SemanticFact {
            fact_id: "f1".into(), subject: "sps".into(), predicate: "is".into(), object: "fast".into(), confidence: OrderedFloat(1.0),
        });
        state.episodic.push(Episode {
            episode_id: "ep1".into(), timestamp_epoch: 0, timestamp_tick: 1, description: "sps started".into(),
        });
        let index = FullTextIndex::build(&state);
        let results = FullTextIndex::search(&index, "sps");
        assert_eq!(results.len(), 2);
    }
}
