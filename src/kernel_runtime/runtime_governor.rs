use crate::kernel_core::event::EventPayload;

pub struct RuntimeGovernor;

impl RuntimeGovernor {
    pub fn propose_govern(rule: &str) -> EventPayload {
        EventPayload::Custom {
            event_type: "RuntimeGovernance".into(),
            data: rule.to_string().into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn govern_event() {
        let p = RuntimeGovernor::propose_govern("Enforce Zone A purity");
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
