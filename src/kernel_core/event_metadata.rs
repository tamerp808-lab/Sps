use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventSource {
    User { user_id: Option<String> },
    Agent { agent_id: String },
    System,
    Llm { model_id: String },
    CapabilityProvider { provider_id: String },
}

impl std::fmt::Display for EventSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventSource::User { user_id } => write!(f, "User({})", user_id.as_deref().unwrap_or("?")),
            EventSource::Agent { agent_id } => write!(f, "Agent({})", agent_id),
            EventSource::System => write!(f, "System"),
            EventSource::Llm { model_id } => write!(f, "Llm({})", model_id),
            EventSource::CapabilityProvider { provider_id } => write!(f, "CapabilityProvider({})", provider_id),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventMetadata {
    pub source: EventSource,
    pub correlation_id: Option<String>,
    pub causation_id: Option<String>,
    pub schema_version: u32,
    pub sequence: u64,
    pub logical_time_epoch: u64,
    pub logical_time_tick: u64,
    pub logical_time_seq: u64,
}

impl EventMetadata {
    pub fn new(source: EventSource, correlation_id: Option<String>, causation_id: Option<String>, schema_version: u32, sequence: u64, logical_time_epoch: u64, logical_time_tick: u64, logical_time_seq: u64) -> Self {
        EventMetadata { source, correlation_id, causation_id, schema_version, sequence, logical_time_epoch, logical_time_tick, logical_time_seq }
    }

    pub const fn logical_time_triple(&self) -> (u64, u64, u64) { (self.logical_time_epoch, self.logical_time_tick, self.logical_time_seq) }
}

impl std::fmt::Display for EventMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "src={} corr={:?} caus={:?} schema={} seq={} lt={}.{}.{}", self.source, self.correlation_id, self.causation_id, self.schema_version, self.sequence, self.logical_time_epoch, self.logical_time_tick, self.logical_time_seq)
    }
}
