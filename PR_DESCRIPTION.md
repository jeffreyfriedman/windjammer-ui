# Fix CI Failures for v0.1.1

## Summary

Fixes all CI failures encountered in v0.1.0 release:
- **Clippy type complexity warnings** - Resolved by introducing event handler type aliases
- **Missing Linux system dependencies** - Added libgtk-3-dev and libglib2.0-dev to CI workflows
- **Publish order issue** - windjammer-ui-macro now publishes before windjammer-ui
- **Pre-commit hooks** - Added to prevent future CI surprises

## Changes

### 1. Event Handler Type Aliases (`src/event_handler.rs`)
Created type aliases to fix clippy `type_complexity` warnings:
```rust
pub type EventHandler = Rc<RefCell<dyn FnMut()>>;
pub type StringEventHandler = Rc<RefCell<dyn FnMut(String)>>;
pub type BoolEventHandler = Rc<RefCell<dyn FnMut(bool)>>;
pub type F64EventHandler = Rc<RefCell<dyn FnMut(f64)>>;
pub type ColorEventHandler = Rc<RefCell<dyn FnMut([f32; 4])>>;
```

Updated 10+ components to use these aliases.

### 2. CI Workflow Updates

**`.github/workflows/test.yml`:**
- Added Linux system dependency installation step
- Installs `libgtk-3-dev` and `libglib2.0-dev` on ubuntu-latest

**`.github/workflows/release.yml`:**
- Added Linux system dependency installation
- Fixed publish order: `windjammer-ui-macro` → `windjammer-ui`
- Added 30s wait for crates.io indexing

### 3. Pre-commit Hooks

**`.pre-commit-config.yaml`:**
- cargo fmt check
- cargo clippy with -D warnings
- cargo test (on push only)

**`.githooks/pre-commit`:**
- Bash script alternative for teams not using pre-commit framework
- Runs fmt, clippy, and tests before each commit

### 4. Code Cleanup
- Removed unused `app_reactive_old.rs` (dead code warnings)
- Fixed `manual_clamp` warning in `progress.rs`
- Removed stale module reference

### 5. Version Bump
- `0.1.0` → `0.1.1`

## Testing

CI will validate:
- ✅ Clippy passes with `-D warnings`
- ✅ Tests pass on Linux, macOS, Windows
- ✅ Publish dry-run succeeds
- ✅ Code is formatted correctly

## Breaking Changes

None - this is a patch release with CI/tooling improvements only.

---

**PR Link:** https://github.com/jeffreyfriedman/windjammer-ui/pull/new/fix/v0.1.1-ci-fixes

