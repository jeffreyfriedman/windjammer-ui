//! `src/components/generated/` is compiler output, never hand-maintained: regenerating it from
//! `src/components_wj/` must reproduce the checked-in tree byte for byte.
//!
//! The compiler is `$WJ` when set, otherwise `wj` from PATH. A missing compiler skips the test unless
//! `WJ_UI_REQUIRE_WJ=1` (CI sets it). To refresh the tree and `src/metadata.json` after editing a component:
//!
//! ```text
//! WJ=/path/to/wj WJ_UI_REGEN=1 cargo test --test generated_fresh_test
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

/// The files that make up a generated tree: `mod.rs` and every module it declares, by file name. (The compiler
/// occasionally also writes a `lib.rs` copy of `mod.rs`; that is not part of the tree.)
fn rs_files(dir: &Path) -> BTreeMap<String, String> {
    let declared = declared_modules(dir);
    let mut map = BTreeMap::new();
    for entry in
        std::fs::read_dir(dir).unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
    {
        let path: PathBuf = entry.expect("dir entry").path();
        if path.extension().map(|x| x == "rs").unwrap_or(false) {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            if name != "mod.rs" && !declared.contains(&name) {
                continue;
            }
            map.insert(
                name,
                std::fs::read_to_string(&path).expect("read generated file"),
            );
        }
    }
    map
}

/// Names of the files that differ between two trees, with a short reason each.
fn tree_diff(
    fresh: &BTreeMap<String, String>,
    checked_in: &BTreeMap<String, String>,
) -> Vec<String> {
    let mut out = Vec::new();
    for (name, text) in fresh {
        match checked_in.get(name) {
            None => out.push(format!(
                "{name}: produced by the compiler but not checked in"
            )),
            Some(old) if old != text => {
                let line = text
                    .lines()
                    .zip(old.lines())
                    .position(|(a, b)| a != b)
                    .map(|i| i + 1);
                out.push(format!(
                    "{name}: differs (first differing line {})",
                    line.unwrap_or(0)
                ));
            }
            _ => {}
        }
    }
    for name in checked_in.keys() {
        if !fresh.contains_key(name) {
            out.push(format!(
                "{name}: checked in but no longer produced from components_wj"
            ));
        }
    }
    out
}

/// Fresh compiler output: the module files and the crate metadata consumers read (see `METADATA`).
fn regenerate(wj: &str) -> (BTreeMap<String, String>, serde_json::Value) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let out_dir =
        std::env::temp_dir().join(format!("wj-ui-generated-fresh-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out_dir);
    let out = Command::new(wj)
        .arg("build")
        .arg(root.join("src/components_wj"))
        .args([
            "--target",
            "rust",
            "--library",
            "--module-file",
            "--no-cargo",
            "--no-generate-cargo-toml",
            "-o",
        ])
        .arg(&out_dir)
        .output()
        .expect("run wj build");
    assert!(
        out.status.success(),
        "wj build failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let files = rs_files(&out_dir);
    let metadata = serde_json::from_str(
        &std::fs::read_to_string(out_dir.join("metadata.json"))
            .expect("compiler wrote metadata.json"),
    )
    .expect("metadata.json is JSON");
    let _ = std::fs::remove_dir_all(&out_dir);
    (files, metadata)
}

/// Crate metadata shipped for consumers. A Windjammer project that depends on this crate by path reads signatures
/// (for example that `Skeleton::width` takes `&str`) from `src/metadata.json`; without it the consumer assumes owned
/// `String` formals and generates calls that do not compile. Compared as JSON values (the compiler's key order varies).
const METADATA: &str = "src/metadata.json";

/// `WJ_UI_REGEN=1` rewrites the checked-in tree and metadata from the compiler instead of comparing.
fn write_regenerated(root: &Path, files: &BTreeMap<String, String>, metadata: &serde_json::Value) {
    let dir = root.join("src/components/generated");
    for entry in std::fs::read_dir(&dir).expect("generated dir") {
        let p = entry.expect("entry").path();
        if p.extension().map(|x| x == "rs").unwrap_or(false) {
            std::fs::remove_file(p).expect("remove old module");
        }
    }
    for (name, text) in files {
        std::fs::write(dir.join(name), text).expect("write module");
    }
    let text = serde_json::to_string_pretty(metadata).expect("serialize") + "\n";
    std::fs::write(root.join(METADATA), text).expect("write metadata");
}

#[test]
fn checked_in_generated_tree_matches_components_wj() {
    let Some(wj) = wj_binary() else { return };
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let (fresh, fresh_metadata) = regenerate(&wj);
    if std::env::var("WJ_UI_REGEN").is_ok() {
        write_regenerated(root, &fresh, &fresh_metadata);
    }
    let checked_in = rs_files(&root.join("src/components/generated"));
    assert!(
        fresh.len() >= 90,
        "expected the compiler to produce the component tree, saw {} files",
        fresh.len()
    );
    let diff = tree_diff(&fresh, &checked_in);
    assert!(
        diff.is_empty(),
        "src/components/generated is stale relative to src/components_wj (regenerate it, see this file's header):\n{}",
        diff.join("\n")
    );
    let shipped: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(root.join(METADATA)).expect("src/metadata.json is checked in"),
    )
    .expect("src/metadata.json is JSON");
    assert!(shipped == fresh_metadata, "src/metadata.json is stale relative to src/components_wj (regenerate with WJ_UI_REGEN=1, see this file's header)");
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

/// File names of the modules a directory's `mod.rs` declares with `pub mod x;`.
fn declared_modules(dir: &Path) -> Vec<String> {
    let text = std::fs::read_to_string(dir.join("mod.rs")).unwrap_or_default();
    text.lines()
        .filter_map(|l| {
            l.trim()
                .strip_prefix("pub mod ")?
                .strip_suffix(';')
                .map(|m| format!("{m}.rs"))
        })
        .collect()
}

#[test]
fn declared_modules_reads_pub_mod_lines_only() {
    let dir = std::env::temp_dir().join(format!("wj-ui-declared-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("mod.rs"),
        "// header\npub mod a;\n#[cfg(test)]\npub mod b_test;\npub use a::*;\n",
    )
    .unwrap();
    assert_eq!(declared_modules(&dir), ["a.rs", "b_test.rs"]);
    let _ = std::fs::remove_dir_all(&dir);
}
