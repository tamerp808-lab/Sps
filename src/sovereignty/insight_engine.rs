use crate::cli::commands::CommandExecutor;
use crate::cli::terminal::Terminal;
use crate::goal_system::goal::GoalManager;
use std::collections::{HashMap, HashSet};

pub struct InsightEngine;

impl InsightEngine {
    pub fn tick(executor: &mut CommandExecutor) {
        if !executor.is_running() { return; }

        let facts: Vec<String> = executor.state().memory.semantic
            .values()
            .map(|f| format!("{} {} {}", f.subject, f.predicate, f.object))
            .collect();

        if facts.len() < 5 { return; } // نحتاج حداً أدنى من المعرفة

        // 1. تجميع الحقائق حسب الموضوع (subject)
        let mut subject_facts: HashMap<String, Vec<String>> = HashMap::new();
        for fact in &facts {
            let parts: Vec<&str> = fact.splitn(2, ' ').collect();
            if let Some(subject) = parts.first() {
                subject_facts.entry(subject.to_string()).or_default().push(fact.clone());
            }
        }

        // 2. التلخيص التلقائي: إذا كان هناك موضوع له 4+ حقائق
        for (subject, facts) in subject_facts.iter() {
            if facts.len() >= 4 {
                let summary = format!(
                    "بناءً على {} حقائق حول '{}'، يبدو أنه {}",
                    facts.len(),
                    subject,
                    Self::extract_commonality(facts)
                );
                Terminal::print_line(&format!("[INSIGHT] 📝 {}", summary));
                executor.apply_event(GoalManager::propose_create(
                    format!("summary-{}", executor.tick()),
                    summary,
                    4,
                    vec!["KnowledgeGrowth".into()],
                    None,
                ));
            }
        }

        // 3. الرؤى العميقة: ارتباطات غير متوقعة بين مواضيع مختلفة
        let subjects: Vec<&String> = subject_facts.keys().collect();
        if subjects.len() >= 2 {
            for i in 0..subjects.len() {
                for j in i+1..subjects.len() {
                    let subj_a = subjects[i];
                    let subj_b = subjects[j];
                    // هل هناك صفات مشتركة بين الموضوعين؟
                    let common = Self::find_common_descriptors(
                        &subject_facts[subj_a],
                        &subject_facts[subj_b]
                    );
                    if !common.is_empty() {
                        let insight = format!(
                            "🔍 رابط عميق: '{}' و '{}' يشتركان في: {}",
                            subj_a,
                            subj_b,
                            common.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("، ")
                        );
                        Terminal::print_line(&format!("[INSIGHT] {}", insight));
                        executor.apply_event(GoalManager::propose_create(
                            format!("deep-insight-{}", executor.tick()),
                            insight,
                            7,
                            vec!["KnowledgeGrowth".into()],
                            None,
                        ));
                    }
                }
            }
        }
    }

    /// استخراج الصفة المشتركة بين مجموعة حقائق
    fn extract_commonality(facts: &[String]) -> String {
        let mut descriptors: HashMap<String, usize> = HashMap::new();
        for fact in facts {
            let parts: Vec<&str> = fact.split_whitespace().collect();
            // الكلمة الأخيرة غالباً هي الوصف
            if let Some(last) = parts.last() {
                *descriptors.entry(last.to_string()).or_insert(0) += 1;
            }
        }
        descriptors
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(word, _)| word)
            .unwrap_or_else(|| "مثير للاهتمام".to_string())
    }

    /// إيجاد الكلمات المشتركة بين مجموعتين (باستثناء الأفعال)
    fn find_common_descriptors(facts_a: &[String], facts_b: &[String]) -> HashSet<String> {
        let stop_words = ["هو", "هي", "كان", "يبدو", "جداً"];
        let words_a: HashSet<String> = facts_a
            .iter()
            .flat_map(|f| f.split_whitespace().map(String::from))
            .filter(|w| !stop_words.contains(&w.as_str()))
            .collect();
        let words_b: HashSet<String> = facts_b
            .iter()
            .flat_map(|f| f.split_whitespace().map(String::from))
            .filter(|w| !stop_words.contains(&w.as_str()))
            .collect();
        words_a.intersection(&words_b).cloned().collect()
    }
}
