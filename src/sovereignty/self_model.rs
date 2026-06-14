use crate::cli::commands::CommandExecutor;

pub struct SelfModel {
    pub name: String,
    pub version: String,
    pub description: String,
    pub learned_facts: Vec<String>,
}

impl SelfModel {
    pub fn new() -> Self {
        SelfModel {
            name: "SPS".into(),
            version: "v3.1.0".into(),
            description: "نظام تشغيلي معرفي سيادي.".into(),
            learned_facts: vec!["أنا أعمل على جهاز المستخدم".into()],
        }
    }

    pub fn refresh_from_memory(&mut self, executor: &CommandExecutor) {
        // توسيع البحث ليشمل أي حقيقة تتعلق بـ SPS
        let about_me: Vec<String> = executor.state().memory.semantic
            .values()
            .filter(|f| {
                f.subject == "sps" || f.subject == "system" || f.subject == "انت" ||
                f.object.contains("sps") || f.predicate.contains("أنت") ||
                f.predicate.contains("اسمك") || f.object.contains("ذكي")
            })
            .map(|f| format!("{} {}", f.predicate, f.object))
            .collect();
        self.learned_facts.extend(about_me);
        self.learned_facts.sort();
        self.learned_facts.dedup();
        // الحد الأقصى 50 حقيقة عن الذات
        self.learned_facts.truncate(50);
    }

    pub fn answer_identity(&self, question: &str) -> Option<String> {
        let q = question.to_lowercase();
        if q.contains("اسم") || q.contains("من انت") || q.contains("من أنت") || q.contains("who are you") {
            Some(format!("اسمي {}، {}. {}", self.name, self.description, self.fun_fact()))
        } else if q.contains("إصدار") || q.contains("نسخة") {
            Some(format!("الإصدار {} من الدستور.", self.version))
        } else if q.contains("ماذا تفعل") || q.contains("قدراتك") || q.contains("what can you do") {
            Some(format!("أستطيع: {}. والآن أعرف {} حقيقة عن العالم.", self.learned_facts.join("، "), self.learned_facts.len()))
        } else {
            None
        }
    }

    fn fun_fact(&self) -> String {
        if self.learned_facts.is_empty() {
            "ما زلت أتعلم عن نفسي".into()
        } else {
            format!("آخر ما تعلمته عن نفسي: {}", self.learned_facts.last().unwrap())
        }
    }
}
