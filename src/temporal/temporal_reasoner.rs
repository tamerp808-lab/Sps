// src/reasoning/temporal/temporal_reasoner.rs
// Phase 5 — Reasoning
// Zone B — Cognitive

use crate::kernel_core::event::EventPayload;

pub struct TemporalReasoner;

impl TemporalReasoner {
    /// Returns true if event A happened before event B.
    pub fn before(a_tick: u64, b_tick: u64) -> bool { a_tick < b_tick }

    /// Proposes a temporal ordering event.
    pub fn propose_ordering(a_id: String, b_id: String, a_before_b: bool) -> EventPayload {
        EventPayload::Custom {
            event_type: "TemporalOrdering".into(),
            data: format!("{}|{}|{}", a_id, b_id, a_before_b).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn before_works() { assert!(TemporalReasoner::before(10, 20)); assert!(!TemporalReasoner::before(30, 20)); }
}
