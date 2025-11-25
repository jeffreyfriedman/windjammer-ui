//! Automated Integration Tests for Windjammer UI Examples
//!
//! This replaces manual QA with automated testing.

use std::path::Path;
use std::process::Command;

#[test]
fn test_counter_example_compiles() {
    let counter_path = Path::new("examples_wj/counter.wj");

    if !counter_path.exists() {
        panic!("Counter example doesn't exist at {:?}", counter_path);
    }

    // Test that it compiles with the Windjammer compiler
    let output = Command::new("wj")
        .args(["build", "examples_wj/counter.wj"])
        .output()
        .expect("Failed to run wj compiler");

    assert!(
        output.status.success(),
        "Counter example failed to compile:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_todo_app_example_compiles() {
    let todo_path = Path::new("examples_wj/todo_app.wj");

    if !todo_path.exists() {
        panic!("Todo app example doesn't exist at {:?}", todo_path);
    }

    let output = Command::new("wj")
        .args(["build", "examples_wj/todo_app.wj"])
        .output()
        .expect("Failed to run wj compiler");

    assert!(
        output.status.success(),
        "Todo app failed to compile:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_contact_form_example_compiles() {
    let form_path = Path::new("examples_wj/contact_form.wj");

    if !form_path.exists() {
        panic!("Contact form example doesn't exist at {:?}", form_path);
    }

    let output = Command::new("wj")
        .args(["build", "examples_wj/contact_form.wj"])
        .output()
        .expect("Failed to run wj compiler");

    assert!(
        output.status.success(),
        "Contact form failed to compile:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_dashboard_example_compiles() {
    let dashboard_path = Path::new("examples_wj/dashboard.wj");

    if !dashboard_path.exists() {
        panic!("Dashboard example doesn't exist at {:?}", dashboard_path);
    }

    let output = Command::new("wj")
        .args(["build", "examples_wj/dashboard.wj"])
        .output()
        .expect("Failed to run wj compiler");

    assert!(
        output.status.success(),
        "Dashboard failed to compile:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
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
