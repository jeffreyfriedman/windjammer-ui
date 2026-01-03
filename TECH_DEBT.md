# Technical Debt - Windjammer UI

## ⚠️ Critical Issue: `fix_generated.sh` Band-Aid

**Status:** 🚨 **BLOCKER** - Must be fixed before v0.4.0

This file documents technical debt that **violates the Windjammer philosophy**:
> **"No Workarounds, No Tech Debt, Only Proper Fixes"**

---

## The Problem

The Windjammer compiler generates **incorrect Rust code** that requires post-processing with a 110-line bash script (`fix_generated.sh`). This is not sustainable.

**Every line in `fix_generated.sh` represents a compiler bug that should be fixed.**

---

## Compiler Bugs Requiring Manual Fixes

### 🐛 **Bug #1: Missing Feature Gates for Desktop-Only Modules**

**File:** `fix_generated.sh` lines 20-38  
**Priority:** HIGH  
**Severity:** Breaking (compilation fails without fix)

**What It Fixes:**
```rust
// Generated (WRONG):
pub mod app;
pub mod desktop_renderer;

// Should Generate:
#[cfg(feature = "desktop")]
pub mod app;
#[cfg(feature = "desktop")]
pub mod desktop_renderer;
```

**Modules Affected:**
- `app`
- `app_docking`
- `app_reactive_eframe`
- `desktop_app_context`
- `desktop_renderer`
- `desktop_renderer_v2`

**Root Cause:**
Compiler doesn't know which modules require desktop features (egui, eframe).

**Proper Fix:**
1. **Option A:** Add metadata to `.wj` files: `// @requires desktop`
2. **Option B:** Analyze imports - if module uses `egui::*` or `eframe::*`, add desktop gate
3. **Option C:** Read `Cargo.toml` features and apply gates based on dependency usage

**Affected Files:**
- `src/codegen/rust/generator.rs` - Module declaration generation
- `src/analyzer.rs` - Feature detection

---

### 🐛 **Bug #2: Generates Declarations for Non-Existent Modules**

**File:** `fix_generated.sh` lines 40-48  
**Priority:** HIGH  
**Severity:** Breaking (compilation fails without fix)

**What It Fixes:**
```rust
// Generated (WRONG):
pub mod lib;          // lib.rs doesn't exist!
pub mod examples_wasm; // examples_wasm.rs is incomplete!
pub use lib::*;
pub use examples_wasm::*;
```

**Modules Affected:**
- `lib` - Never existed, compiler hallucinated it
- `examples_wasm` - Incomplete/broken module
- `dispatcher` (in events.rs) - dispatcher.rs doesn't exist

**Root Cause:**
Compiler generates module declarations without checking if the file exists.

**Proper Fix:**
1. Before generating `pub mod X;`, verify `X.rs` or `X/mod.rs` exists
2. Add filesystem check in codegen phase
3. Skip declaration generation for missing modules

**Affected Files:**
- `src/codegen/rust/generator.rs` - Module declaration generation
- Add: `src/codegen/rust/module_checker.rs` - New module to verify files exist

---

### 🐛 **Bug #3: Ambiguous Glob Re-Exports**

**File:** `fix_generated.sh` lines 41-48  
**Priority:** MEDIUM  
**Severity:** Warning (clippy fails with `-D warnings`)

**What It Fixes:**
```rust
// Generated (WRONG):
pub use reactivity::*;      // Conflicts with signal::*
pub use signal::*;          // Conflicts with reactivity::*
pub use event_handler::*;   // Conflicts with events::*
pub use simple_vnode::*;    // Conflicts with vdom::*
pub use vnode::*;           // Conflicts with simple_vnode::*
pub use desktop_renderer::*; // Conflicts with renderer::*
```

**Root Cause:**
Compiler blindly re-exports everything without checking for naming conflicts.

**Proper Fix:**
1. **Symbol table analysis** - Track all exported symbols
2. **Conflict detection** - Detect when multiple `pub use *;` export same name
3. **Smart re-exports** - Only re-export unique symbols, or use explicit imports
4. **Alternative:** Generate explicit re-exports instead of glob patterns

