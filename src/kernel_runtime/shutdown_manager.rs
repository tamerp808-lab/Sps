use crate::kernel_core::event::EventPayload;

pub struct ShutdownManager;

impl ShutdownManager {
    pub fn propose_shutdown(reason: &str) -> EventPayload {
        EventPayload::Custom {
            event_type: "RuntimeShutdown".into(),
            data: reason.to_string().into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shutdown_event() {
        let p = ShutdownManager::propose_shutdown("user request");
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
