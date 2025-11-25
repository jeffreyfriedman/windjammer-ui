# Migration Plan: src_wj/ → src/ Simplification

## Why Simplify?

**Current (confusing):**
```
windjammer-ui/
  src/                      # Rust library code
    components/
      mod.rs
      generated/            # Transpiled from src_wj/
        *.rs
  src_wj/                   # Windjammer source (weird naming!)
    components/
      *.wj
```

**Proposed (intuitive):**
```
windjammer-ui/
  src/
    components/
      *.wj                  # Source of truth
      *.rs                  # Auto-generated (gitignored)
      mod.rs                # Auto-generated
```

## Benefits

1. ✅ **No weird `src_wj/` folder** - just use standard `src/`
2. ✅ **Co-located files** - `button.wj` next to `button.rs` 
3. ✅ **Standard Rust layout** - matches cargo conventions
4. ✅ **Better IDE support** - src/ is where code lives
5. ✅ **Simpler mental model** - one source directory

## Migration Steps

### Option A: In-Place Transpilation (Recommended)

```bash
# 1. Move .wj files to src/components/
mv src_wj/components/*.wj src/components/

# 2. Update .gitignore
echo "src/components/*.rs" >> .gitignore
echo "!src/components/mod.rs" >> .gitignore

# 3. Update build.rs
# Change: source = "src_wj/components"
# To:     source = "src/components"
# Change: output = "src/components/generated"  
# To:     output = "src/components" (same dir)

# 4. Build
wj build src/components/  # Auto-detects, generates .rs alongside .wj

# 5. Delete old folder
rm -rf src_wj/
```

### Option B: Separate Generated Dir (Current approach)

```bash
# 1. Rename src_wj/ to src/windjammer/
mv src_wj/ src/windjammer/

# 2. Update build.rs
# Change: source = "src_wj/components"
# To:     source = "src/windjammer/components"

# Still generates to src/components/generated/
```

### Option C: Virtual Modules (Future - most ambitious)

```bash
# .wj files are treated as native Rust modules by cargo
# No separate transpilation step needed
# cargo build just works on .wj files directly

# Requires: cargo-windjammer plugin
```

## Recommended: Option A

**Why:** Simplest, most intuitive, best DX.

**Implementation:**
1. `wj` CLI detects .wj files
2. Generates .rs files in same directory
3. .gitignore excludes generated .rs files
4. cargo builds the .rs files normally

**Result:**
```rust
// src/components/button.wj (source of truth)
pub struct Button { ... }

// src/components/button.rs (auto-generated, gitignored)
pub struct Button { ... } // transpiled Rust

// src/components/mod.rs (auto-generated)
pub mod button;
pub use button::*;
```

Developers only edit `.wj` files. Everything else is automatic.

## .gitignore Template

```gitignore
# Windjammer-UI specific
src/components/*.rs
src/components/generated/
!src/components/mod.rs

# Keep manual Rust files if any
!src/components/hand_written_*.rs
```

## Naming Convention

For mixed .wj + .rs projects:
- `.wj` files - Windjammer source (editable)
- `.rs` files - Either hand-written OR generated
- `.generated.rs` suffix - Clearly marks generated files (alternative)

Example:
```
src/components/
  button.wj              # Source
  button.rs              # Generated (or button.generated.rs)
  custom_widget.rs       # Hand-written Rust (if needed)
  mod.rs                 # Generated
```

## Timeline

1. **Phase 1 (Now)**: Document the plan
2. **Phase 2**: Implement in-place transpilation in `wj` CLI
3. **Phase 3**: Migrate windjammer-ui
4. **Phase 4**: Update templates and docs

---

**Status:** Proposed | **Priority:** Medium | **Blocks:** Better DX

