use crate::kernel_core::event::EventPayload;

pub struct BeliefUpdater;
impl BeliefUpdater {
    pub fn update(prior: f64, evidence: f64, weight: f64) -> f64 { prior * weight + evidence * (1.0 - weight) }
    pub fn propose_update(belief_id: String, new_belief: f64) -> EventPayload {
        EventPayload::Custom { event_type: "BeliefUpdated".into(), data: format!("{}|{}", belief_id, new_belief).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn updates() { let b = BeliefUpdater::update(0.5, 0.9, 0.8); assert!(b > 0.5 && b < 0.9); }
}
