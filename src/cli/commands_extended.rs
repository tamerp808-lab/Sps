use crate::canonical_state::CanonicalState;
use crate::cli::terminal::Terminal;
use crate::memory::memory_manager::MemoryManager;
use crate::goal_system::goal::GoalManager;
use crate::reflection::reflector::Reflector;
use crate::reasoning::inductive::pattern_detector::PatternDetector;
use crate::world_model::world_graph::WorldGraph;
use crate::world_model::reality_checker::RealityChecker;
use crate::world_model::causal_graph::CausalGraph;
use crate::software_factory::project_builder::ProjectBuilder;
use crate::software_factory::requirements::RequirementsExtractor;
use crate::software_factory::implementation::CodeValidator;
use crate::autonomy::autonomy_manager::AutonomyManager;
use crate::governance::governance_policy::GovernancePolicy;
use crate::governance::authority_resolver::AuthorityResolver;
use crate::governance::governance_policy::DecisionClass;
use crate::kernel_runtime::checkpoint_manager::CheckpointManager;
use crate::kernel_runtime::recovery_manager::RecoveryManager;
use crate::kernel_runtime::upgrade_manager::UpgradeManager;
use crate::kernel_core::event::EventPayload;
use crate::kernel_core::replay_validator::ReplayValidator;
use crate::kernel_core::invariant_checker::InvariantChecker;
use crate::kernel_core::verification_suite::VerificationSuite;
use crate::kernel_core::reducer::KernelReducer;

pub struct ExtendedCommands;

