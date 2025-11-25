# `wj` CLI DX Improvements Needed

**Issue:** Currently building windjammer-ui requires a custom bash script (`wj-build.sh`) that post-processes the output. This is terrible DX.

## ❌ Current Workflow Problems

### 1. **Test Functions Included in Library Output**
```bash
# Current: wj generates main() test functions
wj build src_wj/components/button.wj -o generated/

# Output includes:
fn main() {
    let button = Button::new("Test".to_string())
    println!("{}", button.render())
}
```

**Problem:** These test functions shouldn't be in library code. We have to manually strip them with `sed`.

**Solution:** Add `--library` flag:
```bash
wj build src_wj/components/ -o generated/ --library
# Should NOT generate main() functions
```

### 2. **No mod.rs Generation**
```bash
# Current: We manually create mod.rs after wj build
cat > generated/mod.rs << EOF
pub mod button;
pub mod checkbox;
...
EOF
```

**Problem:** The CLI should auto-generate module files.

**Solution:** Auto-generate mod.rs:
```bash
wj build src_wj/components/ -o generated/ --library
# Should auto-create:
# generated/mod.rs with:
#   pub mod button;
#   pub use button::{Button, ButtonSize, ButtonVariant};
```

### 3. **No build.rs Integration**
```rust
// Current build.rs is complex and fragile
fn main() {
    let output = Command::new("wj")
        .arg("build")
        .arg("src_wj/components")
        .arg("-o")
        .arg("src/components/generated")
        .status();
    
    // Then manually strip main(), create mod.rs...
}
```

**Problem:** Too manual, error-prone.

**Solution:** Add `wj-build-integration` helper:
```rust
// build.rs
fn main() {
    windjammer_build::transpile_library("src_wj/components", "src/components/generated");
}
```

### 4. **No Watch Mode**
```bash
# Current: Manual rebuild every time
bash wj-build.sh
cargo build
```

**Problem:** Slow iteration cycle.

**Solution:** Add watch mode:
```bash
wj watch src_wj/components/ -o generated/ --library
# Auto-rebuilds on file changes
```

## ✅ Proposed `wj` CLI Improvements

### Auto-Detection: Smart Library Mode

**The `wj` CLI should auto-detect library vs binary mode!**

```bash
# Auto-detects as library (no --library needed!)
wj build src_wj/components/ -o generated/
# ✅ Multiple files + no main() = library mode

# Auto-detects as binary
wj build src/main.wj -o target/
# ✅ Single file with main() = binary mode

# Force override if needed
wj build src/main.wj -o target/ --library
wj build lib/ -o out/ --binary
```

#### Detection Rules (in priority order):

1. **Explicit flags win**: `--library` or `--binary` always respected
2. **No main() function**: If no `fn main()` exists across all files → library mode
3. **Directory patterns**: Paths like `src_wj/`, `components/`, `lib/`, `modules/` → library mode
4. **Project config**: If `.wj-config.toml` exists, read `type = "library"` or `type = "binary"`
5. **Cargo.toml detection**: If nearby Cargo.toml has `[lib]` section → library mode
6. **All items pub**: If >80% of top-level items are `pub` → library mode
7. **Single file with main()**: Default to binary mode

### New Flags

```bash
wj build [SOURCE] -o [OUTPUT] [FLAGS]

Flags:
  --library           Force library mode (no main() functions in output)
  --binary            Force binary mode (keep main() functions)
  --module-file       Auto-generate mod.rs with re-exports (auto-on for library mode)
  --watch             Watch for changes and rebuild
  --integration DIR   Set up as library in existing Rust project
```

### New Subcommands

```bash
# Initialize a new Windjammer library project
wj init my-ui-lib --library

# Set up Windjammer in existing Rust project
wj integrate --source src_wj/ --output src/generated/

# Build library with all conveniences
wj build-lib src_wj/ -o src/generated/
# Equivalent to: wj build src_wj/ -o src/generated/ --library --module-file
```

### Example: Ideal Workflow

```bash
# Step 1: Initialize (one-time)
cd windjammer-ui
wj integrate --source src_wj/components --output src/components/generated

# This creates:
# - build.rs (auto-configured)
# - .wj-config.toml (with type = "library")
# - Cargo.toml updates (if needed)

# Step 2: Develop (ongoing)
wj watch src_wj/components  # Auto-detects library mode! No --library flag needed!
cargo run --example gallery # In another terminal

# Changes to .wj files auto-trigger:
# 1. Transpilation (library mode auto-detected)
# 2. mod.rs regeneration (auto-enabled for libraries)
# 3. Cargo rebuild

# Step 3: Zero configuration! ✅
```

