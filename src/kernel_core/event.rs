// src/kernel_core/event.rs
// Phase 1 — Solid Core
// Zone A — Canonical (100% Deterministic)
//
// Purpose:
//   The Event is the atomic unit of change in SPS. It is the only
//   vehicle through which state may evolve. Every Event carries:
//     - A unique, deterministic EventId
//     - A LogicalTime coordinate
//     - A cryptographic hash chain link (parent_hash + event_hash)
//     - Creation metadata (source, causation, correlation)
//     - A typed payload that describes what happened
//
//   Events are append-only, immutable after creation, and never deleted.
//
// Constitution Compliance:
//   - المادة الثانية  (Zone A)         : Pure construction — no side effects
//   - المادة الرابعة (Event Model)     : The canonical definition
//   - المادة الخامسة (Reducer Law)     : Input to Reducer::apply
//   - المادة السابعة (Replay)          : Every Event must be replayable
//   - المادة الحادية عشرة (Platform)   : No platform dependency
//
// Invariants (enforced at construction):
//   1. event_hash == SHA256(parent_hash || id_bytes || time_bytes || payload_hash)
//   2. id and logical_time are consistent with metadata
//   3. parent_hash is EventHash::ZERO only for the genesis Event

use crate::kernel_core::event_hash::{self, EventHash};
use crate::kernel_core::event_id::EventId;
use crate::kernel_core::event_metadata::{EventMetadata, EventSource};
use crate::kernel_core::logical_time::LogicalTime;

/// The payload of an Event. This is the variant part; the schema is
/// governed by the `event_type` and `schema_version` in the metadata.
///
/// Concrete payloads are defined in downstream phases. For the kernel,
/// we define a set of built-in lifecycle payloads.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventPayload {
    /// Marks the creation of the event log itself.
    Genesis,

    /// A generic lifecycle event (boot, shutdown, checkpoint, ...).
    Lifecycle {
        transition: String,
        from_state: String,
        to_state: String,
    },

    /// An event that carries an opaque byte payload for upper layers.
    Custom {
        event_type: String,
        data: Vec<u8>,
    },

    /// A self-describing failure event (see المادة الخامسة والعشرون).
    Failure {
        component: String,
        class: String,
        message: String,
    },
}

impl EventPayload {
    /// Returns a deterministic, human-readable label for the payload kind.
    pub fn kind(&self) -> &str {
        match self {
            EventPayload::Genesis => "Genesis",
            EventPayload::Lifecycle { .. } => "Lifecycle",
            EventPayload::Custom { .. } => "Custom",
            EventPayload::Failure { .. } => "Failure",
        }
    }
}

/// The canonical Event type.
///
/// Fields:
///   - id: Deterministic unique identifier.
///   - logical_time: Deterministic timestamp.
///   - parent_hash: SHA-256 of the previous Event.
///   - event_hash: SHA-256 of this Event (binds parent + content).
///   - payload: What happened.
///   - metadata: Creation context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: EventId,
    pub logical_time: LogicalTime,
    pub parent_hash: EventHash,
    pub event_hash: EventHash,
    pub payload: EventPayload,
    pub metadata: EventMetadata,
}

impl Event {
    /// Creates the genesis Event — the absolute first Event in any SPS log.
    pub fn genesis() -> Self {
        let id = EventId::ZERO;
        let logical_time = LogicalTime::ZERO;
        let parent_hash = EventHash::ZERO;
        let payload = EventPayload::Genesis;

        let metadata = EventMetadata::new(
            EventSource::System,
            None,
            None,
            1, // schema version
            id.seq,
            logical_time.epoch,
            logical_time.tick,
            logical_time.sequence,
        );

        let payload_hash = event_hash::compute_payload_hash(payload_bytes(&payload).as_ref());
        let event_hash = event_hash::compute_event_hash(
            &parent_hash,
            id.to_string().as_bytes(),
            logical_time.to_string().as_bytes(),
            &payload_hash,
        );

        Event {
            id,
            logical_time,
            parent_hash,
            event_hash,
            payload,
            metadata,
        }
    }

    /// Creates a new Event that follows a previous Event.
    ///
    /// # Arguments
    ///   - prev: The immediately preceding Event in the log.
    ///   - payload: The new Event's payload.
    ///   - source: Who created this Event.
    ///   - correlation_id: Optional grouping key.
    ///   - causation_id: Optional link to the causing Event.
    ///
    /// # Returns
    ///   The new Event with correct hash chain and incremented id/time.
    pub fn new_after(
        prev: &Event,
        payload: EventPayload,
        source: EventSource,
        correlation_id: Option<String>,
        causation_id: Option<String>,
    ) -> Self {
        let id = prev.id.next();
        let logical_time = prev.logical_time.next_sequence();
        let parent_hash = prev.event_hash;
        let schema_version = prev.metadata.schema_version; // inherit

        let metadata = EventMetadata::new(
            source,
            correlation_id,
            causation_id,
            schema_version,
            id.seq,
            logical_time.epoch,
            logical_time.tick,
            logical_time.sequence,
        );

        let payload_hash = event_hash::compute_payload_hash(payload_bytes(&payload).as_ref());
        let event_hash = event_hash::compute_event_hash(
            &parent_hash,
            id.to_string().as_bytes(),
            logical_time.to_string().as_bytes(),
            &payload_hash,
        );

        Event {
            id,
            logical_time,
            parent_hash,
            event_hash,
            payload,
            metadata,
        }
    }

