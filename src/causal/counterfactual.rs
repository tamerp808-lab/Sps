// src/reasoning/causal/counterfactual.rs
// Phase 5 — Reasoning
// Zone B — Cognitive

use crate::kernel_core::event::EventPayload;

pub struct Counterfactual;

impl Counterfactual {
    /// Asks "what if X had not happened?" — returns the alternative outcome.
    pub fn reason(actual_cause: &str, actual_effect: &str, alternative_cause: &str,
                  rules: &[(String, String)]) -> Option<String> {
        if actual_effect == rules.iter().find(|(c,_)| c == actual_cause).map(|(_,e)| e.clone()).unwrap_or_default() {
            rules.iter().find(|(c,_)| c == alternative_cause).map(|(_,e)| e.clone())
        } else { None }
    }

    pub fn propose_counterfactual(actual: String, alternative: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "CounterfactualReasoned".into(),
            data: format!("{}|{}", actual, alternative).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counterfactual_gives_alternative() {
        let rules = vec![("rain".into(), "wet".into()), ("sun".into(), "dry".into())];
        let result = Counterfactual::reason("rain", "wet", "sun", &rules);
        assert_eq!(result, Some("dry".into()));
    }
}
