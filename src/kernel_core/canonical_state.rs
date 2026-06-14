use serde::{Serialize, Deserialize};
use crate::kernel_core::event_hash::EventHash;
use crate::kernel_core::event_id::EventId;
use crate::kernel_core::logical_time::LogicalTime;
use sha2::Digest;
use sha2::Sha256;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalState {
    pub state_hash: EventHash,
    pub last_event_id: EventId,
    pub last_logical_time: LogicalTime,
    pub event_count: u64,
    pub genesis_hash: EventHash,
    pub latest_hash: EventHash,
    pub epoch: u64,
}

impl CanonicalState {
    pub fn initial() -> Self {
        let state = CanonicalState {
            state_hash: EventHash::ZERO,
            last_event_id: EventId::ZERO,
            last_logical_time: LogicalTime::ZERO,
            event_count: 0,
            genesis_hash: EventHash::ZERO,
            latest_hash: EventHash::ZERO,
            epoch: 0,
        };
        let hash = compute_state_hash(&state);
        CanonicalState { state_hash: hash, ..state }
    }

    pub fn is_empty(&self) -> bool { self.event_count == 0 }

    pub fn verify_hash(&self) -> bool {
        let recomputed = compute_state_hash(self);
        self.state_hash == recomputed
    }
}

pub fn compute_state_hash(state: &CanonicalState) -> EventHash {
    let mut hasher = Sha256::new();
    hasher.update(b"CanonicalState v1");
    hasher.update(state.last_event_id.to_string().as_bytes());
    hasher.update(state.last_logical_time.to_string().as_bytes());
    hasher.update(state.event_count.to_le_bytes());
    hasher.update(state.genesis_hash.as_bytes());
    hasher.update(state.latest_hash.as_bytes());
    hasher.update(state.epoch.to_le_bytes());
    let result = hasher.finalize();
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&result);
    EventHash::from_bytes(bytes)
}
