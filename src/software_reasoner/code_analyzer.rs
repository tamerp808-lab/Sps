use crate::kernel_core::event::EventPayload;

pub struct CodeAnalyzer;

impl CodeAnalyzer {
    /// Placeholder: analyses code for simple pattern.
    pub fn analyze(code: &str, pattern: &str) -> bool {
        code.contains(pattern)
    }

    pub fn propose_analysis(result: bool, file: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "CodeAnalyzed".into(),
            data: format!("{}|{}", file, result).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_pattern_in_code() {
        assert!(CodeAnalyzer::analyze("fn main() {}", "fn main"));
    }
}
