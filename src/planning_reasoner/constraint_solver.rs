use crate::kernel_core::event::EventPayload;

pub struct ConstraintSolver;

impl ConstraintSolver {
    /// Simple constraint check: all resources must be <= limits.
    pub fn check_constraints(usage: &[(String, u64)], limits: &[(String, u64)]) -> bool {
        usage.iter().all(|(res, used)| {
            limits.iter().any(|(lres, limit)| res == lres && used <= limit)
        })
    }

    pub fn propose_constraint_check(passed: bool) -> EventPayload {
        EventPayload::Custom {
            event_type: "ConstraintChecked".into(),
            data: format!("{}", passed).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_passes_within_limit() {
        let usage = vec![("cpu".into(), 50)];
        let limits = vec![("cpu".into(), 100)];
        assert!(ConstraintSolver::check_constraints(&usage, &limits));
    }

    #[test]
    fn check_fails_exceeding_limit() {
        let usage = vec![("cpu".into(), 150)];
        let limits = vec![("cpu".into(), 100)];
        assert!(!ConstraintSolver::check_constraints(&usage, &limits));
    }
}
