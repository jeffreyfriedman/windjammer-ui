# Windjammer UI - Component Conversion Plan
**Dogfooding: Rewriting All Components in Pure Windjammer**

## 🎯 Goal
Convert ALL 32 windjammer-ui components from Rust to pure Windjammer (`.wj` files), achieving **100% dogfooding** and zero Rust knowledge required for component development.

## ✅ Phase 1: COMPLETE (7/32 components)

**Converted Components:**
1. ✅ **Button** - Primary UI component with variants
2. ✅ **Checkbox** - Form input with sizes
3. ✅ **Container** - Layout component with Vec<String> children
4. ✅ **Flex** - Flexbox layout (Row/Column)
5. ✅ **Input** - Text input field
6. ✅ **Slider** - Range input with float support
7. ✅ **Text** - Typography component with sizes/weights

**Compiler Bugs Fixed:**
- Bug #5: Copy enum inference ✅
- Bug #6: Format string escaping ✅
- Bug #7: Owned parameter inference (Vec<T>, String) ✅
- Bug #8: pub visibility on structs/enums ✅
- Bug #9: pub visibility on functions in impl blocks ✅

**Infrastructure:**
- ✅ `wj-build.sh` - Manual build script
- ✅ `build.rs` - Automatic transpilation on cargo build
- ✅ `.gitignore` - Generated files excluded
- ✅ All 111 tests passing
- ✅ Examples verified (counter_test, gallery.html)

## 📋 Phase 2: Core UI Components (8 components)

**Priority: HIGH** - Essential for basic UIs

| Component | Complexity | Status | Notes |
|-----------|-----------|--------|-------|
| **Card** | Low | 🔜 | Container with header/footer |
| **Badge** | Low | 🔜 | Label with variants |
| **Alert** | Low | 🔜 | Notification with variants |
| **Divider** | Low | 🔜 | Horizontal/vertical separator |
| **Spacer** | Low | 🔜 | Flexible spacing |
| **Spinner** | Low | 🔜 | Loading indicator |
| **Progress** | Medium | 🔜 | Progress bar (determinate/indeterminate) |
| **Grid** | Medium | 🔜 | Grid layout system |

**Estimated Time:** 4-6 hours  
**Blockers:** None (all compiler features ready)

## 📋 Phase 3: Form Components (4 components)

**Priority: HIGH** - Complete form system

| Component | Complexity | Status | Notes |
|-----------|-----------|--------|-------|
| **Radio** | Low | 🔜 | Radio button group |
| **Select** | Medium | 🔜 | Dropdown select |
| **Switch** | Low | 🔜 | Toggle switch |
| **ColorPicker** | High | 🔜 | Color selection (complex state) |

**Estimated Time:** 3-5 hours  
**Potential Blockers:** ColorPicker may need event handling improvements

## 📋 Phase 4: Navigation & Layout (4 components)

**Priority: MEDIUM** - App structure

| Component | Complexity | Status | Notes |
|-----------|-----------|--------|-------|
| **Tabs** | Medium | 🔜 | Tab navigation |
| **TabPanel** | Medium | 🔜 | Tab content panels |
| **Toolbar** | Low | 🔜 | Action toolbar |
| **Panel** | Low | 🔜 | Generic panel container |

**Estimated Time:** 3-4 hours  
**Blockers:** None

## 📋 Phase 5: Advanced Layout (3 components)

**Priority: MEDIUM** - Complex layouts

| Component | Complexity | Status | Notes |
|-----------|-----------|--------|-------|
| **SplitPanel** | High | 🔜 | Resizable split view |
| **ScrollArea** | Medium | 🔜 | Custom scrollbar |
| **CollapsibleSection** | Medium | 🔜 | Accordion/collapsible |

**Estimated Time:** 4-6 hours  
**Potential Blockers:** SplitPanel needs drag handling

## 📋 Phase 6: Interactive Components (3 components)

**Priority: MEDIUM** - User interaction

| Component | Complexity | Status | Notes |
|-----------|-----------|--------|-------|
| **Dialog** | Medium | 🔜 | Modal dialog |
| **Tooltip** | Medium | 🔜 | Hover tooltip |
| **Tree** | High | 🔜 | Recursive tree structure |

**Estimated Time:** 4-5 hours  
**Potential Blockers:** TreeView needs recursive type support

## 📋 Phase 7: Code Editor Components (3 components)

**Priority: LOW** - game-host specific

| Component | Complexity | Status | Notes |
|-----------|-----------|--------|-------|
| **CodeEditor** | High | 🔜 | Basic code editor |
| **AdvancedCodeEditor** | Very High | 🔜 | Full-featured editor |
| **FileTree** | High | 🔜 | File system tree |

**Estimated Time:** 8-12 hours  
**Potential Blockers:** May need syntax highlighting support in Windjammer

## 🚀 Conversion Strategy

### Approach A: **Incremental** (Recommended)
- Convert 2-3 components per session
- Test thoroughly after each conversion
- Fix compiler bugs as discovered
- **Pros:** Stable, discover bugs early
- **Cons:** Slower, takes multiple sessions

### Approach B: **Batch**
- Convert all simple components first (Low complexity)
- Then medium, then high
- Fix all bugs at end
- **Pros:** Faster initial conversion
- **Cons:** Risky, bugs compound

### Approach C: **Feature-Driven**
- Convert by use case (e.g., "complete form system")
- Convert Card + Badge + Alert together
- **Pros:** Demonstrates complete features
- **Cons:** Mixed complexity levels

## 📊 Estimated Timeline

| Phase | Components | Time | Cumulative |
|-------|-----------|------|------------|
| Phase 1 (✅ Done) | 7 | - | 7/32 (22%) |
| Phase 2 | 8 | 4-6h | 15/32 (47%) |
| Phase 3 | 4 | 3-5h | 19/32 (59%) |
| Phase 4 | 4 | 3-4h | 23/32 (72%) |
| Phase 5 | 3 | 4-6h | 26/32 (81%) |
| Phase 6 | 3 | 4-5h | 29/32 (91%) |
| Phase 7 | 3 | 8-12h | 32/32 (100%) |

**Total Estimated Time:** 26-38 hours of focused work

## 🎯 Immediate Next Steps

1. **Convert Phase 2** (Core UI: Card, Badge, Alert, Divider, Spacer, Spinner, Progress, Grid)
2. **Update examples** to showcase all Windjammer components
3. **Create browser gallery** with all 32 components
4. **Performance testing** - compare .wj vs .rs compile times
5. **Documentation** - Pure Windjammer component guide

## 🔥 Why This Matters

### For Windjammer Language:
- **Proves** Windjammer can build production UIs
- **Discovers** language gaps early
- **Validates** 80/20 philosophy (80% power, 20% complexity)

### For game-host:
- **Zero Rust knowledge** needed for game UI modding
- **Faster iteration** - edit .wj, instant reload
- **Better DX** - simpler syntax, less boilerplate

### For Community:
- **Best-in-class example** of dogfooding
- **Reference implementation** for UI libraries
- **Proves Windjammer is production-ready**

## 📝 Notes

- All new bugs discovered should be documented in `DOGFOODING_GAPS_FOUND.md`
- Each component conversion is a compiler stress test
- ColorPicker and AdvancedCodeEditor are the highest risk items
- Consider creating a `windjammer-ui-native` crate for platform-specific components

---

**Status:** Phase 1 Complete | Next: Phase 2 (Core UI Components)  
**Last Updated:** 2025-11-23

