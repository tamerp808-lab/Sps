// src/reasoning/inductive/confidence_decay.rs
// Phase 5 — Reasoning
// Zone B — Cognitive

use crate::kernel_core::event::EventPayload;

pub struct ConfidenceDecay;

impl ConfidenceDecay {
    /// Decays confidence over time since last evidence.
    pub fn decay(initial: f64, ticks_since_last_evidence: u64, half_life: u64) -> f64 {
        let exponent = ticks_since_last_evidence as f64 / half_life as f64;
        initial * (0.5_f64.powf(exponent))
    }

    pub fn propose_decay(item_id: String, new_confidence: f64) -> EventPayload {
        EventPayload::Custom {
            event_type: "InductiveConfidenceDecayed".into(),
            data: format!("{}|{}", item_id, new_confidence).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decays_to_half_after_half_life() {
        let c = ConfidenceDecay::decay(1.0, 100, 100);
        assert!((c - 0.5).abs() < 0.01);
    }
}
