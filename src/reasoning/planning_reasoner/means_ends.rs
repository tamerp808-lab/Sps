use crate::kernel_core::event::EventPayload;

pub struct MeansEnds;
impl MeansEnds {
    pub fn find_means(goal: &str, capabilities: &[(String, String)]) -> Option<String> {
        capabilities.iter().find(|(_, desc)| goal.contains(desc.as_str())).map(|(id, _)| id.clone())
    }
    pub fn propose_means(goal: String, cap_id: String) -> EventPayload {
        EventPayload::Custom { event_type: "MeansEndsFound".into(), data: format!("{}|{}", goal, cap_id).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn finds() { let caps = vec![("read".into(), "reading files".into())]; assert_eq!(MeansEnds::find_means("reading files quickly", &caps), Some("read".into())); }
}
