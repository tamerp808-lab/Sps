// src/reasoning/inductive/generalization.rs
// Phase 5 — Reasoning
// Zone B — Cognitive

use crate::kernel_core::event::EventPayload;

pub struct Generalization;

impl Generalization {
    /// Generalizes from a set of specific facts to a general rule.
    /// Placeholder: returns the most frequent predicate-object pair.
    pub fn generalize(facts: &[String]) -> Option<String> {
        let mut counts: std::collections::HashMap<&str, u32> = std::collections::HashMap::new();
        for fact in facts {
            *counts.entry(fact.as_str()).or_insert(0) += 1;
        }
        counts.into_iter().max_by_key(|(_,c)| *c).map(|(k,_)| k.to_string())
    }

    pub fn propose_generalization(rule: String, confidence: f64) -> EventPayload {
        EventPayload::Custom {
            event_type: "InductiveGeneralization".into(),
            data: format!("{}|{}", rule, confidence).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_most_common() {
        let facts = vec!["a is fast".into(), "b is fast".into(), "a is fast".into()];
        let result = Generalization::generalize(&facts);
        assert_eq!(result, Some("a is fast".into()));
    }
}
