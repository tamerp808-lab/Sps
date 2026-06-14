use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EventHash([u8; 32]);

impl EventHash {
    pub const ZERO: EventHash = EventHash([0u8; 32]);

    #[inline]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self { EventHash(bytes) }

    #[inline]
    pub const fn as_bytes(&self) -> &[u8; 32] { &self.0 }
}

pub fn compute_event_hash(parent_hash: &EventHash, event_id_bytes: &[u8], logical_time_bytes: &[u8], payload_hash: &EventHash) -> EventHash {
    let mut hasher = Sha256::new();
    hasher.update(parent_hash.as_bytes());
    hasher.update(event_id_bytes);
    hasher.update(logical_time_bytes);
    hasher.update(payload_hash.as_bytes());
    let result = hasher.finalize();
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&result);
    EventHash(bytes)
}

pub fn compute_payload_hash(payload_bytes: &[u8]) -> EventHash {
    let mut hasher = Sha256::new();
    hasher.update(payload_bytes);
    let result = hasher.finalize();
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&result);
    EventHash(bytes)
}

impl std::fmt::Display for EventHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.0 { write!(f, "{:02x}", byte)?; }
        Ok(())
    }
}

impl std::str::FromStr for EventHash {
    type Err = ParseEventHashError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 64 { return Err(ParseEventHashError { input: s.to_string(), reason: ParseEventHashReason::InvalidLength }); }
        let mut bytes = [0u8; 32];
        for i in 0..32 {
            let byte_str = &s[i*2..i*2+2];
            bytes[i] = u8::from_str_radix(byte_str, 16).map_err(|_| ParseEventHashError { input: s.to_string(), reason: ParseEventHashReason::InvalidHex })?;
        }
        Ok(EventHash(bytes))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseEventHashError { pub input: String, pub reason: ParseEventHashReason }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseEventHashReason { EmptyInput, InvalidLength, InvalidHex }
impl std::fmt::Display for ParseEventHashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Failed to parse EventHash: {:?}", self) }
}
impl std::error::Error for ParseEventHashError {}
