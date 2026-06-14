use crate::canonical_state::memory_state::MemoryState;
use crate::kernel_core::event::EventPayload;

pub struct MemoryManager;

impl MemoryManager {
    pub fn query(state: &MemoryState, query: &str) -> Vec<MemoryQueryResult> {
        let mut results = Vec::new();
        for item in &state.working {
            if item.content.contains(query) {
                results.push(MemoryQueryResult { layer: MemoryLayer::Working, id: item.item_id.clone(), content: item.content.clone(), relevance: item.relevance.0 });
            }
        }
        for (id, fact) in &state.semantic {
            if fact.subject.contains(query) || fact.predicate.contains(query) || fact.object.contains(query) {
                results.push(MemoryQueryResult { layer: MemoryLayer::Semantic, id: id.clone(), content: format!("{} {} {}", fact.subject, fact.predicate, fact.object), relevance: fact.confidence.0 });
            }
        }
        for ep in &state.episodic {
            if ep.description.contains(query) {
                results.push(MemoryQueryResult { layer: MemoryLayer::Episodic, id: ep.episode_id.clone(), content: ep.description.clone(), relevance: 0.8 });
            }
        }
        results.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    pub fn propose_semantic_fact(subject: String, predicate: String, object: String, confidence: f64) -> EventPayload {
        EventPayload::Custom { event_type: "MemorySemanticFactProposed".into(), data: format!("{}|{}|{}|{}", subject, predicate, object, confidence).into_bytes() }
    }

    pub fn propose_episode(description: String) -> EventPayload {
        EventPayload::Custom { event_type: "MemoryEpisodeRecorded".into(), data: description.into_bytes() }
    }

    pub fn propose_working_focus(item_id: String, content: String, relevance: f64) -> EventPayload {
        EventPayload::Custom { event_type: "MemoryWorkingFocusSet".into(), data: format!("{}|{}|{}", item_id, content, relevance).into_bytes() }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryQueryResult {
    pub layer: MemoryLayer,
    pub id: String,
    pub content: String,
    pub relevance: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryLayer { Working, Episodic, Semantic, Procedural, LongTerm }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical_state::memory_state::SemanticFact;
    use ordered_float::OrderedFloat;

    #[test]
    fn query_empty() { let state = MemoryState::empty(); assert!(MemoryManager::query(&state, "test").is_empty()); }

    #[test]
    fn query_semantic() {
        let mut state = MemoryState::empty();
        state.semantic.insert("f1".into(), SemanticFact { fact_id: "f1".into(), subject: "user".into(), predicate: "knows".into(), object: "rust".into(), confidence: OrderedFloat(0.95) });
        let results = MemoryManager::query(&state, "rust");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].layer, MemoryLayer::Semantic);
    }
}
