// src/reasoning/abductive/occam_filter.rs
// Phase 5 — Reasoning
// Zone B — Cognitive

use crate::kernel_core::event::EventPayload;

pub struct OccamFilter;

impl OccamFilter {
    /// Keeps only the simplest explanation (shortest).
    pub fn filter(explanations: &[String]) -> Option<String> {
        explanations.iter().min_by_key(|e| e.len()).cloned()
    }

    pub fn propose_filter(best: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "OccamFilterApplied".into(),
            data: best.into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_shortest() {
        let exps = vec!["complex reason".into(), "simple".into()];
        assert_eq!(OccamFilter::filter(&exps), Some("simple".into()));
    }
}
