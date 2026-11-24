// Optional regeneration of src/components/generated/ from src/components_wj/.
//
// Off by default. `WJ_REGEN=1 cargo build` runs the Windjammer compiler (`$WJ`, then ../windjammer/target/release/wj,
// then `wj` on PATH) with exactly the flags tests/generated_fresh_test.rs uses, so the output is the compiler's, byte
// for byte: no formatting pass and no patching. That test fails when the checked-in tree is stale.

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=WJ_REGEN");
    println!("cargo:rerun-if-env-changed=WJ");
    if env::var("WJ_REGEN").is_err() {
        return;
    }
    println!("cargo:rerun-if-changed=src/components_wj");

    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let src_dir = root.join("src/components_wj");
    let out_dir = root.join("src/components/generated");
    let local = root.join("../windjammer/target/release/wj");
    let wj = env::var("WJ").unwrap_or_else(|_| {
        if local.exists() {
            local.to_string_lossy().to_string()
        } else {
            "wj".to_string()
        }
    });

    let status = Command::new(&wj)
        .arg("build")
        .arg(&src_dir)
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
        .status()
        .unwrap_or_else(|e| {
            panic!("could not run the Windjammer compiler `{wj}` ({e}); set WJ=/path/to/wj")
        });
    assert!(status.success(), "wj build failed for src/components_wj");

    // The compiler also writes cross-crate metadata and a stamp; they are not part of the tree.
    for extra in ["metadata.json", ".wj-compiler-stamp"] {
        let _ = std::fs::remove_file(out_dir.join(extra));
    }
}
