// src/memory/memory_index/vector_index.rs
// Phase 3 — Memory
// Zone B — Cognitive
//
// Purpose:
//   VectorIndex is a placeholder for future embedding-based semantic
//   search. In Phase 3, it provides a stub that produces Events for
//   indexing and querying, and a simple fallback to keyword matching.
//   Full vector similarity will be added in Phase 5 (Reasoning).

use crate::kernel_core::event::EventPayload;

pub struct VectorIndex;

impl VectorIndex {
    /// Proposes indexing an item with its embedding (stub).
    pub fn propose_index(item_id: String, embedding: Vec<f64>) -> EventPayload {
        let emb_str = embedding.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",");
        EventPayload::Custom {
            event_type: "VectorIndexAdd".into(),
            data: format!("{}|{}", item_id, emb_str).into_bytes(),
        }
    }

    /// Proposes a vector similarity query (stub).
    pub fn propose_query(query_embedding: Vec<f64>, top_k: usize) -> EventPayload {
        let emb_str = query_embedding.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",");
        EventPayload::Custom {
            event_type: "VectorIndexQuery".into(),
            data: format!("{}|{}", emb_str, top_k).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_index_creates_event() {
        let payload = VectorIndex::propose_index("item1".into(), vec![0.1, 0.2]);
        match payload {
            EventPayload::Custom { event_type, .. } => assert!(event_type.contains("VectorIndex")),
            _ => panic!("Wrong payload"),
        }
    }
}
