use crate::canonical_state::goal_state::{GoalState, Goal, GoalStatus};
use crate::kernel_core::event::EventPayload;

pub struct GoalRegistry;

impl GoalRegistry {
    pub fn active(state: &GoalState) -> Vec<&Goal> {
        state.active_goals.values().filter(|g| g.status == GoalStatus::Active).collect()
    }

    pub fn propose_activate(goal_id: String) -> EventPayload {
        EventPayload::Custom { event_type: "GoalActivated".into(), data: goal_id.into_bytes() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical_state::goal_state::Goal;

    #[test]
    fn active_filters() {
        let mut state = GoalState::empty();
        state.active_goals.insert("g1".into(), Goal { parent_goal_id: None, goal_id: "g1".into(), description: "d".into(), priority: 1, supporting_values: vec![], status: GoalStatus::Active });
        state.active_goals.insert("g2".into(), Goal { parent_goal_id: None, goal_id: "g2".into(), description: "d".into(), priority: 1, supporting_values: vec![], status: GoalStatus::Completed });
        assert_eq!(GoalRegistry::active(&state).len(), 1);
    }
}
