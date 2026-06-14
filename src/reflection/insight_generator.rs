// src/reflection/insight_generator.rs
// Phase 9 — Reflection
// Zone B — Cognitive

use crate::kernel_core::event::EventPayload;

pub struct InsightGenerator;

impl InsightGenerator {
    /// Generates an insight from a successful pattern repetition.
    pub fn generate(pattern: &str, occurrences: u64) -> Option<String> {
        if occurrences >= 3 { Some(format!("Pattern '{}' succeeded {} times", pattern, occurrences)) } else { None }
    }

    pub fn propose_insight(insight: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "InsightGenerated".into(),
            data: insight.into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_after_repetition() {
        assert!(InsightGenerator::generate("read_file", 3).is_some());
    }

    #[test]
    fn no_insight_yet() {
        assert!(InsightGenerator::generate("read_file", 1).is_none());
    }
}
