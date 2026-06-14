use crate::kernel_core::canonical_state::CanonicalState;
use crate::kernel_core::event::Event;
use crate::kernel_core::event_hash::EventHash;
use crate::kernel_core::reducer::Reducer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayResult {
    pub passed: bool,
    pub events_replayed: u64,
    pub replayed_state_hash: EventHash,
    pub expected_state_hash: EventHash,
    pub divergence_at: Option<u64>,
    pub diagnostic: String,
}

pub struct ReplayValidator;

impl ReplayValidator {
    pub fn validate(
        reducer: &dyn Reducer<State = CanonicalState, Event = Event>,
        events: &[Event],
        expected_state: &CanonicalState,
    ) -> ReplayResult {
        if events.is_empty() {
            return ReplayResult {
                passed: expected_state.is_empty(),
                events_replayed: 0,
                replayed_state_hash: expected_state.state_hash,
                expected_state_hash: expected_state.state_hash,
                divergence_at: None,
                diagnostic: if expected_state.is_empty() {
                    "No events and state is empty — consistent.".into()
                } else {
                    "No events provided but expected state is not empty.".into()
                },
            };
        }
        let mut replayed = reducer.initial_state();
        for event in events {
            replayed = reducer.apply(&replayed, event);
        }
        let passed = replayed.state_hash == expected_state.state_hash;
        let diagnostic = if passed {
            format!("Replay successful: {} events replayed.", events.len())
        } else {
            format!("Replay FAILED: replayed hash {} != expected hash {}.", replayed.state_hash, expected_state.state_hash)
        };
        ReplayResult {
            passed,
            events_replayed: events.len() as u64,
            replayed_state_hash: replayed.state_hash,
            expected_state_hash: expected_state.state_hash,
            divergence_at: None,
            diagnostic,
        }
    }

    pub fn validate_live_vs_replay(
        reducer: &dyn Reducer<State = CanonicalState, Event = Event>,
        events: &[Event],
    ) -> ReplayResult {
        let mut live_state = reducer.initial_state();
        for event in events {
            live_state = reducer.apply(&live_state, event);
        }
        Self::validate(reducer, events, &live_state)
    }

    pub fn verify_chain_integrity(events: &[Event]) -> ChainIntegrityResult {
        if events.is_empty() {
            return ChainIntegrityResult { passed: true, events_checked: 0, failures: vec![] };
        }
        let mut failures = vec![];
        let first = &events[0];
        if !first.is_genesis() && first.parent_hash != EventHash::ZERO {
            failures.push(ChainFailure { index: 0, reason: "First event not genesis and parent non-zero.".into() });
        }
        for i in 1..events.len() {
            let prev = &events[i - 1];
            let curr = &events[i];
            if curr.parent_hash != prev.event_hash {
                failures.push(ChainFailure { index: i as u64, reason: format!("parent_hash mismatch at {}", i) });
            }
            if !curr.verify_hash() {
                failures.push(ChainFailure { index: i as u64, reason: "self-hash fail".into() });
            }
        }
        ChainIntegrityResult {
            passed: failures.is_empty(),
            events_checked: events.len() as u64,
            failures,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainIntegrityResult {
    pub passed: bool,
    pub events_checked: u64,
    pub failures: Vec<ChainFailure>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainFailure {
    pub index: u64,
    pub reason: String,
}
