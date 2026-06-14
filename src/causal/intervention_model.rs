// src/reasoning/causal/intervention_model.rs
// Phase 5 — Reasoning
// Zone B — Cognitive

use crate::kernel_core::event::EventPayload;

pub struct InterventionModel;

impl InterventionModel {
    /// Predicts outcome of an intervention given a simple rule.
    pub fn predict(intervention: &str, rules: &[(String, String)]) -> Option<String> {
        rules.iter()
            .find(|(cause, _)| cause == intervention)
            .map(|(_, effect)| effect.clone())
    }

    pub fn propose_prediction(intervention: String, predicted_outcome: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "InterventionPredicted".into(),
            data: format!("{}|{}", intervention, predicted_outcome).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predicts_outcome() {
        let rules = vec![("restart".into(), "service_up".into())];
        let result = InterventionModel::predict("restart", &rules);
        assert_eq!(result, Some("service_up".into()));
    }
}
