use crate::kernel_core::event::EventPayload;

pub struct DeploymentController;

impl DeploymentController {
    pub fn deploy(proposal_id: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "DeploymentStarted".into(),
            data: proposal_id.into_bytes(),
        }
    }
}
