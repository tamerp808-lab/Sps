use crate::canonical_state::world_state::{WorldState, Entity, EntityId, EntityType};
use crate::kernel_core::event::EventPayload;

pub struct EntityRegistry;

impl EntityRegistry {
    pub fn get<'a>(state: &'a WorldState, entity_id: &EntityId) -> Option<&'a Entity> {
        state.entities.get(entity_id)
    }

    pub fn by_type<'a>(state: &'a WorldState, entity_type: &EntityType) -> Vec<&'a Entity> {
        state.entities.values().filter(|e| &e.entity_type == entity_type).collect()
    }

    pub fn search<'a>(state: &'a WorldState, keyword: &str) -> Vec<&'a Entity> {
        state.entities.values().filter(|e| e.id.0.contains(keyword) || e.properties.values().any(|v| v.contains(keyword))).collect()
    }

    pub fn propose_register(entity_id: EntityId, entity_type: EntityType, properties: Vec<(String,String)>) -> EventPayload {
        let props_str = properties.iter().map(|(k,v)| format!("{}={}",k,v)).collect::<Vec<_>>().join(",");
        EventPayload::Custom { event_type: "EntityRegistered".into(), data: format!("{}|{:?}|{}", entity_id.0, entity_type, props_str).into_bytes() }
    }

    pub fn propose_update(entity_id: EntityId, properties: Vec<(String,String)>) -> EventPayload {
        let props_str = properties.iter().map(|(k,v)| format!("{}={}",k,v)).collect::<Vec<_>>().join(",");
        EventPayload::Custom { event_type: "EntityUpdated".into(), data: format!("{}|{}", entity_id.0, props_str).into_bytes() }
    }

    pub fn propose_remove(entity_id: EntityId) -> EventPayload {
        EventPayload::Custom { event_type: "EntityRemoved".into(), data: entity_id.0.into_bytes() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn sample_world() -> WorldState {
        let mut world = WorldState::empty();
        world.entities.insert(EntityId("user.1".into()), Entity { id: EntityId("user.1".into()), entity_type: EntityType::User, properties: BTreeMap::from([("name".into(),"alice".into())]) });
        world
    }

    #[test] fn get_existing() { let world = sample_world(); let e = EntityRegistry::get(&world, &EntityId("user.1".into())); assert!(e.is_some()); }
    #[test] fn get_missing() { assert!(EntityRegistry::get(&WorldState::empty(), &EntityId("x".into())).is_none()); }
    #[test] fn search_works() { assert_eq!(EntityRegistry::search(&sample_world(), "alice").len(), 1); }
    #[test] fn propose_register_ok() {
        let p = EntityRegistry::propose_register(EntityId("a".into()), EntityType::Agent, vec![("role".into(),"x".into())]);
        match p { EventPayload::Custom{event_type,..} => assert!(event_type.contains("EntityRegistered")), _ => panic!() }
    }
}
