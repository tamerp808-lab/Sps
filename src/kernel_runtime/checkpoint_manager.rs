use crate::kernel_core::event::EventPayload;

pub struct CheckpointManager;

impl CheckpointManager {
    pub fn propose_checkpoint() -> EventPayload {
        EventPayload::Custom {
            event_type: "RuntimeCheckpoint".into(),
            data: vec![].into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkpoint_event() {
        let p = CheckpointManager::propose_checkpoint();
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
