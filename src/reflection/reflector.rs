// src/reflection/reflector.rs
// Phase 9 — Reflection
// Zone B — Cognitive

use crate::kernel_core::event::EventPayload;

pub struct Reflector;

impl Reflector {
    /// Compares expected vs actual outcome and produces a reflection.
    pub fn reflect(expected: &str, actual: &str) -> String {
        if expected == actual { "Success".into() } else { format!("Mismatch: expected '{}', got '{}'", expected, actual) }
    }

    pub fn propose_reflection(reflection: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "ReflectionPerformed".into(),
            data: reflection.into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_reflection() {
        assert_eq!(Reflector::reflect("A", "A"), "Success");
    }

    #[test]
    fn mismatch_reflection() {
        let r = Reflector::reflect("A", "B");
        assert!(r.contains("Mismatch"));
    }
}
