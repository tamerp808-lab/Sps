use crate::kernel_core::canonical_state::CanonicalState;
use crate::kernel_core::constitution_checker::ConstitutionChecker;
use crate::kernel_core::event::Event;
use crate::kernel_core::invariant_checker::InvariantChecker;
use crate::kernel_core::reducer::Reducer;
use crate::kernel_core::replay_validator::ReplayValidator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationReport {
    pub passed: bool,
    pub replay: crate::kernel_core::replay_validator::ReplayResult,
    pub invariants: crate::kernel_core::invariant_checker::InvariantReport,
    pub constitution: crate::kernel_core::constitution_checker::ConstitutionReport,
    pub summary: String,
}

pub struct VerificationSuite;

impl VerificationSuite {
    pub fn verify(
        reducer: &dyn Reducer<State = CanonicalState, Event = Event>,
        events: &[Event],
        state: &CanonicalState,
    ) -> VerificationReport {
        let replay = ReplayValidator::validate(reducer, events, state);
        let invariants = InvariantChecker::check_all(state, events, reducer);
        let constitution = ConstitutionChecker::audit(state, events, reducer);
        let passed = replay.passed && invariants.all_passed && constitution.compliant;
        let summary = if passed {
            "All verification steps passed.".into()
        } else {
            let mut f = vec![];
            if !replay.passed { f.push("Replay"); }
            if !invariants.all_passed { f.push("Invariants"); }
            if !constitution.compliant { f.push("Constitution"); }
            format!("FAILED: {}", f.join(", "))
        };
        VerificationReport { passed, replay, invariants, constitution, summary }
    }
}
