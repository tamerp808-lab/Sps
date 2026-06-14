use crate::canonical_state::memory_state::MemoryState;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct EpisodeIndex {
    pub by_entity: BTreeMap<String, Vec<String>>,
    pub by_keyword: BTreeMap<String, Vec<String>>,
}

impl EpisodeIndex {
    pub fn build(state: &MemoryState) -> Self {
        let by_entity = BTreeMap::new();
        let mut by_keyword = BTreeMap::new();
        for ep in &state.episodic {
            for word in ep.description.split_whitespace() {
                let w = word.to_lowercase();
                by_keyword.entry(w).or_insert_with(Vec::new).push(ep.episode_id.clone());
            }
        }
        EpisodeIndex { by_entity, by_keyword }
    }

    pub fn find_by_keyword(&self, keyword: &str) -> Vec<String> {
        self.by_keyword.get(&keyword.to_lowercase()).cloned().unwrap_or_default()
    }

    pub fn find_by_entity(&self, entity: &str) -> Vec<String> {
        self.by_entity.get(entity).cloned().unwrap_or_default()
    }
}
