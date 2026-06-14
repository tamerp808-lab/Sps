use crate::kernel_core::event::EventPayload;

pub struct ArchitectureReasoner;
impl ArchitectureReasoner {
    pub fn infer_dependency(module_a: &str, module_b: &str, imports: &[(String, Vec<String>)]) -> bool {
        imports.iter().any(|(m, deps)| m == module_a && deps.contains(&module_b.to_string()))
    }
    pub fn propose_dependency(from: String, to: String) -> EventPayload {
        EventPayload::Custom { event_type: "ArchitectureDependencyInferred".into(), data: format!("{}|{}", from, to).into_bytes() }
    }
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn infers() { let imports = vec![("kernel".into(), vec!["event".into()])]; assert!(ArchitectureReasoner::infer_dependency("kernel", "event", &imports)); }
}
