#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;
use std::fmt::Write;

use super::ecs_inspector_model::{
    EcsComponentSection, EcsEntityRow, EcsInspectorField, EcsInspectorSnapshot,
};
use super::traits::Renderable;
#[derive(Debug, Clone)]
#[repr(C)]
pub struct EcsInspectorPanel {
    pub snapshot: EcsInspectorSnapshot,
    pub refresh_handler: String,
    pub select_entity_handler: String,
}

impl EcsInspectorPanel {
    #[inline]
    pub fn from_mock() -> EcsInspectorPanel {
        EcsInspectorPanel {
            snapshot: EcsInspectorSnapshot::mock_runtime_demo(),
            refresh_handler: "ecs_inspector_refresh()".to_string(),
            select_entity_handler: "ecs_inspector_select_entity".to_string(),
        }
    }
    #[inline]
    pub fn empty() -> EcsInspectorPanel {
        EcsInspectorPanel {
            snapshot: EcsInspectorSnapshot::empty(),
            refresh_handler: "ecs_inspector_refresh()".to_string(),
            select_entity_handler: "ecs_inspector_select_entity".to_string(),
        }
    }
    #[inline]
    pub fn snapshot(mut self, snap: EcsInspectorSnapshot) -> EcsInspectorPanel {
        self.snapshot = snap;
        self
    }
}

impl Renderable for EcsInspectorPanel {
    #[inline]
    fn render(self) -> String {
        let styles = ecs_inspector_panel_styles();
        let entities: Vec<EcsEntityRow> = self.snapshot.entities.clone();
        let selected_id = self.snapshot.selected_entity_id;
        let components: Vec<EcsComponentSection> = self.snapshot.components.clone();
        let selected_label = selected_entity_label_from_id(selected_id);
        let entity_count = entities.len();
        let entity_html = render_entity_list(entities, selected_id, &self.select_entity_handler);
        let component_html = render_component_pane(components, &selected_label);
        format!("<style>{}</style>\n<div class='ecs-inspector-panel'>\n  <header class='ecs-inspector-header'>\n    <h3>ECS Inspector</h3>\n    <button type='button' onclick='{}'>Refresh</button>\n    <span class='ecs-entity-count'>{} entities</span>\n  </header>\n  <div class='ecs-inspector-body'>\n    <aside class='ecs-entity-list'>\n      <h4>Entities</h4>\n      {}\n    </aside>\n    <section class='ecs-component-pane'>\n      {}\n    </section>\n  </div>\n</div>", styles, self.refresh_handler, entity_count, entity_html, component_html)
    }
}

#[inline]
pub fn entity_row_class(entity_id: i64, selected_id: Option<i64>) -> String {
    if let Some(sel) = selected_id {
        if sel == entity_id {
            return "ecs-entity-row selected".to_string();
        }
    }
    "ecs-entity-row".to_string()
}

#[inline]
pub fn render_entity_row(
    row: &EcsEntityRow,
    selected_id: Option<i64>,
    select_handler: &str,
) -> String {
    let row_class = entity_row_class(row.entity_id, selected_id);
    let mut badges = String::new();
    let mut i = 0;
    while i < row.component_names.len() {
        let name = &row.component_names[i];
        badges = badges + &format!("<span class='ecs-comp-badge'>{}</span>", name);
        i += 1;
    }
    format!("<button type='button' class='{}' onclick=\"{}({})\">\n  <span class='ecs-entity-id'>#{}</span>\n  <span class='ecs-entity-badges'>{}</span>\n</button>", row_class, select_handler, row.entity_id, row.entity_id, badges)
}

#[inline]
pub fn render_entity_list(
    entities: Vec<EcsEntityRow>,
    selected_id: Option<i64>,
    select_handler: &str,
) -> String {
    if entities.is_empty() {
        return "<div class='ecs-inspector-empty'>No entities in world</div>".to_string();
    }
    let mut html = String::new();
    for row in entities {
        html = html + &render_entity_row(&row, selected_id, &select_handler);
    }
    html
}

#[inline]
pub fn render_field_row(field: &EcsInspectorField) -> String {
    format!("<div class='ecs-field-row'>\n  <label class='ecs-field-name'>{}</label>\n  <input class='ecs-field-value' type='text' value='{}' readonly />\n</div>", field.name.clone(), field.display_value.clone())
}

