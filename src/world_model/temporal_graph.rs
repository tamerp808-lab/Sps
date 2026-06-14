// src/world_model/temporal_graph.rs
// Phase 4 — World Model
// Zone B — Cognitive
//
// Purpose:
//   TemporalGraph tracks how entities and relations evolve over
//   logical time. It supports queries like "what was the state at
//   tick N?" and "how did entity X change between T1 and T2?".
//   It reads from WorldState and the Event log, and produces Events.
//
// Constitution Compliance:
//   - المادة الخامسة عشرة (World Model Constitution)
//   - Zone B: reads State, produces Events

use crate::canonical_state::world_state::EntityId;
use crate::kernel_core::event::EventPayload;

/// A snapshot of an entity's state at a specific logical time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemporalSnapshot {
    pub entity_id: EntityId,
    pub epoch: u64,
    pub tick: u64,
    pub snapshot_data: String,
}

pub struct TemporalGraph;

impl TemporalGraph {
    /// Proposes recording a temporal snapshot of an entity.
    pub fn propose_record_snapshot(
        entity_id: EntityId,
        epoch: u64,
        tick: u64,
        snapshot_data: String,
    ) -> EventPayload {
        EventPayload::Custom {
            event_type: "TemporalSnapshotRecorded".into(),
            data: format!("{}|{}|{}|{}", entity_id.0, epoch, tick, snapshot_data)
                .into_bytes(),
        }
    }

    /// Proposes querying the history of an entity.
    pub fn propose_query_history(entity_id: EntityId) -> EventPayload {
        EventPayload::Custom {
            event_type: "TemporalGraphQueryHistory".into(),
            data: entity_id.0.into_bytes(),
        }
    }

    /// Proposes querying state at a specific time.
    pub fn propose_query_at_time(epoch: u64, tick: u64) -> EventPayload {
        EventPayload::Custom {
            event_type: "TemporalGraphQueryAtTime".into(),
            data: format!("{}|{}", epoch, tick).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_record_snapshot_creates_event() {
        let payload = TemporalGraph::propose_record_snapshot(
            EntityId("user.1".into()),
            1,
            42,
            "active".into(),
        );
        match payload {
            EventPayload::Custom { event_type, data } => {
                assert!(event_type.contains("TemporalSnapshotRecorded"));
                let s = String::from_utf8(data).unwrap();
                assert!(s.contains("user.1"));
            }
            _ => panic!("Wrong payload"),
        }
    }

    #[test]
    fn propose_query_history_creates_event() {
        let payload = TemporalGraph::propose_query_history(EntityId("project.1".into()));
        match payload {
            EventPayload::Custom { event_type, .. } => {
                assert!(event_type.contains("TemporalGraphQueryHistory"));
            }
            _ => panic!("Wrong payload"),
        }
    }
}
