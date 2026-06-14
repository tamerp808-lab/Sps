use crate::kernel_core::event::EventPayload;

pub struct Scheduler;

impl Scheduler {
    pub fn propose_schedule(component: String, tick: u64) -> EventPayload {
        EventPayload::Custom {
            event_type: "RuntimeSchedule".into(),
            data: format!("{}|{}", component, tick).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schedule_event() {
        let p = Scheduler::propose_schedule("memory".into(), 42);
        match p { EventPayload::Custom{..} => (), _ => panic!() }
    }
}