**Affected Files:**
- `src/codegen/rust/generator.rs` - Re-export generation
- Add: `src/analyzer/symbol_table.rs` - Track exported symbols
- Add: `src/analyzer/conflict_checker.rs` - Detect naming conflicts

---

### 🐛 **Bug #4: Incorrect Type Coercion for Rc<F> → Rc<dyn Fn()>**

**File:** `fix_generated.sh` lines 59-66  
**Priority:** HIGH  
**Severity:** Breaking (compilation fails without fix)

**What It Fixes:**
```rust
// Generated (WRONG):
Self::run_effect(id, &f);  
// Where f: Rc<F: Fn()> but run_effect expects &Rc<dyn Fn()>

// Should Generate:
let f_trait: Rc<dyn Fn()> = f.clone();
Self::run_effect(id, &f_trait);
```

**Root Cause:**
Codegen doesn't coerce generic `Rc<F>` to trait object `Rc<dyn Fn()>` when required.

**Proper Fix:**
1. **Type inference improvement** - Detect when trait object is needed
2. **Automatic coercion** - Generate intermediate variable with correct type
3. **Smart casting** - Insert coercion code when passing to functions expecting trait objects

**Affected Files:**
- `src/codegen/rust/generator.rs` - Expression generation
- `src/analyzer/type_checker.rs` - Type coercion rules
- `src/codegen/rust/type_casting.rs` - Cast generation

---

### 🐛 **Bug #5: Doc Comments Before Macros**

**File:** `fix_generated.sh` lines 68-69  
**Priority:** LOW  
**Severity:** Warning (rustc warning)

**What It Fixes:**
```rust
// Generated (WRONG):
/// Global reactive context
thread_local! { ... }

// Should Generate:
// Global reactive context
thread_local! { ... }
```

**Root Cause:**
Compiler generates doc comments (`///`) instead of regular comments (`//`) before macros.

**Proper Fix:**
1. **Context detection** - Know when generating comment before macro vs. before item
2. **Smart comment type** - Use `//` for macros, `///` for items
3. **AST analysis** - Check if next item is macro invocation

**Affected Files:**
- `src/codegen/rust/generator.rs` - Comment generation
- `src/parser.rs` - Comment type determination

---

### 🐛 **Bug #6: Unused Fields Not Marked with #[allow(dead_code)]**

**File:** `fix_generated.sh` lines 71-86  
**Priority:** LOW  
**Severity:** Warning (clippy fails with `-D warnings`)

**What It Fixes:**
```rust
// Generated (WRONG):
pub struct Effect {
    id: EffectId,  // Unused but no #[allow(dead_code)]
}

// Should Generate:
pub struct Effect {
    #[allow(dead_code)]
    id: EffectId,
}
```

**Root Cause:**
Compiler doesn't track which fields are actually used.

**Proper Fix:**
1. **Usage analysis** - Track field usage across codebase
2. **Smart attributes** - Add `#[allow(dead_code)]` to unused fields
3. **Dead code detection** - Run analysis pass to find unused items

**Affected Files:**
- `src/analyzer.rs` - Usage tracking
- `src/codegen/rust/generator.rs` - Attribute generation
- Add: `src/analyzer/dead_code_detector.rs` - Find unused fields/methods

---

### 🐛 **Bug #7: Unused Variables Not Prefixed with Underscore**

**File:** `fix_generated.sh` lines 74-75  
**Priority:** LOW  
**Severity:** Warning (rustc warning)

**What It Fixes:**
```rust
// Generated (WRONG):
fn render_panel(&mut self, attrs: &[(String, VAttr)]) {
    // attrs is unused
}

// Should Generate:
fn render_panel(&mut self, _attrs: &[(String, VAttr)]) {
    // _attrs indicates intentionally unused
}
```

**Root Cause:**
Compiler doesn't track variable usage within function bodies.

**Proper Fix:**
1. **Variable usage tracking** - Analyze function bodies for variable usage
2. **Auto-prefix** - Add `_` prefix to unused parameters
3. **Liveness analysis** - Standard compiler technique

