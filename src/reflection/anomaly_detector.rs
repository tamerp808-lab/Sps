// src/reflection/anomaly_detector.rs
// Phase 9 — Reflection
// Zone B — Cognitive

use crate::kernel_core::event::EventPayload;

pub struct AnomalyDetector;

impl AnomalyDetector {
    /// Detects anomaly if execution time exceeds threshold.
    pub fn detect(tick_duration: u64, threshold: u64) -> bool {
        tick_duration > threshold
    }

    pub fn propose_anomaly(description: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "AnomalyDetected".into(),
            data: description.into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_anomaly() {
        assert!(AnomalyDetector::detect(150, 100));
    }

    #[test]
    fn no_anomaly() {
        assert!(!AnomalyDetector::detect(50, 100));
    }
}
