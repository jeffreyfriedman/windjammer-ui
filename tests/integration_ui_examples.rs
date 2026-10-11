//! Integration tests for UI framework builds.
//!
//! These shell out to nested `cargo` builds. They are `#[ignore]`d by default (slow, and the
//! normal `cargo test` run already compiles everything they check) and run with
//! `cargo test --test integration_ui_examples -- --ignored`.
//!
//! Nested builds never share the outer build's target dir (that deadlocks on the
//! "Blocking waiting for file lock" build-directory lock): each uses
//! `$WJ_UI_NESTED_TARGET_DIR` or `<temp>/windjammer-ui-nested-target`, they are serialized
//! with a process-wide mutex, and each has a timeout (`WJ_UI_NESTED_TIMEOUT_SECS`, default 900).

use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant};

static NESTED: Mutex<()> = Mutex::new(());

fn nested_target_dir() -> std::path::PathBuf {
    std::env::var_os("WJ_UI_NESTED_TARGET_DIR")
        .map(Into::into)
        .unwrap_or_else(|| std::env::temp_dir().join("windjammer-ui-nested-target"))
}

/// Run `cargo <args>` in a separate target dir with a timeout; panics with captured stderr on
/// failure or timeout.
fn cargo_ok(what: &str, args: &[&str]) {
    let _guard = NESTED.lock().unwrap_or_else(|e| e.into_inner());
    let timeout = Duration::from_secs(
        std::env::var("WJ_UI_NESTED_TIMEOUT_SECS").ok().and_then(|s| s.parse().ok()).unwrap_or(900),
    );
    let log = std::env::temp_dir().join(format!("wj-ui-nested-{}-{}.log", std::process::id(), what.replace(' ', "_")));
    let file = std::fs::File::create(&log).expect("log file");
    let mut child = Command::new("cargo")
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("CARGO_TARGET_DIR", nested_target_dir())
        .stdin(Stdio::null())
        .stdout(Stdio::from(file.try_clone().unwrap()))
        .stderr(Stdio::from(file))
        .spawn()
        .expect("spawn cargo");
    let start = Instant::now();
    let status = loop {
        if let Some(s) = child.try_wait().expect("wait") {
            break Some(s);
        }
        if start.elapsed() > timeout {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(Duration::from_millis(200));
    };
    let out = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_file(&log);
    match status {
        Some(s) if s.success() => {}
        Some(s) => panic!("{what} failed ({s}):\n{}", tail(&out)),
        None => panic!("{what} timed out after {timeout:?}:\n{}", tail(&out)),
    }
}

fn tail(s: &str) -> String {
    let lines: Vec<&str> = s.lines().collect();
    lines[lines.len().saturating_sub(60)..].join("\n")
}

#[test]
#[ignore = "nested cargo build; run with --ignored"]
fn ui_framework_compiles() {
    cargo_ok("build default", &["build", "-p", "windjammer-ui"]);
}

#[test]
#[ignore = "nested cargo build; run with --ignored"]
fn ui_web_feature_compiles_for_wasm32() {
    cargo_ok(
        "check wasm32",
        &["check", "-p", "windjammer-ui", "--lib", "--target", "wasm32-unknown-unknown"],
    );
}

#[test]
#[ignore = "nested cargo build (heavy: eframe/wgpu); run with --ignored"]
#[cfg(not(target_os = "linux"))]
fn ui_desktop_feature_compiles() {
    cargo_ok("check desktop", &["check", "-p", "windjammer-ui", "--features", "desktop"]);
}

#[test]
#[ignore = "nested cargo build; run with --ignored"]
fn ui_benchmarks_compile() {
    cargo_ok("build benches", &["build", "--benches", "-p", "windjammer-ui"]);
}
