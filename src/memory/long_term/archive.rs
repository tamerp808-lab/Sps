// src/memory/long_term/archive.rs
// Phase 3 — Memory
// Zone B — Cognitive
//
// Purpose:
//   Archive is the long-term storage interface for memories that
//   have been consolidated from episodic and semantic layers.
//   It provides immutable, append-only storage with indexing.
//   All operations read from MemoryState and produce Events.

use crate::canonical_state::memory_state::MemoryState;
use crate::kernel_core::event::EventPayload;

pub struct Archive;

impl Archive {
    /// Proposes archiving an episodic memory.
    pub fn propose_archive_episode(episode_id: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "LongTermArchiveEpisode".into(),
            data: episode_id.into_bytes(),
        }
    }

    /// Proposes archiving a semantic fact.
    pub fn propose_archive_fact(fact_id: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "LongTermArchiveFact".into(),
            data: fact_id.into_bytes(),
        }
    }

    /// Proposes retrieving archived items by keyword.
    pub fn propose_retrieve(keyword: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "LongTermArchiveRetrieve".into(),
            data: keyword.into_bytes(),
        }
    }

    /// Returns count of archived items (from index in state).
    pub fn count(state: &MemoryState) -> usize {
        state.long_term_index.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_archive_episode_creates_event() {
        let payload = Archive::propose_archive_episode("ep.42".into());
        match payload {
            EventPayload::Custom { event_type, data } => {
                assert!(event_type.contains("ArchiveEpisode"));
                assert_eq!(data, b"ep.42");
            }
            _ => panic!("Wrong payload"),
        }
    }
}
