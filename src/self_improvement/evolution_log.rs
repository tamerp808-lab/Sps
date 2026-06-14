use crate::kernel_core::event::EventPayload;

pub struct EvolutionLog;

impl EvolutionLog {
    pub fn log(entry: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "EvolutionLogEntry".into(),
            data: entry.into_bytes(),
        }
    }
}
