use crate::cli::commands::CommandExecutor;
use crate::cli::terminal::Terminal;
use crate::goal_system::goal::GoalManager;
use crate::reasoning::deductive::consistency_checker::ConsistencyChecker;
use crate::reasoning::abductive::hypothesis_generator::HypothesisGenerator;
use crate::reasoning::probabilistic::probabilistic_reasoner::ProbabilisticReasoner;
use crate::reasoning::causal::counterfactual::Counterfactual;
use crate::reasoning::self_reasoner::blind_spot_detector::BlindSpotDetector;
use crate::reasoning::self_reasoner::reasoning_auditor::ReasoningAuditor;
use crate::reasoning::inductive::pattern_detector::PatternDetector;
use crate::sovereignty::software_factory_loop::SoftwareFactoryLoop;
use crate::sovereignty::curiosity::CuriosityEngine;
use crate::sovereignty::insight_engine::InsightEngine;
use crate::sovereignty::proactive_planner::ProactivePlanner;
use crate::sovereignty::reinforcement::Reinforcement;
use crate::sovereignty::memory_governor::MemoryGovernor;
use crate::kernel_core::replay_validator::ReplayValidator;
use crate::kernel_core::invariant_checker::InvariantChecker;
use crate::kernel_core::constitution_checker::ConstitutionChecker;
use crate::kernel_core::reducer::KernelReducer;
use crate::kernel_core::event::EventPayload;
use crate::kernel_runtime::recovery_manager::RecoveryManager;

pub struct SelfAwareness;

impl SelfAwareness {
    pub fn tick(executor: &mut CommandExecutor) {
        if !executor.is_running() { return; }

        let event_count = executor.state().kernel.event_count;

        if event_count > 0 && event_count % 100 == 0 {
            let events = executor.event_log();
            if !events.is_empty() {
                let replay_result = ReplayValidator::validate(&KernelReducer, &events, &executor.state().kernel);
                if !replay_result.passed {
                    let msg = format!("Replay FAILED: {}", replay_result.diagnostic);
                    Terminal::print_line(&format!("[CRITICAL] {}", msg));
                    executor.apply_event(EventPayload::Failure { component: "Replay".into(), class: "Catastrophic".into(), message: msg.clone() });
                    executor.apply_event(RecoveryManager::propose_recover(&msg));
                }

                let invariant_report = InvariantChecker::check_all(&executor.state().kernel, &events, &KernelReducer);
                if !invariant_report.all_passed {
                    for f in &invariant_report.failures {
                        let msg = format!("Invariant: {} - {}", f.invariant_name, f.description);
                        executor.apply_event(EventPayload::Failure { component: "Invariant".into(), class: "Catastrophic".into(), message: msg.clone() });
                        executor.apply_event(RecoveryManager::propose_recover(&msg));
                    }
                }

                let constitution_report = ConstitutionChecker::audit(&executor.state().kernel, &events, &KernelReducer);
                if !constitution_report.compliant {
                    for v in &constitution_report.violations {
                        let msg = format!("Constitution: [{}] {}", v.rule, v.description);
                        executor.apply_event(EventPayload::Failure { component: "Constitution".into(), class: "Catastrophic".into(), message: msg.clone() });
                        executor.apply_event(RecoveryManager::propose_recover(&msg));
                    }
                }
            }
        }

        let facts: Vec<String> = executor.state().memory.semantic.values().map(|f| format!("{} {} {}", f.subject, f.predicate, f.object)).collect();
        if !facts.is_empty() {
            let base_facts: Vec<String> = facts.iter().take(10).cloned().collect();
            for i in 0..base_facts.len() {
                for j in i+1..base_facts.len() {
                    if !ConsistencyChecker::is_consistent(&base_facts[i], &[base_facts[j].clone()]) {
                        executor.apply_event(GoalManager::propose_create(format!("resolve-{}", executor.tick()), format!("Resolve: {} vs {}", base_facts[i], base_facts[j]), 8, vec!["Truth".into()], None));
                    }
                }
            }
            if let Some(latest) = base_facts.last() {
                for hyp in HypothesisGenerator::generate(latest, &base_facts[..base_facts.len()-1]).iter().take(2) {
                    executor.apply_event(GoalManager::propose_create(format!("explore-{}", executor.tick()), format!("Explore: {}", hyp), 5, vec!["KnowledgeGrowth".into()], None));
                }
            }
            if base_facts.len() >= 2 {
                let joint = base_facts.iter().filter(|f| f.contains("Rust")).count() as u64;
                let total = base_facts.len() as u64;
                let prob = ProbabilisticReasoner::conditional(joint, total);
                if prob > 0.5 { Terminal::print_line(&format!("[REASON] High probability ({:.2}) of Rust-related facts", prob)); }
            }
            if let Some(pattern) = PatternDetector::detect(&facts) {
                executor.apply_event(GoalManager::propose_create(format!("pattern-{}", executor.tick()), format!("Investigate: {}", pattern), 6, vec!["KnowledgeGrowth".into()], None));
            }
        }

        // التفكير المضاد الديناميكي
        if executor.tick() % 200 == 0 {
            // بناء قواعد سببية من الذاكرة (مبسطة: إذا تكرر نمط "A causes B")
            let mut rules: Vec<(String, String)> = Vec::new();
            for fact in executor.state().memory.semantic.values() {
                if fact.predicate == "causes" {
                    rules.push((fact.subject.clone(), fact.object.clone()));
                }
            }
            if rules.len() >= 2 {
                if let Some(alt) = Counterfactual::reason(&rules[0].0, &rules[0].1, &rules[1].0, &rules) {
                    Terminal::print_line(&format!("[COUNTERFACTUAL] If not {}, then {}", rules[0].0, alt));
                }
            }

            let domains: Vec<String> = executor.state().goals.active_goals.values().map(|g| g.description.clone()).collect();
            let all = vec!["memory".into(), "world".into(), "reasoning".into(), "goals".into()];
            let spots = BlindSpotDetector::detect(&domains, &all);
            if !spots.is_empty() { Terminal::print_line(&format!("[BLINDSPOT] Missing: {:?}", spots)); }

            let inferences: Vec<(String, f64)> = executor.state().memory.semantic.iter()
                .filter(|(_, f)| f.subject == "inference")
                .map(|(_, f)| (f.object.clone(), f.confidence.0))
                .collect();
            let (count, avg) = ReasoningAuditor::audit(&inferences);
            if count > 0 { Terminal::print_line(&format!("[AUDIT] {} inferences, avg confidence {:.2}", count, avg)); }
        }

        SoftwareFactoryLoop::tick(executor);
        CuriosityEngine::tick(executor);
        InsightEngine::tick(executor);
        ProactivePlanner::tick(executor);
        Reinforcement::tick(executor);
        MemoryGovernor::tick(executor);
    }
}
