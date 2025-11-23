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
    let wj_cli = project_root.join("../windjammer/target/release/wj");
    let src_dir = project_root.join("src/components_wj");
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
        .status()
        .expect("Failed to execute wj build");

    if !status.success() {
        panic!("wj build failed! Check .wj source for errors.");
    }

    println!("cargo:warning=✅ Successfully transpiled Windjammer components!");
}
