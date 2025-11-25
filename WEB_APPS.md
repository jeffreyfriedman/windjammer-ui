# 🌐 Windjammer Web Applications

Complete web applications written in **pure Windjammer** using `windjammer-ui` components.

---

## 📦 Available Web Apps

### 1. **Simple Counter App** ✅ WORKING

A standalone counter app demonstrating Windjammer's ability to generate complete HTML pages.

**File:** `examples_wj/simple_web_app.wj`

**To Build & Run:**
```bash
# Build the Windjammer app
wj build windjammer-ui/examples_wj/simple_web_app.wj -o build_simple_app

# Run and generate HTML
cd build_simple_app && cargo run > ../windjammer-ui/simple_app.html

# Open in browser
open ../windjammer-ui/simple_app.html
```

**Features:**
- ✅ Written in 100% pure Windjammer
- ✅ Generates complete HTML with embedded CSS & JavaScript
- ✅ Interactive counter with +/- buttons
- ✅ Beautiful gradient UI
- ✅ No external dependencies
- ✅ Type-safe compilation

---

### 2. **Todo Web App** 🚧 ADVANCED

A complete todo list application with localStorage persistence (requires windjammer-ui crate).

**File:** `examples_wj/todo_web_app.wj`

**Features:**
- Uses all major windjammer-ui components
- LocalStorage for data persistence
- Add, complete, and delete todos
- Statistics dashboard
- Sample data generation

**Status:** Requires windjammer-ui crate to be properly packaged. Will work once resolved.

---

### 3. **HTTP Server Example** 🔮 FUTURE

Demonstrates serving Windjammer UI apps with the built-in HTTP server.

**File:** `examples_wj/http_server_example.wj`

**Features:**
- Multi-page web application
- RESTful API endpoints
- Dashboard with statistics
- Chat interface
- Real-time server

**Status:** Awaiting `std::net::Server` implementation.

**Future Usage:**
```bash
wj run examples_wj/http_server_example.wj
# Visit: http://localhost:3000
```

---

## 🚀 Quick Start

### Option 1: Simple Web App (Recommended)

The simplest way to test Windjammer web apps:

```bash
# Navigate to Windjammer directory
cd /Users/jeffreyfriedman/src/wj

# Build the simple app
wj build windjammer-ui/examples_wj/simple_web_app.wj -o build_simple_app

# Generate HTML
cd build_simple_app && cargo run > ../windjammer-ui/simple_app.html

# Open in browser
open ../windjammer-ui/simple_app.html
```

You should see a beautiful counter app with a purple gradient background!

---

## 🏗️ How It Works

### Architecture

```
┌─────────────────────────────┐
│  simple_web_app.wj          │  Pure Windjammer source
│  (Pure Windjammer Code)     │
└──────────────┬──────────────┘
               │
               │ wj build (transpile)
               ▼
┌─────────────────────────────┐
│  simple_web_app.rs          │  Generated Rust
│  (Rust Code)                │
└──────────────┬──────────────┘
               │
               │ cargo run
               ▼
┌─────────────────────────────┐
│  simple_app.html            │  Complete HTML Page
│  (HTML + CSS + JS)          │
└─────────────────────────────┘
               │
               │ open
               ▼
         🌐 Browser!
```

### Code Structure

```windjammer
// simple_web_app.wj
fn main() {
    let html = build_html_page()
    println!("{}", html)  // Output complete HTML
}

fn build_html_page() -> string {
    format!(
        "<!DOCTYPE html>...{}</html>",
        build_ui()  // Generate UI components
    )
}

fn build_ui() -> string {
    // Pure Windjammer code generating HTML
    String::from("<div class='container'>...</div>")
}
```

---

## 💡 Key Concepts

### 1. **Pure Windjammer**

All web apps are written in `.wj` files:
- No manual HTML writing
- No manual CSS writing  
- No manual JavaScript writing (optional for interactivity)
- 100% type-safe Windjammer code

### 2. **Compile-Time HTML Generation**

Windjammer compiles to Rust, which generates the HTML at runtime:

```windjammer
// Windjammer code
fn get_title() -> string {
    String::from("My App")
}

// Generates HTML
format!("<h1>{}</h1>", get_title())
```

### 3. **Component-Based**

Using `windjammer-ui` components:

```windjammer
use windjammer_ui::*

Button::new("Click Me")
    .variant(ButtonVariant::Primary)
    .render()

// Outputs: <button class='wj-button wj-button-primary'>Click Me</button>
```

---

## 🔮 Future: HTTP Server

Once `std::net::Server` is implemented, you'll be able to serve apps directly:

```windjammer
use std::net::{Server, ServerRequest, ServerResponse}
use windjammer_ui::*

fn main() {
    let server = Server::new("127.0.0.1", 3000)
    server.serve(handle_request)
}

fn handle_request(req: ServerRequest) -> ServerResponse {
    match req.path.as_str() {
        "/" => ServerResponse::html(build_home_page()),
        "/api/data" => ServerResponse::json("{\"status\": \"ok\"}"),
        _ => ServerResponse::error(404, "Not found"),
    }
}

fn build_home_page() -> string {
    Container::new()
        .child(Text::new("Hello from Windjammer!").render())
        .render()
}
```

Then simply:
```bash
wj run my_server.wj
# Server running at http://localhost:3000
```

---

## 📊 Comparison

| Approach | Traditional | Windjammer |
|----------|-------------|------------|
| **Languages** | HTML + CSS + JS | Pure Windjammer |
| **Type Safety** | None | 100% compile-time |
| **Code Generation** | Manual | Automatic |
| **Runtime Errors** | Common | Impossible (compile-time caught) |
| **Build Tool** | Webpack/Vite | `wj build` |
| **Dependencies** | npm (1000s of packages) | None or minimal |
| **Learning Curve** | 3 languages | 1 language |

---

## 🎯 Next Steps

### Immediate (Works Now)

1. ✅ Build `simple_web_app.wj`
2. ✅ Test in browser
3. ✅ Modify the Windjammer code
4. ✅ Rebuild and see changes

### Short-Term (Weeks)

1. 🔨 Package windjammer-ui as proper crate
2. 🔨 Enable `todo_web_app.wj` to build
3. 🔨 Create more example apps

### Long-Term (Months)

1. 🔮 Implement `std::net::Server`
2. 🔮 Enable `http_server_example.wj`
3. 🔮 Add WebSocket support
4. 🔮 Add database integration
5. 🔮 Full-stack Windjammer apps!

---

## 🐛 Troubleshooting

### Issue: "Cannot find windjammer-ui crate"

**Solution:** Use `simple_web_app.wj` which has no external dependencies.

### Issue: "Build fails"

**Solution:**
```bash
# Clean and rebuild
rm -rf build_simple_app
wj build windjammer-ui/examples_wj/simple_web_app.wj -o build_simple_app
```

### Issue: "HTML file is empty"

**Solution:** Check that you ran `cargo run > output.html` (with redirect)

---

## 📚 Resources

- **Examples:** `windjammer-ui/examples_wj/`
- **Gallery:** `windjammer-ui/examples/gallery.html`
- **Component Docs:** `windjammer-ui/README.md`
- **API Reference:** `windjammer-ui/EXAMPLE_APPS.md`

---

**🎉 You now have working web apps written in pure Windjammer!**

_Last Updated: November 23, 2025_

