// src/reasoning/inductive/pattern_detector.rs
// Phase 5 — Reasoning
// Zone B — Cognitive

use crate::kernel_core::event::EventPayload;

pub struct PatternDetector;

impl PatternDetector {
    /// Detects a simple pattern: if all given examples share a suffix,
    /// it hypothesizes that the pattern generalizes.
    pub fn detect(examples: &[String]) -> Option<String> {
        if examples.is_empty() { return None; }
        let first = &examples[0];
        for ex in &examples[1..] {
            if !ex.ends_with(first.split_whitespace().last().unwrap_or("")) {
                return None;
            }
        }
        Some(format!("All examples end with '{}'", first.split_whitespace().last().unwrap()))
    }

    pub fn propose_pattern(pattern: String, confidence: f64) -> EventPayload {
        EventPayload::Custom {
            event_type: "InductivePatternDetected".into(),
            data: format!("{}|{}", pattern, confidence).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_common_suffix() {
        let examples = vec!["a is fast".into(), "b is fast".into(), "c is fast".into()];
        assert!(PatternDetector::detect(&examples).is_some());
    }

    #[test]
    fn no_pattern_returns_none() {
        let examples = vec!["x is y".into(), "a is b".into()];
        assert_eq!(PatternDetector::detect(&examples), None);
    }
}
