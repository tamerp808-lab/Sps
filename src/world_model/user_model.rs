use crate::canonical_state::world_state::EntityId;
use crate::kernel_core::event::EventPayload;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserPreference { pub key: String, pub value: String, pub confidence: OrderedFloat<f64> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserModel { pub user_id: EntityId, pub name: String, pub skill_level: SkillLevel, pub preferences: Vec<UserPreference>, pub active_project_id: Option<String> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillLevel { Beginner, Intermediate, Advanced }

pub struct UserModelManager;

impl UserModelManager {
    pub fn propose_set_user(user_id: EntityId, name: String, skill_level: SkillLevel, preferences: Vec<UserPreference>) -> EventPayload {
        let prefs_str = preferences.iter().map(|p| format!("{}={}:{}", p.key, p.value, p.confidence.0)).collect::<Vec<_>>().join(",");
        EventPayload::Custom { event_type: "UserModelSet".into(), data: format!("{}|{}|{:?}|{}", user_id.0, name, skill_level, prefs_str).into_bytes() }
    }

    pub fn propose_update_preference(user_id: EntityId, key: String, value: String, confidence: f64) -> EventPayload {
        EventPayload::Custom { event_type: "UserPreferenceUpdated".into(), data: format!("{}|{}={}:{}", user_id.0, key, value, confidence).into_bytes() }
    }

    pub fn propose_set_active_project(user_id: EntityId, project_id: String) -> EventPayload {
        EventPayload::Custom { event_type: "UserActiveProjectSet".into(), data: format!("{}|{}", user_id.0, project_id).into_bytes() }
    }

    pub fn propose_set_skill_level(user_id: EntityId, skill_level: SkillLevel) -> EventPayload {
        EventPayload::Custom { event_type: "UserSkillLevelSet".into(), data: format!("{}|{:?}", user_id.0, skill_level).into_bytes() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn propose_set_user_ok() {
        let p = UserModelManager::propose_set_user(EntityId("u".into()), "alice".into(), SkillLevel::Advanced, vec![UserPreference{key:"lang".into(), value:"rust".into(), confidence:OrderedFloat(1.0)}]);
        match p { EventPayload::Custom{event_type,..}=> assert!(event_type.contains("UserModelSet")), _=>panic!() }
    }
}
