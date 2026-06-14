use crate::kernel_core::event::EventPayload;

pub struct CausalReasoner;
impl CausalReasoner {
    pub fn infer_cause(effect: &str, pairs: &[(String, String)]) -> Option<String> { pairs.iter().find(|(_,e)| e == effect).map(|(c,_)| c.clone()) }
    pub fn propose_causal_link(cause: String, effect: String, confidence: f64) -> EventPayload {
        EventPayload::Custom { event_type: "CausalLinkInferred".into(), data: format!("{}|{}|{}", cause, effect, confidence).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn infers() { assert_eq!(CausalReasoner::infer_cause("wet", &[("rain".into(), "wet".into())]), Some("rain".into())); }
}
