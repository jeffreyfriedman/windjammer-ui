# Windjammer UI - Style Guide

**Version:** 0.34.0  
**Purpose:** Document supported syntax patterns and best practices

---

## Overview

This guide shows the **correct, working syntax** for Windjammer UI applications. Following these patterns ensures your code compiles and runs successfully.

---

## ✅ Supported Patterns

### Module Imports

**✅ Correct:**
```windjammer
use std::ui::*
use std::collections::*
```

**❌ Wrong:**
```windjammer
use std.ui.*  // Wrong: Use :: not .
```

---

### Builder Pattern (Recommended)

**✅ Correct:**
```windjammer
Button::new("Click me")
    .variant(ButtonVariant::Primary)
    .size(ButtonSize::Large)
    .on_click(handler)
```

**❌ Not Supported Yet:**
```windjammer
// Named arguments syntax not supported in current version
Button::new(
    text: "Click me",
    variant: ButtonVariant::Primary,
    on_click: handler
)
```

**Why:** Parser expects positional arguments + builder methods. Named arguments are planned for v0.2.0.

---

### Signal Creation & Usage

**✅ Correct:**
```windjammer
// Create Signal
let count = Signal::new(0)

// Clone for closures
let count_ref = count.clone()

// Use in closure
Button::new("Inc").on_click(move || {
    count_ref.set(count_ref.get() + 1)
})

// Display in UI
Text::new(format!("Count: {}", count.get()))
```

**❌ Common Mistakes:**
```windjammer
// Don't forget to clone before move
let count = Signal::new(0)
Button::new("Inc").on_click(move || {
    count.set(count.get() + 1)  // ❌ Error: count moved
})

// Don't use mutable references
let mut count = Signal::new(0)  // ❌ Unnecessary, Signals use interior mutability
```

---

### Component Composition

**✅ Correct:**
```windjammer
Container::new()
    .max_width("800px")
    .child(
        Flex::new()
            .direction(FlexDirection::Column)
            .gap("16px")
            .child(Text::new("Item 1"))
            .child(Text::new("Item 2"))
    )
```

**Key Points:**
- Use `.child()` to add children
- Chain methods for properties
- Convert to VNode with `.to_vnode()` when needed

---

### Struct Definitions

**✅ Correct:**
```windjammer
@derive(Clone)
struct TodoItem {
    id: int,
    text: string,
    completed: bool
}
```

**❌ Common Mistakes:**
```windjammer
// Missing Clone derive for use in Signal<Vec<T>>
struct TodoItem {  // ❌ Error if used in Signal<Vec<TodoItem>>
    id: int,
    text: string
}
```

**Why:** `Signal<T>` requires `T: Clone`. When using structs in Signals, add `@derive(Clone)`.

---

### Function Decorators

**✅ Correct:**
```windjammer
@export
fn start() {
    // App code here
}

fn main() {
    start()
}
```

**❌ Wrong:**
```windjammer
@export()  // ❌ Don't use parens for decorators with no args
fn start() {}
```

---

### String Interpolation

**✅ Correct:**
```windjammer
Text::new(format!("Count: {}", count.get()))
Text::new(format!("Hello, {}!", name))
```

**Note:** Direct string interpolation syntax (like `"Count: {count}"`) is planned but not yet implemented. Use `format!()` for now.

---

### Event Handlers

**✅ Correct:**
```windjammer
let count = Signal::new(0)
let count_ref = count.clone()

Button::new("Click").on_click(move || {
    // Closure body
    let current = count_ref.get()
    count_ref.set(current + 1)
    println!("Clicked!")
})
```

**Key Points:**
- Use `move ||` for closures
- Clone Signals before moving into closure
- Current version: handlers are `Fn()` (no parameters)

---

### Conditional Rendering

**✅ Supported (Rust-style):**
```windjammer
let ui = if condition {
    Text::new("True case")
} else {
    Text::new("False case")
}
```

**⚠️ Experimental (Inline):**
```windjammer
// This may work but is less tested
.child(
    if condition { Text::new("Yes") } else { Text::new("No") }
)
```

---

### Collections

**✅ Correct:**
```windjammer
let items = Signal::new(vec![1, 2, 3])

// Loop to build UI (outside component tree)
let mut children = vec![]
for item in items.get() {
    children.push(Text::new(format!("Item: {}", item)).to_vnode())
}

Flex::new()
    .children(children)
```

**❌ Not Yet Supported:**
```windjammer
// For-in directly in UI tree
Flex::new()
    for item in items.get() {  // ❌ Parser error
        .child(Text::new(item))
    }
```