#[inline]
pub fn render_component_section(section: &EcsComponentSection) -> String {
    let mut fields_html = String::new();
    for field in &section.fields {
        fields_html = fields_html + &render_field_row(&field);
    }
    format!("<details class='ecs-component-section' open>\n  <summary class='ecs-component-title'>{}</summary>\n  <div class='ecs-component-fields'>{}</div>\n</details>", section.name.clone(), fields_html)
}

#[inline]
pub fn render_component_pane(components: Vec<EcsComponentSection>, selected_label: &str) -> String {
    if components.is_empty() {
        return {
            let mut __s = String::with_capacity(64);
            write!(&mut __s, "<div class='ecs-inspector-detail-empty'>\n  <p class='ecs-detail-title'>{}</p>\n  <p class='ecs-detail-hint'>Select an entity to view read-only component fields.</p>\n</div>", selected_label).unwrap();
            __s
        };
    }
    let mut html = format!("<p class='ecs-detail-title'>{}</p>", selected_label);
    for section in components {
        html = html + &render_component_section(&section);
    }
    html
}

#[inline]
pub fn selected_entity_label_from_id(selected_id: Option<i64>) -> String {
    if let Some(id) = selected_id {
        format!("Entity #{}", id)
    } else {
        "No entity selected".to_string()
    }
}

#[inline]
pub fn ecs_inspector_panel_styles() -> String {
    String::from("\n.ecs-inspector-panel { background:#0f0f1a; color:#e5e7eb; padding:12px; font-family:monospace; font-size:12px; min-height:280px; }\n.ecs-inspector-header { display:flex; align-items:center; gap:12px; margin-bottom:12px; border-bottom:1px solid #333; padding-bottom:8px; }\n.ecs-inspector-header h3 { margin:0; flex:1; color:#a78bfa; }\n.ecs-inspector-header button { background:#0f3460; color:#e5e7eb; border:none; padding:6px 10px; border-radius:4px; cursor:pointer; }\n.ecs-entity-count { color:#888; font-size:11px; }\n.ecs-inspector-body { display:flex; gap:12px; min-height:220px; }\n.ecs-entity-list { flex:0 0 38%; background:#16213e; border-radius:6px; padding:8px; overflow-y:auto; max-height:320px; }\n.ecs-entity-list h4 { margin:0 0 8px 0; color:#888; font-size:11px; text-transform:uppercase; }\n.ecs-entity-row { display:flex; flex-direction:column; align-items:flex-start; width:100%; text-align:left; background:#1a1a2e; border:1px solid transparent; border-radius:4px; padding:8px; margin-bottom:6px; cursor:pointer; color:#e0e0e0; }\n.ecs-entity-row:hover { border-color:#60a5fa; }\n.ecs-entity-row.selected { border-color:#a78bfa; background:#1e1b4b; }\n.ecs-entity-id { font-weight:600; color:#a78bfa; margin-bottom:4px; }\n.ecs-entity-badges { display:flex; flex-wrap:wrap; gap:4px; }\n.ecs-comp-badge { background:#0f3460; color:#93c5fd; font-size:10px; padding:2px 6px; border-radius:3px; }\n.ecs-component-pane { flex:1; background:#16213e; border-radius:6px; padding:10px; overflow-y:auto; max-height:320px; }\n.ecs-detail-title { margin:0 0 10px 0; color:#a78bfa; font-weight:600; }\n.ecs-detail-hint { color:#666; margin:0; }\n.ecs-component-section { margin-bottom:10px; border:1px solid #333; border-radius:4px; padding:6px 8px; background:#1a1a2e; }\n.ecs-component-title { cursor:pointer; color:#60a5fa; font-weight:600; }\n.ecs-field-row { display:flex; align-items:center; gap:8px; margin:4px 0; }\n.ecs-field-name { flex:0 0 42%; color:#888; font-size:11px; }\n.ecs-field-value { flex:1; background:#09090f; border:1px solid #333; border-radius:3px; color:#e5e7eb; padding:4px 6px; font-family:inherit; font-size:11px; }\n.ecs-inspector-empty, .ecs-inspector-detail-empty { color:#666; padding:16px; text-align:center; }\n")
}
