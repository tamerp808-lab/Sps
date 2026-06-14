use crate::kernel_core::event::EventPayload;

pub struct ExplanationRanker;
impl ExplanationRanker {
    pub fn rank(explanations: &[String]) -> Vec<String> { let mut v = explanations.to_vec(); v.sort_by_key(|e| e.len()); v }
    pub fn propose_ranking(ranked: Vec<String>) -> EventPayload {
        EventPayload::Custom { event_type: "AbductiveExplanationsRanked".into(), data: ranked.join(";").into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn shortest_first() { assert_eq!(ExplanationRanker::rank(&["long".into(), "sh".into()]), vec!["sh", "long"]); }
}
