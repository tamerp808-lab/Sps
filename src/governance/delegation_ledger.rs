use crate::kernel_core::event::EventPayload;

pub struct DelegationLedger;

impl DelegationLedger {
    pub fn propose_delegate(from: &str, to: &str, decision_class: &str, allowed: bool) -> EventPayload {
        EventPayload::Custom {
            event_type: "DelegationRecorded".into(),
            data: format!("{}|{}|{}|{}", from, to, decision_class, allowed).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delegation_event() {
        let p = DelegationLedger::propose_delegate("User", "System", "Minor", true);
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
