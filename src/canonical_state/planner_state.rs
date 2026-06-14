use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanStep { pub step_id: String, pub description: String, pub capability_id: Option<String>, pub estimated_cost: u64, pub dependencies: Vec<String>, pub status: StepStatus }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepStatus { Pending, Ready, InProgress, Completed, Failed }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plan { pub plan_id: String, pub goal_id: String, pub steps: Vec<PlanStep>, pub total_estimated_cost: u64, pub plan_status: PlanStatus }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanStatus { Draft, Validated, Executing, Completed, Failed }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannerState {
    pub active_plans: BTreeMap<String, Plan>,
    pub completed_plans: Vec<Plan>,
    pub failed_plans: Vec<Plan>,
    pub plan_counter: u64,
}

impl PlannerState {
    pub fn empty() -> Self { PlannerState { active_plans: BTreeMap::new(), completed_plans: Vec::new(), failed_plans: Vec::new(), plan_counter: 0 } }
    pub fn is_empty(&self) -> bool { self.active_plans.is_empty() && self.completed_plans.is_empty() && self.failed_plans.is_empty() && self.plan_counter == 0 }
}
