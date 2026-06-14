use serde::{Serialize, Deserialize};
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Insight { pub insight_id: String, pub description: String, pub confidence: OrderedFloat<f64>, pub generated_epoch: u64, pub generated_tick: u64 }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Anomaly { pub anomaly_id: String, pub description: String, pub severity: String, pub detected_epoch: u64, pub detected_tick: u64 }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReflectionState {
    pub insights: Vec<Insight>,
    pub anomalies: Vec<Anomaly>,
    pub last_reflection_epoch: u64,
    pub last_reflection_tick: u64,
    pub total_reflections: u64,
}

impl ReflectionState {
    pub fn empty() -> Self { ReflectionState { insights: Vec::new(), anomalies: Vec::new(), last_reflection_epoch: 0, last_reflection_tick: 0, total_reflections: 0 } }
    pub fn is_empty(&self) -> bool { self.insights.is_empty() && self.anomalies.is_empty() && self.total_reflections == 0 }
}
