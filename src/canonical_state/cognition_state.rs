use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hypothesis { pub hypothesis_id: String, pub description: String, pub confidence: OrderedFloat<f64>, pub evidence_count: u64, pub last_updated_epoch: u64, pub last_updated_tick: u64 }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rule { pub rule_id: String, pub name: String, pub premises: Vec<String>, pub conclusion: String }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningContext { pub context_id: String, pub query: String, pub hypotheses_active: Vec<String> }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitionState {
    pub rules: BTreeMap<String, Rule>,
    pub hypotheses: BTreeMap<String, Hypothesis>,
    pub active_contexts: Vec<ReasoningContext>,
    pub cycles_completed: u64,
}

impl CognitionState {
    pub fn empty() -> Self { CognitionState { rules: BTreeMap::new(), hypotheses: BTreeMap::new(), active_contexts: Vec::new(), cycles_completed: 0 } }
    pub fn is_empty(&self) -> bool { self.rules.is_empty() && self.hypotheses.is_empty() && self.active_contexts.is_empty() && self.cycles_completed == 0 }
}
