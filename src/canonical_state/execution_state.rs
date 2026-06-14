use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionRecord { pub execution_id: String, pub plan_id: String, pub step_id: String, pub provider_id: String, pub status: ExecutionStatus, pub resources_consumed: ResourceUsage }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionStatus { Pending, Running, Completed, Failed }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceUsage { pub cpu_ms: u64, pub memory_bytes: u64, pub tokens_used: u64 }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionState {
    pub active_executions: BTreeMap<String, ExecutionRecord>,
    pub completed_executions: Vec<ExecutionRecord>,
    pub failed_executions: Vec<ExecutionRecord>,
    pub execution_counter: u64,
}

impl ExecutionState {
    pub fn empty() -> Self { ExecutionState { active_executions: BTreeMap::new(), completed_executions: Vec::new(), failed_executions: Vec::new(), execution_counter: 0 } }
    pub fn is_empty(&self) -> bool { self.active_executions.is_empty() && self.completed_executions.is_empty() && self.failed_executions.is_empty() && self.execution_counter == 0 }
}
