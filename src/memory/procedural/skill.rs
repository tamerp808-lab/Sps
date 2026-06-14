use crate::kernel_core::event::EventPayload;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillStep { pub step_number: u32, pub description: String, pub capability_id: Option<String>, pub timeout_ticks: Option<u64> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skill {
    pub skill_id: String, pub name: String, pub description: String,
    pub steps: Vec<SkillStep>, pub prerequisites: Vec<String>,
    pub success_rate: OrderedFloat<f64>, pub times_executed: u64,
}

pub struct SkillManager;

impl SkillManager {
    pub fn propose_register(skill_id: String, name: String, description: String, steps: Vec<SkillStep>, prerequisites: Vec<String>) -> EventPayload {
        let steps_str = steps.iter().map(|s| format!("{}:{}", s.step_number, s.description)).collect::<Vec<_>>().join(";");
        let prereq_str = prerequisites.join(",");
        EventPayload::Custom { event_type: "ProceduralSkillRegistered".into(), data: format!("{}|{}|{}|{}|{}", skill_id, name, description, steps_str, prereq_str).into_bytes() }
    }

    pub fn propose_update_success_rate(skill_id: String, new_rate: f64, times_executed: u64) -> EventPayload {
        EventPayload::Custom { event_type: "ProceduralSkillRateUpdated".into(), data: format!("{}|{}|{}", skill_id, new_rate, times_executed).into_bytes() }
    }

    pub fn propose_remove(skill_id: String) -> EventPayload {
        EventPayload::Custom { event_type: "ProceduralSkillRemoved".into(), data: skill_id.into_bytes() }
    }

    pub fn propose_execute(skill_id: String, context: String) -> EventPayload {
        EventPayload::Custom { event_type: "ProceduralSkillExecute".into(), data: format!("{}|{}", skill_id, context).into_bytes() }
    }
}
