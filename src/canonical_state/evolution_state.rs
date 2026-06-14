use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvolutionProposal { pub proposal_id: String, pub description: String, pub target_component: String, pub proposed_by: String, pub epoch: u64, pub tick: u64, pub status: ProposalStatus }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalStatus { Draft, ImpactAnalysis, SandboxTesting, Approved, Deployed, Rejected { reason: String }, RolledBack { reason: String } }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxRun { pub run_id: String, pub proposal_id: String, pub passed: bool, pub replay_passed: bool, pub constitution_passed: bool, pub details: String }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvolutionState {
    pub proposals: BTreeMap<String, EvolutionProposal>,
    pub sandbox_runs: Vec<SandboxRun>,
    pub deployed_count: u64,
    pub rejected_count: u64,
    pub rolled_back_count: u64,
}

impl EvolutionState {
    pub fn empty() -> Self { EvolutionState { proposals: BTreeMap::new(), sandbox_runs: Vec::new(), deployed_count: 0, rejected_count: 0, rolled_back_count: 0 } }
    pub fn is_empty(&self) -> bool { self.proposals.is_empty() && self.sandbox_runs.is_empty() && self.deployed_count == 0 && self.rejected_count == 0 && self.rolled_back_count == 0 }
}
