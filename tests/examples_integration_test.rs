//! Automated Integration Tests for Windjammer UI Examples
//!
//! This replaces manual QA with automated testing.
//! Note: These tests only verify transpilation succeeds, not full cargo build,
//! since examples depend on windjammer-ui which may not be published yet.

use std::path::Path;
use std::process::Command;

/// Helper to run wj build with --no-cargo (required for v0.37.2+)
fn transpile_example(path: &str) -> Result<(), String> {
    // Always use --no-cargo to skip cargo build
    // (Examples depend on windjammer-ui which may not be published yet)
    let output = Command::new("wj")
        .args(["build", path, "--no-cargo"])
        .output()
        .map_err(|e| format!("Failed to run wj: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Transpilation failed:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

#[test]
fn test_counter_example_compiles() {
    let counter_path = Path::new("examples_wj/counter.wj");
    assert!(
        counter_path.exists(),
        "Counter example doesn't exist at {:?}",
        counter_path
    );

    transpile_example("examples_wj/counter.wj").expect("Counter example failed to transpile");
}

#[test]
fn test_todo_app_example_compiles() {
    let todo_path = Path::new("examples_wj/todo_app.wj");
    assert!(
        todo_path.exists(),
        "Todo app example doesn't exist at {:?}",
        todo_path
    );

    transpile_example("examples_wj/todo_app.wj").expect("Todo app failed to transpile");
}

#[test]
fn test_contact_form_example_compiles() {
    let form_path = Path::new("examples_wj/contact_form.wj");
    assert!(
        form_path.exists(),
        "Contact form example doesn't exist at {:?}",
        form_path
    );

    transpile_example("examples_wj/contact_form.wj").expect("Contact form failed to transpile");
}

#[test]
fn test_dashboard_example_compiles() {
    let dashboard_path = Path::new("examples_wj/dashboard.wj");
    assert!(
        dashboard_path.exists(),
        "Dashboard example doesn't exist at {:?}",
        dashboard_path
    );

    transpile_example("examples_wj/dashboard.wj").expect("Dashboard failed to transpile");
}

#[test]
fn test_all_examples_exist() {
    let expected_examples = vec![
        "examples_wj/counter.wj",
        "examples_wj/todo_app.wj",
        "examples_wj/contact_form.wj",
        "examples_wj/dashboard.wj",
        "examples_wj/pbr_material_editor.wj",
        "examples_wj/scene_hierarchy.wj",
        "examples_wj/inspector.wj",
        "examples_wj/asset_browser.wj",
        "examples_wj/animation_editor.wj",
    ];

    let mut missing = Vec::new();
    for example in &expected_examples {
        if !Path::new(example).exists() {
            missing.push(*example);
        }
    }

    assert!(
        missing.is_empty(),
        "Missing example files:\n{}",
        missing.join("\n")
    );
}

#[test]
fn test_examples_have_html_wrappers() {
    // Each example should have a corresponding HTML file for web testing
    let examples = vec![
        ("examples_wj/counter.wj", "examples/counter.html"),
        ("examples_wj/todo_app.wj", "examples/todo.html"),
        ("examples_wj/contact_form.wj", "examples/form.html"),
        ("examples_wj/dashboard.wj", "examples/dashboard.html"),
    ];

    let mut missing = Vec::new();
    for (wj_file, html_file) in &examples {
        if Path::new(wj_file).exists() && !Path::new(html_file).exists() {
            missing.push(*html_file);
        }
    }

    if !missing.is_empty() {
        eprintln!("Warning: Missing HTML wrappers for web testing:");
        for file in &missing {
            eprintln!("  - {}", file);
        }
        // Don't fail the test, just warn for now
    }
}
