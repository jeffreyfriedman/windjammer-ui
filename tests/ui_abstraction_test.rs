/// TDD Test: Verify windjammer-ui provides generic abstractions
///
/// The UI framework should provide its OWN types (Color, Position, etc.)
/// that are NOT tied to any specific implementation (egui, gtk, etc.).
///
/// Implementation details like egui should be completely hidden.

#[test]
fn test_color_abstraction() {
    // Should be able to use windjammer-ui colors without knowing about egui
    use windjammer_ui::Color;

    let red = Color::rgb(255, 0, 0);
    let blue = Color::rgba(0, 0, 255, 255);

    assert_eq!(red.r(), 255);
    assert_eq!(red.g(), 0);
    assert_eq!(red.b(), 0);
    assert_eq!(blue.a(), 255);
}

#[test]
fn test_position_abstraction() {
    // Should be able to use positions without knowing about egui::Pos2
    use windjammer_ui::Position;

    let pos = Position::new(100.0, 200.0);
    assert_eq!(pos.x(), 100.0);
    assert_eq!(pos.y(), 200.0);
}

#[test]
fn test_size_abstraction() {
    // Should be able to use sizes without knowing about egui::Vec2
    use windjammer_ui::Size;

    let size = Size::new(640.0, 480.0);
    assert_eq!(size.width(), 640.0);
    assert_eq!(size.height(), 480.0);
}

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
#[test]
fn test_egui_not_exposed() {
    // This should NOT compile - egui should be private
    // Uncommenting this should cause a compile error:
    // use windjammer_ui::egui; // ❌ Should not be accessible
}

#[test]
fn test_serde_serialization_works() {
    // serde and serde_json are legitimate re-exports for serialization
    use windjammer_ui::serde::{Deserialize, Serialize};
    use windjammer_ui::Color;

    #[derive(Serialize, Deserialize)]
    struct TestStruct {
        color: Color,
        name: String,
    }

    let test = TestStruct {
        color: Color::rgb(255, 0, 0),
        name: "red".to_string(),
    };

    // Verify serialization works with our Color abstraction
    let json = windjammer_ui::serde_json::to_string(&test).unwrap();
    assert!(json.contains("red"));
}
