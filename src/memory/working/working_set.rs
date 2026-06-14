use crate::canonical_state::memory_state::{MemoryState, WorkingMemoryItem};
use crate::kernel_core::event::EventPayload;

const MAX_WORKING_SET_SIZE: usize = 20;

pub struct WorkingSet;

impl WorkingSet {
    pub fn get_all(state: &MemoryState) -> &[WorkingMemoryItem] { &state.working }
    pub fn size(state: &MemoryState) -> usize { state.working.len() }
    pub fn is_full(state: &MemoryState) -> bool { state.working.len() >= MAX_WORKING_SET_SIZE }

    pub fn propose_add(item_id: String, content: String, relevance: f64) -> Vec<EventPayload> {
        vec![EventPayload::Custom { event_type: "WorkingSetAdd".into(), data: format!("{}|{}|{}", item_id, content, relevance).into_bytes() }]
    }

    pub fn propose_remove(item_id: String) -> EventPayload {
        EventPayload::Custom { event_type: "WorkingSetRemove".into(), data: item_id.into_bytes() }
    }

    pub fn propose_eviction(state: &MemoryState, needed_slots: usize) -> Vec<EventPayload> {
        if state.working.len() + needed_slots <= MAX_WORKING_SET_SIZE { return vec![]; }
        let excess = (state.working.len() + needed_slots) - MAX_WORKING_SET_SIZE;
        let mut sorted: Vec<&WorkingMemoryItem> = state.working.iter().collect();
        sorted.sort_by(|a, b| a.relevance.partial_cmp(&b.relevance).unwrap_or(std::cmp::Ordering::Equal));
        sorted.into_iter().take(excess).map(|item| EventPayload::Custom { event_type: "WorkingSetEvict".into(), data: format!("{}|low_relevance", item.item_id).into_bytes() }).collect()
    }

    pub fn propose_clear() -> EventPayload { EventPayload::Custom { event_type: "WorkingSetClear".into(), data: vec![].into() } }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ordered_float::OrderedFloat;

    #[test] fn empty_is_empty() { let s = MemoryState::empty(); assert_eq!(WorkingSet::size(&s), 0); assert!(!WorkingSet::is_full(&s)); }
    #[test] fn add_produces_event() { let e = WorkingSet::propose_add("item1".into(), "test".into(), 0.9); assert_eq!(e.len(), 1); }
    #[test] fn eviction_when_full() {
        let mut state = MemoryState::empty();
        for i in 0..MAX_WORKING_SET_SIZE { state.working.push(WorkingMemoryItem { item_id: format!("item{}", i), content: format!("c{}", i), relevance: OrderedFloat(i as f64) }); }
        let ev = WorkingSet::propose_eviction(&state, 2);
        assert!(!ev.is_empty());
    }
}
