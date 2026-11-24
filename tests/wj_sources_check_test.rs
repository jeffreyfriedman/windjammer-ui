//! Every `.wj` source in the repository must be accepted by the current compiler.
//!
//! - `src/components_wj/` is checked as one project with `wj check` (0 errors).
//! - every `examples/*.wj` and `examples_wj/*.wj` transpiles with `wj build --no-cargo`.
//!
//! The compiler is `$WJ` when set, otherwise `wj` from PATH. A missing compiler skips the test unless
//! `WJ_UI_REQUIRE_WJ=1`, which turns it into a failure (CI sets it).

use std::path::{Path, PathBuf};
use std::process::Command;

fn wj_binary() -> Option<String> {
    let wj = std::env::var("WJ").unwrap_or_else(|_| "wj".to_string());
    if Command::new(&wj).arg("--version").output().is_ok() {
        return Some(wj);
    }
    if std::env::var("WJ_UI_REQUIRE_WJ").is_ok() {
        panic!("WJ_UI_REQUIRE_WJ is set but `{wj}` is not runnable");
    }
    eprintln!("skipping: no `wj` compiler (set WJ=/path/to/wj)");
    None
}

fn wj_files(dir: &str) -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(dir);
    let mut files: Vec<PathBuf> = std::fs::read_dir(&root)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", root.display()))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().map(|x| x == "wj").unwrap_or(false))
        .collect();
    files.sort();
    files
}

#[test]
fn components_wj_passes_wj_check() {
    let Some(wj) = wj_binary() else { return };
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/components_wj");
    let out = Command::new(&wj)
        .arg("check")
        .arg(&dir)
        .output()
        .expect("run wj check");
    let text =
        String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr);
    let errors: Vec<&str> = text
        .lines()
        .filter(|l| l.starts_with("error"))
        .take(20)
        .collect();
    assert!(
        out.status.success() && errors.is_empty(),
        "wj check src/components_wj failed:\n{}",
        errors.join("\n")
    );
}

#[test]
fn every_example_transpiles() {
    let Some(wj) = wj_binary() else { return };
    let mut failures = Vec::new();
    let mut checked = 0;
    for dir in ["examples", "examples_wj"] {
        for file in wj_files(dir) {
            checked += 1;
            let out_dir = std::env::temp_dir().join(format!(
                "wj-ui-src-check-{}-{}-{}",
                std::process::id(),
                dir,
                file.file_stem().and_then(|s| s.to_str()).unwrap_or("x")
            ));
            let out = Command::new(&wj)
                .args(["build"])
                .arg(&file)
                .args(["--no-cargo", "-o"])
                .arg(&out_dir)
                .output()
                .expect("run wj build");
            let _ = std::fs::remove_dir_all(&out_dir);
            if !out.status.success() {
                let text = String::from_utf8_lossy(&out.stderr);
                let first = text
                    .lines()
                    .find(|l| l.contains("rror"))
                    .unwrap_or("(no message)");
                failures.push(format!("{}: {}", file.display(), first));
            }
        }
    }
    assert!(
        checked >= 7,
        "expected the examples to be found, saw {checked}"
    );
    assert!(
        failures.is_empty(),
        "examples that no longer transpile:\n{}",
        failures.join("\n")
    );
}
