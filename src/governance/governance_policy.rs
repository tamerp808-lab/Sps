use crate::kernel_core::event::EventPayload;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Authority {
    User,
    Constitution,
    System,
    Hybrid { primary: Box<Authority>, secondary: Box<Authority> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionClass {
    Minor,
    Major,
    Critical,
    Forbidden,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegationRule {
    pub decision_class: DecisionClass,
    pub required_authority: Authority,
    pub can_be_delegated: bool,
}

pub struct GovernancePolicy;

impl GovernancePolicy {
    pub fn default_rules() -> Vec<DelegationRule> {
        vec![
            DelegationRule { decision_class: DecisionClass::Minor, required_authority: Authority::System, can_be_delegated: true },
            DelegationRule { decision_class: DecisionClass::Major, required_authority: Authority::Hybrid { primary: Box::new(Authority::System), secondary: Box::new(Authority::User) }, can_be_delegated: false },
            DelegationRule { decision_class: DecisionClass::Critical, required_authority: Authority::User, can_be_delegated: false },
            DelegationRule { decision_class: DecisionClass::Forbidden, required_authority: Authority::Constitution, can_be_delegated: false },
        ]
    }

    pub fn propose_set_policy(rules: &[DelegationRule]) -> EventPayload {
        let serialized = rules.iter().map(|r| format!("{:?}|{:?}|{}", r.decision_class, r.required_authority, r.can_be_delegated)).collect::<Vec<_>>().join(";");
        EventPayload::Custom {
            event_type: "GovernancePolicySet".into(),
            data: serialized.into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_rules_exist() {
        let rules = GovernancePolicy::default_rules();
        assert_eq!(rules.len(), 4);
    }
}