**Affected Files:**
- `src/analyzer.rs` - Variable liveness analysis
- `src/codegen/rust/generator.rs` - Parameter name generation

---

## Impact Analysis

### Development Impact
- **Time waste:** ~5-10 minutes per regeneration waiting for script + debugging
- **Fragility:** Script breaks when generated code structure changes
- **Debugging difficulty:** Hard to tell if issue is compiler or script
- **Cross-platform issues:** macOS vs Linux `sed` differences

### CI Impact
- **Complexity:** Pre-commit hook must run script
- **Trust issues:** CI passes but compiler is fundamentally broken
- **False confidence:** Tests pass on "fixed" code, not generated code

### User Impact (External Users)
- **Blocked:** Cannot use `windjammer-ui` without our internal script
- **Confusion:** Why does official library require bash post-processing?
- **Trust erosion:** Looks unprofessional and unfinished

---

## Action Plan

### Phase 1: Documentation (DONE)
- [x] Document all bugs in this file
- [x] Link each bug to script line numbers
- [x] Prioritize by severity

### Phase 2: Compiler Fixes (IN PROGRESS)
Priority order:
1. **Bug #1:** Feature gates (HIGH, breaking)
2. **Bug #2:** Module existence checks (HIGH, breaking)
3. **Bug #4:** Rc<F> coercion (HIGH, breaking)
4. **Bug #3:** Ambiguous re-exports (MEDIUM, clippy warning)
5. **Bug #6:** Dead code attributes (LOW, warning)
6. **Bug #7:** Unused variable prefixes (LOW, warning)
7. **Bug #5:** Doc comments vs regular comments (LOW, warning)

### Phase 3: Validation
For each bug fix:
1. Add test in `windjammer/tests/`
2. Regenerate `windjammer-ui` components
3. Remove corresponding fix from `fix_generated.sh`
4. Verify: `cargo build --all-features`
5. Verify: `cargo clippy --all-features -- -D warnings`
6. Verify: `cargo test --all-features`

### Phase 4: Cleanup
- [ ] Delete `fix_generated.sh` entirely
- [ ] Remove `SKIP_WJ_REGEN` flag from `build.rs`
- [ ] Update CI to remove script execution
- [ ] Update docs to remove mention of script
- [ ] Close this TECH_DEBT.md file

---

## Success Criteria

✅ **Definition of Done:**
1. `fix_generated.sh` deleted
2. `SKIP_WJ_REGEN` removed
3. Generated code compiles without post-processing
4. All tests pass
5. Clippy passes with `-D warnings`
6. CI is green on all platforms
7. No manual intervention needed after regeneration

---

## Timeline

**Estimated effort:** 8-12 hours (1-2 sessions)

**Target completion:** v0.4.0 (before next release)

**Risk if delayed:** Technical debt compounds, more bugs require more fixes, script grows larger

---

## Lessons Learned

### What Went Wrong
1. **Took shortcuts** - Used script instead of fixing compiler
2. **Allowed workarounds** - Violated "no tech debt" philosophy
3. **Didn't dogfood enough** - Should have caught these earlier

### How to Prevent
1. **Strict policy:** No bash scripts to "fix" generated code
2. **Test generated code directly** - Don't allow post-processing
3. **Fail fast** - Compilation errors should block, not be scripted away
4. **TDD for codegen** - Write expected output, make compiler generate it

### Moving Forward
**From now on:** If generated code is wrong, **fix the compiler, not the output.**

---

## References

- **Windjammer Philosophy:** `docs/windjammer-development.mdc`
- **Compiler Architecture:** `docs/COMPILER_ARCHITECTURE.md`
- **Codegen:** `src/codegen/rust/generator.rs`
- **This script:** `fix_generated.sh` (to be deleted)

---

**Last Updated:** 2026-01-03  
**Status:** 🔴 ACTIVE TECH DEBT - MUST FIX  
**Owner:** @jeffreyfriedman  
**Next Action:** Fix Bug #1 (Feature Gates)