**Workaround:** Build children vec separately, then pass to `.children()`.

---

## 📏 Code Style

### Indentation

Use **4 spaces** (matches Rust convention):

```windjammer
fn start() {
    let count = Signal::new(0)
    
    Container::new()
        .child(
            Flex::new()
                .child(Text::new("Hello"))
        )
}
```

### Line Length

**Recommended:** 100 characters max

**Long chains:** Break after each method:
```windjammer
Button::new("Very Long Button Text Here")
    .variant(ButtonVariant::Primary)
    .size(ButtonSize::Large)
    .disabled(false)
    .on_click(handler)
```

### Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Variables | snake_case | `user_count` |
| Functions | snake_case | `build_ui()` |
| Structs | PascalCase | `TodoItem` |
| Enums | PascalCase | `ButtonVariant` |
| Enum Variants | PascalCase | `ButtonVariant::Primary` |
| Signals | snake_case | `count`, `is_visible` |

---

## 🎯 Best Practices

### 1. Clone Signals Before Closures

```windjammer
// ✅ Good
let count = Signal::new(0)
let count_inc = count.clone()  // Explicit clone
let count_display = count.clone()

Button::new("Inc").on_click(move || {
    count_inc.set(count_inc.get() + 1)
})

Text::new(format!("Count: {}", count_display.get()))
```

### 2. Use Descriptive Signal Names

```windjammer
// ✅ Good
let is_loading = Signal::new(false)
let user_email = Signal::new("".to_string())
let selected_tab_index = Signal::new(0)

// ❌ Avoid
let s1 = Signal::new(false)
let data = Signal::new("".to_string())
let x = Signal::new(0)
```

### 3. Group Related Signals

```windjammer
// ✅ Good: Related state together
let email = Signal::new("".to_string())
let password = Signal::new("".to_string())
let is_valid = Computed::new(move || {
    !email.get().is_empty() && !password.get().is_empty()
})
```

### 4. Extract Complex UI to Functions

```windjammer
// ✅ Good
fn build_header(title: &str) -> VNode {
    Flex::new()
        .direction(FlexDirection::Row)
        .child(Text::new(title).size(TextSize::Large))
        .to_vnode()
}

fn start() {
    Container::new()
        .child(build_header("My App"))
        .child(build_content())
}
```

### 5. Use Computed for Derived Values

```windjammer
// ✅ Good: Computed auto-updates
let items = Signal::new(vec![1, 2, 3])
let total = Computed::new(move || {
    items.get().iter().sum::<i32>()
})

Text::new(format!("Total: {}", total.get()))

// ❌ Avoid: Manual recalculation
let sum = items.get().iter().sum::<i32>()  // Doesn't update!
```

---

## ⚠️ Common Pitfalls

### Pitfall 1: Forgetting .to_vnode()

```windjammer
// ❌ Wrong
Container::new()
    .child(Text::new("Hello"))  // Returns Text, not VNode

// ✅ Correct
Container::new()
    .child(Text::new("Hello").to_vnode())
```

### Pitfall 2: Moving Signal Instead of Cloning

```windjammer
// ❌ Wrong
let count = Signal::new(0)
Button::new("Inc").on_click(move || {
    count.set(count.get() + 1)  // count moved here
})
Text::new(format!("{}", count.get()))  // ❌ Error: count moved

// ✅ Correct
let count = Signal::new(0)
let count_inc = count.clone()
let count_display = count.clone()

Button::new("Inc").on_click(move || {
    count_inc.set(count_inc.get() + 1)
})
Text::new(format!("{}", count_display.get()))
```

### Pitfall 3: Using Static UI for Dynamic Content

```windjammer
// ❌ Wrong: UI won't update
let count = Signal::new(0)
let ui = Text::new(format!("Count: {}", count.get()))  // Evaluated once!
App::new("Counter", ui.to_vnode()).run()

// ✅ Correct: Use reactive app
let count = Signal::new(0)
let count_ref = count.clone()
App::new_reactive("Counter", move || {
    Text::new(format!("Count: {}", count_ref.get())).to_vnode()
}).run()
```

### Pitfall 4: Incorrect Enum Access

```windjammer
// ❌ Wrong
Button::new("Click").variant(Primary)  // Primary not in scope

// ✅ Correct
Button::new("Click").variant(ButtonVariant::Primary)
```

---

## 🚀 Performance Tips

### 1. Minimize Signal Clones

