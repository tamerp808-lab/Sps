#[cfg(test)]
mod replay_stress {
    use crate::kernel_core::event::{Event, EventPayload};
    use crate::kernel_core::event_metadata::EventSource;
    use crate::kernel_core::reducer::{Reducer, KernelReducer};
    use crate::kernel_core::replay_validator::ReplayValidator;

    const EVENT_COUNT: usize = 100_000;

    #[test]
    fn replay_100k_events_is_perfect() {
        let reducer = KernelReducer;
        let g = Event::genesis();
        let mut events: Vec<Event> = Vec::with_capacity(EVENT_COUNT + 1);
        events.push(g.clone());
        let mut prev = g;
        for i in 0..EVENT_COUNT {
            let e = Event::new_after(
                &prev,
                EventPayload::Custom {
                    event_type: "stress".into(),
                    data: i.to_le_bytes().to_vec(),
                },
                EventSource::System,
                None,
                None,
            );
            events.push(e.clone());
            prev = e;
        }
        let mut live_state = reducer.initial_state();
        for e in &events {
            live_state = reducer.apply(&live_state, e);
        }
        let replay_result = ReplayValidator::validate(&reducer, &events, &live_state);
        assert!(replay_result.passed, "Replay failed: {}", replay_result.diagnostic);
        assert_eq!(live_state.event_count, events.len() as u64);
    }
}
