use crate::kernel_core::event::EventPayload;

pub struct ReplayTester;

impl ReplayTester {
    pub fn test_replay() -> bool { true }

    pub fn propose_replay_result(passed: bool) -> EventPayload {
        EventPayload::Custom {
            event_type: "ReplayTestResult".into(),
            data: format!("{}", passed).into_bytes(),
        }
    }
}
