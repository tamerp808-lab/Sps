use crate::kernel_core::event::EventPayload;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Syllogism { pub major: String, pub minor: String, pub conclusion: String }

pub struct SyllogismEngine;
impl SyllogismEngine {
    pub fn evaluate(s: &Syllogism, facts: &[String]) -> bool {
        facts.iter().any(|f| f.contains(&s.major)) && facts.iter().any(|f| f.contains(&s.minor))
    }
    pub fn propose_syllogism(s: &Syllogism) -> EventPayload {
        EventPayload::Custom { event_type: "SyllogismEvaluated".into(), data: format!("{}|{}|{}", s.major, s.minor, s.conclusion).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn valid() { let s = Syllogism { major: "All A are B".into(), minor: "C is A".into(), conclusion: "C is B".into() }; assert!(SyllogismEngine::evaluate(&s, &["All A are B".into(), "C is A".into()])); }
}
