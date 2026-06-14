// src/kernel_core/reducer.rs - معمم على نوع الحالة

use crate::kernel_core::canonical_state::CanonicalState as KernelCanonicalState;
use crate::kernel_core::event::Event;
use crate::kernel_core::event_hash::EventHash;

pub trait Reducer {
    type State;
    type Event;
    fn apply(&self, state: &Self::State, event: &Self::Event) -> Self::State;
    fn initial_state(&self) -> Self::State;
    #[allow(dead_code)]
    #[allow(dead_code)]
    fn schema_version(&self) -> u32;
}

pub struct KernelReducer;

impl Reducer for KernelReducer {
    type State = KernelCanonicalState;
    type Event = Event;

    fn apply(&self, state: &KernelCanonicalState, event: &Event) -> KernelCanonicalState {
        let genesis_hash = if state.is_empty() {
            event.event_hash
        } else {
            state.genesis_hash
        };
        let new_state = KernelCanonicalState {
            state_hash: EventHash::ZERO,
            last_event_id: event.id,
            last_logical_time: event.logical_time,
            event_count: state.event_count + 1,
            genesis_hash,
            latest_hash: event.event_hash,
            epoch: event.logical_time.epoch,
        };
        let hash = crate::kernel_core::canonical_state::compute_state_hash(&new_state);
        KernelCanonicalState { state_hash: hash, ..new_state }
    }

    fn initial_state(&self) -> KernelCanonicalState {
        KernelCanonicalState::initial()
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    fn schema_version(&self) -> u32 { 1 }
}
