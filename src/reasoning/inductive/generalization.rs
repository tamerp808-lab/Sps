use crate::kernel_core::event::EventPayload;
use std::collections::HashMap;

pub struct Generalization;
impl Generalization {
    pub fn generalize(facts: &[String]) -> Option<String> {
        let mut counts = HashMap::new();
        for f in facts { *counts.entry(f.as_str()).or_insert(0) += 1; }
        counts.into_iter().max_by_key(|(_,c)| *c).map(|(k,_)| k.to_string())
    }
    pub fn propose_generalization(rule: String, confidence: f64) -> EventPayload {
        EventPayload::Custom { event_type: "InductiveGeneralization".into(), data: format!("{}|{}", rule, confidence).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn picks_most() { assert_eq!(Generalization::generalize(&["a".into(), "a".into(), "b".into()]), Some("a".into())); }
}
