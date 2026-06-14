use crate::kernel_core::event::EventPayload;

pub struct ReasoningAuditor;
impl ReasoningAuditor {
    pub fn audit(inferences: &[(String, f64)]) -> (usize, f64) {
        let count = inferences.len();
        let avg = if count == 0 { 0.0 } else { inferences.iter().map(|(_,c)| c).sum::<f64>() / count as f64 };
        (count, avg)
    }
    pub fn propose_audit(count: usize, avg_confidence: f64) -> EventPayload {
        EventPayload::Custom { event_type: "ReasoningAudited".into(), data: format!("{}|{}", count, avg_confidence).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn audits() { let (c, a) = ReasoningAuditor::audit(&[("i1".into(), 0.8), ("i2".into(), 0.9)]); assert_eq!(c, 2); assert!((a - 0.85).abs() < 0.01); }
}
