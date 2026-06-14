use crate::canonical_state::memory_state::MemoryState;
use crate::kernel_core::event::EventPayload;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextItem {
    pub item_id: String,
    pub item_type: ContextItemType,
    pub description: String,
    pub relevance_score: OrderedFloat<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextItemType { Entity, Goal, Fact, Query, Hypothesis }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveContext {
    pub session_id: String,
    pub items: Vec<ContextItem>,
    pub primary_focus: Option<String>,
    pub created_at_tick: u64,
}

pub struct Context;

impl Context {
    pub fn build_context(state: &MemoryState, session_id: String, query: Option<&str>, current_tick: u64) -> ActiveContext {
        let mut items = Vec::new();
        for wm in &state.working {
            items.push(ContextItem {
                item_id: wm.item_id.clone(),
                item_type: ContextItemType::Entity,
                description: wm.content.clone(),
                relevance_score: wm.relevance,
            });
        }
        let primary_focus = query.map(|q| {
            items.push(ContextItem {
                item_id: format!("query.{}", current_tick),
                item_type: ContextItemType::Query,
                description: q.to_string(),
                relevance_score: OrderedFloat(1.0),
            });
            format!("query.{}", current_tick)
        });
        items.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal));
        ActiveContext { session_id, items, primary_focus, created_at_tick: current_tick }
    }

    pub fn find_relevant<'a>(context: &'a ActiveContext, keyword: &str) -> Vec<&'a ContextItem> {
        context.items.iter().filter(|item| item.description.contains(keyword)).collect()
    }

    pub fn propose_set_focus(item_id: String) -> EventPayload {
        EventPayload::Custom { event_type: "ContextFocusSet".into(), data: item_id.into_bytes() }
    }

    pub fn propose_clear_context(session_id: String) -> EventPayload {
        EventPayload::Custom { event_type: "ContextCleared".into(), data: session_id.into_bytes() }
    }
}
