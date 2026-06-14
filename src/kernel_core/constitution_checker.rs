use crate::kernel_core::canonical_state::CanonicalState;
use crate::kernel_core::event::Event;
use crate::kernel_core::event_hash::EventHash;
use crate::kernel_core::reducer::Reducer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionReport {
    pub compliant: bool,
    pub rules_checked: usize,
    pub violations: Vec<ConstitutionalViolation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalViolation {
    pub rule: &'static str,
    pub article: &'static str,
    pub severity: ViolationSeverity,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationSeverity { Advisory, Major, Catastrophic }

pub struct ConstitutionChecker;

impl ConstitutionChecker {
    pub fn audit(
        state: &CanonicalState,
        events: &[Event],
        reducer: &dyn Reducer<State = CanonicalState, Event = Event>,
    ) -> ConstitutionReport {
        let mut violations = vec![];
        if let Some(first) = events.first() {
            let initial = reducer.initial_state();
            let r1 = reducer.apply(&initial, first);
            let r2 = reducer.apply(&initial, first);
            if r1.state_hash != r2.state_hash {
                violations.push(ConstitutionalViolation {
                    rule: "C1", article: "المادة الثانية", severity: ViolationSeverity::Catastrophic,
                    description: "Reducer not pure".into(),
                });
            }
        }
        for i in 0..events.len() {
            let event = &events[i];
            if i == 0 && !event.is_genesis() {
                violations.push(ConstitutionalViolation {
                    rule: "C2", article: "المادة الرابعة", severity: ViolationSeverity::Advisory,
                    description: "First event not genesis".into(),
                });
            }
            if i > 0 && !event.follows(&events[i-1]) {
                violations.push(ConstitutionalViolation {
                    rule: "C2", article: "المادة الرابعة", severity: ViolationSeverity::Catastrophic,
                    description: format!("Event {} does not follow", event.id),
                });
            }
            if event.metadata.schema_version == 0 {
                violations.push(ConstitutionalViolation {
                    rule: "C3", article: "المادة الثالثة", severity: ViolationSeverity::Major,
                    description: format!("Event {} schema 0", event.id),
                });
            }
            if event.metadata.logical_time_epoch != event.logical_time.epoch
                || event.metadata.logical_time_tick != event.logical_time.tick
                || event.metadata.logical_time_seq != event.logical_time.sequence
            {
                violations.push(ConstitutionalViolation {
                    rule: "C3", article: "المادة الثالثة", severity: ViolationSeverity::Major,
                    description: "Metadata time mismatch".into(),
                });
            }
        }
        if !events.is_empty() {
            let mut replay_state = reducer.initial_state();
            for event in events {
                replay_state = reducer.apply(&replay_state, event);
            }
            if replay_state.state_hash != state.state_hash {
                violations.push(ConstitutionalViolation {
                    rule: "C4", article: "المادة الخامسة", severity: ViolationSeverity::Catastrophic,
                    description: "Replay divergence".into(),
                });
            }
        }
        for (i, event) in events.iter().enumerate() {
            if i == 0 && event.parent_hash != EventHash::ZERO {
                violations.push(ConstitutionalViolation {
                    rule: "C5", article: "المادة الرابعة", severity: ViolationSeverity::Catastrophic,
                    description: "Genesis parent non-zero".into(),
                });
            }
            if i > 0 && event.parent_hash != events[i-1].event_hash {
                violations.push(ConstitutionalViolation {
                    rule: "C5", article: "المادة الرابعة", severity: ViolationSeverity::Catastrophic,
                    description: "Hash chain broken".into(),
                });
            }
            if !event.verify_hash() {
                violations.push(ConstitutionalViolation {
                    rule: "C5", article: "المادة الرابعة", severity: ViolationSeverity::Catastrophic,
                    description: "Self-hash fail".into(),
                });
            }
        }
        if events.len() > 1 {
            let base = events[0].metadata.schema_version;
            for event in &events[1..] {
                if event.metadata.schema_version != base {
                    violations.push(ConstitutionalViolation {
                        rule: "C7", article: "المادة التاسعة عشرة", severity: ViolationSeverity::Catastrophic,
                        description: "Schema version change without sandbox".into(),
                    });
                }
            }
        }
        ConstitutionReport {
            compliant: violations.is_empty(),
            rules_checked: 7,
            violations,
        }
    }
}
