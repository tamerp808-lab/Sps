use crate::canonical_state::memory_state::{MemoryState, WorkingMemoryItem};
use crate::kernel_core::event::EventPayload;

const MAX_WORKING_ITEMS: usize = 7;

pub struct Attention;

impl Attention {
    pub fn focus(state: &MemoryState, _context_hint: &str) -> Vec<WorkingMemoryItem> {
        let mut candidates: Vec<&WorkingMemoryItem> = state.working.iter().collect();
        candidates.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap_or(std::cmp::Ordering::Equal));
        candidates.into_iter().take(MAX_WORKING_ITEMS).cloned().collect()
    }

    pub fn bring_to_focus(item_id: String, content: String, relevance: f64) -> EventPayload {
        EventPayload::Custom { event_type: "WorkingMemoryFocus".into(), data: format!("{}|{}|{}", item_id, content, relevance).into_bytes() }
    }

    pub fn remove_from_focus(item_id: String) -> EventPayload {
        EventPayload::Custom { event_type: "WorkingMemoryRemove".into(), data: item_id.into_bytes() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ordered_float::OrderedFloat;

    #[test]
    fn focus_selects_top_n() {
        let mut state = MemoryState::empty();
        for i in 0..10 { state.working.push(WorkingMemoryItem { item_id: format!("item{}", i), content: format!("c{}", i), relevance: OrderedFloat(i as f64) }); }
        let focused = Attention::focus(&state, "");
        assert_eq!(focused.len(), MAX_WORKING_ITEMS);
        assert_eq!(focused[0].item_id, "item9");
    }
}
