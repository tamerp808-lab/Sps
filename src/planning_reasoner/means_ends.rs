use crate::kernel_core::event::EventPayload;

pub struct MeansEnds;

impl MeansEnds {
    /// Finds the first capability that matches the goal description.
    pub fn find_means(goal: &str, capabilities: &[(String, String)]) -> Option<String> {
        capabilities.iter()
            .find(|(_, desc)| goal.contains(desc.as_str()))
            .map(|(id, _)| id.clone())
    }

    pub fn propose_means(goal: String, capability_id: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "MeansEndsFound".into(),
            data: format!("{}|{}", goal, capability_id).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_matching_capability() {
        let caps = vec![("read".into(), "reading files".into())];
        let result = MeansEnds::find_means("reading files quickly", &caps);
        assert_eq!(result, Some("read".into()));
    }
}
