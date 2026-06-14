use crate::canonical_state::memory_state::{MemoryState, Skill as StoredSkill};
use crate::kernel_core::event::EventPayload;

pub struct SkillStore;

impl SkillStore {
    pub fn get_all_skills(state: &MemoryState) -> Vec<&StoredSkill> { state.procedural.values().collect() }

    pub fn get_skill<'a>(state: &'a MemoryState, skill_id: &str) -> Option<&'a StoredSkill> { state.procedural.get(skill_id) }

    pub fn search_skills<'a>(state: &'a MemoryState, keyword: &str) -> Vec<&'a StoredSkill> {
        state.procedural.values().filter(|s| s.name.contains(keyword)).collect()
    }

    pub fn propose_add_skill(skill_id: String, name: String, steps: Vec<String>) -> EventPayload {
        let steps_str = steps.join(";");
        EventPayload::Custom { event_type: "SkillStoreAdd".into(), data: format!("{}|{}|{}", skill_id, name, steps_str).into_bytes() }
    }

    pub fn propose_remove_skill(skill_id: String) -> EventPayload {
        EventPayload::Custom { event_type: "SkillStoreRemove".into(), data: skill_id.into_bytes() }
    }
}
