use crate::cli::commands::CommandExecutor;
use crate::cli::terminal::Terminal;
use crate::goal_system::goal_evaluator::GoalEvaluator;
use crate::planner::planner_core::PlannerCore;
use crate::planner::plan::PlanManager;
use crate::canonical_state::goal_state::ValuePriority;
use crate::canonical_state::planner_state::StepStatus;
use crate::kernel_core::event::EventPayload;

pub struct ProactivePlanner;

impl ProactivePlanner {
    pub fn tick(executor: &mut CommandExecutor) {
        if !executor.is_running() { return; }

        let values = ValuePriority::default();
        let goals: Vec<_> = executor.state().goals.active_goals.values().cloned().collect();
        if goals.is_empty() { return; }

        let mut scored_goals: Vec<_> = goals.iter().map(|g| (g.clone(), GoalEvaluator::score(g, &values))).collect();
        scored_goals.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // أولاً جمع البيانات التي نحتاجها لتجنب الاقتراض المتداخل
        let mut plans_to_create = Vec::new();
        let mut steps_to_execute = Vec::new();

        for (goal, score) in scored_goals.iter().take(1) {
            let has_plan = executor.state().planner.active_plans.values().any(|p| p.goal_id == goal.goal_id);
            if !has_plan {
                let caps = if let Some(ref llm) = executor.llm() {
                    let prompt = format!("لتحقيق الهدف '{}'، اقترح 3 خطوات بالعربية.", goal.description);
                    if let Ok(reply) = llm.chat(&prompt) {
                        reply.lines().take(3).enumerate().map(|(i, step)| {
                            (format!("step-{}", i+1), step.to_string())
                        }).collect()
                    } else {
                        vec![("analyze".into(), format!("Analyze: {}", goal.description))]
                    }
                } else {
                    vec![("analyze".into(), format!("Analyze: {}", goal.description))]
                };

                if let Some(plan) = PlannerCore::create_plan(
                    format!("auto-plan-{}", executor.tick()),
                    goal,
                    &caps,
                    &values,
                ) {
                    plans_to_create.push((plan.plan_id.clone(), goal.goal_id.clone(), plan.steps.clone(), format!("[PROACTIVE] Auto-plan for '{}' (score: {:.0})", goal.description, score)));
                }
            }
        }

        // جمع الخطوات المعلقة
        for (goal, _) in scored_goals.iter().take(1) {
            if let Some(plan) = executor.state().planner.active_plans.values().find(|p| p.goal_id == goal.goal_id) {
                if let Some(step) = plan.steps.iter().find(|s| s.status == StepStatus::Pending) {
                    steps_to_execute.push((plan.plan_id.clone(), step.step_id.clone(), format!("[PROACTIVE] Auto-executing step '{}'", step.step_id)));
                }
            }
        }

        // الآن نطبق التغييرات (نقترض بشكل متغير مرة واحدة)
        for (plan_id, goal_id, steps, msg) in plans_to_create {
            executor.apply_event(PlanManager::propose_create(plan_id, goal_id, steps));
            Terminal::print_line(&msg);
        }

        for (plan_id, step_id, msg) in steps_to_execute {
            executor.apply_event(EventPayload::Custom {
                event_type: "ExecutionStep".into(),
                data: format!("step={}|plan={}", step_id, plan_id).into_bytes(),
            });
            Terminal::print_line(&msg);
        }
    }
}
