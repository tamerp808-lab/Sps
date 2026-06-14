use crate::kernel_core::event::EventPayload;

pub struct ArchitectureReasoner;

impl ArchitectureReasoner {
    /// Simple heuristic: if a module imports another, a dependency exists.
    pub fn infer_dependency(module_a: &str, module_b: &str, imports: &[(String, Vec<String>)]) -> bool {
        imports.iter()
            .find(|(m, _)| m == module_a)
            .map(|(_, deps)| deps.contains(&module_b.to_string()))
            .unwrap_or(false)
    }

    pub fn propose_dependency(from: String, to: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "ArchitectureDependencyInferred".into(),
            data: format!("{}|{}", from, to).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infers_existing_dependency() {
        let imports = vec![("kernel".into(), vec!["event".into()])];
        assert!(ArchitectureReasoner::infer_dependency("kernel", "event", &imports));
    }
}
