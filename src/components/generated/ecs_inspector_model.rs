#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct EcsEntityRow {
    pub entity_id: i64,
    pub component_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct EcsInspectorField {
    pub name: String,
    pub display_value: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct EcsComponentSection {
    pub name: String,
    pub fields: Vec<EcsInspectorField>,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct EcsInspectorSnapshot {
    pub entities: Vec<EcsEntityRow>,
    pub selected_entity_id: Option<i64>,
    pub components: Vec<EcsComponentSection>,
}

impl EcsInspectorSnapshot {
    #[inline]
    pub fn empty() -> EcsInspectorSnapshot {
        EcsInspectorSnapshot {
            entities: Vec::new(),
            selected_entity_id: None,
            components: Vec::new(),
        }
    }
    #[inline]
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }
    #[inline]
    pub fn selected_entity_label(&self) -> String {
        if let Some(id) = self.selected_entity_id {
            format!("Entity #{}", id)
        } else {
            "No entity selected".to_string().to_string()
        }
    }
    #[inline]
    pub fn mock_runtime_demo() -> EcsInspectorSnapshot {
        let mut entities: Vec<EcsEntityRow> = Vec::new();
        entities.push(EcsEntityRow {
            entity_id: 1_i64,
            component_names: vec!["Transform".to_string(), "Velocity".to_string()],
        });
        entities.push(EcsEntityRow {
            entity_id: 2_i64,
            component_names: vec!["Transform".to_string()],
        });
        entities.push(EcsEntityRow {
            entity_id: 42_i64,
            component_names: vec![
                "Transform".to_string(),
                "Velocity".to_string(),
                "Health".to_string(),
            ],
        });
        let mut components: Vec<EcsComponentSection> = Vec::new();
        components.push(EcsComponentSection {
            name: "Transform".to_string().to_string(),
            fields: vec![
                EcsInspectorField {
                    name: "position.x".to_string().to_string(),
                    display_value: "1.0".to_string().to_string(),
                },
                EcsInspectorField {
                    name: "position.y".to_string().to_string(),
                    display_value: "0.0".to_string().to_string(),
                },
                EcsInspectorField {
                    name: "position.z".to_string().to_string(),
                    display_value: "0.0".to_string().to_string(),
                },
                EcsInspectorField {
                    name: "rotation.x".to_string().to_string(),
                    display_value: "0.0".to_string().to_string(),
                },
                EcsInspectorField {
                    name: "rotation.y".to_string().to_string(),
                    display_value: "0.0".to_string().to_string(),
                },
                EcsInspectorField {
                    name: "rotation.z".to_string().to_string(),
                    display_value: "0.0".to_string().to_string(),
                },
                EcsInspectorField {
                    name: "scale.x".to_string().to_string(),
                    display_value: "1.0".to_string().to_string(),
                },
                EcsInspectorField {
                    name: "scale.y".to_string().to_string(),
                    display_value: "1.0".to_string().to_string(),
                },
                EcsInspectorField {
                    name: "scale.z".to_string().to_string(),
                    display_value: "1.0".to_string().to_string(),
                },
            ],
        });
        components.push(EcsComponentSection {
            name: "Velocity".to_string().to_string(),
            fields: vec![
                EcsInspectorField {
                    name: "value.x".to_string().to_string(),
                    display_value: "0.0".to_string().to_string(),
                },
                EcsInspectorField {
                    name: "value.y".to_string().to_string(),
                    display_value: "1.0".to_string().to_string(),
                },
                EcsInspectorField {
                    name: "value.z".to_string().to_string(),
                    display_value: "0.0".to_string().to_string(),
                },
            ],
        });
        EcsInspectorSnapshot {
            entities,
            selected_entity_id: Some(1_i64),
            components,
        }
    }
}
