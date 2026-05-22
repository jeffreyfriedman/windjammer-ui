// Build script to auto-generate Rust code from Windjammer (.wj) source
// This runs automatically when developers run: cargo build
// Zero Rust knowledge needed - just edit .wj files!

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Allow skipping regeneration for manual fixes (temporary development mode)
    if env::var("SKIP_WJ_REGEN").is_ok() {
        println!("cargo:warning=⏭️  Skipping .wj regeneration (SKIP_WJ_REGEN set)");
        return;
    }

    // Tell Cargo to rerun this build script if .wj files change
    println!("cargo:rerun-if-changed=src/components_wj");

    // Get project root
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_root = PathBuf::from(&manifest_dir);

    // Paths
    let src_dir = project_root.join("src/components_wj");
    let out_dir = project_root.join("src/components/generated");

    // Check if we're in a cargo package/publish verification context
    // During verification, Cargo extracts the package to target/package/crate-name-version/
    let is_package_verification = manifest_dir.contains("/target/package/");

    // If we're in package verification and generated files already exist, skip generation
    // This prevents "Source directory was modified by build.rs" errors during cargo publish
    if is_package_verification {
        let mod_file = out_dir.join("mod.rs");
        if mod_file.exists() {
            println!("cargo:warning=📦 Skipping generation (package verification, files exist)");
            return;
        }
        // If files don't exist in package verification, we have a problem
        // but let generation proceed - it will fail with a clear error
    }

    // Try to find wj CLI - first check local build, then system PATH
    let local_wj = project_root.join("../windjammer/target/release/wj");
    let wj_cli = if local_wj.exists() {
        local_wj.to_str().unwrap().to_string()
    } else {
        // Try to use wj from PATH (installed via cargo install)
        "wj".to_string()
    };

    // Check if wj CLI is available and version
    let wj_check = Command::new(&wj_cli).arg("--version").output();

    if wj_check.is_err() {
        eprintln!("⚠️  Warning: wj CLI not found!");
        eprintln!("   Skipping .wj transpilation. To fix:");
        eprintln!("   Option 1: cargo install windjammer --version ^0.38.3");
        eprintln!("   Option 2: cd ../windjammer && cargo build --release");
        eprintln!();
        eprintln!("   Note: windjammer-ui v0.3.0 requires Windjammer v0.38.3+");
        eprintln!("   (for trait implementation visibility fixes)");
        return;
    }

    // Parse version and check minimum requirement
    if let Ok(output) = wj_check {
        let version_str = String::from_utf8_lossy(&output.stdout);
        if let Some(version_line) = version_str.lines().next() {
            // Extract version number (format: "windjammer 0.38.3")
            if let Some(version) = version_line.split_whitespace().nth(1) {
                let parts: Vec<&str> = version.split('.').collect();
                if parts.len() >= 2 {
                    if let (Ok(major), Ok(minor)) =
                        (parts[0].parse::<u32>(), parts[1].parse::<u32>())
                    {
                        let patch = parts
                            .get(2)
                            .and_then(|p| p.parse::<u32>().ok())
                            .unwrap_or(0);

                        // Require v0.38.3+
                        if major == 0 && (minor < 38 || (minor == 38 && patch < 3)) {
                            eprintln!("⚠️  Warning: Windjammer version {} is too old!", version);
                            eprintln!("   windjammer-ui v0.3.0 requires Windjammer v0.38.3+");
                            eprintln!(
                                "   Please upgrade: cargo install windjammer --version ^0.38.3"
                            );
                            eprintln!();
                            eprintln!("   Skipping .wj transpilation to avoid compilation errors.");
                            return;
                        }
                    }
                }
            }
        }
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

    // Remove the generated Cargo.toml to prevent cargo from treating it as a separate package
    let cargo_toml = out_dir.join("Cargo.toml");
    if cargo_toml.exists() {
        let _ = std::fs::remove_file(&cargo_toml);
        println!("cargo:warning=🗑️  Removed generated Cargo.toml (not needed for library)");
    }

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

    // Batch wj codegen wraps vnode_ffi with windjammer_runtime::ffi (not linked in this crate).
    let stable_vnode = project_root.join("src/components/vnode.stable.rs");
    let vnode_out = out_dir.join("vnode.rs");
    if stable_vnode.exists() {
        std::fs::copy(&stable_vnode, &vnode_out)
            .expect("Failed to restore stable vnode.rs after wj build");
        println!("cargo:warning=📌 Restored vnode.rs from vnode.stable.rs");
    }

    // wj --module-file may append stray modules for files outside components_wj; strip known bad entries.
    let mod_rs = out_dir.join("mod.rs");
    if let Ok(content) = std::fs::read_to_string(&mod_rs) {
        let mut lines: Vec<&str> = content.lines().collect();
        lines.retain(|line| {
            !line.contains("pub mod bt_visual_")
                && !line.contains("pub use bt_visual_")
                && !line.contains("pub mod components;")
                && !line.contains("pub use components::*")
                && !line.contains("pub mod components_wj")
                && !line.contains("pub use components_wj::*")
                && !line.contains("pub mod vnode.stable")
                && !line.contains("pub use vnode.stable::*")
        });
        let cleaned = lines.join("\n");
        if cleaned != content {
            std::fs::write(&mod_rs, format!("{cleaned}\n")).expect("Failed to clean mod.rs");
            println!("cargo:warning=📌 Cleaned stray entries from generated/mod.rs");
        }
    }

    // Trait-impl codegen still emits &String for vnode builders; patch button.rs until fixed upstream.
    let button_path = out_dir.join("button.rs");
    if let Ok(content) = std::fs::read_to_string(&button_path) {
        let patched = content
            .replace(".add_class(&\"wj-button\")", ".add_class(\"wj-button\".to_string())")
            .replace(".add_class(&self.get_variant_class())", ".add_class(self.get_variant_class())")
            .replace(".add_class(&self.get_size_class())", ".add_class(self.get_size_class())")
            .replace(".add_style(&self.get_style())", ".add_style(self.get_style())")
            .replace(".add_text(&self.label)", ".add_text(self.label.clone())");
        if patched != content {
            std::fs::write(&button_path, patched).expect("Failed to patch button.rs");
            println!("cargo:warning=📌 Patched button.rs VNode builder signatures");
        }
    }
}
