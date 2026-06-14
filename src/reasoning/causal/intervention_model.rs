use crate::kernel_core::event::EventPayload;

pub struct InterventionModel;
impl InterventionModel {
    pub fn predict(intervention: &str, rules: &[(String, String)]) -> Option<String> { rules.iter().find(|(c,_)| c == intervention).map(|(_,e)| e.clone()) }
    pub fn propose_prediction(intervention: String, outcome: String) -> EventPayload {
        EventPayload::Custom { event_type: "InterventionPredicted".into(), data: format!("{}|{}", intervention, outcome).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn predicts() { assert_eq!(InterventionModel::predict("restart", &[("restart".into(), "up".into())]), Some("up".into())); }
}