```windjammer
// ⚠️ Not ideal: Too many clones
let c1 = count.clone()
let c2 = count.clone()
let c3 = count.clone()
let c4 = count.clone()

// ✅ Better: Only clone what you need
let count_for_button = count.clone()
let count_for_display = count.clone()
```

### 2. Use get_untracked() When Appropriate

```windjammer
// ✅ Good: No tracking in debug logs
Effect::new(move || {
    println!("Debug: {}", count.get_untracked())
})
```

### 3. Batch Signal Updates

```windjammer
// ⚠️ Not ideal: Multiple updates trigger multiple re-renders
count.set(1)
count.set(2)
count.set(3)

// ✅ Better: Single update
count.set(3)
```

---

## 📚 Example Templates

### Minimal App

```windjammer
use std::ui::*

@export
fn start() {
    let ui = Container::new()
        .child(Text::new("Hello, Windjammer!"))
    
    App::new("My App", ui.to_vnode()).run()
}

fn main() {
    start()
}
```

### Counter with Reactive State

```windjammer
use std::ui::*

@export
fn start() {
    let count = Signal::new(0)
    let count_ref = count.clone()
    
    App::new_reactive("Counter", move || {
        Container::new()
            .child(
                Flex::new()
                    .direction(FlexDirection::Column)
                    .gap("16px")
                    .child(
                        Text::new(format!("Count: {}", count_ref.get()))
                            .size(TextSize::XLarge)
                    )
                    .child(
                        Button::new("Increment")
                            .variant(ButtonVariant::Primary)
                            .on_click({
                                let c = count_ref.clone()
                                move || c.set(c.get() + 1)
                            })
                    )
            )
            .to_vnode()
    }).run()
}

fn main() {
    start()
}
```

### Form with Multiple Fields

```windjammer
use std::ui::*

@export
fn start() {
    let name = Signal::new("".to_string())
    let email = Signal::new("".to_string())
    
    let name_ref = name.clone()
    let email_ref = email.clone()
    
    let ui = Container::new()
        .child(
            Flex::new()
                .direction(FlexDirection::Column)
                .gap("12px")
                .child(Text::new("Name:"))
                .child(
                    Input::new()
                        .placeholder("Your name")
                        .value(name.get())
                )
                .child(Text::new("Email:"))
                .child(
                    Input::new()
                        .placeholder("your@email.com")
                        .value(email.get())
                )
                .child(
                    Button::new("Submit")
                        .variant(ButtonVariant::Primary)
                        .on_click(move || {
                            println!("Name: {}", name_ref.get())
                            println!("Email: {}", email_ref.get())
                        })
                )
        )
    
    App::new("Form", ui.to_vnode()).run()
}

fn main() {
    start()
}
```

---

## 🔄 Migration from v0.33.0

### Changed APIs

1. **Input widget now has builder methods:**
   ```windjammer
   // Old (v0.33.0)
   Input::new(value, placeholder)  // ❌ Doesn't compile

   // New (v0.34.0)
   Input::new()
       .value(value)
       .placeholder(placeholder)  // ✅ Works
   ```

2. **App::new_reactive for dynamic UIs:**
   ```windjammer
   // Old (worked but didn't update)
   App::new("Title", ui).run()

   // New (updates when Signals change)
   App::new_reactive("Title", || build_ui()).run()
   ```

---

## 📝 Checklist

Before shipping your Windjammer UI app:

- [ ] All Signals used in closures are cloned
- [ ] Components use `.to_vnode()` where needed
- [ ] Enum variants are fully qualified (e.g. `ButtonVariant::Primary`)
- [ ] Module imports use `::` not `.`
- [ ] Structs used in `Signal<T>` have `@derive(Clone)`
- [ ] Dynamic UIs use `App::new_reactive()`
- [ ] Code follows naming conventions
- [ ] No compiler warnings

---

## 🆘 Troubleshooting

### "Expected RParen, got Assign"
**Cause:** Using named argument syntax  
**Fix:** Use builder pattern instead

### "Trait bound T: Clone is not satisfied"
**Cause:** Struct missing Clone derive  
**Fix:** Add `@derive(Clone)` to struct definition

### "Value used after move"
**Cause:** Forgot to clone Signal  
**Fix:** Clone before moving into closure

### "UI doesn't update when Signal changes"
**Cause:** Using `App::new()` instead of `App::new_reactive()`  
**Fix:** Use `App::new_reactive()` for reactive UIs

---

**Last Updated:** November 23, 2025  
**Questions?** See [API_REFERENCE.md](API_REFERENCE.md) or file an issue on GitHub.


