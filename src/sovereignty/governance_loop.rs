use crate::governance::governance_policy::{DecisionClass, GovernancePolicy};
use crate::governance::authority_resolver::AuthorityResolver;
use crate::cli::terminal::Terminal;

pub struct GovernanceLoop;

impl GovernanceLoop {
    pub fn evaluate(_executor: &mut crate::cli::commands::CommandExecutor, event_type: &str) {
        let dc = match event_type {
            "MemorySemanticFactProposed" | "PlanCreated" => DecisionClass::Minor,
            "GoalCreated" | "ExecutionStep" => DecisionClass::Major,
            _ => DecisionClass::Minor,
        };

        let rules = GovernancePolicy::default_rules();
        if let Some(auth) = AuthorityResolver::resolve(&dc, &rules) {
            if dc == DecisionClass::Minor {
                // يُنفذ تلقائياً
            } else {
                Terminal::print_line(&format!("[GOV] Event '{}' requires {:?}", event_type, auth));
            }
        }
    }
}
