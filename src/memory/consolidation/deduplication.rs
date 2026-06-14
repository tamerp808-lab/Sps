// src/memory/consolidation/deduplication.rs
// Phase 3 — Memory
// Zone B — Cognitive
//
// Purpose:
//   Deduplication detects and merges duplicate memories across
//   layers to prevent knowledge bloat. It compares facts and
//   episodes by content similarity and produces Events for merges.

use crate::kernel_core::event::EventPayload;

pub struct Deduplication;

impl Deduplication {
    /// Proposes merging two duplicate items (winner absorbs loser).
    pub fn propose_merge(winner_id: String, loser_id: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "MemoryDeduplicationMerge".into(),
            data: format!("{}|{}", winner_id, loser_id).into_bytes(),
        }
    }

    /// Simple similarity check between two strings (placeholder for
    /// future embedding-based comparison).
    pub fn similarity(a: &str, b: &str) -> f64 {
        if a == b { return 1.0; }
        if a.contains(b) || b.contains(a) { return 0.8; }
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_strings_are_duplicates() {
        assert_eq!(Deduplication::similarity("hello", "hello"), 1.0);
    }

    #[test]
    fn substring_is_similar() {
        assert!(Deduplication::similarity("hello world", "hello") > 0.7);
    }
}
