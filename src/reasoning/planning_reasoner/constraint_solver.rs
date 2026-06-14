use crate::kernel_core::event::EventPayload;

pub struct ConstraintSolver;
impl ConstraintSolver {
    pub fn check_constraints(usage: &[(String, u64)], limits: &[(String, u64)]) -> bool {
        usage.iter().all(|(res, used)| limits.iter().any(|(lres, limit)| res == lres && used <= limit))
    }
    pub fn propose_constraint_check(passed: bool) -> EventPayload {
        EventPayload::Custom { event_type: "ConstraintChecked".into(), data: format!("{}", passed).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn passes() { assert!(ConstraintSolver::check_constraints(&[("cpu".into(), 50)], &[("cpu".into(), 100)])); }
    #[test] fn fails() { assert!(!ConstraintSolver::check_constraints(&[("cpu".into(), 150)], &[("cpu".into(), 100)])); }
}
