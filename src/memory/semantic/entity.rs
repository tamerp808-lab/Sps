use crate::kernel_core::event::EventPayload;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityType { User, Agent, Project, Tool, Resource, Concept, Goal, Environment }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    pub entity_id: String,
    pub entity_type: EntityType,
    pub properties: Vec<(String, String)>,
    pub confidence: OrderedFloat<f64>,
}

pub struct EntityManager;

impl EntityManager {
    pub fn propose_register(entity_id: String, entity_type: EntityType, properties: Vec<(String, String)>, confidence: f64) -> EventPayload {
        let props_str = properties.iter().map(|(k,v)| format!("{}={}", k, v)).collect::<Vec<_>>().join(",");
        let type_str = format!("{:?}", entity_type);
        EventPayload::Custom { event_type: "SemanticEntityRegistered".into(), data: format!("{}|{}|{}|{}", entity_id, type_str, props_str, confidence).into_bytes() }
    }

    pub fn propose_update(entity_id: String, properties: Vec<(String, String)>) -> EventPayload {
        let props_str = properties.iter().map(|(k,v)| format!("{}={}", k, v)).collect::<Vec<_>>().join(",");
        EventPayload::Custom { event_type: "SemanticEntityUpdated".into(), data: format!("{}|{}", entity_id, props_str).into_bytes() }
    }

    pub fn propose_remove(entity_id: String) -> EventPayload {
        EventPayload::Custom { event_type: "SemanticEntityRemoved".into(), data: entity_id.into_bytes() }
    }
}
