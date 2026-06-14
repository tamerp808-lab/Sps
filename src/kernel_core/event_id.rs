use serde::{Serialize, Deserialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EventId {
    pub epoch: u64,
    pub seq: u64,
}

impl EventId {
    pub const ZERO: EventId = EventId { epoch: 0, seq: 0 };

    #[inline]
    pub const fn new(epoch: u64, seq: u64) -> Self { EventId { epoch, seq } }

    #[inline]
    pub const fn next(&self) -> Self { EventId { epoch: self.epoch, seq: self.seq + 1 } }

    #[inline]
    pub const fn next_epoch(&self) -> Self { EventId { epoch: self.epoch + 1, seq: 1 } }

    #[inline]
    pub const fn is_zero(&self) -> bool { self.epoch == 0 && self.seq == 0 }

    #[inline]
    pub const fn previous(&self) -> Option<Self> {
        if self.seq == 0 { None } else { Some(EventId { epoch: self.epoch, seq: self.seq - 1 }) }
    }

    #[inline]
    pub const fn is_next_of(&self, other: &EventId) -> bool {
        if self.epoch == other.epoch { self.seq == other.seq + 1 }
        else if other.epoch + 1 == self.epoch { self.seq == 1 }
        else { false }
    }
}

impl fmt::Display for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}.{}", self.epoch, self.seq) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseEventIdError { pub input: String, pub reason: ParseEventIdReason }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseEventIdReason { MissingDot, InvalidEpoch, InvalidSeq, EmptyInput }
impl fmt::Display for ParseEventIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse EventId from '{}': {:?}", self.input, self.reason)
    }
}
impl std::error::Error for ParseEventIdError {}

impl FromStr for EventId {
    type Err = ParseEventIdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() { return Err(ParseEventIdError { input: s.to_string(), reason: ParseEventIdReason::EmptyInput }); }
        let (ep, sq) = s.split_once('.').ok_or_else(|| ParseEventIdError { input: s.to_string(), reason: ParseEventIdReason::MissingDot })?;
        let epoch = ep.parse::<u64>().map_err(|_| ParseEventIdError { input: s.to_string(), reason: ParseEventIdReason::InvalidEpoch })?;
        let seq = sq.parse::<u64>().map_err(|_| ParseEventIdError { input: s.to_string(), reason: ParseEventIdReason::InvalidSeq })?;
        Ok(EventId { epoch, seq })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test] fn zero_is_genesis() { let z = EventId::ZERO; assert!(z.is_zero()); }
    #[test] fn next_increments() { let a = EventId::new(1,5); assert_eq!(a.next(), EventId::new(1,6)); }
    #[test] fn next_epoch() { let a = EventId::new(1,999); assert_eq!(a.next_epoch(), EventId::new(2,1)); }
    #[test] fn ordering() { assert!(EventId::new(1,10) < EventId::new(2,1)); }
    #[test] fn previous() { assert_eq!(EventId::new(1,1).previous(), Some(EventId::new(1,0))); }
    #[test] fn is_next_of_chain() {
        let a = EventId::new(1,10); let b = EventId::new(1,11); let c = EventId::new(1,12);
        assert!(b.is_next_of(&a)); assert!(c.is_next_of(&b)); assert!(!c.is_next_of(&a));
    }
    #[test] fn is_next_of_epochs() {
        let last = EventId::new(1,999); let first = EventId::new(2,1);
        assert!(first.is_next_of(&last));
    }
    #[test] fn display_and_parse() {
        let id = EventId::new(3,42);
        let s = id.to_string();
        assert_eq!(s.parse::<EventId>().unwrap(), id);
    }
    #[test] fn deterministic_hash() {
        let id = EventId::new(7,13);
        let mut h1 = DefaultHasher::new(); let mut h2 = DefaultHasher::new();
        id.hash(&mut h1); id.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }
}