### Real-World Examples

```bash
# Building windjammer-ui (auto-detects as library)
cd windjammer-ui
wj build src/components/ -o target/generated/
# ✅ Detects: directory + no main() → library mode
# ✅ Auto-generates mod.rs
# ✅ Strips test main() functions

# Or even simpler with in-place transpilation:
wj build src/components/
# ✅ Generates .rs files alongside .wj files
# ✅ button.wj → button.rs (auto-detected as library)

# Building a game (auto-detects as binary)
wj build game/main.wj -o target/
# ✅ Detects: single file + main() → binary mode
# ✅ Keeps main() function

# Building tests (explicit flag)
wj build tests/ -o target/tests/ --binary
# ✅ Force binary mode to keep test main() functions

# Mixed project (use config)
cat > .wj-config.toml << EOF
type = "library"
source = "src_wj/"
output = "src/generated/"
EOF

wj build
# ✅ Reads config, builds in library mode
```

## 🎯 Priority Order

1. **HIGH: Auto-detection** - Smart library vs binary mode detection
2. **HIGH: `--library`/`--binary` flags** - Override auto-detection
3. **HIGH: `--module-file` flag** - Auto-generate mod.rs (auto-on for libraries)
4. **MEDIUM: `.wj-config.toml`** - Project configuration file
5. **MEDIUM: `wj integrate`** - Set up in existing projects
6. **MEDIUM: `--watch` mode** - Developer productivity
7. **LOW: Helper crate** - `windjammer-build` for build.rs

## 📝 Implementation Notes

### For Auto-Detection:
```rust
fn detect_build_mode(source_path: &Path, files: &[PathBuf]) -> BuildMode {
    // 1. Check for explicit config
    if let Some(config) = read_wj_config(source_path) {
        return config.build_mode;
    }
    
    // 2. Check for main() function
    let has_main = files.iter().any(|f| file_has_main_function(f));
    if !has_main {
        return BuildMode::Library;
    }
    
    // 3. Check directory patterns (no more src_wj/ needed!)
    let path_str = source_path.to_string_lossy();
    if path_str.contains("components") || path_str.contains("lib") 
        || path_str.contains("modules") {
        return BuildMode::Library;
    }
    
    // 4. Check if near Cargo.toml with [lib]
    if let Some(cargo_toml) = find_cargo_toml(source_path) {
        if cargo_toml.has_lib_section() {
            return BuildMode::Library;
        }
    }
    
    // 5. Check pub item percentage
    let pub_percentage = calculate_pub_item_percentage(files);
    if pub_percentage > 0.8 {
        return BuildMode::Library;
    }
    
    // Default: binary if single file with main()
    if files.len() == 1 && has_main {
        return BuildMode::Binary;
    }
    
    // Multiple files without main: library
    BuildMode::Library
}
```

### For `--library` flag:
- In codegen, check `is_test_function()` before emitting `fn main()`
- Or: Don't emit any `fn main()` when library mode
- Test functions should go in `#[cfg(test)]` blocks instead

### For `--module-file`:
- Scan generated .rs files
- Extract `pub struct`, `pub enum`, `pub fn` declarations
- Generate mod.rs with:
  ```rust
  pub mod button;
  pub use button::{Button, ButtonSize, ButtonVariant};
  ```

### For `wj integrate`:
- Create `.wj-config.toml`:
  ```toml
  [library]
  source = "src_wj/components"
  output = "src/components/generated"
  auto_mod = true
  ```
- Generate build.rs that reads config
- Add to .gitignore: `src/components/generated/`

## 💡 Dogfooding Insight

**The fact that we needed `wj-build.sh` means the `wj` CLI isn't production-ready for library development.**

This is exactly what dogfooding is for - we discovered:
1. Language gaps (5 compiler bugs ✅ FIXED)
2. **Tooling gaps** (this document ⚠️ NEEDS FIX)

Once these CLI improvements land, building windjammer-ui becomes:
```bash
wj integrate src_wj/components src/components/generated
cargo build  # Just works! ✨
```

No bash scripts, no sed, no manual mod.rs. **That's proper DX.**

---

**Status:** Documented | **Priority:** High | **Blocks:** Production readiness

