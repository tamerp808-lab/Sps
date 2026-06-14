use crate::kernel_core::event::EventPayload;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relation {
    pub relation_id: String,
    pub from_entity: String,
    pub relation_type: String,
    pub to_entity: String,
    pub weight: OrderedFloat<f64>,
    pub bidirectional: bool,
}

pub struct RelationManager;

impl RelationManager {
    pub fn propose_create(relation_id: String, from_entity: String, relation_type: String, to_entity: String, weight: f64, bidirectional: bool) -> EventPayload {
        EventPayload::Custom { event_type: "SemanticRelationCreated".into(), data: format!("{}|{}|{}|{}|{}|{}", relation_id, from_entity, relation_type, to_entity, weight, if bidirectional {"1"} else {"0"}).into_bytes() }
    }

    pub fn propose_update_weight(relation_id: String, new_weight: f64) -> EventPayload {
        EventPayload::Custom { event_type: "SemanticRelationWeightUpdated".into(), data: format!("{}|{}", relation_id, new_weight).into_bytes() }
    }

    pub fn propose_remove(relation_id: String) -> EventPayload {
        EventPayload::Custom { event_type: "SemanticRelationRemoved".into(), data: relation_id.into_bytes() }
    }

    pub fn propose_find_between(entity_a: &str, entity_b: &str) -> EventPayload {
        EventPayload::Custom { event_type: "SemanticRelationQuery".into(), data: format!("{}|{}", entity_a, entity_b).into_bytes() }
    }
}
