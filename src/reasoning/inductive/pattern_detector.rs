use crate::kernel_core::event::EventPayload;

pub struct PatternDetector;
impl PatternDetector {
    pub fn detect(examples: &[String]) -> Option<String> {
        if examples.is_empty() { return None; }
        let suffix = examples[0].split_whitespace().last()?;
        if examples.iter().all(|e| e.ends_with(suffix)) { Some(format!("All end with '{}'", suffix)) } else { None }
    }
    pub fn propose_pattern(pattern: String, confidence: f64) -> EventPayload {
        EventPayload::Custom { event_type: "InductivePatternDetected".into(), data: format!("{}|{}", pattern, confidence).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn finds() { assert!(PatternDetector::detect(&["a is X".into(), "b is X".into()]).is_some()); }
    #[test] fn no() { assert!(PatternDetector::detect(&["a X".into(), "b Y".into()]).is_none()); }
}
