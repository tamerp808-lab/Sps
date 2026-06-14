use crate::canonical_state::memory_state::{MemoryState, SemanticFact};
use crate::kernel_core::event::EventPayload;

pub struct FactManager;

impl FactManager {
    pub fn get<'a>(state: &'a MemoryState, fact_id: &str) -> Option<&'a SemanticFact> { state.semantic.get(fact_id) }

    pub fn about<'a>(state: &'a MemoryState, subject: &str) -> Vec<&'a SemanticFact> {
        state.semantic.values().filter(|f| f.subject == subject).collect()
    }

    pub fn with_predicate<'a>(state: &'a MemoryState, predicate: &str) -> Vec<&'a SemanticFact> {
        state.semantic.values().filter(|f| f.predicate == predicate).collect()
    }

    pub fn propose_assert(fact_id: String, subject: String, predicate: String, object: String, confidence: f64) -> EventPayload {
        EventPayload::Custom { event_type: "SemanticFactAsserted".into(), data: format!("{}|{}|{}|{}|{}", fact_id, subject, predicate, object, confidence).into_bytes() }
    }

    pub fn propose_retract(fact_id: String, reason: String) -> EventPayload {
        EventPayload::Custom { event_type: "SemanticFactRetracted".into(), data: format!("{}|{}", fact_id, reason).into_bytes() }
    }

    pub fn propose_update_confidence(fact_id: String, new_confidence: f64) -> EventPayload {
        EventPayload::Custom { event_type: "SemanticFactConfidenceUpdated".into(), data: format!("{}|{}", fact_id, new_confidence).into_bytes() }
    }

    pub fn count(state: &MemoryState) -> usize { state.semantic.len() }
}
