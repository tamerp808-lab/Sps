use crate::kernel_core::event::EventPayload;

pub struct BeliefUpdater;

impl BeliefUpdater {
    /// Updates belief using a simple Bayesian-like formula:
    /// new_belief = (prior * weight + evidence * (1 - weight)).
    pub fn update(prior: f64, evidence: f64, weight: f64) -> f64 {
        prior * weight + evidence * (1.0 - weight)
    }

    pub fn propose_update(belief_id: String, new_belief: f64) -> EventPayload {
        EventPayload::Custom {
            event_type: "BeliefUpdated".into(),
            data: format!("{}|{}", belief_id, new_belief).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn updates_belief_toward_evidence() {
        let new_belief = BeliefUpdater::update(0.5, 0.9, 0.8);
        assert!(new_belief > 0.5);
        assert!(new_belief < 0.9);
    }
}
