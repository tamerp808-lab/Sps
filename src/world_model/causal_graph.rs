use crate::canonical_state::world_state::EntityId;
use crate::kernel_core::event::EventPayload;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CausalLink {
    pub cause_entity: Option<EntityId>,
    pub effect_entity: Option<EntityId>,
    pub cause_event_id: Option<String>,
    pub effect_event_id: Option<String>,
    pub confidence: OrderedFloat<f64>,
    pub description: String,
}

pub struct CausalGraph;

impl CausalGraph {
    pub fn propose_record_link(
        cause_entity: Option<EntityId>, effect_entity: Option<EntityId>,
        cause_event_id: Option<String>, effect_event_id: Option<String>,
        confidence: f64, description: String,
    ) -> EventPayload {
        let ce = cause_entity.map(|e| e.0).unwrap_or_default();
        let ee = effect_entity.map(|e| e.0).unwrap_or_default();
        let cev = cause_event_id.unwrap_or_default();
        let eev = effect_event_id.unwrap_or_default();
        EventPayload::Custom { event_type: "CausalLinkRecorded".into(), data: format!("{}|{}|{}|{}|{}|{}", ce, ee, cev, eev, confidence, description).into_bytes() }
    }

    pub fn propose_query_causes(entity_id: EntityId) -> EventPayload {
        EventPayload::Custom { event_type: "CausalGraphQueryCauses".into(), data: entity_id.0.into_bytes() }
    }

    pub fn propose_query_effects(entity_id: EntityId) -> EventPayload {
        EventPayload::Custom { event_type: "CausalGraphQueryEffects".into(), data: entity_id.0.into_bytes() }
    }

    pub fn propose_remove_link(link_id: String) -> EventPayload {
        EventPayload::Custom { event_type: "CausalLinkRemoved".into(), data: link_id.into_bytes() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn propose_record_ok() {
        let p = CausalGraph::propose_record_link(Some(EntityId("u".into())), Some(EntityId("e".into())), Some("e1".into()), Some("e2".into()), 0.85, "cause".into());
        match p { EventPayload::Custom{event_type,..}=> assert!(event_type.contains("CausalLinkRecorded")), _=>panic!() }
    }
}
