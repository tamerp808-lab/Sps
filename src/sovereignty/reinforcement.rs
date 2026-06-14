use crate::cli::commands::CommandExecutor;
use crate::kernel_core::event::EventPayload;

pub struct Reinforcement;

impl Reinforcement {
    pub fn tick(executor: &mut CommandExecutor) {
        if !executor.is_running() { return; }
        if executor.tick() % 50 != 0 { return; }

        // جمع البيانات أولاً
        let completed_goal_ids: Vec<String> = executor.state().goals.active_goals
            .values()
            .filter(|g| g.description.contains("Completed"))
            .map(|g| g.goal_id.clone())
            .collect();

        let high_confidence_ids: Vec<String> = executor.state().memory.semantic
            .iter()
            .filter(|(_, f)| f.confidence.0 > 0.8)
            .map(|(id, _)| id.clone())
            .collect();

        // الآن نطبق التغييرات
        for goal_id in completed_goal_ids {
            executor.apply_event(EventPayload::Custom {
                event_type: "Reinforcement".into(),
                data: format!("goal={}|outcome=success", goal_id).into_bytes(),
            });
        }

        for id in high_confidence_ids {
            executor.apply_event(EventPayload::Custom {
                event_type: "ConfidenceBoost".into(),
                data: format!("{}|0.01", id).into_bytes(),
            });
        }
    }
}
