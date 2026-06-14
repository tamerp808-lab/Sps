use crate::kernel_core::event::EventPayload;

pub struct ConfidenceEngine;

impl ConfidenceEngine {
    /// Combines independent confidences (simple product).
    pub fn combine(confidences: &[f64]) -> f64 {
        confidences.iter().product()
    }

    pub fn propose_combined_confidence(item_id: String, combined: f64) -> EventPayload {
        EventPayload::Custom {
            event_type: "ConfidenceCombined".into(),
            data: format!("{}|{}", item_id, combined).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combines_confidences() {
        assert!((ConfidenceEngine::combine(&[0.9, 0.9]) - 0.81).abs() < 0.001);
    }
}
