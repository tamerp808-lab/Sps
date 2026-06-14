use crate::kernel_core::event::EventPayload;

pub struct OccamFilter;
impl OccamFilter {
    pub fn filter(explanations: &[String]) -> Option<String> { explanations.iter().min_by_key(|e| e.len()).cloned() }
    pub fn propose_filter(best: String) -> EventPayload { EventPayload::Custom { event_type: "OccamFilterApplied".into(), data: best.into_bytes() } }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn picks_shortest() { assert_eq!(OccamFilter::filter(&["complex".into(), "s".into()]), Some("s".into())); }
}
