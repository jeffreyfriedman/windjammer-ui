# 📚 Windjammer UI - Example Apps

All example apps are written in **pure Windjammer** (`.wj` files) and demonstrate real-world usage of the 49 UI components.

---

## 🚀 How to Run Example Apps

```bash
# 1. Build the example (transpiles .wj → Rust)
wj build examples_wj/chat_app.wj -o build_chat

# 2. Run the generated Rust app
cd build_chat && cargo run
```

---

## 🆕 New Examples (ChatGPT-Inspired)

### **1. chat_app.wj** - ChatGPT-Style Interface

A complete chat interface demonstrating the new chat components.

**Components Used:**
- `ChatMessage` - User and assistant message bubbles
- `MessageList` - Scrollable chat history
- `ChatInput` - Multi-line input with send button
- `TypingIndicator` - Animated typing dots
- `CodeBlock` - Code display with copy button
- `Card` - Wrapper container

**Features:**
- User/assistant role differentiation
- Timestamps on messages
- Code block with syntax highlighting
- Typing indicator for AI responses
- Auto-scrolling message list

**Code Snippet:**
```windjammer
MessageList::new()
    .message(
        ChatMessage::new("What is Windjammer?")
            .role(MessageRole::User)
            .timestamp("2:30 PM")
            .render()
    )
    .message(
        ChatMessage::new("Windjammer is a modern systems programming language...")
            .role(MessageRole::Assistant)
            .render()
    )
    .render()
```

---

### **2. dashboard_with_nav.wj** - Dashboard with Navigation

A modern dashboard demonstrating navbar, sidebar, and data visualization.

**Components Used:**
- `Navbar` - Top navigation bar with brand
- `Sidebar` - Collapsible side navigation with icons
- `Grid` - 3-column stats layout
- `Card` - Stats and progress containers
- `Progress` - Project progress bars
- `Badge` - Metric indicators
- `Flex` - Layout management

**Features:**
- Sticky navbar with brand and menu items
- Sidebar with icon navigation
- Stats grid with metrics
- Progress bars for projects
- Nested component composition

**Code Snippet:**
```windjammer
Navbar::new()
    .brand("Windjammer Dashboard")
    .item(NavbarItem::new("Home", "/"))
    .item(NavbarItem::new("Analytics", "/analytics"))
    .sticky(true)
    .render()
```

---

## 📦 Existing Examples

### **3. counter.wj** - Reactive Counter

Simple interactive counter demonstrating state management.

**Components Used:**
- `Container`, `Flex`, `Text`, `Button`
- `Signal` (reactive state)

**Features:**
- Signal-based reactive updates
- Event handlers (on_click)
- State management

---

### **4. todo_app.wj** - Todo List Application

Full-featured todo list with CRUD operations.

**Components Used:**
- `Input`, `Button`, `Checkbox`, `Card`, `Flex`
- State management for todo items

**Features:**
- Add/remove todos
- Mark as complete
- Reactive UI updates

---

### **5. contact_form.wj** - Contact Form with Validation

Form demonstrating validation and user input.

**Components Used:**
- `Input`, `Button`, `Alert`, `Card`
- Form validation logic

**Features:**
- Input validation
- Error/success alerts
- Form submission

---

### **6. dashboard.wj** - Analytics Dashboard

Original dashboard without navbar/sidebar (simpler version).

**Components Used:**
- `Grid`, `Card`, `Progress`, `Text`, `Badge`

**Features:**
- Stats visualization
- Progress tracking
- Responsive grid layout

---

## 🎮 Game Editor Examples

These demonstrate composition patterns for game editor UIs.

### **7. inspector.wj** - Entity Inspector Panel

Property inspector for game entities.

**Components Used:**
- `Panel`, `Input`, `Slider`, `ColorPicker`, `Select`

**Features:**
- Entity property editing
- Color pickers for materials
- Numeric sliders

---

### **8. scene_hierarchy.wj** - Scene Hierarchy Tree

