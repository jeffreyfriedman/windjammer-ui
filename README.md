# Windjammer UI

**Cross-platform UI framework for building desktop and web applications with Windjammer**

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)

---

## Features

- 🎯 **Reactive State** - Signal-based reactivity like Solid.js
- 🏗️ **Component Model** - Clean, composable UI components  
- 🖥️ **Cross-Platform** - Desktop (native) and Web (WASM)
- 🚀 **Performance** - Compiled to native Rust
- 📦 **Zero Config** - Automatic dependency management
- 🎨 **30+ Components** - Rich widget library
- 📚 **Well Documented** - Comprehensive API reference

---

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/jeffreyfriedman/windjammer
cd windjammer-ui
```

### Hello World Example

Create `hello.wj`:

```windjammer
use std::ui::*

@export
fn start() {
    let ui = Container::new()
        .child(Text::new("Hello, Windjammer!"))
    
    App::new("Hello World", ui.to_vnode()).run()
}

fn main() {
    start()
}
```

Build and run:

```bash
wj build hello.wj --output build_hello
cd build_hello && cargo run
```

### Counter Example (with Reactivity)

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

fn main() {
    start()
}
```

---

## Platform Support

| Platform | Status | Backend | Notes |
|----------|--------|---------|-------|
| **Desktop** (Windows/Mac/Linux) | ✅ **Stable** | egui + eframe | Production ready |
| **Web** (WASM) | ⚠️ **Experimental** | web-sys | Limited features (v0.2.0) |
| **Mobile** (iOS/Android) | 🚧 **Planned** | Native renderers | Future release |

### Desktop Requirements
- Rust 1.90+
- Native display server (X11/Wayland/Cocoa/Windows)

### Web Requirements
- `wasm32-unknown-unknown` target
- `wasm-bindgen`
- Modern browser with WebAssembly support

---

## Components

### Layout
- **Container** - Max-width wrapper with centering
- **Flex** - Flexbox layout (row/column)
- **Grid** - CSS Grid layout
- **ScrollArea** - Scrollable container
- **Spacer** - Flexible spacing
- **Divider** - Visual separator

### Text
- **Text** - Styled text display

### Input
- **Button** - Clickable button with variants
- **Input** - Text input field
- **Checkbox** - Boolean toggle
- **Radio** - Radio button group
- **Select** - Dropdown selection
- **Slider** - Numeric slider

### Display
- **Alert** - Notification banner
- **Card** - Container with border/shadow
- **Badge** - Status indicator
- **Progress** - Progress bar or spinner
- **Spinner** - Loading indicator

### Advanced
- **CollapsibleSection** - Expandable panel
- **ColorPicker** - Color selection widget
- **CodeEditor** - Syntax-highlighted code editor
- **Tabs** - Tab navigation
- **Dialog** - Modal dialog
- **Tooltip** - Hover tooltip
- **FileTree** - Hierarchical file browser

---

## Reactivity System

### Signals

Reactive values that notify subscribers on change:

```windjammer
let count = Signal::new(0)
count.set(1)  // Updates and notifies
let value = count.get()  // Reads and tracks dependency
```

### Computed

Derived values that auto-update:

```windjammer
let count = Signal::new(0)
let doubled = Computed::new(move || count.get() * 2)
```

### Effects

Side effects that run when dependencies change:

```windjammer
Effect::new(move || {
    println!("Count: {}", count.get())
})
```

---

## Documentation

- **[API Reference](docs/API_REFERENCE.md)** - Complete API documentation
- **[Style Guide](docs/STYLE_GUIDE.md)** - Best practices and patterns
- **[Examples](examples_wj/)** - Working code examples

---

## Examples

See the `examples_wj/` directory for more examples:

- `counter.wj` - Simple counter with reactivity ✅
- `todo_app.wj` - Todo list application
- `contact_form.wj` - Form with validation
- `dashboard.wj` - Dashboard UI
- And more...

---

## Building from Source

```bash
# Build the library
cargo build --release --features desktop

# Run tests
cargo test

# Build for WASM
cargo build --target wasm32-unknown-unknown --release
```

---

## Architecture

```
windjammer-ui/
├── src/
│   ├── lib.rs              # Library root
│   ├── app.rs              # Application runtime
│   ├── reactivity.rs       # Signal/Computed/Effect
│   ├── desktop_renderer.rs # egui-based renderer
│   ├── components/         # Widget library
│   │   ├── button.rs
│   │   ├── text.rs
│   │   └── ...
│   ├── simple_vnode.rs     # Virtual DOM
│   └── ...
├── docs/                   # Documentation
│   ├── API_REFERENCE.md
│   └── STYLE_GUIDE.md
└── examples_wj/            # Windjammer examples
    ├── counter.wj
    ├── todo_app.wj
    └── ...
```

---

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

---

## Versioning

We follow [Semantic Versioning](https://semver.org/):

- **v0.34.0** (Current) - Desktop renderer, Signal reactivity
- **v0.35.0** (Planned) - WASM improvements, more widgets
- **v1.0.0** (Future) - Stable API, mobile support

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

## Acknowledgments

- **egui** - Immediate mode GUI library for desktop rendering
- **Solid.js** - Inspiration for reactive system
- **Vue 3** - Reactivity patterns
- **Leptos** - Rust UI framework design

---

**Built with ❤️ by the Windjammer team**

