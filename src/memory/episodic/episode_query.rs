use crate::canonical_state::memory_state::MemoryState;
use crate::memory::episodic::episode_index::EpisodeIndex;
use crate::kernel_core::event::EventPayload;

#[derive(Debug, Clone)]
pub struct EpisodeQuery {
    pub keyword: Option<String>,
    pub entity: Option<String>,
    pub from_tick: Option<u64>,
    pub to_tick: Option<u64>,
    pub min_importance: Option<f64>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct EpisodeQueryResult {
    pub episode_id: String,
    pub description: String,
    pub timestamp_epoch: u64,
    pub timestamp_tick: u64,
    pub relevance: f64,
}

pub struct EpisodeQueryEngine;

impl EpisodeQueryEngine {
    pub fn query(state: &MemoryState, index: &EpisodeIndex, query: &EpisodeQuery) -> Vec<EpisodeQueryResult> {
        let candidate_ids: Option<Vec<String>> = if let Some(ref keyword) = query.keyword {
            Some(index.find_by_keyword(keyword))
        } else if let Some(ref entity) = query.entity {
            Some(index.find_by_entity(entity))
        } else { None };

        let mut results = Vec::new();
        for ep in &state.episodic {
            if let Some(ref ids) = candidate_ids { if !ids.contains(&ep.episode_id) { continue; } }
            if let Some(from) = query.from_tick { if ep.timestamp_tick < from { continue; } }
            if let Some(to) = query.to_tick { if ep.timestamp_tick > to { continue; } }
            let mut relevance = 0.5;
            if let Some(ref keyword) = query.keyword { if ep.description.to_lowercase().contains(&keyword.to_lowercase()) { relevance += 0.3; } }
            results.push(EpisodeQueryResult { episode_id: ep.episode_id.clone(), description: ep.description.clone(), timestamp_epoch: ep.timestamp_epoch, timestamp_tick: ep.timestamp_tick, relevance });
        }
        results.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap_or(std::cmp::Ordering::Equal).then_with(|| b.timestamp_tick.cmp(&a.timestamp_tick)));
        if let Some(limit) = query.limit { results.truncate(limit); }
        results
    }

    pub fn propose_record_query(query: &EpisodeQuery, result_count: usize) -> EventPayload {
        EventPayload::Custom { event_type: "EpisodeQueryExecuted".into(), data: format!("keyword={:?}|entity={:?}|results={}", query.keyword, query.entity, result_count).into_bytes() }
    }
}
