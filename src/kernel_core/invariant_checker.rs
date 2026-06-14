use crate::kernel_core::canonical_state::CanonicalState;
use crate::kernel_core::event::Event;
use crate::kernel_core::event_hash::EventHash;
use crate::kernel_core::reducer::Reducer;
use crate::kernel_core::replay_validator::ReplayValidator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantReport {
    pub all_passed: bool,
    pub invariants_checked: usize,
    pub failures: Vec<InvariantFailure>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantFailure {
    pub invariant_name: &'static str,
    pub description: String,
}

pub struct InvariantChecker;

impl InvariantChecker {
    pub fn check_all(
        state: &CanonicalState,
        events: &[Event],
        reducer: &dyn Reducer<State = CanonicalState, Event = Event>,
    ) -> InvariantReport {
        let mut failures = vec![];
        if !state.verify_hash() {
            failures.push(InvariantFailure { invariant_name: "I1", description: "State hash mismatch".into() });
        }
        if !state.is_empty() && state.genesis_hash == EventHash::ZERO {
            failures.push(InvariantFailure { invariant_name: "I3", description: "Non-empty state genesis zero".into() });
        }
        if !state.is_empty() && state.latest_hash == EventHash::ZERO {
            failures.push(InvariantFailure { invariant_name: "I4", description: "Non-empty state latest zero".into() });
        }
        if let Some(last_event) = events.last() {
            if last_event.logical_time != state.last_logical_time {
                failures.push(InvariantFailure { invariant_name: "I5", description: "Last event time mismatch".into() });
            }
            for i in 1..events.len() {
                if events[i].logical_time <= events[i-1].logical_time {
                    failures.push(InvariantFailure { invariant_name: "I5", description: format!("Time order violation at {}", i) });
                    break;
                }
            }
        } else if !state.is_empty() {
            failures.push(InvariantFailure { invariant_name: "I5", description: "No events but state non-empty".into() });
        }
        let chain = ReplayValidator::verify_chain_integrity(events);
        for f in chain.failures {
            failures.push(InvariantFailure { invariant_name: "I6", description: format!("Chain broken at {}: {}", f.index, f.reason) });
        }
        if let Some(first) = events.first() {
            let initial = reducer.initial_state();
            let r1 = reducer.apply(&initial, first);
            let r2 = reducer.apply(&initial, first);
            if r1.state_hash != r2.state_hash {
                failures.push(InvariantFailure { invariant_name: "I7", description: "Reducer not pure".into() });
            }
        }
        InvariantReport {
            all_passed: failures.is_empty(),
            invariants_checked: 7,
            failures,
        }
    }
}
