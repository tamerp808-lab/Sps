use crate::canonical_state::world_state::{WorldState, EntityId, Relation, RelationId};
use crate::kernel_core::event::EventPayload;

pub struct WorldGraph;

impl WorldGraph {
    pub fn all_relations(state: &WorldState) -> Vec<&Relation> { state.relations.values().collect() }

    pub fn relations_from<'a>(state: &'a WorldState, entity_id: &EntityId) -> Vec<&'a Relation> {
        state.relations.values().filter(|r| &r.from == entity_id).collect()
    }

    pub fn relations_to<'a>(state: &'a WorldState, entity_id: &EntityId) -> Vec<&'a Relation> {
        state.relations.values().filter(|r| &r.to == entity_id).collect()
    }

    pub fn relations_of_type<'a>(state: &'a WorldState, relation_type: &str) -> Vec<&'a Relation> {
        state.relations.values().filter(|r| r.relation_type == relation_type).collect()
    }

    pub fn propose_create(relation_id: RelationId, from: EntityId, relation_type: String, to: EntityId, weight: f64, bidirectional: bool) -> EventPayload {
        EventPayload::Custom { event_type: "WorldGraphRelationCreated".into(), data: format!("{}|{}|{}|{}|{}|{}", relation_id.0, from.0, relation_type, to.0, weight, if bidirectional {"1"} else {"0"}).into_bytes() }
    }

    pub fn propose_remove(relation_id: RelationId) -> EventPayload {
        EventPayload::Custom { event_type: "WorldGraphRelationRemoved".into(), data: relation_id.0.into_bytes() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ordered_float::OrderedFloat;

    fn sample() -> WorldState {
        let mut w = WorldState::empty();
        w.relations.insert(RelationId("r1".into()), Relation { id: RelationId("r1".into()), from: EntityId("u".into()), relation_type: "owns".into(), to: EntityId("p".into()), weight: OrderedFloat(0.9), bidirectional: false });
        w
    }

    #[test] fn from_works() { let w = sample(); assert_eq!(WorldGraph::relations_from(&w, &EntityId("u".into())).len(), 1); }
    #[test] fn propose_ok() { let p = WorldGraph::propose_create(RelationId("r".into()), EntityId("a".into()), "x".into(), EntityId("b".into()), 0.5, true); match p { EventPayload::Custom{..}=>(), _=>panic!() } }
}
