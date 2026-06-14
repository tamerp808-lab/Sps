use crate::canonical_state::memory_state::MemoryState;
use crate::kernel_core::event::EventPayload;

#[derive(Debug, Clone)]
pub enum ValidationResult {
    Accepted { fact_id: String, evidence_level: String },
    AcceptedAsHypothesis { fact_id: String, reason: String },
    Rejected { fact_id: String, reason: String, conflicting_fact_id: Option<String> },
}

pub struct SemanticValidator;

impl SemanticValidator {
    pub fn validate(state: &MemoryState, proposed_fact_id: &str, subject: &str, predicate: &str, object: &str, confidence: f64, source: &str) -> ValidationResult {
        for (existing_id, existing_fact) in &state.semantic {
            if existing_fact.subject == subject && existing_fact.predicate == predicate && existing_fact.object == object {
                if existing_fact.confidence.0 >= confidence {
                    return ValidationResult::Rejected { fact_id: proposed_fact_id.into(), reason: "Duplicate fact with equal or higher confidence already exists.".into(), conflicting_fact_id: Some(existing_id.clone()) };
                } else {
                    return ValidationResult::Accepted { fact_id: proposed_fact_id.into(), evidence_level: "Fact".into() };
                }
            }
            if existing_fact.subject == subject && existing_fact.predicate == predicate && existing_fact.object != object {
                if existing_fact.confidence.0 >= 0.9 && confidence < 0.95 {
                    return ValidationResult::Rejected { fact_id: proposed_fact_id.into(), reason: format!("Contradicts existing fact {} (confidence {}): {} {} {} vs {} {} {}", existing_id, existing_fact.confidence.0, existing_fact.subject, existing_fact.predicate, existing_fact.object, subject, predicate, object), conflicting_fact_id: Some(existing_id.clone()) };
                }
            }
        }
        if confidence < 0.5 { return ValidationResult::Rejected { fact_id: proposed_fact_id.into(), reason: format!("Confidence too low: {} < 0.5", confidence), conflicting_fact_id: None }; }
        if confidence < 0.8 { return ValidationResult::AcceptedAsHypothesis { fact_id: proposed_fact_id.into(), reason: format!("Confidence moderate: {} — needs further evidence.", confidence) }; }
        if source == "llm" && confidence < 0.9 { return ValidationResult::AcceptedAsHypothesis { fact_id: proposed_fact_id.into(), reason: "LLM-sourced fact requires additional verification.".into() }; }
        ValidationResult::Accepted { fact_id: proposed_fact_id.into(), evidence_level: "Fact".into() }
    }

    pub fn propose_validation_event(result: &ValidationResult) -> EventPayload {
        match result {
            ValidationResult::Accepted { fact_id, evidence_level } => EventPayload::Custom { event_type: "SemanticValidationAccepted".into(), data: format!("{}|{}", fact_id, evidence_level).into_bytes() },
            ValidationResult::AcceptedAsHypothesis { fact_id, reason } => EventPayload::Custom { event_type: "SemanticValidationHypothesis".into(), data: format!("{}|{}", fact_id, reason).into_bytes() },
            ValidationResult::Rejected { fact_id, reason, conflicting_fact_id } => {
                let conflict = conflicting_fact_id.clone().unwrap_or_else(|| "none".into());
                EventPayload::Custom { event_type: "SemanticValidationRejected".into(), data: format!("{}|{}|conflict={}", fact_id, reason, conflict).into_bytes() }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical_state::memory_state::SemanticFact;
    use ordered_float::OrderedFloat;

    fn sample_state() -> MemoryState {
        let mut state = MemoryState::empty();
        state.semantic.insert("f1".into(), SemanticFact { fact_id: "f1".into(), subject: "sps".into(), predicate: "is".into(), object: "deterministic".into(), confidence: OrderedFloat(0.99) });
        state
    }

    #[test] fn accept_new_high() { match SemanticValidator::validate(&sample_state(), "f2", "rust", "is", "safe", 0.95, "user") { ValidationResult::Accepted { .. } => (), _ => panic!() } }
    #[test] fn reject_duplicate_lower() { match SemanticValidator::validate(&sample_state(), "f.new", "sps", "is", "deterministic", 0.5, "user") { ValidationResult::Rejected { reason, .. } => assert!(reason.contains("Duplicate")), _ => panic!() } }
    #[test] fn accept_duplicate_higher() { match SemanticValidator::validate(&sample_state(), "f.new", "sps", "is", "deterministic", 1.0, "user") { ValidationResult::Accepted { .. } => (), _ => panic!() } }
    #[test] fn reject_contradiction() { match SemanticValidator::validate(&sample_state(), "f.new", "sps", "is", "nondeterministic", 0.6, "user") { ValidationResult::Rejected { reason, .. } => assert!(reason.contains("Contradicts")), _ => panic!() } }
    #[test] fn low_confidence_hypothesis() { match SemanticValidator::validate(&sample_state(), "f.new", "sps", "supports", "async", 0.6, "user") { ValidationResult::AcceptedAsHypothesis { .. } => (), _ => panic!() } }
    #[test] fn llm_source_requires_higher() { match SemanticValidator::validate(&sample_state(), "f.new", "sps", "uses", "gpu", 0.85, "llm") { ValidationResult::AcceptedAsHypothesis { reason, .. } => assert!(reason.contains("LLM")), _ => panic!() } }
}
