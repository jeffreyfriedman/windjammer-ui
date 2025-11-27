// Build script to auto-generate Rust code from Windjammer (.wj) source
// This runs automatically when developers run: cargo build
// Zero Rust knowledge needed - just edit .wj files!

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Tell Cargo to rerun this build script if .wj files change
    println!("cargo:rerun-if-changed=src/components_wj");

    // Get project root
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_root = PathBuf::from(&manifest_dir);

    // Paths
    let src_dir = project_root.join("src/components_wj");
    let out_dir = project_root.join("src/components/generated");

    // Try to find wj CLI - first check local build, then system PATH
    let local_wj = project_root.join("../windjammer/target/release/wj");
    let wj_cli = if local_wj.exists() {
        local_wj.to_str().unwrap().to_string()
    } else {
        // Try to use wj from PATH (installed via cargo install)
        "wj".to_string()
    };

    // Check if wj CLI is available
    let wj_check = Command::new(&wj_cli).arg("--version").output();

    if wj_check.is_err() {
        eprintln!("⚠️  Warning: wj CLI not found!");
        eprintln!("   Skipping .wj transpilation. To fix:");
        eprintln!("   Option 1: cargo install windjammer");
        eprintln!("   Option 2: cd ../windjammer && cargo build --release");
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

    // Run wj build with --library and --module-file flags for automated library generation
    let status = Command::new(&wj_cli)
        .arg("build")
        .arg(&src_dir)
        .arg("-o")
        .arg(&out_dir)
        .arg("--target")
        .arg("rust")
        .arg("--library") // Auto-strip main() functions
        .arg("--module-file") // Auto-generate mod.rs
        .arg("--no-cargo") // Skip cargo build (we'll do it ourselves)
        .status()
        .expect("Failed to execute wj build");

    if !status.success() {
        panic!("wj build failed! Check .wj source for errors.");
    }

    println!("cargo:warning=✅ Successfully transpiled Windjammer components!");

    // Format the generated Rust code and add clippy allow directives
    println!("cargo:warning=🎨 Formatting generated Rust code...");

    // Find all .rs files in the output directory
    if let Ok(entries) = std::fs::read_dir(&out_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                // Add allow directives to the top of each generated file
                if let Ok(content) = std::fs::read_to_string(&path) {
                    let new_content = format!(
                        "#![allow(clippy::all)]\n#![allow(noop_method_call)]\n{}",
                        content
                    );
                    let _ = std::fs::write(&path, new_content);
                }

                // Format the file
                let _ = Command::new("rustfmt")
                    .arg("--edition")
                    .arg("2021")
                    .arg(&path)
                    .status();
            }
        }
        println!("cargo:warning=✅ Generated code formatted!");
    }
}
