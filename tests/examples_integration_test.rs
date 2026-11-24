//! Automated Integration Tests for Windjammer UI Examples
//!
//! This replaces manual QA with automated testing.
//! These tests verify that each example type-checks and transpiles (`wj build --no-cargo`).

use std::path::Path;
use std::process::Command;

/// Transpile one example with `wj build --no-cargo`.
/// Returns Ok if transpilation succeeds, Err if it fails, or skips if wj is not available
fn transpile_example(path: &str) -> Result<(), String> {
    let wj = std::env::var("WJ").unwrap_or_else(|_| "wj".to_string());
    if Command::new(&wj).arg("--version").output().is_err() {
        if std::env::var("WJ_UI_REQUIRE_WJ").is_ok() {
            panic!("WJ_UI_REQUIRE_WJ is set but `{wj}` is not runnable");
        }
        eprintln!("Skipping test: no `wj` compiler (set WJ=/path/to/wj)");
        return Ok(());
    }

    // Transpile only (--no-cargo); generated Rust goes to a throwaway dir, never into the checkout.
    let out_dir = std::env::temp_dir().join(format!(
        "wj-ui-example-{}-{}",
        std::process::id(),
        Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("x")
    ));
    let output = Command::new(&wj)
        .args(["build", path, "--no-cargo", "-o"])
        .arg(&out_dir)
        .output()
        .map_err(|e| format!("Failed to run wj: {}", e))?;

    let _ = std::fs::remove_dir_all(&out_dir);
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
fn test_chat_app_example_compiles() {
    assert!(Path::new("examples_wj/chat_app.wj").exists());
    transpile_example("examples_wj/chat_app.wj").expect("Chat app failed to transpile");
}

#[test]
fn test_dashboard_with_nav_example_compiles() {
    assert!(Path::new("examples_wj/dashboard_with_nav.wj").exists());
    transpile_example("examples_wj/dashboard_with_nav.wj")
        .expect("Dashboard with navigation failed to transpile");
}

#[test]
fn test_all_examples_exist() {
    let expected_examples = vec![
        "examples_wj/counter.wj",
        "examples_wj/todo_app.wj",
        "examples_wj/contact_form.wj",
        "examples_wj/dashboard.wj",
        "examples_wj/chat_app.wj",
        "examples_wj/dashboard_with_nav.wj",
        "examples/counter.wj",
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
