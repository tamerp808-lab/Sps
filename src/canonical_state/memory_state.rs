use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SemanticFact { pub fact_id: String, pub subject: String, pub predicate: String, pub object: String, pub confidence: OrderedFloat<f64> }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Episode { pub episode_id: String, pub timestamp_epoch: u64, pub timestamp_tick: u64, pub description: String }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Skill { pub skill_id: String, pub name: String, pub steps: Vec<String> }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkingMemoryItem { pub item_id: String, pub content: String, pub relevance: OrderedFloat<f64> }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryState {
    pub semantic: BTreeMap<String, SemanticFact>,
    pub episodic: Vec<Episode>,
    pub procedural: BTreeMap<String, Skill>,
    pub working: Vec<WorkingMemoryItem>,
    pub long_term_index: BTreeMap<String, String>,
}

impl MemoryState {
    pub fn empty() -> Self {
        MemoryState { semantic: BTreeMap::new(), episodic: Vec::new(), procedural: BTreeMap::new(), working: Vec::new(), long_term_index: BTreeMap::new() }
    }
    pub fn is_empty(&self) -> bool { self.semantic.is_empty() && self.episodic.is_empty() && self.procedural.is_empty() && self.working.is_empty() && self.long_term_index.is_empty() }
}
