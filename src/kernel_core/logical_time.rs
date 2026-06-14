use serde::{Serialize, Deserialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LogicalTime {
    pub epoch: u64,
    pub tick: u64,
    pub sequence: u64,
}

impl LogicalTime {
    pub const ZERO: LogicalTime = LogicalTime { epoch: 0, tick: 0, sequence: 0 };

    #[inline]
    pub const fn new(epoch: u64, tick: u64, sequence: u64) -> Self { LogicalTime { epoch, tick, sequence } }

    #[inline] pub const fn next_sequence(&self) -> Self { LogicalTime { epoch: self.epoch, tick: self.tick, sequence: self.sequence + 1 } }
    #[inline] pub const fn next_tick(&self) -> Self { LogicalTime { epoch: self.epoch, tick: self.tick + 1, sequence: 0 } }
    #[inline] pub const fn next_epoch(&self) -> Self { LogicalTime { epoch: self.epoch + 1, tick: 0, sequence: 0 } }
    #[inline] pub const fn is_zero(&self) -> bool { self.epoch == 0 && self.tick == 0 && self.sequence == 0 }

    pub fn events_between(&self, other: &LogicalTime) -> Option<u64> {
        if self.epoch != other.epoch { return None; }
        let self_total = self.tick as u128 * (u64::MAX as u128) + self.sequence as u128;
        let other_total = other.tick as u128 * (u64::MAX as u128) + other.sequence as u128;
        if other_total >= self_total { Some((other_total - self_total) as u64) } else { None }
    }
}

impl fmt::Display for LogicalTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}.{}.{}", self.epoch, self.tick, self.sequence) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseLogicalTimeError { pub input: String, pub reason: ParseLogicalTimeReason }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseLogicalTimeReason { EmptyInput, MissingDot, InvalidEpoch, InvalidTick, InvalidSequence }
impl fmt::Display for ParseLogicalTimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "Failed to parse LogicalTime: {:?}", self) }
}
impl std::error::Error for ParseLogicalTimeError {}

impl FromStr for LogicalTime {
    type Err = ParseLogicalTimeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() { return Err(ParseLogicalTimeError { input: s.to_string(), reason: ParseLogicalTimeReason::EmptyInput }); }
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 { return Err(ParseLogicalTimeError { input: s.to_string(), reason: ParseLogicalTimeReason::MissingDot }); }
        let epoch = parts[0].parse::<u64>().map_err(|_| ParseLogicalTimeError { input: s.to_string(), reason: ParseLogicalTimeReason::InvalidEpoch })?;
        let tick = parts[1].parse::<u64>().map_err(|_| ParseLogicalTimeError { input: s.to_string(), reason: ParseLogicalTimeReason::InvalidTick })?;
        let sequence = parts[2].parse::<u64>().map_err(|_| ParseLogicalTimeError { input: s.to_string(), reason: ParseLogicalTimeReason::InvalidSequence })?;
        Ok(LogicalTime { epoch, tick, sequence })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn zero() { assert!(LogicalTime::ZERO.is_zero()); }
    #[test] fn next_seq() { let t = LogicalTime::new(1,5,10); let n = t.next_sequence(); assert_eq!(n.sequence, 11); }
    #[test] fn ordering() { assert!(LogicalTime::new(0,0,0) < LogicalTime::new(0,0,1)); }
    #[test] fn display_parse() {
        let t = LogicalTime::new(1,2,3);
        let s = t.to_string();
        assert_eq!(s.parse::<LogicalTime>().unwrap(), t);
    }
    #[test] fn events_between() {
        let t0 = LogicalTime::new(1,0,0);
        let t1 = LogicalTime::new(1,0,5);
        assert_eq!(t0.events_between(&t1), Some(5));
        assert_eq!(t1.events_between(&t0), None);
    }
}
