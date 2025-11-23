# Windjammer UI Language Specification v0.1.0

**Author:** Windjammer Team  
**Status:** Draft  
**Last Updated:** November 22, 2025

---

## Table of Contents

1. [Philosophy](#philosophy)
2. [Core Concepts](#core-concepts)
3. [Component Syntax](#component-syntax)
4. [Widget Catalog](#widget-catalog)
5. [State Management](#state-management)
6. [Event Handling](#event-handling)
7. [Layout System](#layout-system)
8. [Styling](#styling)
9. [Compilation Targets](#compilation-targets)
10. [Examples](#examples)

---

## Philosophy

### Zero Backend Leakage

**Developers write pure Windjammer UI code:**
```windjammer
// ✅ Good: Pure Windjammer
@component
fn Counter() -> UI {
    let count = signal(0)
    
    VStack {
        Text("Count: {count}")
        Button("Increment", on_click: || count += 1)
    }
}
```

**NOT Rust or JavaScript:**
```rust
// ❌ Bad: Backend-specific code
use egui::Ui;
fn counter(ui: &mut Ui) { ... }
```

### Write Once, Run Everywhere

**Same source code compiles to multiple targets:**
```bash
# Desktop (native, fast)
wj build counter.wj --target desktop --backend egui
# → Rust + egui binary

# Browser (WASM, portable)
wj build counter.wj --target web
# → WASM + JavaScript glue
```

### Declarative & Reactive

**Components describe WHAT, not HOW:**
```windjammer
// Declarative: describe the UI
Text("Hello")

// Reactive: UI updates automatically
let name = signal("World")
Text("Hello {name}")  // Updates when name changes
```

---

## Core Concepts

### 1. Components

**Components are reusable UI functions:**

```windjammer
@component
fn Greeting(name: String) -> UI {
    Text("Hello, {name}!")
}

// Usage:
Greeting("Alice")
```

**Components can have state:**

```windjammer
@component
fn Counter(initial: i32) -> UI {
    let count = signal(initial)
    
    VStack {
        Text("Count: {count}")
        Button("Increment", on_click: || count += 1)
    }
}
```

### 2. Signals (Reactive State)

**Signals are reactive values that trigger re-renders:**

```windjammer
let count = signal(0)        // Create signal
let value = count.get()      // Read value
count.set(10)                // Write value
count += 1                   // Shorthand for count.set(count.get() + 1)
```

### 3. Props

**Components accept typed props:**

```windjammer
@component
fn UserCard(
    name: String,
    email: String,
    avatar_url: String,
    on_click: fn() -> void
) -> UI {
    HStack {
        Image(url: avatar_url, width: 64, height: 64)
        VStack {
            Text(name, style: "heading")
            Text(email, style: "subtitle")
        }
        Button("View Profile", on_click: on_click)
    }
}
```

### 4. Children

**Components can accept child elements:**

```windjammer
@component
fn Card(children: UI) -> UI {
    VStack(padding: 16, border: 1, border_radius: 8) {
        children
    }
}

// Usage:
Card {
    Text("Title")
    Text("Description")
}
```

---

## Component Syntax

### Basic Component

```windjammer
@component
fn MyComponent() -> UI {
    Text("Hello, World!")
}
```

### Component with Props

```windjammer
@component
fn Greeting(name: String, age: i32) -> UI {
    Text("Hello, {name}! You are {age} years old.")
}

// Usage:
Greeting(name: "Alice", age: 30)
// OR shorthand:
Greeting("Alice", 30)
```

### Component with State

```windjammer
@component
fn ToggleButton() -> UI {
    let is_on = signal(false)
    
    Button(
        text: if is_on { "ON" } else { "OFF" },
        on_click: || is_on = !is_on
    )
}
```

### Component with Children

```windjammer
@component
fn Panel(title: String, children: UI) -> UI {
    VStack {
        Text(title, style: "heading")
        Separator()
        children
    }
}

// Usage:
Panel(title: "Settings") {
    Checkbox("Enable notifications")
    Checkbox("Dark mode")
}
```

### Computed Values (Memos)

```windjammer
@component
fn ExpensiveComponent() -> UI {
    let input = signal(10)
    
    // Memo: only recomputes when input changes
    let doubled = memo(|| input * 2)
    let factorial = memo(|| compute_factorial(input))
    
    VStack {
        TextInput(value: input)
        Text("Doubled: {doubled}")
        Text("Factorial: {factorial}")
    }
}
```

### Effects (Side Effects)

```windjammer
@component
fn Logger() -> UI {
    let count = signal(0)
    
    // Effect: runs when count changes
    effect(|| {
        print("Count changed to: {count}")
    })
    
    Button("Increment", on_click: || count += 1)
}
```

---

## Widget Catalog

### Text & Input

#### Text
```windjammer
Text("Hello, World!")
Text("Bold text", weight: "bold")
Text("Large text", size: 24)
Text("Colored text", color: Color.rgb(255, 0, 0))
```

#### TextInput
```windjammer
let text = signal("")
TextInput(value: text, placeholder: "Enter text...")
TextInput(value: text, password: true)  // Password field
TextInput(value: text, multiline: true, rows: 5)  // Text area
```

#### Label
```windjammer
Label("Username:", for: "username_input")
```

### Buttons

#### Button
```windjammer
Button("Click me", on_click: || print("Clicked!"))
Button("Primary", style: "primary")
Button("Disabled", disabled: true)
Button(icon: "save", text: "Save")
```

#### IconButton
```windjammer
IconButton(icon: "trash", on_click: || delete_item())
IconButton(icon: "settings", tooltip: "Settings")
```

### Selection

#### Checkbox
```windjammer
let checked = signal(false)
Checkbox(checked: checked, label: "Enable feature")
```

#### RadioButton
```windjammer
let selected = signal("option1")
RadioGroup(value: selected) {
    RadioButton("option1", label: "Option 1")
    RadioButton("option2", label: "Option 2")
    RadioButton("option3", label: "Option 3")
}
```

#### ComboBox
```windjammer
let selected = signal("apple")
ComboBox(value: selected, options: ["apple", "banana", "cherry"])
ComboBox(value: selected, options: fruits, searchable: true)
```

### Sliders

#### Slider
```windjammer
let value = signal(0.5)
Slider(value: value, min: 0.0, max: 1.0)
Slider(value: value, min: 0.0, max: 1.0, step: 0.1)
Slider(value: value, min: 0.0, max: 1.0, label: "Volume")
```

#### RangeSlider
```windjammer
let range = signal((0.2, 0.8))
RangeSlider(value: range, min: 0.0, max: 1.0)
```

### Colors

#### ColorPicker
```windjammer
let color = signal(Color.rgb(255, 0, 0))
ColorPicker(value: color)
ColorPicker(value: color, alpha: true)  // With alpha channel
```

### Hierarchy

#### TreeView
```windjammer
let tree = signal([
    TreeNode(id: "root", label: "Root", children: [
        TreeNode(id: "child1", label: "Child 1"),
        TreeNode(id: "child2", label: "Child 2"),
    ])
])
let selected = signal(null)

TreeView(
    data: tree,
    selected: selected,
    on_select: |node| print("Selected: {node.label}"),
    draggable: true,
    on_drop: |source, target| reorder(source, target)
)
```

### Images & Media

#### Image
```windjammer
Image(url: "avatar.png", width: 64, height: 64)
Image(url: avatar_url, fit: "cover", border_radius: 8)
```

#### Canvas
```windjammer
Canvas(width: 400, height: 300, on_draw: |ctx| {
    ctx.fill_rect(0, 0, 100, 100, Color.rgb(255, 0, 0))
    ctx.stroke_circle(200, 150, 50, Color.rgb(0, 0, 255))
})
```

---

## Layout System

### VStack (Vertical Stack)

```windjammer
VStack {
    Text("Item 1")
    Text("Item 2")
    Text("Item 3")
}

VStack(spacing: 16, padding: 8, align: "center") {
    Text("Centered with spacing")
}
```

### HStack (Horizontal Stack)

```windjammer
HStack {
    Button("Cancel")
    Button("OK")
}

HStack(spacing: 8, justify: "space-between") {
    Text("Left")
    Text("Right")
}
```

### Grid

```windjammer
Grid(columns: 3, gap: 8) {
    for item in items {
        Card { Text(item.name) }
    }
}
```

### ScrollArea

```windjammer
ScrollArea(height: 400) {
    VStack {
        for i in 0..100 {
            Text("Item {i}")
        }
    }
}
```

### SplitPanel

```windjammer
SplitPanel(direction: "horizontal", ratio: 0.3) {
    left: VStack { Text("Sidebar") }
    right: VStack { Text("Main content") }
}
```

### Spacer

```windjammer
HStack {
    Text("Left")
    Spacer()  // Pushes content apart
    Text("Right")
}
```

---

## State Management

### Local State (Signal)

```windjammer
@component
fn Counter() -> UI {
    let count = signal(0)  // Local to this component
    
    Button("Count: {count}", on_click: || count += 1)
}
```

### Shared State (Context)

```windjammer
// Define context
context ThemeContext {
    dark_mode: bool,
    primary_color: Color,
}

// Provide context
@component
fn App() -> UI {
    let theme = signal(ThemeContext {
        dark_mode: true,
        primary_color: Color.rgb(0, 122, 255)
    })
    
    ProvideContext(theme: theme) {
        MainLayout()
    }
}

// Consume context
@component
fn ThemedButton() -> UI {
    let theme = use_context::<ThemeContext>()
    
    Button(
        "Themed Button",
        background: theme.primary_color
    )
}
```

### Props Drilling (Explicit Passing)

```windjammer
@component
fn Parent() -> UI {
    let user = signal(User { name: "Alice", age: 30 })
    
    Child(user: user)
}

@component
fn Child(user: Signal<User>) -> UI {
    Text("User: {user.name}, {user.age}")
}
```

---

## Event Handling

### Click Events

```windjammer
Button("Click me", on_click: || print("Clicked!"))

Button("With data", on_click: |event| {
    print("Clicked at: {event.x}, {event.y}")
})
```

### Input Events

```windjammer
let text = signal("")
TextInput(
    value: text,
    on_change: |new_value| print("Changed to: {new_value}"),
    on_focus: || print("Focused"),
    on_blur: || print("Blurred")
)
```

### Keyboard Events

```windjammer
TextInput(
    value: text,
    on_key_down: |key| {
        if key == "Enter" {
            submit_form()
        }
    }
)
```

### Drag Events

```windjammer
Draggable(
    data: entity,
    on_drag_start: |e| print("Drag started"),
    on_drag_end: |e| print("Drag ended")
) {
    Card { Text(entity.name) }
}

DropTarget(
    on_drop: |data| {
        print("Dropped: {data}")
        add_to_list(data)
    }
) {
    VStack { Text("Drop zone") }
}
```

---

## Styling

### Inline Styles

```windjammer
Text("Styled text",
    color: Color.rgb(255, 0, 0),
    size: 24,
    weight: "bold",
    background: Color.rgb(240, 240, 240),
    padding: 8,
    border_radius: 4
)
```

### Style Presets

```windjammer
Text("Heading", style: "heading")
Text("Subtitle", style: "subtitle")
Button("Primary", style: "primary")
Button("Secondary", style: "secondary")
```

### Themes

```windjammer
// Define theme
theme DarkTheme {
    background: Color.rgb(30, 30, 30),
    foreground: Color.rgb(255, 255, 255),
    primary: Color.rgb(0, 122, 255),
    secondary: Color.rgb(88, 86, 214),
}

// Apply theme
@component
fn App() -> UI {
    ApplyTheme(DarkTheme) {
        MainLayout()
    }
}
```

### Custom Styles

```windjammer
style MyButton {
    background: Color.rgb(0, 122, 255),
    color: Color.rgb(255, 255, 255),
    padding: 12,
    border_radius: 8,
    hover: {
        background: Color.rgb(0, 100, 200)
    }
}

Button("Custom", style: MyButton)
```

---

## Compilation Targets

### Desktop (Native)

**Compiles to Rust + egui:**

```bash
wj build editor.wj --target desktop --backend egui
# → Generates Rust code using egui
# → Compiles to native binary
# → Fast, native performance
```

**Generated Rust (internal, developers never see this):**

```rust
// AUTO-GENERATED - DO NOT EDIT
use egui::{Context, Ui};

pub fn render_counter(ctx: &Context) {
    let count = use_signal(0);
    
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.label(format!("Count: {}", count.get()));
            if ui.button("Increment").clicked() {
                count.set(count.get() + 1);
            }
        });
    });
}
```

### Browser (WASM)

**Compiles to WASM + JavaScript glue:**

```bash
wj build editor.wj --target web
# → Generates WASM module
# → Generates JavaScript bindings
# → Runs in any modern browser
```

**Generated JavaScript (internal, developers never see this):**

```javascript
// AUTO-GENERATED - DO NOT EDIT
import init, { render_counter } from './editor.js';

async function run() {
    await init();
    
    const app = document.getElementById('app');
    render_counter(app);
}

run();
```

---

## Examples

### Example 1: Todo App

```windjammer
use windjammer_ui.*

struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

@component
fn TodoApp() -> UI {
    let todos = signal([] as Vec<Todo>)
    let input = signal("")
    let next_id = signal(0)
    
    let add_todo = || {
        if input.trim() != "" {
            todos.push(Todo {
                id: next_id,
                text: input.clone(),
                completed: false
            })
            next_id += 1
            input.set("")
        }
    }
    
    let toggle_todo = |id: i32| {
        for todo in todos.iter_mut() {
            if todo.id == id {
                todo.completed = !todo.completed
            }
        }
    }
    
    VStack(padding: 16, spacing: 8) {
        Text("Todo App", size: 24, weight: "bold")
        
        HStack(spacing: 8) {
            TextInput(
                value: input,
                placeholder: "What needs to be done?",
                on_key_down: |key| if key == "Enter" { add_todo() }
            )
            Button("Add", on_click: add_todo)
        }
        
        ScrollArea(height: 400) {
            VStack(spacing: 4) {
                for todo in todos {
                    TodoItem(todo: todo, on_toggle: toggle_todo)
                }
            }
        }
        
        Text("Total: {todos.len()}, Completed: {todos.filter(|t| t.completed).len()}")
    }
}

@component
fn TodoItem(todo: Todo, on_toggle: fn(i32) -> void) -> UI {
    HStack(spacing: 8, padding: 8, background: Color.rgb(240, 240, 240), border_radius: 4) {
        Checkbox(
            checked: todo.completed,
            on_change: || on_toggle(todo.id)
        )
        Text(
            todo.text,
            strikethrough: todo.completed,
            color: if todo.completed { Color.rgb(150, 150, 150) } else { Color.rgb(0, 0, 0) }
        )
    }
}
```

### Example 2: Material Editor

```windjammer
use windjammer_ui.*

struct Material {
    albedo: Color,
    metallic: f32,
    roughness: f32,
    emissive: Color,
}

@component
fn MaterialEditor() -> UI {
    let material = signal(Material {
        albedo: Color.rgb(255, 255, 255),
        metallic: 0.0,
        roughness: 0.5,
        emissive: Color.rgb(0, 0, 0),
    })
    
    VStack(padding: 16, spacing: 16) {
        Text("🎨 PBR Material Editor", size: 20, weight: "bold")
        
        Section(title: "Base Color") {
            ColorPicker(
                value: material.albedo,
                label: "Albedo"
            )
        }
        
        Section(title: "Properties") {
            Slider(
                value: material.metallic,
                min: 0.0,
                max: 1.0,
                label: "Metallic"
            )
            Slider(
                value: material.roughness,
                min: 0.0,
                max: 1.0,
                label: "Roughness"
            )
        }
        
        Section(title: "Emissive") {
            ColorPicker(
                value: material.emissive,
                label: "Emissive Color"
            )
        }
        
        HStack(spacing: 8, justify: "flex-end") {
            Button("Reset", on_click: || reset_material())
            Button("Save", style: "primary", on_click: || save_material(material))
        }
    }
}

@component
fn Section(title: String, children: UI) -> UI {
    VStack(spacing: 8, padding: 12, background: Color.rgb(250, 250, 250), border_radius: 8) {
        Text(title, size: 16, weight: "semibold")
        Separator()
        children
    }
}
```

### Example 3: Scene Hierarchy

```windjammer
use windjammer_ui.*

struct Entity {
    id: String,
    name: String,
    children: Vec<Entity>,
    visible: bool,
    locked: bool,
}

@component
fn SceneHierarchy() -> UI {
    let scene = signal(load_scene())
    let selected = signal(null as Option<String>)
    
    VStack(padding: 0, spacing: 0) {
        // Toolbar
        HStack(padding: 8, spacing: 4, background: Color.rgb(240, 240, 240)) {
            IconButton(icon: "add", tooltip: "Add entity", on_click: || add_entity())
            IconButton(icon: "trash", tooltip: "Delete", on_click: || delete_selected())
            Spacer()
            IconButton(icon: "refresh", tooltip: "Refresh", on_click: || refresh_scene())
        }
        
        Separator()
        
        // Tree view
        ScrollArea {
            TreeView(
                data: scene,
                selected: selected,
                draggable: true,
                on_select: |entity| selected.set(Some(entity.id)),
                on_drop: |source, target| reparent(source, target),
                render_node: |entity| {
                    HStack(spacing: 4) {
                        if !entity.visible {
                            Icon("eye-off", size: 16, color: Color.rgb(150, 150, 150))
                        }
                        if entity.locked {
                            Icon("lock", size: 16, color: Color.rgb(150, 150, 150))
                        }
                        Text(entity.name)
                    }
                },
                on_context_menu: |entity| show_entity_menu(entity)
            )
        }
    }
}
```

---

## Platform-Specific Features

### File Operations

```windjammer
// Cross-platform file picker
let file = file_picker(
    title: "Open File",
    filters: [("Images", ["png", "jpg", "jpeg"])],
    multiple: false
)

if let Some(path) = file {
    let content = read_file(path)
    print("Loaded: {content}")
}
```

### Clipboard

```windjammer
// Copy to clipboard
clipboard.set_text("Hello, clipboard!")

// Paste from clipboard
let text = clipboard.get_text()
```

### Native Menus (Desktop only)

```windjammer
Menu {
    MenuItem("File") {
        MenuItem("New", shortcut: "Ctrl+N", on_click: || new_file())
        MenuItem("Open", shortcut: "Ctrl+O", on_click: || open_file())
        Separator()
        MenuItem("Exit", on_click: || exit_app())
    }
    MenuItem("Edit") {
        MenuItem("Copy", shortcut: "Ctrl+C", on_click: || copy())
        MenuItem("Paste", shortcut: "Ctrl+V", on_click: || paste())
    }
}
```

---

## Conclusion

**Windjammer UI provides:**
- ✅ Pure Windjammer syntax (no Rust/JS)
- ✅ Cross-platform (desktop + browser)
- ✅ Declarative & reactive
- ✅ Type-safe
- ✅ Fast (native compilation)
- ✅ Rich widget library
- ✅ Beautiful by default

**Next Steps:**
1. Implement compiler support for `@component` decorator
2. Build widget library (Phase 2)
3. Test with real editor panels (Phase 3)
4. Iterate based on feedback (Phase 4)

**Target:** windjammer-ui v0.1.0 in ~4 months

