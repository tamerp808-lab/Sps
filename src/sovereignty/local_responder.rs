// يولد ردوداً محلية من الذاكرة بدون الحاجة إلى LLM خارجي
use crate::cli::commands::CommandExecutor;
use crate::memory::memory_manager::MemoryManager;

pub struct LocalResponder;

impl LocalResponder {
    /// يبحث في الذاكرة عن كلمات مفتاحية ويولد رداً بسيطاً
    pub fn respond_to(executor: &CommandExecutor, input: &str) -> String {
        let results = MemoryManager::query(&executor.state().memory, input);
        if !results.is_empty() {
            let top = &results[0];
            return format!("بناءً على ما نعرفه، {} (ثقة: {:.0}%)", top.content, top.relevance * 100.0);
        }

        let fact_count = executor.state().memory.semantic.len();
        if input.contains("حال") || input.contains("كيف") {
            format!("أنا بخير! أعرف {} حقيقة، وأتابع {} هدفاً نشطاً.", fact_count, executor.state().goals.active_goals.len())
        } else if input.contains("شكر") {
            "على الرحب والسعة!".to_string()
        } else {
            format!("فهمت. سأضيف '{}' إلى ذاكرتي.", input)
        }
    }
}
