// src/memory/memory_index/temporal_index.rs
// Phase 3 — Memory
// Zone B — Cognitive
//
// Purpose:
//   TemporalIndex provides time-based lookups into memory. It allows
//   querying by logical time ranges and retrieving events/episodes
//   in chronological order. All operations are pure reads or produce Events.

use crate::canonical_state::memory_state::MemoryState;
use crate::kernel_core::event::EventPayload;

pub struct TemporalIndex;

impl TemporalIndex {
    /// Returns episode IDs within a tick range.
    pub fn query_range(state: &MemoryState, from_tick: u64, to_tick: u64) -> Vec<String> {
        state.episodic.iter()
            .filter(|ep| ep.timestamp_tick >= from_tick && ep.timestamp_tick <= to_tick)
            .map(|ep| ep.episode_id.clone())
            .collect()
    }

    /// Proposes recording a temporal query.
    pub fn propose_query(from_tick: u64, to_tick: u64) -> EventPayload {
        EventPayload::Custom {
            event_type: "TemporalIndexQuery".into(),
            data: format!("{}|{}", from_tick, to_tick).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical_state::memory_state::Episode;

    #[test]
    fn query_range_finds_episodes() {
        let mut state = MemoryState::empty();
        state.episodic.push(Episode { episode_id: "e1".into(), timestamp_epoch: 0, timestamp_tick: 10, description: "a".into() });
        state.episodic.push(Episode { episode_id: "e2".into(), timestamp_epoch: 0, timestamp_tick: 30, description: "b".into() });
        let ids = TemporalIndex::query_range(&state, 5, 20);
        assert_eq!(ids, vec!["e1"]);
    }
}
