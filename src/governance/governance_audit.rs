use crate::kernel_core::event::EventPayload;

pub struct GovernanceAudit;

impl GovernanceAudit {
    pub fn propose_audit_entry(entry: &str) -> EventPayload {
        EventPayload::Custom {
            event_type: "GovernanceAudit".into(),
            data: entry.to_string().into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_event() {
        let p = GovernanceAudit::propose_audit_entry("Decision X was made by System without delegation");
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
