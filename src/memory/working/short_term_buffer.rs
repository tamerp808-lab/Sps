// src/memory/working/short_term_buffer.rs
// Phase 3 — Memory
// Zone B — Cognitive
//
// Purpose:
//   ShortTermBuffer is a transient, bounded buffer that holds the
//   most recent events or observations before they are consolidated
//   into episodic memory. It acts as an intermediate staging area
//   and is cleared at the end of each session or cycle. It produces
//   Events for every buffer operation.
//
// Constitution Compliance:
//   - المادة الرابعة عشرة (Memory Constitution) — Working Memory
//   - Zone B: reads State, produces Events

use crate::canonical_state::memory_state::MemoryState;
use crate::kernel_core::event::EventPayload;

/// Maximum number of items in the short-term buffer.
const MAX_BUFFER_SIZE: usize = 50;

/// A single item in the short-term buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferItem {
    pub item_id: String,
    pub content: String,
    pub timestamp_tick: u64,
}

pub struct ShortTermBuffer;

impl ShortTermBuffer {
    /// Returns the items currently in the short-term buffer.
    /// In a full implementation, the buffer would be stored in MemoryState.
    /// For now, we return an empty vector.
    pub fn get_all(_state: &MemoryState) -> &[BufferItem] {
        // Placeholder: buffer is not yet part of MemoryState
        // Will be added when MemoryState schema is upgraded
        &[]
    }

    /// Proposes adding an observation to the short-term buffer.
    pub fn propose_add(item_id: String, content: String, tick: u64) -> EventPayload {
        EventPayload::Custom {
            event_type: "ShortTermBufferAdd".into(),
            data: format!("{}|{}|{}", item_id, content, tick).into_bytes(),
        }
    }

    /// Proposes flushing the buffer to episodic memory.
    pub fn propose_flush() -> EventPayload {
        EventPayload::Custom {
            event_type: "ShortTermBufferFlush".into(),
            data: vec![].into(),
        }
    }

    /// Proposes clearing the buffer (e.g., at session end).
    pub fn propose_clear() -> EventPayload {
        EventPayload::Custom {
            event_type: "ShortTermBufferClear".into(),
            data: vec![].into(),
        }
    }

    /// Returns the maximum buffer size.
    pub fn max_size() -> usize {
        MAX_BUFFER_SIZE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_add_produces_event() {
        let payload = ShortTermBuffer::propose_add("b1".into(), "observed event".into(), 100);
        match payload {
            EventPayload::Custom { event_type, data } => {
                assert!(event_type.contains("BufferAdd"));
                assert!(!data.is_empty());
            }
            _ => panic!("Wrong payload"),
        }
    }

    #[test]
    fn propose_flush_produces_event() {
        let payload = ShortTermBuffer::propose_flush();
        match payload {
            EventPayload::Custom { event_type, .. } => {
                assert!(event_type.contains("BufferFlush"));
            }
            _ => panic!("Wrong payload"),
        }
    }
}
