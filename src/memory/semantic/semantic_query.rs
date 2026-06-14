use crate::canonical_state::memory_state::MemoryState;
use crate::kernel_core::event::EventPayload;

#[derive(Debug, Clone)]
pub struct SemanticQuery {
    pub subject: Option<String>,
    pub predicate: Option<String>,
    pub object: Option<String>,
    pub min_confidence: Option<f64>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct SemanticQueryResult {
    pub fact_id: String,
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub confidence: f64,
}

pub struct SemanticQueryEngine;

impl SemanticQueryEngine {
    pub fn query(state: &MemoryState, query: &SemanticQuery) -> Vec<SemanticQueryResult> {
        state
            .semantic
            .values()
            .filter(|fact| {
                if let Some(ref s) = query.subject { if fact.subject != *s { return false; } }
                if let Some(ref p) = query.predicate { if fact.predicate != *p { return false; } }
                if let Some(ref o) = query.object { if fact.object != *o { return false; } }
                if let Some(min_conf) = query.min_confidence { if fact.confidence.0 < min_conf { return false; } }
                true
            })
            .map(|fact| SemanticQueryResult {
                fact_id: fact.fact_id.clone(),
                subject: fact.subject.clone(),
                predicate: fact.predicate.clone(),
                object: fact.object.clone(),
                confidence: fact.confidence.0,
            })
            .take(query.limit.unwrap_or(usize::MAX))
            .collect()
    }

    pub fn by_subject(state: &MemoryState, subject: &str) -> Vec<SemanticQueryResult> {
        Self::query(state, &SemanticQuery { subject: Some(subject.into()), predicate: None, object: None, min_confidence: None, limit: None })
    }

    pub fn by_triple(state: &MemoryState, subject: &str, predicate: &str, object: &str) -> Vec<SemanticQueryResult> {
        Self::query(state, &SemanticQuery { subject: Some(subject.into()), predicate: Some(predicate.into()), object: Some(object.into()), min_confidence: None, limit: None })
    }

    pub fn propose_log_query(query: &SemanticQuery, result_count: usize) -> EventPayload {
        EventPayload::Custom { event_type: "SemanticQueryExecuted".into(), data: format!("subj={:?}|pred={:?}|obj={:?}|results={}", query.subject, query.predicate, query.object, result_count).into_bytes() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical_state::memory_state::SemanticFact;
    use ordered_float::OrderedFloat;

    fn sample_state() -> MemoryState {
        let mut state = MemoryState::empty();
        state.semantic.insert("f1".into(), SemanticFact { fact_id: "f1".into(), subject: "sps".into(), predicate: "is".into(), object: "deterministic".into(), confidence: OrderedFloat(0.99) });
        state.semantic.insert("f2".into(), SemanticFact { fact_id: "f2".into(), subject: "sps".into(), predicate: "runs_on".into(), object: "linux".into(), confidence: OrderedFloat(0.8) });
        state.semantic.insert("f3".into(), SemanticFact { fact_id: "f3".into(), subject: "rust".into(), predicate: "is".into(), object: "safe".into(), confidence: OrderedFloat(1.0) });
        state
    }

    #[test] fn query_all() { assert_eq!(SemanticQueryEngine::query(&sample_state(), &SemanticQuery { subject: None, predicate: None, object: None, min_confidence: None, limit: None }).len(), 3); }
    #[test] fn query_by_subject() { assert_eq!(SemanticQueryEngine::by_subject(&sample_state(), "sps").len(), 2); }
    #[test] fn query_by_triple() { assert_eq!(SemanticQueryEngine::by_triple(&sample_state(), "sps", "is", "deterministic").len(), 1); }
    #[test] fn query_conf_threshold() { assert_eq!(SemanticQueryEngine::query(&sample_state(), &SemanticQuery { subject: None, predicate: None, object: None, min_confidence: Some(0.9), limit: None }).len(), 2); }
    #[test] fn query_with_limit() { assert_eq!(SemanticQueryEngine::query(&sample_state(), &SemanticQuery { subject: None, predicate: None, object: None, min_confidence: None, limit: Some(1) }).len(), 1); }
}
