use crate::governance::governance_policy::{Authority, DecisionClass, DelegationRule};
use crate::kernel_core::event::EventPayload;

pub struct AuthorityResolver;

impl AuthorityResolver {
    pub fn resolve(decision: &DecisionClass, rules: &[DelegationRule]) -> Option<Authority> {
        rules.iter().find(|r| r.decision_class == *decision).map(|r| r.required_authority.clone())
    }

    pub fn propose_resolution(decision: DecisionClass, required: Authority) -> EventPayload {
        EventPayload::Custom {
            event_type: "AuthorityResolved".into(),
            data: format!("{:?}|{:?}", decision, required).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::governance::governance_policy::GovernancePolicy;

    #[test]
    fn minor_requires_system() {
        let rules = GovernancePolicy::default_rules();
        let authority = AuthorityResolver::resolve(&DecisionClass::Minor, &rules);
        assert_eq!(authority, Some(Authority::System));
    }

    #[test]
    fn critical_requires_user() {
        let rules = GovernancePolicy::default_rules();
        let authority = AuthorityResolver::resolve(&DecisionClass::Critical, &rules);
        assert_eq!(authority, Some(Authority::User));
    }
}
