// src/memory/procedural/recipe.rs
// Phase 3 — Memory
// Zone B — Cognitive
//
// Purpose:
//   Recipe represents a reusable solution pattern for a recurring
//   problem. Unlike a Procedure (which is a concrete sequence),
//   a Recipe may be parameterized and adapted to different contexts.
//   It is stored in procedural memory and instantiated into a
//   Workflow or Procedure when executed. All operations produce Events.
//
// Constitution Compliance:
//   - المادة الرابعة عشرة (Memory Constitution) — Procedural Memory
//   - Zone B: reads State, produces Events

use crate::kernel_core::event::EventPayload;

/// A parameter in a recipe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeParameter {
    pub name: String,
    pub param_type: String,
    pub default_value: Option<String>,
}

/// A recipe template stored in procedural memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recipe {
    pub recipe_id: String,
    pub name: String,
    pub description: String,
    pub parameters: Vec<RecipeParameter>,
    pub steps_template: Vec<String>, // steps with placeholders like {param}
    pub tags: Vec<String>,
}

pub struct RecipeManager;

impl RecipeManager {
    /// Proposes storing a new recipe.
    pub fn propose_store(
        recipe_id: String,
        name: String,
        description: String,
        parameters: Vec<RecipeParameter>,
        steps_template: Vec<String>,
        tags: Vec<String>,
    ) -> EventPayload {
        let params_str = parameters
            .iter()
            .map(|p| format!("{}:{}", p.name, p.param_type))
            .collect::<Vec<_>>()
            .join(",");
        let steps_str = steps_template.join(";");
        let tags_str = tags.join(",");
        EventPayload::Custom {
            event_type: "ProceduralRecipeStored".into(),
            data: format!(
                "{}|{}|{}|{}|{}|{}",
                recipe_id, name, description, params_str, steps_str, tags_str
            )
            .into_bytes(),
        }
    }

    /// Proposes instantiating a recipe with concrete parameters.
    pub fn propose_instantiate(
        recipe_id: String,
        instance_id: String,
        parameter_values: Vec<(String, String)>,
    ) -> EventPayload {
        let values_str = parameter_values
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(",");
        EventPayload::Custom {
            event_type: "ProceduralRecipeInstantiated".into(),
            data: format!("{}|{}|{}", recipe_id, instance_id, values_str).into_bytes(),
        }
    }

    /// Proposes removing a recipe.
    pub fn propose_remove(recipe_id: String) -> EventPayload {
        EventPayload::Custom {
            event_type: "ProceduralRecipeRemoved".into(),
            data: recipe_id.into_bytes(),
        }
    }

    /// Proposes updating a recipe.
    pub fn propose_update(
        recipe_id: String,
        new_steps_template: Vec<String>,
    ) -> EventPayload {
        let steps_str = new_steps_template.join(";");
        EventPayload::Custom {
            event_type: "ProceduralRecipeUpdated".into(),
            data: format!("{}|{}", recipe_id, steps_str).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_store_creates_event() {
        let params = vec![
            RecipeParameter {
                name: "file_path".into(),
                param_type: "String".into(),
                default_value: None,
            },
        ];
        let steps = vec![
            "read({file_path})".into(),
            "parse()".into(),
        ];
        let payload = RecipeManager::propose_store(
            "recipe.1".into(),
            "Load File".into(),
            "Template for loading a file".into(),
            params,
            steps,
            vec!["io".into()],
        );
        match payload {
            EventPayload::Custom { event_type, data } => {
                assert!(event_type.contains("RecipeStored"));
                let s = String::from_utf8(data).unwrap();
                assert!(s.contains("Load File"));
            }
            _ => panic!("Wrong payload"),
        }
    }

    #[test]
    fn propose_instantiate_creates_event() {
        let values = vec![("file_path".into(), "/etc/config.json".into())];
        let payload = RecipeManager::propose_instantiate(
            "recipe.1".into(),
            "inst.1".into(),
            values,
        );
        match payload {
            EventPayload::Custom { event_type, data } => {
                assert!(event_type.contains("RecipeInstantiated"));
                let s = String::from_utf8(data).unwrap();
                assert!(s.contains("inst.1"));
            }
            _ => panic!("Wrong payload"),
        }
    }
}
