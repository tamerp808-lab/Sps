use crate::kernel_core::event::EventPayload;

pub struct ProbabilisticReasoner;

impl ProbabilisticReasoner {
    /// Simple conditional probability estimate: count(A&B) / count(B).
    pub fn conditional(joint_count: u64, condition_count: u64) -> f64 {
        if condition_count == 0 { 0.0 } else { joint_count as f64 / condition_count as f64 }
    }

    pub fn propose_estimate(rule: String, probability: f64) -> EventPayload {
        EventPayload::Custom {
            event_type: "ProbabilisticEstimate".into(),
            data: format!("{}|{}", rule, probability).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_probability() {
        let p = ProbabilisticReasoner::conditional(3, 5);
        assert!((p - 0.6).abs() < 0.001);
    }
}
