use crate::canonical_state::memory_state::{MemoryState, SemanticFact};
use crate::kernel_core::event::EventPayload;

pub struct FactStore;

impl FactStore {
    pub fn get_all(state: &MemoryState) -> Vec<&SemanticFact> { state.semantic.values().collect() }

    pub fn get_confident(state: &MemoryState, threshold: f64) -> Vec<&SemanticFact> {
        state.semantic.values().filter(|f| f.confidence.0 >= threshold).collect()
    }

    pub fn detect_conflict(state: &MemoryState, subject: &str, predicate: &str, object: &str) -> Option<String> {
        state.semantic.values().find(|f| f.subject == subject && f.predicate == predicate && f.object == object).map(|f| f.fact_id.clone())
    }

    pub fn propose_add(fact_id: String, subject: String, predicate: String, object: String, confidence: f64) -> EventPayload {
        EventPayload::Custom { event_type: "FactStoreAdd".into(), data: format!("{}|{}|{}|{}|{}", fact_id, subject, predicate, object, confidence).into_bytes() }
    }

    pub fn propose_remove(fact_id: String) -> EventPayload {
        EventPayload::Custom { event_type: "FactStoreRemove".into(), data: fact_id.into_bytes() }
    }

    pub fn propose_merge(winner_id: String, loser_id: String) -> Vec<EventPayload> {
        vec![
            EventPayload::Custom { event_type: "FactStoreMerge".into(), data: format!("{}|{}", winner_id, loser_id).into_bytes() },
            EventPayload::Custom { event_type: "FactStoreRemove".into(), data: loser_id.into_bytes() },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ordered_float::OrderedFloat;

    fn sample_state() -> MemoryState {
        let mut state = MemoryState::empty();
        state.semantic.insert("f1".into(), SemanticFact { fact_id: "f1".into(), subject: "sps".into(), predicate: "is".into(), object: "deterministic".into(), confidence: OrderedFloat(0.99) });
        state
    }

    #[test] fn get_all() { assert_eq!(FactStore::get_all(&sample_state()).len(), 1); }
    #[test] fn get_confident() { assert_eq!(FactStore::get_confident(&sample_state(), 0.95).len(), 1); assert!(FactStore::get_confident(&sample_state(), 0.999).is_empty()); }
    #[test] fn detect_conflict_found() { assert_eq!(FactStore::detect_conflict(&sample_state(), "sps", "is", "deterministic"), Some("f1".into())); }
    #[test] fn detect_conflict_none() { assert!(FactStore::detect_conflict(&sample_state(), "sps", "is", "fast").is_none()); }
}
