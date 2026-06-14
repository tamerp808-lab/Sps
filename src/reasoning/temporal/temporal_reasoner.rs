use crate::kernel_core::event::EventPayload;

pub struct TemporalReasoner;
impl TemporalReasoner {
    pub fn before(a: u64, b: u64) -> bool { a < b }
    pub fn propose_ordering(a_id: String, b_id: String, before: bool) -> EventPayload {
        EventPayload::Custom { event_type: "TemporalOrdering".into(), data: format!("{}|{}|{}", a_id, b_id, before).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn before_test() { assert!(TemporalReasoner::before(10,20)); assert!(!TemporalReasoner::before(20,10)); }
}
