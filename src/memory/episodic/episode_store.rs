use crate::canonical_state::memory_state::{MemoryState, Episode as StateEpisode};
use crate::kernel_core::event::EventPayload;

pub struct EpisodeStore;

impl EpisodeStore {
    pub fn get_all(state: &MemoryState) -> &[StateEpisode] { &state.episodic }

    pub fn get_by_time(state: &MemoryState, from_tick: u64, to_tick: u64) -> Vec<&StateEpisode> {
        state.episodic.iter().filter(|ep| ep.timestamp_tick >= from_tick && ep.timestamp_tick <= to_tick).collect()
    }

    pub fn search<'a>(state: &'a MemoryState, keyword: &str) -> Vec<&'a StateEpisode> {
        state.episodic.iter().filter(|ep| ep.description.contains(keyword)).collect()
    }

    pub fn count(state: &MemoryState) -> usize { state.episodic.len() }

    pub fn propose_add(episode_id: String, timestamp_epoch: u64, timestamp_tick: u64, description: String) -> EventPayload {
        EventPayload::Custom { event_type: "EpisodeStoreAdd".into(), data: format!("{}|{}|{}|{}", episode_id, timestamp_epoch, timestamp_tick, description).into_bytes() }
    }

    pub fn propose_remove(episode_id: String) -> EventPayload {
        EventPayload::Custom { event_type: "EpisodeStoreRemove".into(), data: episode_id.into_bytes() }
    }
}