impl ExtendedCommands {
    pub fn memory_add(executor: &mut super::commands::CommandExecutor, content: &str) {
        if !executor.is_running() { Terminal::print_line("Boot first."); return; }
        executor.apply_event(MemoryManager::propose_semantic_fact("user".into(), "said".into(), content.to_string(), 0.9));
        Terminal::print_line(&format!("Added to memory: {}", content));
    }
    pub fn memory_episode(executor: &mut super::commands::CommandExecutor, desc: &str) {
        if !executor.is_running() { Terminal::print_line("Boot first."); return; }
        executor.apply_event(MemoryManager::propose_episode(desc.to_string()));
        Terminal::print_line(&format!("Episode recorded: {}", desc));
    }
    pub fn memory_search(state: &CanonicalState, keyword: &str) {
        let results = MemoryManager::query(&state.memory, keyword);
        if results.is_empty() { Terminal::print_line(&format!("No memories for '{}'.", keyword)); }
        else {
            Terminal::print_line(&format!("Found {} memories:", results.len()));
            for (i, r) in results.iter().enumerate() {
                Terminal::print_line(&format!("  {}. [{}] {} (relevance: {:.2})", i+1, format!("{:?}", r.layer), r.content, r.relevance));
            }
        }
    }
    pub fn world_entities(state: &CanonicalState) {
        let entities: Vec<_> = state.world.entities.values().collect();
        if entities.is_empty() { Terminal::print_line("No entities."); }
        else {
            Terminal::print_line(&format!("Entities ({}):", entities.len()));
            for e in entities { Terminal::print_line(&format!("  {} — {:?}", e.id.0, e.entity_type)); }
        }
    }
    pub fn world_relation_add(executor: &mut super::commands::CommandExecutor, from: &str, relation: &str, to: &str) {
        if !executor.is_running() { Terminal::print_line("Boot first."); return; }
        let payload = WorldGraph::propose_create(
            crate::canonical_state::world_state::RelationId(format!("rel.{}", executor.tick())),
            crate::canonical_state::world_state::EntityId(from.to_string()),
            relation.to_string(),
            crate::canonical_state::world_state::EntityId(to.to_string()),
            0.8, false,
        );
        executor.apply_event(payload);
        Terminal::print_line(&format!("Relation added: {} --[{}]--> {}", from, relation, to));
    }
    pub fn world_check(executor: &mut super::commands::CommandExecutor) {
        if !executor.is_running() { Terminal::print_line("Boot first."); return; }
        let result = RealityChecker::check(&executor.state().world);
        if result.passed { Terminal::print_line("Reality check passed."); }
        else {
            Terminal::print_line("Reality check FAILED:");
            for v in &result.violations { Terminal::print_line(&format!("  - {}", v.description)); }
        }
        executor.apply_event(EventPayload::Custom { event_type: "RealityCheck".into(), data: result.passed.to_string().into_bytes() });
    }
    pub fn goal_create(executor: &mut super::commands::CommandExecutor, desc: &str) {
        if !executor.is_running() { Terminal::print_line("Boot first."); return; }
        let goal_id = format!("goal.{}", executor.tick() + 1);
        let payload = GoalManager::propose_create(goal_id.clone(), desc.to_string(), 5, vec!["UserBenefit".into()], None);
        executor.apply_event(payload);
        Terminal::print_line(&format!("Goal created: {} — '{}'", goal_id, desc));
    }
    pub fn goal_list(state: &CanonicalState) {
        let goals = crate::goal_system::goal_registry::GoalRegistry::active(&state.goals);
        if goals.is_empty() { Terminal::print_line("No active goals."); }
        else {
            Terminal::print_line("Active goals:");
            for g in goals { Terminal::print_line(&format!("  {} [priority={}] — {}", g.goal_id, g.priority, g.description)); }
        }
    }
    pub fn reason_causal(executor: &mut super::commands::CommandExecutor, cause: &str, effect: &str) {
        if !executor.is_running() { Terminal::print_line("Boot first."); return; }
        let payload = CausalGraph::propose_record_link(
            Some(crate::canonical_state::world_state::EntityId(cause.to_string())),
            Some(crate::canonical_state::world_state::EntityId(effect.to_string())),
            None, None, 0.9, format!("{} causes {}", cause, effect),
        );
        executor.apply_event(payload);
        Terminal::print_line(&format!("Causal link recorded: {} → {}", cause, effect));
    }
    pub fn reason_pattern(state: &CanonicalState) {
        let facts: Vec<String> = state.memory.semantic.values().map(|f| format!("{} {} {}", f.subject, f.predicate, f.object)).collect();
        if let Some(pattern) = PatternDetector::detect(&facts) { Terminal::print_line(&format!("Pattern: {}", pattern)); }
        else { Terminal::print_line("No pattern detected."); }
    }
    pub fn factory_start(executor: &mut super::commands::CommandExecutor, project: &str) {
        if !executor.is_running() { Terminal::print_line("Boot first."); return; }
        executor.apply_event(ProjectBuilder::propose_build(project.to_string(), vec!["fast".into(), "reliable".into()]));
        Terminal::print_line(&format!("Software factory started for project: {}", project));
    }
    pub fn factory_requirements(executor: &mut super::commands::CommandExecutor, desc: &str) {
        if !executor.is_running() { Terminal::print_line("Boot first."); return; }
        executor.apply_event(RequirementsExtractor::propose_requirement("proj.1".into(), desc.to_string(), 5));
        Terminal::print_line(&format!("Requirement added: {}", desc));
    }
    pub fn factory_validate_code(code: &str) {
        if CodeValidator::validate(code) { Terminal::print_line("Code validation passed."); }
        else { Terminal::print_line("Code validation failed."); }
    }
    pub fn autonomy_grant(executor: &mut super::commands::CommandExecutor, agent: &str, domain: &str) {
        if !executor.is_running() { Terminal::print_line("Boot first."); return; }
        executor.apply_event(AutonomyManager::propose_grant(agent.to_string(), domain.to_string()));
        Terminal::print_line(&format!("Autonomy granted to {} in domain {}", agent, domain));
    }
    pub fn governance_policy(executor: &mut super::commands::CommandExecutor) {
        if !executor.is_running() { Terminal::print_line("Boot first."); return; }
        let rules = GovernancePolicy::default_rules();
        executor.apply_event(GovernancePolicy::propose_set_policy(&rules));
        Terminal::print_line("Governance policy set.");
    }
    pub fn governance_check(decision: &str) {
        let dc = match decision {
            "minor" => DecisionClass::Minor, "major" => DecisionClass::Major, "critical" => DecisionClass::Critical,
            _ => { Terminal::print_line("Invalid class."); return; }
        };
        let rules = GovernancePolicy::default_rules();
        if let Some(auth) = AuthorityResolver::resolve(&dc, &rules) { Terminal::print_line(&format!("Decision '{}' requires {:?}", decision, auth)); }
    }
    pub fn runtime_checkpoint(executor: &mut super::commands::CommandExecutor) {
        if !executor.is_running() { Terminal::print_line("Boot first."); return; }
        executor.apply_event(CheckpointManager::propose_checkpoint());
        Terminal::print_line("Checkpoint created.");
    }
    pub fn runtime_recover(executor: &mut super::commands::CommandExecutor, reason: &str) {
        if !executor.is_running() { Terminal::print_line("Boot first."); return; }
        executor.apply_event(RecoveryManager::propose_recover(reason));
        Terminal::print_line(&format!("Recovery initiated: {}", reason));
    }
    pub fn runtime_upgrade(executor: &mut super::commands::CommandExecutor, version: &str) {
        if !executor.is_running() { Terminal::print_line("Boot first."); return; }
        executor.apply_event(UpgradeManager::propose_upgrade(version));
        Terminal::print_line(&format!("Upgrade to {} initiated.", version));
    }
    pub fn reflect(executor: &mut super::commands::CommandExecutor) {
        if !executor.is_running() { Terminal::print_line("Boot first."); return; }
        let reflection = Reflector::reflect("expected", "actual");
        executor.apply_event(EventPayload::Custom { event_type: "Reflection".into(), data: reflection.clone().into_bytes() });
        Terminal::print_line(&format!("Reflection: {}", reflection));
    }
    pub fn insight(state: &CanonicalState) {
        let facts: Vec<String> = state.memory.semantic.values().map(|f| format!("{} {} {}", f.subject, f.predicate, f.object)).collect();
        if facts.len() < 3 { Terminal::print_line("Need ≥3 facts."); return; }
        if let Some(pattern) = PatternDetector::detect(&facts) { Terminal::print_line(&format!("Insight: {}", pattern)); }
        else { Terminal::print_line("No clear insight."); }
    }
    pub fn analyze(state: &CanonicalState) {
        Terminal::print_line(&format!("Facts: {}, Episodes: {}, Goals: {}, Events: {}, Hash: {}",
            state.memory.semantic.len(), state.memory.episodic.len(), state.goals.active_goals.len(), state.kernel.event_count, state.kernel.state_hash));
    }
    pub fn show_hash(state: &CanonicalState) { Terminal::print_line(&format!("State hash: {}", state.kernel.state_hash)); }
    pub fn show_events(state: &CanonicalState) { Terminal::print_line(&format!("Events: {}, Last ID: {}", state.kernel.event_count, state.kernel.last_event_id)); }
    pub fn replay_verify(executor: &mut super::commands::CommandExecutor) {
        if !executor.is_running() { Terminal::print_line("Boot first."); return; }
        let events = executor.event_log();
        if events.is_empty() { Terminal::print_line("No events to replay."); return; }
        let result = ReplayValidator::validate(&KernelReducer, &events, &executor.state().kernel);
        if result.passed { Terminal::print_line("✅ Replay verification PASSED."); }
        else { Terminal::print_line(&format!("❌ Replay FAILED: {}", result.diagnostic)); }
    }
    pub fn invariant_check(executor: &mut super::commands::CommandExecutor) {
        if !executor.is_running() { Terminal::print_line("Boot first."); return; }
        let events = executor.event_log();
        if events.is_empty() { Terminal::print_line("No events to check."); return; }
        let report = InvariantChecker::check_all(&executor.state().kernel, &events, &KernelReducer);
        if report.all_passed { Terminal::print_line("✅ All invariants passed."); }
        else {
            Terminal::print_line("❌ Invariant violations:");
            for f in &report.failures { Terminal::print_line(&format!("  - {}: {}", f.invariant_name, f.description)); }
        }
    }
    pub fn full_verification(executor: &mut super::commands::CommandExecutor) {
        if !executor.is_running() { Terminal::print_line("Boot first."); return; }
        let events = executor.event_log();
        if events.is_empty() { Terminal::print_line("No events to verify."); return; }
        let report = VerificationSuite::verify(&KernelReducer, &events, &executor.state().kernel);
        if report.passed { Terminal::print_line("✅ Full verification PASSED."); }
        else { Terminal::print_line(&format!("❌ Verification FAILED: {}", report.summary)); }
    }
    pub fn approve_proposal(executor: &mut super::commands::CommandExecutor, proposal_id: &str) {
        if !executor.is_running() { Terminal::print_line("Boot first."); return; }
        executor.apply_event(EventPayload::Custom {
            event_type: "ProposalApproved".into(),
            data: format!("{}|approved by user", proposal_id).into_bytes(),
        });
        Terminal::print_line(&format!("Proposal '{}' approved and deployed.", proposal_id));
    }
    pub fn deny_proposal(executor: &mut super::commands::CommandExecutor, proposal_id: &str) {
        if !executor.is_running() { Terminal::print_line("Boot first."); return; }
        executor.apply_event(EventPayload::Custom {
            event_type: "ProposalDenied".into(),
            data: format!("{}|denied by user", proposal_id).into_bytes(),
        });
        Terminal::print_line(&format!("Proposal '{}' denied.", proposal_id));
    }
}
