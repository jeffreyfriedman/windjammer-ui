//! `src/components/generated/` is compiler output, never hand-maintained: regenerating it from
//! `src/components_wj/` must reproduce the checked-in tree byte for byte.
//!
//! The compiler is `$WJ` when set, otherwise `wj` from PATH. A missing compiler skips the test unless
//! `WJ_UI_REQUIRE_WJ=1` (CI sets it). To refresh the tree after editing a component:
//!
//! ```text
//! wj build src/components_wj -o src/components/generated --target rust --library --module-file \
//!     --no-cargo --no-generate-cargo-toml && rm -f src/components/generated/metadata.json \
//!     src/components/generated/.wj-compiler-stamp
//! ```

use std::collections::BTreeMap;
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

/// Every `.rs` file of a directory, by file name.
fn rs_files(dir: &Path) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for entry in std::fs::read_dir(dir).unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display())) {
        let path: PathBuf = entry.expect("dir entry").path();
        if path.extension().map(|x| x == "rs").unwrap_or(false) {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            map.insert(name, std::fs::read_to_string(&path).expect("read generated file"));
        }
    }
    map
}

/// Names of the files that differ between two trees, with a short reason each.
fn tree_diff(fresh: &BTreeMap<String, String>, checked_in: &BTreeMap<String, String>) -> Vec<String> {
    let mut out = Vec::new();
    for (name, text) in fresh {
        match checked_in.get(name) {
            None => out.push(format!("{name}: produced by the compiler but not checked in")),
            Some(old) if old != text => {
                let line = text.lines().zip(old.lines()).position(|(a, b)| a != b).map(|i| i + 1);
                out.push(format!("{name}: differs (first differing line {})", line.unwrap_or(0)));
            }
            _ => {}
        }
    }
    for name in checked_in.keys() {
        if !fresh.contains_key(name) {
            out.push(format!("{name}: checked in but no longer produced from components_wj"));
        }
    }
    out
}

fn regenerate(wj: &str) -> BTreeMap<String, String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let out_dir = std::env::temp_dir().join(format!("wj-ui-generated-fresh-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out_dir);
    let out = Command::new(wj)
        .arg("build")
        .arg(root.join("src/components_wj"))
        .args(["--target", "rust", "--library", "--module-file", "--no-cargo", "--no-generate-cargo-toml", "-o"])
        .arg(&out_dir)
        .output()
        .expect("run wj build");
    assert!(out.status.success(), "wj build failed:\n{}", String::from_utf8_lossy(&out.stderr));
    let files = rs_files(&out_dir);
    let _ = std::fs::remove_dir_all(&out_dir);
    files
}

#[test]
fn checked_in_generated_tree_matches_components_wj() {
    let Some(wj) = wj_binary() else { return };
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let fresh = regenerate(&wj);
    let checked_in = rs_files(&root.join("src/components/generated"));
    assert!(fresh.len() >= 90, "expected the compiler to produce the component tree, saw {} files", fresh.len());
    let diff = tree_diff(&fresh, &checked_in);
    assert!(
        diff.is_empty(),
        "src/components/generated is stale relative to src/components_wj (regenerate it, see this file's header):\n{}",
        diff.join("\n")
    );
}

/// Mutation self-test: the comparison really reports an edited, a missing and an extra file.
#[test]
fn tree_diff_reports_edits_missing_and_extra_files() {
    let mut fresh = BTreeMap::new();
    fresh.insert("a.rs".to_string(), "x\ny\n".to_string());
    fresh.insert("b.rs".to_string(), "same\n".to_string());
    fresh.insert("c.rs".to_string(), "new\n".to_string());
    let mut checked_in = BTreeMap::new();
    checked_in.insert("a.rs".to_string(), "x\nz\n".to_string());
    checked_in.insert("b.rs".to_string(), "same\n".to_string());
    checked_in.insert("d.rs".to_string(), "old\n".to_string());
    let diff = tree_diff(&fresh, &checked_in);
    assert_eq!(diff.len(), 3, "{diff:?}");
    assert!(diff[0].contains("a.rs") && diff[0].contains("line 2"));
    assert!(diff[1].contains("c.rs") && diff[2].contains("d.rs"));
    assert!(tree_diff(&fresh, &fresh).is_empty());
}
