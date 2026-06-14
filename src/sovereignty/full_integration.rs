// المُكامل الشامل – يربط كل الحلقات المتبقية في دورة واحدة
// يُستدعى مرة كل 10 دورات (ticks) بواسطة SelfAwareness

use crate::cli::commands::CommandExecutor;
use crate::cli::terminal::Terminal;
use crate::memory::consolidation::deduplication::Deduplication;
use crate::memory::consolidation::consolidator::Consolidator;
use crate::memory::long_term::retention_policy::RetentionPolicy;
use crate::memory::long_term::archive::Archive;
use crate::reasoning::causal::counterfactual::Counterfactual;
use crate::reasoning::self_reasoner::blind_spot_detector::BlindSpotDetector;
use crate::reasoning::self_reasoner::reasoning_auditor::ReasoningAuditor;
use crate::kernel_core::event::EventPayload;

pub struct FullIntegration;

impl FullIntegration {
    pub fn tick(executor: &mut CommandExecutor) {
        if !executor.is_running() { return; }

        // 1. دمج الحقائق المتشابهة (Deduplication)
        let semantic = &executor.state().memory.semantic;
        let mut merges = Vec::new();
        let ids: Vec<&String> = semantic.keys().collect();
        for i in 0..ids.len() {
            for j in i+1..ids.len() {
                let fact_a = &semantic[ids[i]];
                let fact_b = &semantic[ids[j]];
                let sim = Deduplication::similarity(
                    &format!("{} {} {}", fact_a.subject, fact_a.predicate, fact_a.object),
                    &format!("{} {} {}", fact_b.subject, fact_b.predicate, fact_b.object),
                );
                if sim > 0.9 {
                    merges.push((ids[i].clone(), ids[j].clone()));
                }
            }
        }
        for (winner, loser) in merges {
            executor.apply_event(Deduplication::propose_merge(winner, loser));
            Terminal::print_line(&format!("[INTEGRATE] Merged duplicate: {} absorbed {}", winner, loser));
        }

        // 2. توطيد الذكريات (Consolidation)
        for ep in &executor.state().memory.episodic {
            let importance = 0.8; // تقديري – يمكن حسابه من Episode
            if Consolidator::should_consolidate(importance, executor.tick() % 500) {
                executor.apply_event(Consolidator::propose_consolidate_episode(
                    ep.episode_id.clone(),
                    importance,
                ));
            }
        }

        // 3. الأرشفة طويلة المدى (Retention + Archive)
        for (id, fact) in &executor.state().memory.semantic {
            if RetentionPolicy::should_retain(fact.confidence.0, executor.tick() % 2000) {
                executor.apply_event(Archive::propose_archive_fact(id.clone()));
            } else {
                executor.apply_event(Archive::propose_retrieve(id.clone()));
            }
        }

        // 4. التفكير السببي المتقدم (Counterfactual)
        let facts: Vec<String> = semantic
            .values()
            .map(|f| format!("{} {} {}", f.subject, f.predicate, f.object))
            .collect();
        if facts.len() >= 4 {
            let rules: Vec<(String, String)> = facts
                .windows(2)
                .map(|w| (w[0].clone(), w[1].clone()))
                .collect();
            if let Some(alt) = Counterfactual::reason("A", "B", "C", &rules) {
                executor.apply_event(EventPayload::Custom {
                    event_type: "CounterfactualReasoning".into(),
                    data: format!("If not A then {}", alt).into_bytes(),
                });
            }
        }

        // 5. كشف النقاط العمياء (BlindSpotDetector)
        let known_domains: Vec<String> = semantic
            .values()
            .map(|f| f.subject.clone())
            .collect();
        let all_domains = vec!["memory", "world", "reasoning", "goals", "planner", "execution"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        let blind = BlindSpotDetector::detect(&known_domains, &all_domains);
        if !blind.is_empty() {
            executor.apply_event(EventPayload::Custom {
                event_type: "BlindSpotDetected".into(),
                data: blind.join(",").into_bytes(),
            });
            Terminal::print_line(&format!(
                "[INTEGRATE] Blind spots detected: {}",
                blind.join(", ")
            ));
        }

        // 6. تدقيق الاستدلال (ReasoningAuditor)
        let inferences: Vec<(String, f64)> = facts
            .iter()
            .map(|f| (f.clone(), 0.8))
            .collect();
        let (count, avg) = ReasoningAuditor::audit(&inferences);
        executor.apply_event(ReasoningAuditor::propose_audit(count, avg));
    }
}
