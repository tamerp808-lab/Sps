use crate::kernel_core::event::EventPayload;

pub struct BlindSpotDetector;
impl BlindSpotDetector {
    pub fn detect(covered: &[String], all: &[String]) -> Vec<String> {
        all.iter().filter(|d| !covered.contains(d)).cloned().collect()
    }
    pub fn propose_blind_spot(domains: Vec<String>) -> EventPayload {
        EventPayload::Custom { event_type: "BlindSpotDetected".into(), data: domains.join(",").into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn finds() { let spots = BlindSpotDetector::detect(&["memory".into()], &["memory".into(), "world_model".into()]); assert_eq!(spots, vec!["world_model"]); }
}
