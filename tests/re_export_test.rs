/// TDD Test: Verify windjammer-ui re-exports egui and serde_json
///
/// The editor should only depend on windjammer-ui, not directly on egui or serde_json.
/// This test ensures windjammer-ui properly re-exports these dependencies.

#[test]
fn test_egui_re_export() {
    // Should be able to import egui types from windjammer_ui
    use windjammer_ui::egui;

    // Verify we can use egui types
    let _pos = egui::Pos2::new(0.0, 0.0);
    let _color = egui::Color32::RED;
}

#[test]
fn test_serde_json_re_export() {
    // Should be able to import serde_json from windjammer_ui
    use windjammer_ui::serde_json;

    // Verify we can use serde_json functions
    let json = serde_json::json!({"test": "value"});
    assert_eq!(json["test"], "value");
}

#[test]
fn test_serde_re_export() {
    // Should be able to import serde from windjammer_ui
    use windjammer_ui::serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct TestStruct {
        field: String,
    }

    let test = TestStruct {
        field: "test".to_string(),
    };

    // Verify serialization works
    let _json = windjammer_ui::serde_json::to_string(&test).unwrap();
}
