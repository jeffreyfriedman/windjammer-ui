//! Compiles `wj/src/*.wj` with the Windjammer compiler (transpile only) into `$OUT_DIR/generated.rs`.
//! The compiler is found through `WJ_BIN` (or `wj` on PATH). The known-good compiler commit is in
//! `wj-pin.txt`; generated Rust is never committed.
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=wj/src");
    println!("cargo:rerun-if-changed=wj/blocked");
    println!("cargo:rerun-if-changed=wj/wj.toml");
    println!("cargo:rerun-if-env-changed=WJ_BIN");
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let gen_dir = out.join("wjgen");
    let _ = std::fs::remove_dir_all(&gen_dir);
    let wj = std::env::var("WJ_BIN").unwrap_or_else(|_| "wj".into());
    let status = Command::new(&wj)
        .current_dir("wj")
        .args(["build", "src", "--library", "--no-cargo", "--no-lint", "-o"])
        .arg(&gen_dir)
        .status()
        .unwrap_or_else(|e| panic!("could not run the Windjammer compiler `{wj}` ({e}); set WJ_BIN (pin in wj-pin.txt)"));
    assert!(status.success(), "wj build failed for wj/src");
    let mut names: Vec<String> = std::fs::read_dir("wj/src")
        .unwrap()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.path().file_stem().map(|s| s.to_string_lossy().to_string()).filter(|_| e.path().extension().map_or(false, |x| x == "wj")))
        .collect();
    names.sort();
    let mut all = String::new();
    for n in &names {
        let src = std::fs::read_to_string(gen_dir.join(format!("{n}.rs"))).unwrap_or_else(|_| panic!("compiler produced no {n}.rs"));
        all.push_str(&format!("pub mod {n} {{\n{src}\n}}\n"));
    }
    std::fs::write(out.join("generated.rs"), all).unwrap();

    // Samples the compiler cannot compile yet are transpiled only, so tests can pin the defect.
    let blocked = out.join("blocked");
    let _ = std::fs::remove_dir_all(&blocked);
    let status = Command::new(&wj)
        .current_dir("wj")
        .args(["build", "blocked", "--library", "--no-cargo", "--no-lint", "-o"])
        .arg(&blocked)
        .status()
        .expect("run wj for blocked samples");
    assert!(status.success(), "wj build failed for wj/blocked");
    println!("cargo:rustc-env=WJ_BLOCKED_DIR={}", blocked.display());
}
