//! Public-repo hygiene guard: this repository is public, so names of private
//! projects and personal paths must never appear in tracked content or file names.
//!
//! The forbidden terms are stored as fragments joined at runtime so this file
//! does not itself contain them. A mutation self-test proves the scanner works.

use std::fs;
use std::path::{Path, PathBuf};

/// Each entry is a list of fragments; the forbidden term is their concatenation.
const FRAGMENTS: &[&[&str]] = &[
    &["windjammer", "db"],
    &["windjammer-", "db"],
    &["wd", "b-"],
    &["wd", "b_"],
    &["windjammer-", "game"],
    &["windjammer_", "game"],
    &["breach", "-protocol"],
    &["breach", "_protocol"],
    &["breach", " protocol"],
    &["ledger", "kit"],
    &["financial", "-management"],
    &["finance", "-screens"],
    &["trash", " panda"],
    &["jeffrey", "friedman"],
    &["wj", "_pin"],
];

/// The one tolerated occurrence: the public repository URL of this project.
fn allowed_url() -> String {
    ["github.com/jeffrey", "friedman/windjammer-ui"].concat()
}

fn terms() -> Vec<String> {
    FRAGMENTS.iter().map(|f| f.concat()).collect()
}

fn find_hits(text: &str) -> Vec<String> {
    let lower = text.to_lowercase().replace(&allowed_url(), "");
    terms().into_iter().filter(|t| lower.contains(t.as_str())).collect()
}

fn skip_dir(name: &str) -> bool {
    matches!(name, ".git" | "target" | "node_modules")
}

fn scan_tree(root: &Path) -> Vec<String> {
    let mut hits = Vec::new();
    let mut stack: Vec<PathBuf> = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else { continue };
        for e in entries.flatten() {
            let path = e.path();
            let name = e.file_name().to_string_lossy().to_string();
            let rel = path.strip_prefix(root).unwrap_or(&path).display().to_string();
            if path.is_dir() {
                if !skip_dir(&name) {
                    stack.push(path);
                }
                continue;
            }
            if name == "Cargo.lock" || name == ".git" {
                continue;
            }
            for t in find_hits(&rel) {
                hits.push(format!("{rel}: file name matches '{t}'"));
            }
            if let Ok(text) = fs::read_to_string(&path) {
                for t in find_hits(&text) {
                    hits.push(format!("{rel}: content matches '{t}'"));
                }
            }
        }
    }
    hits.sort();
    hits
}

#[test]
fn repository_has_no_private_project_references() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let hits = scan_tree(root);
    assert!(hits.is_empty(), "private references found:\n{}", hits.join("\n"));
}

#[test]
fn scanner_detects_planted_forbidden_strings() {
    let dir = std::env::temp_dir().join(format!("wj_ui_hygiene_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("sub")).unwrap();
    fs::write(dir.join("clean.txt"), "generic data table component").unwrap();
    assert!(scan_tree(&dir).is_empty(), "clean tree must pass");

    for t in terms() {
        fs::write(dir.join("sub/planted.txt"), format!("see {} for details", t.to_uppercase())).unwrap();
        let hits = scan_tree(&dir);
        assert!(hits.iter().any(|h| h.contains("content matches")), "term not detected: {t}");
        fs::remove_file(dir.join("sub/planted.txt")).unwrap();
    }
    // file-name detection
    fs::write(dir.join(format!("{}.md", terms()[9])), "x").unwrap();
    assert!(scan_tree(&dir).iter().any(|h| h.contains("file name")));
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn allowed_repo_url_is_not_flagged() {
    assert!(find_hits(&format!("repository = \"https://{}\"", allowed_url())).is_empty());
}
