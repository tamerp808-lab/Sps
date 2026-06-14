use crate::kernel_core::event::EventPayload;

pub struct UpgradeManager;

impl UpgradeManager {
    pub fn propose_upgrade(version: &str) -> EventPayload {
        EventPayload::Custom {
            event_type: "RuntimeUpgrade".into(),
            data: version.to_string().into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upgrade_event() {
        let p = UpgradeManager::propose_upgrade("v3.1.0");
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
