use crate::cli::commands::CommandExecutor;
use crate::cli::terminal::Terminal;
use crate::memory::working::attention::Attention;
use crate::memory::forgetting::decay_model::DecayModel;
use crate::memory::forgetting::pruning_policy::PruningPolicy;
use crate::memory::consolidation::deduplication::Deduplication;
use crate::memory::consolidation::consolidator::Consolidator;
use crate::memory::long_term::archive::Archive;
use crate::kernel_core::event::EventPayload;

pub struct MemoryGovernor;

impl MemoryGovernor {
    pub fn tick(executor: &mut CommandExecutor) {
        if !executor.is_running() { return; }

        let focused = Attention::focus(&executor.state().memory, "");
        let working_ids: Vec<String> = executor.state().memory.working.iter().map(|w| w.item_id.clone()).collect();
        let facts_for_dedup: Vec<(String, String)> = executor.state().memory.semantic.iter().map(|(id, f)| (id.clone(), format!("{} {} {}", f.subject, f.predicate, f.object))).collect();
        
        // هنا كان الخطأ: importance f64 وليس u64
        let episodes_to_consolidate: Vec<(String, f64, u64)> = executor.state().memory.episodic.iter().map(|ep| {
            let importance = 0.7;
            let age = executor.tick().saturating_sub(ep.timestamp_tick);
            (ep.episode_id.clone(), importance, age)
        }).collect();

        let decay_candidates: Vec<(String, f64)> = executor.state().memory.semantic.iter().map(|(id, fact)| {
            let age_ticks = executor.tick().saturating_sub(0);
            let decayed = DecayModel::decayed_relevance(fact.confidence.0, age_ticks % 1000, 0.001);
            (id.clone(), decayed)
        }).filter(|(_, d)| *d < 1.0).collect();

        let prune_candidates: Vec<(String, f64, u64)> = if executor.tick() % 50 == 0 {
            executor.state().memory.semantic.iter().map(|(id, f)| (id.clone(), f.confidence.0, 0u64)).collect()
        } else { vec![] };

        // التطبيق
        for item in &focused {
            if !working_ids.contains(&item.item_id) {
                executor.apply_event(Attention::bring_to_focus(item.item_id.clone(), item.content.clone(), item.relevance.0));
            }
        }
        for old_id in working_ids {
            if !focused.iter().any(|f| f.item_id == old_id) {
                executor.apply_event(Attention::remove_from_focus(old_id));
            }
        }

        for i in 0..facts_for_dedup.len() {
            for j in i+1..facts_for_dedup.len() {
                if Deduplication::similarity(&facts_for_dedup[i].1, &facts_for_dedup[j].1) > 0.8 {
                    executor.apply_event(Deduplication::propose_merge(facts_for_dedup[i].0.clone(), facts_for_dedup[j].0.clone()));
                }
            }
        }

        for (ep_id, importance, age) in episodes_to_consolidate {
            // age u64، والدالة تتوقع u64 (سنصلح الدالة نفسها إذا لزم)
            if Consolidator::should_consolidate(importance, age as u64) {
                executor.apply_event(Consolidator::propose_consolidate_episode(ep_id.clone(), importance));
                executor.apply_event(Archive::propose_archive_episode(ep_id));
            }
        }

        for (id, decayed) in decay_candidates {
            executor.apply_event(EventPayload::Custom { event_type: "MemoryDecayApplied".into(), data: format!("{}|{}", id, decayed).into_bytes() });
        }

        if !prune_candidates.is_empty() {
            let to_prune = PruningPolicy::select_prune_candidates(&prune_candidates, executor.tick(), 0.2, 1000);
            for id in to_prune {
                executor.apply_event(EventPayload::Custom { event_type: "MemoryPruned".into(), data: format!("{}|low importance", id).into_bytes() });
                Terminal::print_line(&format!("[MEMORY] Pruned: {}", id));
            }
        }
    }
}
