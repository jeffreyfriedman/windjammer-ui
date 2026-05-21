//! ECS Inspector panel smoke tests (generated .wj components).

use windjammer_ui::components::generated::ecs_inspector_model::EcsInspectorSnapshot;
use windjammer_ui::components::generated::ecs_inspector_panel::EcsInspectorPanel;
use windjammer_ui::components::generated::traits::Renderable;

#[test]
fn test_ecs_inspector_snapshot_mock_entities() {
    let snap = EcsInspectorSnapshot::mock_runtime_demo();
    assert_eq!(snap.entity_count(), 3);
    assert_eq!(snap.selected_entity_id, Some(1));
}

#[test]
fn test_ecs_inspector_panel_renders_entity_list() {
    let html = EcsInspectorPanel::from_mock().render();
    assert!(html.contains("ECS Inspector"));
    assert!(html.contains("3 entities"));
    assert!(html.contains("ecs_inspector_refresh"));
    assert!(html.contains("#1"));
    assert!(html.contains("#42"));
    assert!(html.contains("Transform"));
}

#[test]
fn test_ecs_inspector_panel_renders_readonly_fields() {
    let html = EcsInspectorPanel::from_mock().render();
    assert!(html.contains("position.x"));
    assert!(html.contains("value.y"));
    assert!(html.contains("readonly"));
    assert!(html.contains("ecs-field-value"));
}

#[test]
fn test_ecs_inspector_empty_state() {
    let html = EcsInspectorPanel::empty().render();
    assert!(html.contains("No entities in world"));
    let snap = EcsInspectorSnapshot::empty();
    assert_eq!(snap.entity_count(), 0);
    assert_eq!(snap.selected_entity_label(), "No entity selected");
}
