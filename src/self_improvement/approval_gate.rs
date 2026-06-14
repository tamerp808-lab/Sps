use crate::kernel_core::event::EventPayload;

pub struct ApprovalGate;

impl ApprovalGate {
    pub fn approve(passed_sandbox: bool, passed_constitution: bool) -> bool {
        passed_sandbox && passed_constitution
    }

    pub fn propose_approval(proposal_id: String, approved: bool) -> EventPayload {
        EventPayload::Custom {
            event_type: "ProposalApproval".into(),
            data: format!("{}|{}", proposal_id, approved).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approves_only_if_both_pass() {
        assert!(ApprovalGate::approve(true, true));
        assert!(!ApprovalGate::approve(true, false));
    }
}
