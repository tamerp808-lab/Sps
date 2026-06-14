use crate::kernel_core::event::EventPayload;

pub struct ProbabilisticReasoner;
impl ProbabilisticReasoner {
    pub fn conditional(joint: u64, cond: u64) -> f64 { if cond == 0 { 0.0 } else { joint as f64 / cond as f64 } }
    pub fn propose_estimate(rule: String, prob: f64) -> EventPayload {
        EventPayload::Custom { event_type: "ProbabilisticEstimate".into(), data: format!("{}|{}", rule, prob).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn computes() { assert!((ProbabilisticReasoner::conditional(3,5) - 0.6).abs() < 0.001); }
}
