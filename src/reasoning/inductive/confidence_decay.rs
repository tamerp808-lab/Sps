use crate::kernel_core::event::EventPayload;

pub struct ConfidenceDecay;
impl ConfidenceDecay {
    pub fn decay(initial: f64, ticks: u64, half_life: u64) -> f64 { initial * 0.5_f64.powf(ticks as f64 / half_life as f64) }
    pub fn propose_decay(item_id: String, new_conf: f64) -> EventPayload {
        EventPayload::Custom { event_type: "InductiveConfidenceDecayed".into(), data: format!("{}|{}", item_id, new_conf).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn decays() { assert!((ConfidenceDecay::decay(1.0, 100, 100) - 0.5).abs() < 0.01); }
}
