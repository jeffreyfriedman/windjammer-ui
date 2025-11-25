# Windjammer UI Tests

## Test Status for v0.1.1

### ✅ Enabled Tests
- `crud_operations_test.rs` - Tests CRUD patterns (Card, Text, Badge, etc.)
- `dashboard_patterns_test.rs` - Tests dashboard UI patterns
- `examples_integration_test.rs` - Tests .wj example compilation
- `form_validation_test.rs` - Tests form validation patterns
- `integration_ui_examples.rs` - Tests UI framework compilation  
- `widget_integration_test.rs` - Tests widget integration

### 🚧 Temporarily Disabled (egui_kittest issues)
- `button_click_test.rs` - Needs egui_kittest dev dependency
- `simple_button_test.rs` - Needs egui_kittest dev dependency

### ⏭️ Moved to game-host
- `integration_tests.rs` - Uses undo_redo, build_system, file_watcher (game editor modules)
- `editor_comprehensive_test.rs` - Uses EditorApp/scene_gizmos/asset_browser
- `editor_kittest.rs` - Game editor UI tests
- `editor_visual_test.rs` - Game editor visual tests

## Running Tests

```bash
# Run all enabled tests
cargo test

# Run clippy on all targets
cargo clippy --all-targets --all-features
```
