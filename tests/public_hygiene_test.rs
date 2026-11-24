//! Public-repo hygiene guard: this repository is public, so names of private
//! projects and personal paths must never appear in tracked content or file names.
//!
//! The forbidden terms are stored as fragments joined at runtime so this file
//! does not itself contain them. A mutation self-test proves the scanner works.

use std::fs;
use std::path::{Path, PathBuf};

/// One forbidden term: its pieces must appear in order, optionally separated by one character of `seps`.
struct Term {
    seps: Vec<char>,
    pieces: Vec<String>,
}

const PATTERNS: &str = "docs/public-hygiene-patterns.txt";

fn parse_patterns(text: &str) -> Vec<Term> {
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| {
            let (seps, pieces) = l.split_once(':').expect("pattern line needs ':'");
            Term {
                seps: seps.chars().collect(),
                pieces: pieces.split('|').map(|p| p.to_lowercase()).collect(),
            }
        })
        .collect()
}

fn terms() -> Vec<Term> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    parse_patterns(&fs::read_to_string(root.join(PATTERNS)).expect("patterns file"))
}

fn match_from(term: &Term, hay: &str, idx: usize, pos: usize) -> bool {
    if !hay[pos..].starts_with(term.pieces[idx].as_str()) {
        return false;
    }
    let after = pos + term.pieces[idx].len();
    if idx + 1 == term.pieces.len() || match_from(term, hay, idx + 1, after) {
        return true;
    }
    match hay[after..].chars().next() {
        Some(c) if term.seps.contains(&c) => match_from(term, hay, idx + 1, after + c.len_utf8()),
        _ => false,
    }
}

/// Forbidden terms found in `text`, each rendered as its pieces joined by `+`.
fn find_hits(text: &str) -> Vec<String> {
    let hay = text.to_lowercase();
    terms()
        .into_iter()
        .filter(|t| {
            hay.match_indices(t.pieces[0].as_str())
                .any(|(i, _)| match_from(t, &hay, 0, i))
        })
        .map(|t| t.pieces.join("+"))
        .collect()
}

fn skip_dir(name: &str) -> bool {
    matches!(name, ".git" | "target" | "node_modules" | "build")
}

fn scan_tree(root: &Path) -> Vec<String> {
    let mut hits = Vec::new();
    let mut stack: Vec<PathBuf> = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for e in entries.flatten() {
            let path = e.path();
            let name = e.file_name().to_string_lossy().to_string();
            let rel = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .display()
                .to_string();
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
    assert!(
        hits.is_empty(),
        "private references found:\n{}",
        hits.join("\n")
    );
}

#[test]
fn scanner_detects_planted_forbidden_strings() {
    let dir = std::env::temp_dir().join(format!("wj_ui_hygiene_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("sub")).unwrap();
    fs::write(dir.join("clean.txt"), "generic data table component").unwrap();
    assert!(scan_tree(&dir).is_empty(), "clean tree must pass");

    for t in terms() {
        let adjacent = t.pieces.concat().to_uppercase();
        fs::write(
            dir.join("sub/planted.txt"),
            format!("see {adjacent} for details"),
        )
        .unwrap();
        let hits = scan_tree(&dir);
        assert!(
            hits.iter().any(|h| h.contains("content matches")),
            "term not detected: {}",
            t.pieces.join("+")
        );
        if let Some(sep) = t.seps.first() {
            let spaced = t.pieces.join(&sep.to_string());
            fs::write(
                dir.join("sub/planted.txt"),
                format!("see {spaced} for details"),
            )
            .unwrap();
            assert!(
                !scan_tree(&dir).is_empty(),
                "separated form not detected: {spaced}"
            );
        }
        fs::remove_file(dir.join("sub/planted.txt")).unwrap();
    }
    // file-name detection
    fs::write(dir.join(format!("{}.md", terms()[0].pieces.concat())), "x").unwrap();
    assert!(scan_tree(&dir).iter().any(|h| h.contains("file name")));
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn own_repository_url_is_not_flagged() {
    assert!(
        find_hits("repository = \"https://github.com/jeffreyfriedman/windjammer-ui\"").is_empty()
    );
}
