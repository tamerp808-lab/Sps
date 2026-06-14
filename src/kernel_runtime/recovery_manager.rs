use crate::kernel_core::event::EventPayload;

pub struct RecoveryManager;

impl RecoveryManager {
    pub fn propose_recover(reason: &str) -> EventPayload {
        EventPayload::Custom {
            event_type: "RuntimeRecovery".into(),
            data: reason.to_string().into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recovery_event() {
        let p = RecoveryManager::propose_recover("panic");
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