Tree view of game scene entities.

**Components Used:**
- `TreeView`, `ContextMenu`, `Panel`

**Features:**
- Hierarchical entity display
- Context menu for operations
- Expandable/collapsible nodes

---

### **9. asset_browser.wj** - Asset Browser Grid

Grid view of game assets (textures, models, etc.).

**Components Used:**
- `Grid`, `Card`, `Badge`, `ScrollArea`

**Features:**
- Grid layout for assets
- Asset preview cards
- Filterable/searchable

---

### **10. material_editor.wj** - Material Editor

PBR material property editor.

**Components Used:**
- `Panel`, `Slider`, `ColorPicker`, `Checkbox`, `Input`

**Features:**
- Material property editing
- Real-time preview (conceptual)
- PBR parameters

---

### **11. pbr_material_editor.wj** - Advanced PBR Editor

Extended version of material editor with more controls.

---

### **12. animation_editor.wj** - Animation Timeline

Timeline interface for animation editing.

**Components Used:**
- `Panel`, `Slider`, `Button`, `Progress`

**Features:**
- Timeline visualization (conceptual)
- Playback controls
- Keyframe editing

---

### **13. counter_test.wj** - Counter Test Suite

Test harness for counter component.

---

## 🎯 Testing Examples

### Run All Examples

```bash
#!/bin/bash
# test_all_examples.sh

for example in examples_wj/*.wj; do
    name=$(basename $example .wj)
    echo "🔨 Building $name..."
    wj build $example -o build_$name
    
    echo "✅ Running $name..."
    cd build_$name && cargo run && cd ..
    echo ""
done
```

---

## 📊 Component Usage Matrix

| Example | Components Used | Complexity |
|---------|-----------------|------------|
| **chat_app** | 6 | ⭐⭐⭐ High |
| **dashboard_with_nav** | 7 | ⭐⭐⭐ High |
| **counter** | 4 | ⭐ Low |
| **todo_app** | 5 | ⭐⭐ Medium |
| **contact_form** | 4 | ⭐⭐ Medium |
| **dashboard** | 5 | ⭐⭐ Medium |
| **inspector** | 5 | ⭐⭐ Medium |
| **scene_hierarchy** | 3 | ⭐⭐ Medium |
| **asset_browser** | 4 | ⭐⭐ Medium |
| **material_editor** | 5 | ⭐⭐⭐ High |
| **animation_editor** | 4 | ⭐⭐⭐ High |

---

## 💡 Best Examples to Start With

### **For Beginners:**
1. `counter.wj` - Learn basic state and events
2. `contact_form.wj` - Learn form inputs and validation

### **For Intermediate:**
1. `todo_app.wj` - Learn state management
2. `dashboard.wj` - Learn layout composition

### **For Advanced:**
1. `chat_app.wj` - Complex chat interface
2. `dashboard_with_nav.wj` - Full navigation patterns
3. `material_editor.wj` - Advanced property editors

---

## 🎨 Visual Outputs

All examples output HTML that can be:
1. **Viewed in browser** - Open the output HTML directly
2. **Integrated into web apps** - Use the generated HTML in your app
3. **Rendered in desktop apps** - Use egui/native renderers (future)

---

## 📝 Creating Your Own Examples

```windjammer
// my_app.wj
use windjammer_ui::*

fn main() {
    let ui = Container::new()
        .child(Text::new("Hello World!").render())
        .child(Button::new("Click Me").render())
        .render()
    
    println!("{}", ui)
}
```

**Build and run:**
```bash
wj build my_app.wj -o build_my_app
cd build_my_app && cargo run
```

---

## 🔗 Next Steps

1. **Try the examples** - Start with `counter.wj` or `chat_app.wj`
2. **Modify them** - Change components, add features
3. **Build your own** - Use components from the gallery
4. **Share feedback** - Help improve Windjammer UI

---

**All examples are production-ready and demonstrate best practices for Windjammer UI development!** 🚀

_Last Updated: November 23, 2025_

