use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalStatus { Proposed, Active, Suspended, Completed, Failed }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal { pub goal_id: String, pub description: String, pub priority: u64, pub supporting_values: Vec<String>, pub status: GoalStatus, pub parent_goal_id: Option<String> }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoalState {
    pub active_goals: BTreeMap<String, Goal>,
    pub completed_goals: Vec<Goal>,
    pub failed_goals: Vec<Goal>,
    pub value_priorities: ValuePriority,
    pub goal_counter: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValuePriority {
    pub core_order: Vec<CoreValue>,
    pub operational_order: Vec<OperationalValue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoreValue { Truth, Safety, ConstitutionCompliance }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationalValue { UserBenefit, ResourceEfficiency, KnowledgeGrowth, Autonomy, Transparency }

impl Default for ValuePriority {
    fn default() -> Self {
        ValuePriority {
            core_order: vec![CoreValue::Truth, CoreValue::Safety, CoreValue::ConstitutionCompliance],
            operational_order: vec![OperationalValue::UserBenefit, OperationalValue::ResourceEfficiency, OperationalValue::KnowledgeGrowth, OperationalValue::Autonomy, OperationalValue::Transparency],
        }
    }
}

impl GoalState {
    pub fn empty() -> Self { GoalState { active_goals: BTreeMap::new(), completed_goals: Vec::new(), failed_goals: Vec::new(), value_priorities: ValuePriority::default(), goal_counter: 0 } }
    pub fn is_empty(&self) -> bool { self.active_goals.is_empty() && self.completed_goals.is_empty() && self.failed_goals.is_empty() && self.goal_counter == 0 }
}
