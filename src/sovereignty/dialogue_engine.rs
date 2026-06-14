use crate::cli::commands::CommandExecutor;
use crate::memory::memory_manager::MemoryManager;
use crate::sovereignty::self_model::SelfModel;
use std::collections::HashMap;
use std::cell::RefCell;

pub struct DialogueEngine {
    // تخزين مؤقت للأسئلة التي فشل LLM في الإجابة عنها (لا نكرر السؤال)
    failed_questions: RefCell<HashMap<String, u64>>,
}

impl DialogueEngine {
    pub fn new() -> Self {
        DialogueEngine {
            failed_questions: RefCell::new(HashMap::new()),
        }
    }

    pub fn respond(executor: &CommandExecutor, input: &str) -> String {
        let mut self_model = SelfModel::new();
        self_model.refresh_from_memory(executor);
        let engine = DialogueEngine::new();

        // 1. أسئلة الهوية (محلي بحت)
        if let Some(identity_answer) = self_model.answer_identity(input) {
            return identity_answer;
        }

        // 2. ردود المجاملات السريعة (محلي بحت)
        let lower = input.trim().to_lowercase();
        let greeting = engine.check_greetings(&lower);
        if !greeting.is_empty() {
            return greeting;
        }

        // 3. البحث في الذاكرة عن ردود سابقة (sps replied ...)
        let query = format!("replied {}", input);
        let past_replies = MemoryManager::query(&executor.state().memory, &query);
        if !past_replies.is_empty() {
            let top = &past_replies[0];
            return format!("💡 تذكرتُ أنني قلتُ سابقاً: {}", top.content.replace("replied ", ""));
        }

        // 4. البحث عن أي حقائق متعلقة
        let keywords: Vec<&str> = input.split_whitespace().filter(|w| w.len() > 2).collect();
        let mut all_results = Vec::new();
        for kw in &keywords {
            all_results.extend(MemoryManager::query(&executor.state().memory, kw));
        }
        all_results.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap_or(std::cmp::Ordering::Equal));
        all_results.dedup_by(|a, b| a.id == b.id);

        if !all_results.is_empty() {
            let top = &all_results[0];
            return format!("💡 بناءً على معرفتي: {}. ({} نتائج)", top.content, all_results.len());
        }

        // 5. هل يستحق السؤال استدعاء LLM؟
        if engine.should_ask_llm(input) {
            if let Some(ref llm) = executor.llm() {
                let prompt = format!("Answer in Arabic only. Be direct and brief. Question: {}", input);
                if let Ok(reply) = llm.chat(&prompt) {
                    return format!("🌐 {}", reply);
                } else {
                    // فشل LLM، نسجل السؤال لتجنب تكراره
                    engine.mark_failed(input);
                }
            }
        }

        // 6. رد عام (يُحفظ لاحقاً)
        format!("🤔 لا أعرف '{}' بعد. علمني لأتذكره.", input)
    }

    // دوال مساعدة خاصة
    fn check_greetings(&self, input: &str) -> String {
        match input {
            "مرحباً" | "مرحبا" | "هاي" | "hello" | "hi" => "👋 مرحباً! أنا SPS، رفيقك المعرفي.".into(),
            _ if input.contains("شكر") => "🤝 عفواً! سعيد بخدمتك.".into(),
            _ if input.contains("صباح") => "🌅 صباح النور!".into(),
            _ if input.contains("مساء") => "🌙 مساء الخير!".into(),
            "وداعاً" | "bye" | "مع السلامة" => "👋 وداعاً! سأحفظ كل ما تعلمته منك.".into(),
            _ => String::new(),
        }
    }

    fn should_ask_llm(&self, input: &str) -> bool {
        // لا نسأل LLM إذا كان السؤال قصيراً جداً أو فشل سابقاً
        if input.len() < 10 { return false; }
        let failed = self.failed_questions.borrow();
        !failed.contains_key(input)
    }

    fn mark_failed(&self, input: &str) {
        let mut failed = self.failed_questions.borrow_mut();
        failed.insert(input.to_string(), 1);
    }
}
