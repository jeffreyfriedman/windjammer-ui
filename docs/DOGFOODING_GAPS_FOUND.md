# Language Gaps Found During windjammer-ui Dogfooding

**Date:** November 23, 2025  
**Status:** In Progress

---

## Compiler Bugs Found

### 1. **`mut self` Parameter Doubled** ❌ CRITICAL
**Location:** Method parameters  
**Issue:** Compiler generates `mut mut self` instead of `mut self`

**Windjammer Code:**
```windjammer
pub fn size(mut self, size: TextSize) -> Text {
    self.size = size
    self
}
```

**Generated Rust Code:**
```rust
fn size(&mut self, mut mut self: Self, size: &TextSize) -> Text {
    //             ^^^^^^^^ BUG: doubled mut
    self.size = self.size;  // Also wrong assignment
    self
}
```

**Expected Rust Code:**
```rust
fn size(mut self, size: TextSize) -> Text {
    self.size = size;
    self
}
```

**Impact:** Builder pattern methods don't compile  
**Priority:** HIGH - blocks all component development

---

### 2. **`pub fn new()` Missing `pub` Modifier** ❌
**Issue:** `pub fn new()` generates `fn new(&self, ...)` (private, with self param)

**Windjammer Code:**
```windjammer
pub fn new(content: string) -> Text {
    Text { content: content, ... }
}
```

**Generated Rust Code:**
```rust
fn new(&self, content: &String) -> Text {
//  ^^ missing pub
//      ^^ shouldn't have &self
    ...
}
```

**Expected:**
```rust
pub fn new(content: String) -> Text {
    ...
}
```

**Impact:** Constructors not accessible  
**Priority:** HIGH

---

### 3. **Assignment Using Wrong Variable** ❌
**Issue:** `self.size = size` becomes `self.size = self.size`

**Windjammer Code:**
```windjammer
pub fn size(mut self, size: TextSize) -> Text {
    self.size = size  // Assign parameter to field
    self
}
```

**Generated:**
```rust
self.size = self.size;  // BUG: using self.size instead of parameter
```

**Expected:**
```rust
self.size = size;
```

**Impact:** Builder methods don't actually set values  
**Priority:** HIGH

---

### 4. **Missing External Type Bindings** ❌
**Issue:** No way to reference types from Rust crates (VNode, VAttr, ToVNode)

**Need:**
```windjammer
// External type declarations
external type VNode from "windjammer_ui::simple_vnode"
external type VAttr from "windjammer_ui::simple_vnode"
external trait ToVNode from "windjammer_ui::to_vnode"
```

**Impact:** Can't use existing Rust types  
**Priority:** MEDIUM - can work around initially

---

### 5. **`vec![]` Macro Not Supported** ⚠️
**Issue:** Compiler doesn't pass through `vec![]` macro

**Windjammer Code:**
```windjammer
let mut classes = vec!["wj-text"]
```

**Need:** Macro passthrough or alternative syntax

**Workaround:** Could use `Vec::new()` + `push()`

**Priority:** MEDIUM

---

## Missing Language Features

### 6. **Struct Field Initialization Shorthand** ⚠️
**Issue:** Can't use `{ content }` shorthand

**Windjammer Code:**
```windjammer
Text { content: content, size: TextSize::Medium }
```

**Rust has shorthand:**
```rust
Text { content, size: TextSize::Medium }
```

**Priority:** LOW - syntax sugar

---

### 7. **Method Visibility (`pub`) in Impl Blocks** ⚠️
**Issue:** Unclear if `pub fn` in impl blocks is being honored

**Priority:** MEDIUM - need to verify

---

## Test Case for Reproduction

```windjammer
// test_builder_pattern.wj
pub struct Counter {
    value: i32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { value: 0 }
    }
    
    pub fn increment(mut self) -> Counter {
        self.value = self.value + 1
        self
    }
    
    pub fn set(mut self, value: i32) -> Counter {
        self.value = value
        self
    }
}

fn main() {
    let counter = Counter::new().increment().set(10)
    println!("Counter: {}", counter.value)
}
```

**Expected behavior:** Should compile and print "Counter: 10"  
**Actual behavior:** Compilation errors with `mut mut self`

---

## Action Items

### Immediate (Blocking):
1. [ ] Fix `mut self` parameter codegen (doubled mut)
2. [ ] Fix `pub fn` visibility in impl blocks
3. [ ] Fix parameter assignment codegen

### Short-term:
4. [ ] Add external type binding support
5. [ ] Add macro passthrough for `vec![]`

### Long-term:
6. [ ] Struct initialization shorthand
7. [ ] Full trait implementation support

---

## Next Steps

1. Create minimal test case for each bug
2. Fix bugs in windjammer compiler
3. Re-attempt Text component compilation
4. Iterate until components compile
5. Document patterns and best practices

---

**This is exactly the point of dogfooding!** 🎯  
Finding and fixing these gaps makes Windjammer better for everyone.

