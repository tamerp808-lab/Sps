use crate::kernel_core::event::EventPayload;

pub struct MetaCognition;

impl MetaCognition {
    /// Compares actual vs expected outcome and reflects.
    pub fn reflect(expected: &str, actual: &str) -> String {
        if expected == actual { "Success".into() } else { format!("Mismatch: expected '{}', got '{}'", expected, actual) }
    }

    pub fn propose_reflection(reflection: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "MetaCognitionReflection".into(),
            data: reflection.into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reflects_on_mismatch() {
        let r = MetaCognition::reflect("A", "B");
        assert!(r.contains("Mismatch"));
    }
}
