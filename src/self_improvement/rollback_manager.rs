use crate::kernel_core::event::EventPayload;

pub struct RollbackManager;

impl RollbackManager {
    pub fn rollback(proposal_id: String, reason: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "RollbackExecuted".into(),
            data: format!("{}|{}", proposal_id, reason).into_bytes(),
        }
    }
}
