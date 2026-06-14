// src/memory/episodic/episode_replay.rs
// Phase 3 — Memory
// Zone B — Cognitive
//
// Purpose:
//   EpisodeReplay enables SPS to re-experience past episodes.
//   It produces Events that trigger the retrieval and re-simulation
//   of a recorded experience, allowing the system to learn from
//   past events or revisit contexts. It never modifies state directly.
//
// Constitution Compliance:
//   - المادة الرابعة عشرة (Memory Constitution) — Episodic Memory
//   - Zone B: reads State, produces Events
//   - Replay is deterministic: same episode → same event sequence

use crate::canonical_state::memory_state::MemoryState;
use crate::kernel_core::event::EventPayload;

/// The episode replay engine.
pub struct EpisodeReplay;

impl EpisodeReplay {
    /// Proposes replaying a single episode by ID.
    pub fn propose_replay_episode(episode_id: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "EpisodeReplaySingle".into(),
            data: episode_id.into_bytes(),
        }
    }

    /// Proposes replaying a range of episodes by tick range.
    pub fn propose_replay_range(from_tick: u64, to_tick: u64) -> EventPayload {
        EventPayload::Custom {
            event_type: "EpisodeReplayRange".into(),
            data: format!("{}|{}", from_tick, to_tick).into_bytes(),
        }
    }

    /// Proposes replaying episodes involving a specific entity.
    pub fn propose_replay_by_entity(entity: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "EpisodeReplayByEntity".into(),
            data: entity.into_bytes(),
        }
    }

    /// Proposes replaying episodes that contain a specific keyword.
    pub fn propose_replay_by_keyword(keyword: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "EpisodeReplayByKeyword".into(),
            data: keyword.into_bytes(),
        }
    }

    /// Returns the list of episode IDs that would be replayed
    /// for a given range. Pure read operation — no Events.
    pub fn preview_range(state: &MemoryState, from_tick: u64, to_tick: u64) -> Vec<String> {
        state
            .episodic
            .iter()
            .filter(|ep| {
                ep.timestamp_tick >= from_tick && ep.timestamp_tick <= to_tick
            })
            .map(|ep| ep.episode_id.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical_state::memory_state::Episode;

    #[test]
    fn propose_replay_single_creates_event() {
        let payload = EpisodeReplay::propose_replay_episode("ep.5".into());
        match payload {
            EventPayload::Custom { event_type, data } => {
                assert!(event_type.contains("ReplaySingle"));
                assert_eq!(data, b"ep.5");
            }
            _ => panic!("Wrong payload"),
        }
    }

    #[test]
    fn propose_replay_range_creates_event() {
        let payload = EpisodeReplay::propose_replay_range(10, 20);
        match payload {
            EventPayload::Custom { event_type, data } => {
                assert!(event_type.contains("ReplayRange"));
                let s = String::from_utf8(data).unwrap();
                assert!(s.contains("10|20"));
            }
            _ => panic!("Wrong payload"),
        }
    }

    #[test]
    fn preview_range_returns_matching_ids() {
        let mut state = MemoryState::empty();
        state.episodic.push(Episode {
            episode_id: "ep1".into(),
            timestamp_epoch: 0,
            timestamp_tick: 5,
            description: "a".into(),
        });
        state.episodic.push(Episode {
            episode_id: "ep2".into(),
            timestamp_epoch: 0,
            timestamp_tick: 15,
            description: "b".into(),
        });
        let ids = EpisodeReplay::preview_range(&state, 10, 20);
        assert_eq!(ids, vec!["ep2"]);
    }
}
