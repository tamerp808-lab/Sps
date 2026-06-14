use crate::canonical_state::goal_state::GoalStatus;
use crate::kernel_core::event::EventPayload;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Goal {
    pub goal_id: String,
    pub description: String,
    pub priority: u64,
    pub supporting_values: Vec<String>,
    pub status: GoalStatus,
    pub parent_goal_id: Option<String>,
}

pub struct GoalManager;

impl GoalManager {
    pub fn propose_create(
        goal_id: String, description: String, priority: u64,
        supporting_values: Vec<String>, parent_goal_id: Option<String>,
    ) -> EventPayload {
        let values_str = supporting_values.join(",");
        let parent = parent_goal_id.unwrap_or_default();
        EventPayload::Custom {
            event_type: "GoalCreated".into(),
            data: format!("{}|{}|{}|{}|{}", goal_id, description, priority, values_str, parent).into_bytes(),
        }
    }

    pub fn propose_update_status(goal_id: String, status: GoalStatus) -> EventPayload {
        EventPayload::Custom {
            event_type: "GoalStatusUpdated".into(),
            data: format!("{}|{:?}", goal_id, status).into_bytes(),
        }
    }
}
