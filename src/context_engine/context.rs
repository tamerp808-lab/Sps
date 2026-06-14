// src/context_engine/context.rs
// محرك السياق الحي – يحتفظ بآخر N رسالة ويفهم الإشارات

#[derive(Debug, Clone)]
pub struct ContextWindow {
    messages: Vec<ContextMessage>,
    max_size: usize,
}

#[derive(Debug, Clone)]
pub struct ContextMessage {
    pub role: String, // "user" or "system"
    pub content: String,
    pub timestamp: u64,
}

impl ContextWindow {
    pub fn new(max_size: usize) -> Self {
        ContextWindow {
            messages: Vec::new(),
            max_size,
        }
    }

    /// إضافة رسالة جديدة إلى السياق
    pub fn add(&mut self, role: &str, content: &str, tick: u64) {
        self.messages.push(ContextMessage {
            role: role.to_string(),
            content: content.to_string(),
            timestamp: tick,
        });
        // الحفاظ على الحد الأقصى
        if self.messages.len() > self.max_size {
            self.messages.remove(0);
        }
    }

    /// استرجاع آخر N رسالة كسياق
    pub fn recent(&self, n: usize) -> Vec<&ContextMessage> {
        self.messages.iter().rev().take(n).rev().collect()
    }

    /// تفسير الإشارات الضمنية: "هذا"، "ذلك"، "هو"…
    pub fn resolve_reference(&self, text: &str) -> String {
        let mut resolved = text.to_string();

        // البحث عن كلمات الإشارة
        let references = ["هذا", "ذلك", "هو", "هي", "هؤلاء"];
        for r in references.iter() {
            if resolved.contains(r) {
                // العودة إلى آخر جملة في السياق
                if let Some(last) = self.messages.last() {
                    // استبدال الإشارة بآخر موضوع ذُكر
                    let last_words: Vec<&str> = last.content.split_whitespace().collect();
                    if let Some(last_word) = last_words.last() {
                        resolved = resolved.replace(r, &format!("({})", last_word));
                    }
                }
            }
        }

        resolved
    }

    /// إعادة بناء السياق الكامل كسلسلة نصية (لتغذية LLM)
    pub fn as_prompt(&self) -> String {
        self.messages
            .iter()
            .map(|m| format!("{}: {}", m.role, m.content))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_remembers_messages() {
        let mut ctx = ContextWindow::new(5);
        ctx.add("user", "مرحبا", 1);
        ctx.add("system", "أهلاً بك", 2);
        assert_eq!(ctx.recent(2).len(), 2);
    }

    #[test]
    fn resolves_reference() {
        let mut ctx = ContextWindow::new(5);
        ctx.add("user", "أنا أحب Rust", 1);
        ctx.add("user", "هذا رائع", 2);
        let resolved = ctx.resolve_reference("هذا رائع");
        assert!(resolved.contains("Rust"));
    }

    #[test]
    fn max_size_is_respected() {
        let mut ctx = ContextWindow::new(3);
        for i in 1..=5 {
            ctx.add("user", &format!("msg{}", i), i);
        }
        assert_eq!(ctx.recent(10).len(), 3);
    }
}
