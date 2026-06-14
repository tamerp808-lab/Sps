use crate::kernel_core::event::EventPayload;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule { pub rule_id: String, pub name: String, pub premises: Vec<String>, pub conclusion: String }

pub struct RuleEngine;
impl RuleEngine {
    pub fn apply(rule: &Rule, facts: &[String]) -> Option<String> {
        if rule.premises.iter().all(|p| facts.iter().any(|f| f.contains(p))) { Some(rule.conclusion.clone()) } else { None }
    }
    pub fn propose_inference(rule_id: String, premises: Vec<String>, conclusion: String, confidence: f64) -> EventPayload {
        EventPayload::Custom { event_type: "DeductiveInference".into(), data: format!("{}|{}|{}|{}", rule_id, premises.join(","), conclusion, confidence).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn fires() { let r = Rule { rule_id: "r1".into(), name: "mp".into(), premises: vec!["X is Y".into()], conclusion: "Z".into() }; assert_eq!(RuleEngine::apply(&r, &["X is Y".into()]), Some("Z".into())); }
    #[test] fn no_fire() { let r = Rule { rule_id: "r2".into(), name: "x".into(), premises: vec!["A".into()], conclusion: "B".into() }; assert_eq!(RuleEngine::apply(&r, &[]), None); }
}
