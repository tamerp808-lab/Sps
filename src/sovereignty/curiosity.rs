use crate::cli::commands::CommandExecutor;
use crate::cli::terminal::Terminal;
use std::collections::HashMap;

pub struct CuriosityEngine;

impl CuriosityEngine {
    pub fn tick(executor: &mut CommandExecutor) {
        if !executor.is_running() { return; }

        let facts: Vec<String> = executor.state().memory.semantic
            .values()
            .map(|f| format!("{} {} {}", f.subject, f.predicate, f.object))
            .collect();

        if facts.is_empty() { return; }

        let mut subject_count: HashMap<String, usize> = HashMap::new();
        for fact in &facts {
            let parts: Vec<&str> = fact.splitn(2, ' ').collect();
            if let Some(subject) = parts.first() {
                *subject_count.entry(subject.to_string()).or_insert(0) += 1;
            }
        }

        if let Some((least_known, count)) = subject_count.into_iter().min_by_key(|(_, c)| *c) {
            if count < 2 {
                if let Some(ref llm) = executor.llm() {
                    let prompt = format!("اشرح بإيجاز: ما هو {}؟", least_known);
                    if let Ok(reply) = llm.chat(&prompt) {
                        executor.apply_event(
                            crate::memory::memory_manager::MemoryManager::propose_semantic_fact(
                                least_known.clone(), "is".into(), reply.clone(), 0.7
                            )
                        );
                        Terminal::print_line(&format!("[CURIOSITY] Learned about '{}' from LLM", least_known));
                        return;
                    }
                }
            }
        }
    }
}
