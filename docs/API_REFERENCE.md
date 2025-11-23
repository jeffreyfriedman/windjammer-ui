# Windjammer UI - API Reference

**Version:** 0.34.0  
**Last Updated:** November 23, 2025

---

## Table of Contents

1. [Core Concepts](#core-concepts)
2. [Reactivity System](#reactivity-system)
3. [Component Library](#component-library)
4. [App & Rendering](#app--rendering)
5. [Events](#events)
6. [Styling](#styling)
7. [Examples](#examples)

---

## Core Concepts

### What is Windjammer UI?

Windjammer UI is a cross-platform UI framework for building desktop and web applications using the Windjammer programming language.

**Key Features:**
- 🎯 **Reactive State** - Signal-based reactivity like Solid.js
- 🏗️ **Component Model** - Clean, composable UI components
- 🖥️ **Cross-Platform** - Desktop (egui) and Web (WASM)
- 🚀 **Performance** - Compiled to native Rust
- 📦 **Zero Config** - Automatic dependency management

### Basic Example

```windjammer
use std::ui::*

@export
fn start() {
    let count = Signal::new(0)
    
    let ui = Container::new()
        .child(
            Flex::new()
                .direction(FlexDirection::Column)
                .child(Text::new(format!("Count: {}", count.get())))
                .child(
                    Button::new("Click me")
                        .on_click(move || count.set(count.get() + 1))
                )
        )
    
    App::new("My App", ui.to_vnode()).run()
}
```

---

## Reactivity System

### Signal<T>

Reactive value that notifies subscribers when it changes.

#### Creation

```windjammer
let count = Signal::new(0)
let name = Signal::new("Alice".to_string())
let items = Signal::new(vec![1, 2, 3])
```

#### Reading Values

```windjammer
let value = count.get()  // Tracks dependency
let value = count.get_untracked()  // No tracking
```

#### Writing Values

```windjammer
// Set new value
count.set(42)

// Update with function
count.update(|c| *c += 1)
```

#### Cloning for Closures

```windjammer
let count = Signal::new(0)
let count_ref = count.clone()  // Cheap clone (Rc internally)

Button::new("Click").on_click(move || {
    count_ref.set(count_ref.get() + 1)
})
```

### Computed<T>

Derived value that auto-updates when dependencies change.

```windjammer
let count = Signal::new(0)
let doubled = Computed::new(move || count.get() * 2)

println!("{}", doubled.get())  // 0
count.set(5)
println!("{}", doubled.get())  // 10
```

### Effect

Side effect that runs when dependencies change.

```windjammer
let count = Signal::new(0)

Effect::new(move || {
    println!("Count is now: {}", count.get())
})

count.set(5)  // Prints: "Count is now: 5"
```

---

## Component Library

### Layout Components

#### Container

Wrapper with max-width and centering.

```windjammer
Container::new()
    .max_width("800px")
    .child(/* content */)
```

**Methods:**
- `.max_width(width: &str)` - Set maximum width
- `.child(node: VNode)` - Add child element

---

#### Flex

Flexbox layout container.

```windjammer
Flex::new()
    .direction(FlexDirection::Column)
    .gap("16px")
    .align(Align::Center)
    .justify(Justify::SpaceBetween)
    .child(Text::new("Item 1"))
    .child(Text::new("Item 2"))
```

**Methods:**
- `.direction(dir: FlexDirection)` - Row or Column
- `.gap(size: &str)` - Space between children
- `.align(align: Align)` - Cross-axis alignment
- `.justify(justify: Justify)` - Main-axis alignment
- `.child(node: VNode)` - Add child

**Enums:**
```windjammer
FlexDirection::Row     // Horizontal
FlexDirection::Column  // Vertical

Align::Start | Align::Center | Align::End

Justify::Start | Justify::Center | Justify::End | Justify::SpaceBetween
```

---

#### Grid

Grid layout container.

```windjammer
Grid::new()
    .columns(3)
    .gap("16px")
    .child(Card::new().title("Card 1"))
    .child(Card::new().title("Card 2"))
    .child(Card::new().title("Card 3"))
```

**Methods:**
- `.columns(n: usize)` - Number of columns
- `.gap(size: &str)` - Space between cells
- `.child(node: VNode)` - Add grid item

---

### Text Components

#### Text

Basic text display with styling.

```windjammer
Text::new("Hello, World!")
    .size(TextSize::Large)
    .bold()
    .color(Color::rgb(100, 150, 200))
```

**Methods:**
- `.size(size: TextSize)` - Set font size
- `.bold()` - Make text bold
- `.italic()` - Make text italic
- `.color(color: Color)` - Set text color

**TextSize Enum:**
```windjammer
TextSize::Small    // 12px
TextSize::Medium   // 14px
TextSize::Large    // 18px
TextSize::XLarge   // 24px
```

---

### Input Components

#### Button

Clickable button with variants.

```windjammer
Button::new("Click me")
    .variant(ButtonVariant::Primary)
    .size(ButtonSize::Medium)
    .disabled(false)
    .on_click(move || {
        println!("Button clicked!")
    })
```

**Methods:**
- `.variant(v: ButtonVariant)` - Visual style
- `.size(s: ButtonSize)` - Button size
- `.disabled(bool)` - Disable button
- `.on_click(handler: Fn())` - Click handler

**ButtonVariant:**
```windjammer
ButtonVariant::Primary    // Blue, prominent
ButtonVariant::Secondary  // Gray, subtle
ButtonVariant::Danger     // Red, destructive
```

**ButtonSize:**
```windjammer
ButtonSize::Small    // 28px height
ButtonSize::Medium   // 32px height
ButtonSize::Large    // 40px height
```

---

#### Input

Text input field.

```windjammer
let text = Signal::new("".to_string())
let text_ref = text.clone()

Input::new()
    .value(text.get())
    .placeholder("Enter text...")
    .on_change(move |new_value| {
        text_ref.set(new_value)
    })
```

**Methods:**
- `.value(v: String)` - Current value
- `.placeholder(p: String)` - Placeholder text
- `.on_change(handler: Fn(String))` - Change handler

**Note:** Event handlers with parameters are limited in current version. Desktop renderer handles this directly.

---

#### Checkbox

Boolean toggle control.

```windjammer
let checked = Signal::new(false)

Checkbox::new()
    .checked(checked.get())
    .label("Accept terms")
    .on_change(move |new_value| {
        checked.set(new_value)
    })
```

---

#### Select

Dropdown selection.

```windjammer
let selected = Signal::new("option1".to_string())

Select::new()
    .value(selected.get())
    .option("option1", "Option 1")
    .option("option2", "Option 2")
    .option("option3", "Option 3")
    .on_change(move |new_value| {
        selected.set(new_value)
    })
```

---

### Display Components

#### Alert

Notification banner.

```windjammer
Alert::new()
    .variant(AlertVariant::Success)
    .message("Operation completed successfully!")
```

**AlertVariant:**
```windjammer
AlertVariant::Info       // Blue
AlertVariant::Success    // Green
AlertVariant::Warning    // Yellow
AlertVariant::Error      // Red
```

---

#### Card

Container with border and shadow.

```windjammer
Card::new()
    .title("Card Title")
    .child(Text::new("Card content here"))
```

---

#### Progress

Progress bar or spinner.

```windjammer
Progress::new()
    .variant(ProgressVariant::Bar)
    .value(0.7)  // 70%

Progress::new()
    .variant(ProgressVariant::Spinner)
```

---

### Advanced Components

#### ScrollArea

Scrollable container.

```windjammer
ScrollArea::new()
    .height("300px")
    .direction(ScrollDirection::Vertical)
    .child(
        // Long content here
    )
```

---

#### CollapsibleSection

Expandable/collapsible section.

```windjammer
CollapsibleSection::new()
    .title("Advanced Options")
    .expanded(false)
    .child(
        // Hidden content
    )
```

---

#### ColorPicker

Color selection widget.

```windjammer
let color = Signal::new(Color::rgb(255, 0, 0))

ColorPicker::new()
    .color(color.get())
    .on_change(move |new_color| {
        color.set(new_color)
    })
```

---

## App & Rendering

### App

Main application runtime.

#### Static App (No Reactivity)

```windjammer
let ui = build_static_ui()
App::new("My App", ui.to_vnode()).run()
```

#### Reactive App (With Signals)

```windjammer
let count = Signal::new(0)
let count_ref = count.clone()

App::new_reactive("My App", move || {
    build_ui_with_count(&count_ref)
}).run()
```

**Methods:**
- `App::new(title: &str, root: VNode)` - Static UI
- `App::new_reactive(title: &str, render_fn: Fn() -> VNode)` - Reactive UI
- `.run()` - Start the application

---

### VNode

Virtual DOM node representation.

```windjammer
// Text node
VNode::Text("Hello".to_string())

// Element node
VNode::Element {
    tag: "div".to_string(),
    attrs: vec![
        ("class".to_string(), VAttr::Static("container".to_string())),
    ],
    children: vec![
        VNode::Text("Content".to_string()),
    ],
}
```

Most components provide `.to_vnode()` method for conversion.

---

## Events

### Event Handlers

#### Basic Click Handler

```windjammer
Button::new("Click").on_click(move || {
    println!("Clicked!")
})
```

#### With Signal Updates

```windjammer
let count = Signal::new(0)
let count_ref = count.clone()

Button::new("Increment").on_click(move || {
    count_ref.set(count_ref.get() + 1)
})
```

### Limitations

**Current Version:**
- Event handlers must be `Fn()` (no parameters)
- Input change handlers work through desktop renderer
- Web events use wasm_bindgen

**Future:**
- Generic event system with type-safe parameters
- More event types (keyboard, mouse, focus, etc.)

---

## Styling

### Colors

```windjammer
Color::rgb(255, 0, 0)        // Red
Color::hex("#FF0000")        // Red from hex
Color::white()               // Predefined color
Color::black()
Color::gray(128)             // Gray with intensity
```

### Sizes

Use string values with units:
```windjammer
"16px"      // Pixels
"2em"       // Em units
"50%"       // Percentage
"auto"      // Automatic
```

---

## Examples

### Counter App

```windjammer
use std::ui::*

@export
fn start() {
    let count = Signal::new(0)
    let count_inc = count.clone()
    let count_display = count.clone()
    
    let ui = Container::new()
        .child(
            Flex::new()
                .direction(FlexDirection::Column)
                .gap("16px")
                .child(
                    Text::new(format!("Count: {}", count_display.get()))
                        .size(TextSize::XLarge)
                        .bold()
                )
                .child(
                    Button::new("Increment")
                        .variant(ButtonVariant::Primary)
                        .on_click(move || {
                            count_inc.set(count_inc.get() + 1)
                        })
                )
        )
    
    App::new("Counter", ui.to_vnode()).run()
}
```

### Form with Validation

```windjammer
use std::ui::*

@export
fn start() {
    let email = Signal::new("".to_string())
    let email_ref = email.clone()
    let email_display = email.clone()
    
    let is_valid = Computed::new(move || {
        let e = email_display.get()
        e.contains("@") && e.contains(".")
    })
    
    let ui = Container::new()
        .child(
            Flex::new()
                .direction(FlexDirection::Column)
                .gap("12px")
                .child(Text::new("Email:"))
                .child(
                    Input::new()
                        .placeholder("user@example.com")
                        .value(email.get())
                )
                .child(
                    if is_valid.get() {
                        Alert::new()
                            .variant(AlertVariant::Success)
                            .message("Valid email!")
                    } else {
                        Alert::new()
                            .variant(AlertVariant::Error)
                            .message("Invalid email")
                    }
                )
        )
    
    App::new("Form", ui.to_vnode()).run()
}
```

---

## Best Practices

### 1. Clone Signals for Closures

```windjammer
let count = Signal::new(0)
let count_ref = count.clone()  // Clone before move

Button::new("Click").on_click(move || {
    count_ref.set(count_ref.get() + 1)  // Use cloned reference
})
```

### 2. Use Computed for Derived Values

```windjammer
let items = Signal::new(vec![1, 2, 3])
let total = Computed::new(move || {
    items.get().iter().sum()
})
```

### 3. Builder Pattern for Components

```windjammer
// Good: Chainable methods
Button::new("Text")
    .variant(ButtonVariant::Primary)
    .size(ButtonSize::Large)
    .on_click(handler)

// Avoid: Positional arguments (not supported yet)
Button::new(text: "Text", variant: Primary)  // ❌ Parser error
```

### 4. Reactive Apps for Dynamic UIs

```windjammer
// Use App::new_reactive when Signals change UI
App::new_reactive("My App", move || {
    build_ui()  // Called every frame
}).run()
```

---

## Platform Support

| Platform | Status | Backend |
|----------|--------|---------|
| Desktop (Windows/Mac/Linux) | ✅ Stable | egui + eframe |
| Web (WASM) | ⚠️ Experimental | web-sys + wasm-bindgen |
| Mobile (iOS/Android) | 🚧 Planned | Native renderers |

---

## Version History

### v0.34.0 (Current)
- Signal-based reactivity
- Desktop renderer (egui)
- 30+ components
- Event handling
- Auto-dependency detection

### v0.35.0 (Planned)
- WASM renderer improvements
- Virtual DOM diffing
- More widgets
- Mobile support

---

## Getting Help

- **GitHub:** [windjammer/issues](https://github.com/jeffreyfriedman/windjammer)
- **Docs:** `windjammer-ui/docs/`
- **Examples:** `windjammer-ui/examples_wj/`

---

**Happy Building!** 🚀


