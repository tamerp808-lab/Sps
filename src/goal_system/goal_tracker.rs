use crate::kernel_core::event::EventPayload;

pub struct GoalTracker;

impl GoalTracker {
    pub fn is_stuck(ticks_since_update: u64, threshold: u64) -> bool { ticks_since_update > threshold }

    pub fn propose_escalate(goal_id: String, reason: String) -> EventPayload {
        EventPayload::Custom { event_type: "GoalEscalated".into(), data: format!("{}|{}", goal_id, reason).into_bytes() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn detects() { assert!(GoalTracker::is_stuck(101, 100)); }
}
