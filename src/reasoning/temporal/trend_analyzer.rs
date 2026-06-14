use crate::kernel_core::event::EventPayload;

pub struct TrendAnalyzer;
impl TrendAnalyzer {
    pub fn trend(values: &[f64]) -> &str {
        if values.len() < 2 { return "stable"; }
        if values.last().unwrap() > values.first().unwrap() { "increasing" } else { "decreasing" }
    }
    pub fn propose_trend(values: Vec<f64>, trend: String) -> EventPayload {
        EventPayload::Custom { event_type: "TrendAnalyzed".into(), data: format!("{:?}|{}", values, trend).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn increasing() { assert_eq!(TrendAnalyzer::trend(&[1.0,2.0,3.0]), "increasing"); }
}
