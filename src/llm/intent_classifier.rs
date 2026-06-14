pub struct IntentClassifier;

impl IntentClassifier {
    /// يصنف الجملة إلى أمر داخلي
    pub fn classify(text: &str) -> Option<(&str, String)> {
        let lower = text.to_lowercase();
        if lower.starts_with("تذكر") || lower.starts_with("احفظ") {
            let content = text.replacen("تذكر ", "").replacen("احفظ ", "").trim().to_string();
            Some(("memory", content))
        } else if lower.starts_with("ابحث عن") || lower.starts_with("ماذا تعرف عن") {
            let keyword = text.replacen("ابحث عن ", "").replacen("ماذا تعرف عن ", "").trim().to_string();
            Some(("search", keyword))
        } else if lower.starts_with("خطط ل") || lower.starts_with("أنشئ خطة") {
            let goal = text.replacen("خطط ل", "").replacen("أنشئ خطة ل", "").trim().to_string();
            Some(("plan", goal))
        } else {
            None // سؤال عام → يُرسل إلى LLM
        }
    }
}
