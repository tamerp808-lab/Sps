// src/memory/long_term/retention_policy.rs
// Phase 3 — Memory
// Zone B — Cognitive
//
// Purpose:
//   RetentionPolicy defines rules for how long memories are kept
//   in long-term storage. It evaluates importance, recency, and
//   access patterns to decide what to retain or prune. Produces
//   Events — never mutates state directly.

use crate::kernel_core::event::EventPayload;

pub struct RetentionPolicy;

impl RetentionPolicy {
    /// Proposes retaining an item permanently.
    pub fn propose_retain(item_id: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "RetentionPolicyRetain".into(),
            data: item_id.into_bytes(),
        }
    }

    /// Proposes pruning an item due to low importance/age.
    pub fn propose_prune(item_id: String, reason: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "RetentionPolicyPrune".into(),
            data: format!("{}|{}", item_id, reason).into_bytes(),
        }
    }

    /// Evaluates whether an item should be retained based on importance and age.
    pub fn should_retain(importance: f64, age_in_ticks: u64) -> bool {
        // Simple heuristic: high importance items live longer
        let threshold = if importance > 0.8 { 10_000 } else if importance > 0.5 { 5_000 } else { 1_000 };
        age_in_ticks < threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_importance_retained_longer() {
        assert!(RetentionPolicy::should_retain(0.9, 9_000));
    }

    #[test]
    fn low_importance_pruned_sooner() {
        assert!(!RetentionPolicy::should_retain(0.3, 2_000));
    }
}
