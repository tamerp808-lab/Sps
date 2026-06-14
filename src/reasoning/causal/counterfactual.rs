use crate::kernel_core::event::EventPayload;

pub struct Counterfactual;
impl Counterfactual {
    pub fn reason(actual_cause: &str, actual_effect: &str, alternative: &str, rules: &[(String, String)]) -> Option<String> {
        if Some(actual_effect.to_string()) == rules.iter().find(|(c,_)| c == actual_cause).map(|(_,e)| e.clone()) {
            rules.iter().find(|(c,_)| c == alternative).map(|(_,e)| e.clone())
        } else { None }
    }
    pub fn propose_counterfactual(actual: String, alternative: String) -> EventPayload {
        EventPayload::Custom { event_type: "CounterfactualReasoned".into(), data: format!("{}|{}", actual, alternative).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn reasons() {
        let rules = vec![("rain".into(), "wet".into()), ("sun".into(), "dry".into())];
        assert_eq!(Counterfactual::reason("rain", "wet", "sun", &rules), Some("dry".into()));
    }
}
