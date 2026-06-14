// src/memory/forgetting/decay_model.rs
use crate::kernel_core::event::EventPayload;

pub struct DecayModel;

impl DecayModel {
    /// Computes decayed relevance given initial relevance, age in ticks,
    /// and a decay rate. Returns 0.0 if the decayed value is below
    /// machine epsilon to avoid floating-point underflow noise.
    pub fn decayed_relevance(initial: f64, age_ticks: u64, decay_rate: f64) -> f64 {
        let factor = (-decay_rate * age_ticks as f64).exp();
        let decayed = initial * factor;
        if decayed < f64::EPSILON { 0.0 } else { decayed }
    }

    pub fn propose_decay(item_id: String, new_relevance: f64) -> EventPayload {
        EventPayload::Custom {
            event_type: "MemoryDecayApplied".into(),
            data: format!("{}|{}", item_id, new_relevance).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decay_reduces_relevance() {
        let new_rel = DecayModel::decayed_relevance(1.0, 100, 0.01);
        assert!(new_rel < 1.0);
        assert!(new_rel > 0.0);
    }

    #[test]
    fn decay_never_negative() {
        let new_rel = DecayModel::decayed_relevance(0.1, 1000, 0.1);
        assert_eq!(new_rel, 0.0);
    }
}
