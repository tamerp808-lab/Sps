// src/self_improvement/impact_analyzer.rs
// Phase 10 — Self-Improvement

use crate::kernel_core::event::EventPayload;

pub struct ImpactAnalyzer;

impl ImpactAnalyzer {
    /// Simple impact score based on component criticality.
    pub fn analyze(target: &str) -> u64 {
        match target {
            "kernel_core" => 100,
            "memory" => 70,
            _ => 30,
        }
    }

    pub fn propose_analysis(target: String, impact: u64) -> EventPayload {
        EventPayload::Custom {
            event_type: "ImpactAnalyzed".into(),
            data: format!("{}|{}", target, impact).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_high_impact() {
        assert_eq!(ImpactAnalyzer::analyze("kernel_core"), 100);
    }
}
