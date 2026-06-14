use crate::kernel_core::event::EventPayload;

pub struct LifecycleManager;

impl LifecycleManager {
    pub fn propose_transition(from: &str, to: &str) -> EventPayload {
        EventPayload::Custom {
            event_type: "RuntimeLifecycleTransition".into(),
            data: format!("{}|{}", from, to).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transition_event() {
        let p = LifecycleManager::propose_transition("Booting", "Running");
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
