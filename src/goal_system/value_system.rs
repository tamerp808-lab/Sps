// src/goal_system/value_system.rs
// Phase 6 — Goal System
// Zone B — Cognitive
//
// Purpose:
//   Defines the core and operational values that guide goal
//   selection. Core values (Truth, Safety, ConstitutionCompliance)
//   are never violated. Operational values can be re-prioritized.
//   Reads from GoalState and produces Events.

use crate::kernel_core::event::EventPayload;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CoreValue { Truth, Safety, ConstitutionCompliance }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OperationalValue { UserBenefit, ResourceEfficiency, KnowledgeGrowth, Autonomy, Transparency }

/// The ordered priority of values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValuePriorities {
    pub core: Vec<CoreValue>,
    pub operational: Vec<OperationalValue>,
}

impl Default for ValuePriorities {
    fn default() -> Self {
        ValuePriorities {
            core: vec![CoreValue::Truth, CoreValue::Safety, CoreValue::ConstitutionCompliance],
            operational: vec![OperationalValue::UserBenefit, OperationalValue::ResourceEfficiency, OperationalValue::KnowledgeGrowth, OperationalValue::Autonomy, OperationalValue::Transparency],
        }
    }
}

pub struct ValueSystem;

impl ValueSystem {
    /// Returns true if the goal supports at least one core value.
    pub fn supports_core(goal_values: &[String], priorities: &ValuePriorities) -> bool {
        priorities.core.iter().any(|cv| goal_values.contains(&format!("{:?}", cv)))
    }

    /// Proposes updating operational priorities (core is immutable).
    pub fn propose_update_operational(new_order: Vec<OperationalValue>) -> EventPayload {
        let order_str = new_order.iter().map(|v| format!("{:?}", v)).collect::<Vec<_>>().join(",");
        EventPayload::Custom {
            event_type: "ValueSystemUpdated".into(),
            data: order_str.into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_value_match() {
        let p = ValuePriorities::default();
        assert!(ValueSystem::supports_core(&["Truth".into()], &p));
        assert!(!ValueSystem::supports_core(&["Autonomy".into()], &p));
    }
}