    /// Returns true if this Event is the genesis Event.
    pub fn is_genesis(&self) -> bool {
        self.id.is_zero()
    }

    /// Verifies that this Event's event_hash matches a recomputed hash.
    ///
    /// This is the fundamental integrity check: if it fails, the Event
    /// has been tampered with or constructed incorrectly.
    pub fn verify_hash(&self) -> bool {
        let payload_hash = event_hash::compute_payload_hash(payload_bytes(&self.payload).as_ref());
        let expected = event_hash::compute_event_hash(
            &self.parent_hash,
            self.id.to_string().as_bytes(),
            self.logical_time.to_string().as_bytes(),
            &payload_hash,
        );
        self.event_hash == expected
    }

    /// Verifies that this Event correctly follows `prev`:
    ///   - id is the next id after prev
    ///   - parent_hash equals prev.event_hash
    ///   - logical_time is after prev (basic ordering)
    pub fn follows(&self, prev: &Event) -> bool {
        self.id.is_next_of(&prev.id)
            && self.parent_hash == prev.event_hash
            && self.logical_time > prev.logical_time
    }
}

/// Deterministically serializes an EventPayload to bytes for hashing.
///
/// The serialization format is:
///   kind_byte || data
/// where kind_byte is a single ASCII digit and data is the variant-specific
/// canonical representation.
fn payload_bytes(payload: &EventPayload) -> Vec<u8> {
    let mut buf = Vec::new();
    match payload {
        EventPayload::Genesis => {
            buf.push(b'0');
        }
        EventPayload::Lifecycle {
            transition,
            from_state,
            to_state,
        } => {
            buf.push(b'1');
            buf.extend(transition.as_bytes());
            buf.push(0);
            buf.extend(from_state.as_bytes());
            buf.push(0);
            buf.extend(to_state.as_bytes());
        }
        EventPayload::Custom { event_type, data } => {
            buf.push(b'2');
            buf.extend(event_type.as_bytes());
            buf.push(0);
            buf.extend(data);
        }
        EventPayload::Failure {
            component,
            class,
            message,
        } => {
            buf.push(b'3');
            buf.extend(component.as_bytes());
            buf.push(0);
            buf.extend(class.as_bytes());
            buf.push(0);
            buf.extend(message.as_bytes());
        }
    }
    buf
}

// ─── Tests ─────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_has_correct_zero_fields() {
        let g = Event::genesis();
        assert!(g.is_genesis());
        assert_eq!(g.id, EventId::ZERO);
        assert_eq!(g.logical_time, LogicalTime::ZERO);
        assert_eq!(g.parent_hash, EventHash::ZERO);
        assert!(g.verify_hash());
    }

    #[test]
    fn new_after_follows_chain() {
        let g = Event::genesis();
        let e1 = Event::new_after(
            &g,
            EventPayload::Lifecycle {
                transition: "Boot".into(),
                from_state: "Offline".into(),
                to_state: "Running".into(),
            },
            EventSource::System,
            None,
            None,
        );

        assert!(e1.follows(&g));
        assert!(e1.verify_hash());
        assert_eq!(e1.id.epoch, 0);
        assert_eq!(e1.id.seq, 1);
    }

    #[test]
    fn chain_of_three_events() {
        let g = Event::genesis();
        let e1 = Event::new_after(
            &g,
            EventPayload::Custom {
                event_type: "Test".into(),
                data: vec![1, 2, 3],
            },
            EventSource::Agent {
                agent_id: "a1".into(),
            },
            Some("corr-1".into()),
            Some("cause-0".into()),
        );
        let e2 = Event::new_after(
            &e1,
            EventPayload::Custom {
                event_type: "Test2".into(),
                data: vec![4, 5],
            },
            EventSource::User {
                user_id: Some("u1".into()),
            },
            None,
            None,
        );

        assert!(e1.follows(&g));
        assert!(e2.follows(&e1));
        assert!(e1.verify_hash());
        assert!(e2.verify_hash());

        // Tamper detection
        let mut tampered = e1.clone();
        match &mut tampered.payload {
            EventPayload::Custom { data, .. } => data.push(99),
            _ => unreachable!(),
        }
        assert!(!tampered.verify_hash());
    }

    #[test]
    fn hash_chain_breaks_on_parent_mismatch() {
        let g = Event::genesis();
        let e1 = Event::new_after(&g, EventPayload::Genesis, EventSource::System, None, None);
        // Manually break parent
        let broken = Event {
            parent_hash: EventHash::ZERO, // should be g.event_hash
            ..e1.clone()
        };
        assert!(!broken.follows(&g));
        assert!(!broken.verify_hash());
    }

    #[test]
    fn payload_bytes_are_deterministic() {
        let p1 = EventPayload::Lifecycle {
            transition: "Boot".into(),
            from_state: "Off".into(),
            to_state: "On".into(),
        };
        let p2 = EventPayload::Lifecycle {
            transition: "Boot".into(),
            from_state: "Off".into(),
            to_state: "On".into(),
        };
        assert_eq!(payload_bytes(&p1), payload_bytes(&p2));

        let c1 = EventPayload::Custom {
            event_type: "X".into(),
            data: vec![1, 2],
        };
        let c2 = EventPayload::Custom {
            event_type: "X".into(),
            data: vec![1, 2],
        };
        assert_eq!(payload_bytes(&c1), payload_bytes(&c2));
    }
}
