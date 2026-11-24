//! Every kit component is documented with an example and has a browser test. The component list is read from the
//! generated sources (`impl Interactive for X`), so a new component without a docs section or a DOM test fails here.

use std::path::Path;

fn read(rel: &str) -> String {
    std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(rel))
        .unwrap_or_else(|e| panic!("{rel}: {e}"))
}

fn interactive_types() -> Vec<String> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/components/generated");
    let mut out = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let text = std::fs::read_to_string(entry.unwrap().path()).unwrap_or_default();
        for line in text.lines() {
            if let Some(name) = line.strip_prefix("impl Interactive for ") {
                out.push(name.trim_end_matches(" {").to_string());
            }
        }
    }
    out.sort();
    out
}

#[test]
fn every_interactive_component_has_a_documented_example_and_a_dom_test() {
    let kit = interactive_types();
    assert!(
        kit.len() >= 14,
        "expected the kit components, found {kit:?}"
    );
    let docs = read("docs/KIT.md");
    let tests = format!(
        "{}\n{}",
        read("tests/wasm_kit_a11y_test.rs"),
        read("tests/wasm_kit_data_test.rs")
    );
    for name in &kit {
        let heading = docs
            .lines()
            .position(|l| l.starts_with("### ") && l.contains(name.as_str()));
        let Some(h) = heading else {
            panic!("docs/KIT.md has no `### {name}` section")
        };
        let section: String = docs
            .lines()
            .skip(h + 1)
            .take_while(|l| !l.starts_with("### ") && !l.starts_with("## "))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            section.contains("```rust"),
            "the `{name}` section needs a code example"
        );
        assert!(
            tests.contains(&format!("{name}::new(")),
            "no browser test constructs `{name}`"
        );
    }
    // Support types used by the examples are documented where they appear.
    for name in ["FormField", "GridColumn"] {
        assert!(docs.contains(name));
    }
}
