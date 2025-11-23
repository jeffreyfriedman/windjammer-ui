// Build script to auto-generate Rust code from Windjammer (.wj) source
// This runs automatically when developers run: cargo build
// Zero Rust knowledge needed - just edit .wj files!

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Tell Cargo to rerun this build script if .wj files change
    println!("cargo:rerun-if-changed=src_wj/components");

    // Get project root
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_root = PathBuf::from(&manifest_dir);

    // Paths
    let wj_cli = project_root.join("../windjammer/target/release/wj");
    let src_dir = project_root.join("src_wj/components");
    let out_dir = project_root.join("src/components/generated");

    // Check if wj CLI exists
    if !wj_cli.exists() {
        eprintln!("⚠️  Warning: wj CLI not found at {:?}", wj_cli);
        eprintln!("   Skipping .wj transpilation. To fix:");
        eprintln!("   cd ../windjammer && cargo build --release");
        return;
    }

    // Check if source directory exists
    if !src_dir.exists() {
        eprintln!("⚠️  Warning: No .wj source found at {:?}", src_dir);
        return;
    }

    println!(
        "cargo:warning=🔨 Transpiling Windjammer components from {:?}",
        src_dir
    );

    // Create output directory
    std::fs::create_dir_all(&out_dir).expect("Failed to create output directory");

    // Run wj build
    let status = Command::new(&wj_cli)
        .arg("build")
        .arg(&src_dir)
        .arg("-o")
        .arg(&out_dir)
        .arg("--target")
        .arg("rust")
        .status()
        .expect("Failed to execute wj build");

    if !status.success() {
        panic!("wj build failed! Check .wj source for errors.");
    }

    // Clean up: remove main() test functions from generated files
    for entry in std::fs::read_dir(&out_dir).expect("Failed to read output dir") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("rs")
            && path.file_name().and_then(|s| s.to_str()) != Some("mod.rs")
        {
            let content = std::fs::read_to_string(&path).expect("Failed to read generated file");

            // Remove fn main() and its body (simple regex-free approach)
            let lines: Vec<&str> = content.lines().collect();
            let mut output = Vec::new();
            let mut in_main = false;
            let mut brace_depth = 0;

            for line in lines {
                if line.trim().starts_with("fn main()") {
                    in_main = true;
                    brace_depth = 0;
                    continue;
                }

                if in_main {
                    // Count braces to find the end of main()
                    for ch in line.chars() {
                        if ch == '{' {
                            brace_depth += 1;
                        }
                        if ch == '}' {
                            brace_depth -= 1;
                        }
                    }

                    if brace_depth < 0 {
                        // End of main() function
                        in_main = false;
                    }
                    continue;
                }

                output.push(line);
            }

            std::fs::write(&path, output.join("\n")).expect("Failed to write cleaned file");
        }
    }

    // Create mod.rs to export all generated modules
    let mod_rs_content = r#"// Generated Windjammer UI components
// Auto-generated from src_wj/components/*.wj
// DO NOT EDIT MANUALLY - run wj-build.sh to regenerate

pub mod button;
pub mod checkbox;
pub mod container;
pub mod flex;
pub mod input;
pub mod slider;
pub mod text;

// Re-export main types for convenience
pub use button::{Button, ButtonSize, ButtonVariant};
pub use checkbox::{Checkbox, CheckboxSize};
pub use container::Container;
pub use flex::{Flex, FlexDirection};
pub use input::Input;
pub use slider::Slider;
pub use text::{Text, TextSize, TextWeight};
"#;

    std::fs::write(out_dir.join("mod.rs"), mod_rs_content).expect("Failed to write mod.rs");

    println!("cargo:warning=✅ Successfully transpiled Windjammer components!");
}
