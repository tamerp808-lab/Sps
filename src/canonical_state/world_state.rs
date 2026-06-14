use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EntityId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RelationId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FactId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CapabilityId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType { User, Agent, Project, Tool, Resource, Concept, Goal, Environment }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entity { pub id: EntityId, pub entity_type: EntityType, pub properties: BTreeMap<String, String> }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Relation { pub id: RelationId, pub from: EntityId, pub relation_type: String, pub to: EntityId, pub weight: OrderedFloat<f64>, pub bidirectional: bool }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fact { pub id: FactId, pub subject: EntityId, pub predicate: String, pub object: String }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Capability { pub id: CapabilityId, pub owner: EntityId, pub capability_type: String }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldState {
    pub entities: BTreeMap<EntityId, Entity>,
    pub relations: BTreeMap<RelationId, Relation>,
    pub facts: BTreeMap<FactId, Fact>,
    pub capabilities: BTreeMap<CapabilityId, Capability>,
}

impl WorldState {
    pub fn empty() -> Self {
        WorldState { entities: BTreeMap::new(), relations: BTreeMap::new(), facts: BTreeMap::new(), capabilities: BTreeMap::new() }
    }
    pub fn is_empty(&self) -> bool { self.entities.is_empty() && self.relations.is_empty() && self.facts.is_empty() && self.capabilities.is_empty() }
}
