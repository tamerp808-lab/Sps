use crate::kernel_core::event::EventPayload;

pub struct GovernanceEvent;

impl GovernanceEvent {
    pub fn propose_decision(decision_id: &str, outcome: &str) -> EventPayload {
        EventPayload::Custom {
            event_type: "GovernanceDecision".into(),
            data: format!("{}|{}", decision_id, outcome).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decision_event() {
        let p = GovernanceEvent::propose_decision("d1", "Approved");
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
