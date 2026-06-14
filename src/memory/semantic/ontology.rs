// src/memory/semantic/ontology.rs
// Phase 3 — Memory
// Zone B — Cognitive
//
// Purpose:
//   Ontology provides a hierarchical classification system for
//   concepts in semantic memory. It enables SPS to understand
//   type relationships (e.g., "a dog is an animal") and inherit
//   facts across categories. All operations produce Events.
//
// Constitution Compliance:
//   - المادة الرابعة عشرة (Memory Constitution) — Semantic Memory
//   - Zone B: reads State, produces Events

use crate::kernel_core::event::EventPayload;

/// A concept node in the ontology.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Concept {
    pub concept_id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub description: String,
}

pub struct Ontology;

impl Ontology {
    /// Proposes adding a new concept to the ontology.
    pub fn propose_add_concept(
        concept_id: String,
        name: String,
        parent_id: Option<String>,
        description: String,
    ) -> EventPayload {
        let parent = parent_id.unwrap_or_else(|| "root".into());
        EventPayload::Custom {
            event_type: "OntologyConceptAdded".into(),
            data: format!("{}|{}|{}|{}", concept_id, name, parent, description)
                .into_bytes(),
        }
    }

    /// Proposes removing a concept from the ontology.
    pub fn propose_remove_concept(concept_id: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "OntologyConceptRemoved".into(),
            data: concept_id.into_bytes(),
        }
    }

    /// Proposes linking a concept to a new parent.
    pub fn propose_reparent(concept_id: String, new_parent_id: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "OntologyConceptReparented".into(),
            data: format!("{}|{}", concept_id, new_parent_id).into_bytes(),
        }
    }

    /// Proposes querying the ontology for all descendants of a concept.
    pub fn propose_query_descendants(concept_id: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "OntologyDescendantsQuery".into(),
            data: concept_id.into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_add_concept_creates_event() {
        let payload = Ontology::propose_add_concept(
            "c1".into(),
            "Dog".into(),
            Some("Animal".into()),
            "A domesticated canine".into(),
        );
        match payload {
            EventPayload::Custom { event_type, data } => {
                assert!(event_type.contains("ConceptAdded"));
                let s = String::from_utf8(data).unwrap();
                assert!(s.contains("Dog"));
            }
            _ => panic!("Wrong payload"),
        }
    }

    #[test]
    fn propose_reparent_creates_event() {
        let payload = Ontology::propose_reparent("c1".into(), "Canine".into());
        match payload {
            EventPayload::Custom { event_type, .. } => {
                assert!(event_type.contains("Reparented"));
            }
            _ => panic!("Wrong payload"),
        }
    }
}
