// src/reflection/lesson_learner.rs
// Phase 9 — Reflection
// Zone B — Cognitive

use crate::kernel_core::event::EventPayload;

pub struct LessonLearner;

impl LessonLearner {
    /// Learns a lesson from a failure: associates error with context.
    pub fn learn(failure_reason: &str, context: &str) -> String {
        format!("Lesson: when '{}', avoid '{}'", context, failure_reason)
    }

    pub fn propose_lesson(lesson: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "LessonLearned".into(),
            data: lesson.into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn learns_from_failure() {
        let lesson = LessonLearner::learn("timeout", "reading large file");
        assert!(lesson.contains("timeout"));
    }
}
