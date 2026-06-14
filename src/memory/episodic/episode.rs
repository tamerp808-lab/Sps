use crate::kernel_core::event::EventPayload;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Episode {
    pub episode_id: String,
    pub source_event_id: String,
    pub epoch: u64,
    pub tick: u64,
    pub description: String,
    pub participants: Vec<String>,
    pub facts_observed: Vec<String>,
    pub importance: OrderedFloat<f64>,
}

pub struct EpisodeRecorder;

impl EpisodeRecorder {
    pub fn propose_record(episode_id: String, source_event_id: String, epoch: u64, tick: u64, description: String, participants: Vec<String>, facts_observed: Vec<String>, importance: f64) -> EventPayload {
        let participants_str = participants.join(",");
        let facts_str = facts_observed.join(",");
        EventPayload::Custom {
            event_type: "EpisodeRecorded".into(),
            data: format!("{}|{}|{}|{}|{}|{}|{}|{}", episode_id, source_event_id, epoch, tick, description, participants_str, facts_str, importance).into_bytes(),
        }
    }

    pub fn propose_replay(episode_id: String) -> EventPayload {
        EventPayload::Custom { event_type: "EpisodeReplay".into(), data: episode_id.into_bytes() }
    }

    pub fn propose_forget(episode_id: String, reason: String) -> EventPayload {
        EventPayload::Custom { event_type: "EpisodeForgotten".into(), data: format!("{}|{}", episode_id, reason).into_bytes() }
    }
}
