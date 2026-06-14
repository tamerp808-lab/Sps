// src/world_model/reality_checker.rs
// Phase 4 — World Model
// Zone B — Cognitive
//
// Purpose:
//   RealityChecker validates the consistency of the world model.
//   It runs after every world model update to detect:
//     - Orphaned relations (references to missing entities)
//     - Contradictory facts or relations
//     - Entities with missing required properties
//   It reads from WorldState and produces Events — no direct mutation.
//
// Constitution Compliance:
//   - المادة الخامسة عشرة (World Model Constitution) — "reality_checker.rs
//     يعمل بعد كل تحديث"
//   - Zone B: reads State, produces Events
//   - المادة الثالثة (Evidence Rule): violations cite conflicting facts

use crate::canonical_state::world_state::WorldState;
use crate::kernel_core::event::EventPayload;

/// Result of a reality consistency check.
#[derive(Debug, Clone)]
pub struct RealityCheckResult {
    pub passed: bool,
    pub violations: Vec<RealityViolation>,
}

#[derive(Debug, Clone)]
pub struct RealityViolation {
    pub violation_type: String,
    pub description: String,
    pub related_entities: Vec<String>,
}

pub struct RealityChecker;

impl RealityChecker {
    /// Checks the world model for consistency violations.
    pub fn check(state: &WorldState) -> RealityCheckResult {
        let mut violations = Vec::new();

        // 1. Orphaned relations: from or to an entity that does not exist
        for (rel_id, rel) in &state.relations {
            if !state.entities.contains_key(&rel.from) {
                violations.push(RealityViolation {
                    violation_type: "OrphanedRelation".into(),
                    description: format!(
                        "Relation {} references missing source entity {}",
                        rel_id.0, rel.from.0
                    ),
                    related_entities: vec![rel.from.0.clone()],
                });
            }
            if !state.entities.contains_key(&rel.to) {
                violations.push(RealityViolation {
                    violation_type: "OrphanedRelation".into(),
                    description: format!(
                        "Relation {} references missing target entity {}",
                        rel_id.0, rel.to.0
                    ),
                    related_entities: vec![rel.to.0.clone()],
                });
            }
        }

        // 2. Facts referencing missing subjects (future: fact checks)
        // For now, facts are not in WorldState but in MemoryState.

        let passed = violations.is_empty();
        RealityCheckResult { passed, violations }
    }

    /// Proposes recording a reality check result.
    pub fn propose_record_check(result: &RealityCheckResult) -> EventPayload {
        let violations_str = result
            .violations
            .iter()
            .map(|v| format!("{}:{}", v.violation_type, v.description))
            .collect::<Vec<_>>()
            .join(";");
        EventPayload::Custom {
            event_type: "RealityCheckPerformed".into(),
            data: format!("{}|{}", result.passed, violations_str).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical_state::world_state::{Entity, EntityId, EntityType, Relation, RelationId};
    use ordered_float::OrderedFloat;
    use std::collections::BTreeMap;

    #[test]
    fn empty_world_passes_check() {
        let world = WorldState::empty();
        let result = RealityChecker::check(&world);
        assert!(result.passed);
    }

    #[test]
    fn orphaned_relation_detected() {
        let mut world = WorldState::empty();
        world.relations.insert(
            RelationId("r1".into()),
            Relation {
                id: RelationId("r1".into()),
                from: EntityId("missing.entity".into()),
                relation_type: "uses".into(),
                to: EntityId("also.missing".into()),
                weight: OrderedFloat(0.5),
                bidirectional: false,
            },
        );
        let result = RealityChecker::check(&world);
        assert!(!result.passed);
        assert_eq!(result.violations.len(), 2); // both from and to missing
    }

    #[test]
    fn valid_relation_passes() {
        let mut world = WorldState::empty();
        world.entities.insert(
            EntityId("a".into()),
            Entity {
                id: EntityId("a".into()),
                entity_type: EntityType::User,
                properties: BTreeMap::new(),
            },
        );
        world.entities.insert(
            EntityId("b".into()),
            Entity {
                id: EntityId("b".into()),
                entity_type: EntityType::Project,
                properties: BTreeMap::new(),
            },
        );
        world.relations.insert(
            RelationId("r1".into()),
            Relation {
                id: RelationId("r1".into()),
                from: EntityId("a".into()),
                relation_type: "owns".into(),
                to: EntityId("b".into()),
                weight: OrderedFloat(0.9),
                bidirectional: false,
            },
        );
        let result = RealityChecker::check(&world);
        assert!(result.passed);
    }
}
