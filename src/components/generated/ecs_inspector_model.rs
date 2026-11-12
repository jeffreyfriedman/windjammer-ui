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
        EcsInspectorSnapshot { entities: Vec::new(), selected_entity_id: None, components: Vec::new() }
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
            String::from("No entity selected")
        }
}

pub fn mock_runtime_demo() -> EcsInspectorSnapshot {
        let mut entities: Vec<EcsEntityRow> = Vec::new();
        entities.push(EcsEntityRow { entity_id: 1_i64, component_names: vec!["Transform".to_string(), "Velocity".to_string()] });
        entities.push(EcsEntityRow { entity_id: 2_i64, component_names: vec!["Transform".to_string()] });
        entities.push(EcsEntityRow { entity_id: 42_i64, component_names: vec!["Transform".to_string(), "Velocity".to_string(), "Health".to_string()] });
        let mut components: Vec<EcsComponentSection> = Vec::new();
        components.push(EcsComponentSection { name: String::from("Transform"), fields: vec![EcsInspectorField { name: String::from("position.x"), display_value: String::from("1.0") }, EcsInspectorField { name: String::from("position.y"), display_value: String::from("0.0") }, EcsInspectorField { name: String::from("position.z"), display_value: String::from("0.0") }, EcsInspectorField { name: String::from("rotation.x"), display_value: String::from("0.0") }, EcsInspectorField { name: String::from("rotation.y"), display_value: String::from("0.0") }, EcsInspectorField { name: String::from("rotation.z"), display_value: String::from("0.0") }, EcsInspectorField { name: String::from("scale.x"), display_value: String::from("1.0") }, EcsInspectorField { name: String::from("scale.y"), display_value: String::from("1.0") }, EcsInspectorField { name: String::from("scale.z"), display_value: String::from("1.0") }] });
        components.push(EcsComponentSection { name: String::from("Velocity"), fields: vec![EcsInspectorField { name: String::from("value.x"), display_value: String::from("0.0") }, EcsInspectorField { name: String::from("value.y"), display_value: String::from("1.0") }, EcsInspectorField { name: String::from("value.z"), display_value: String::from("0.0") }] });
        EcsInspectorSnapshot { entities, selected_entity_id: Some(1_i64), components }
}
}

