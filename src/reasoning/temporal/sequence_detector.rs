use crate::kernel_core::event::EventPayload;

pub struct SequenceDetector;
impl SequenceDetector {
    pub fn detect_sequence(events: &[u64], pattern: &[u64]) -> bool { events.windows(pattern.len()).any(|w| w == pattern) }
    pub fn propose_sequence(pattern: Vec<u64>, found: bool) -> EventPayload {
        EventPayload::Custom { event_type: "SequenceDetected".into(), data: format!("{:?}|{}", pattern, found).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn detects() { assert!(SequenceDetector::detect_sequence(&[1,2,3], &[2,3])); }
}
