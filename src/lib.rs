pub mod kernel_core {
    pub mod event_id;
    pub mod logical_time;
    pub mod event_hash;
    pub mod event_metadata;
    pub mod event;
    pub mod canonical_state;
    pub mod reducer;
    pub mod replay_validator;
    pub mod invariant_checker;
    pub mod constitution_checker;
    pub mod verification_suite;
    #[cfg(test)]
    pub mod replay_stress_test;
}
pub mod canonical_state;
pub mod memory;
pub mod world_model;
pub mod reasoning;
pub mod goal_system;
pub mod planner;
pub mod execution;
pub mod reflection;
pub mod self_improvement;
pub mod software_factory;
pub mod autonomy;
pub mod kernel_runtime;
pub mod governance;
pub mod cli {
    pub mod terminal;
    pub mod parser;
    pub mod commands;
    pub mod commands_extended;
}
pub mod storage;
pub mod llm;
pub mod context_engine;
pub mod sovereignty;
