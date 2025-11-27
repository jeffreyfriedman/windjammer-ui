# Windjammer UI v0.3.0 - API Design

## Philosophy: Type-Safe, Composable, Zero Raw HTML/CSS

### Problems with v0.2.x API

```windjammer
// Old API - too much raw HTML/CSS
Row::new()
    .child("<div>Item 1</div>")
    .child("<div style='color: red;'>Item 2</div>")
    .render()
```

**Issues:**
- ❌ No type safety for children
- ❌ Raw HTML strings are error-prone
- ❌ Raw CSS strings are hard to maintain
- ❌ No IDE autocomplete for HTML/CSS
- ❌ Easy to make syntax errors

### New v0.3.0 API

```windjammer
// New API - fully typed and composable
Row::new()
    .child(
        Div::new()
            .text("Item 1")
            .render()
    )
    .child(
        Div::new()
            .text("Item 2")
            .style(
                Style::new()
                    .color("red")
                    .to_string()
            )
            .render()
    )
    .gap("16px")
    .align(RowAlign::Center)
    .render()
```

**Benefits:**
- ✅ Fully type-safe
- ✅ Composable components
- ✅ IDE autocomplete everywhere
- ✅ Compile-time error checking
- ✅ No raw HTML/CSS strings
- ✅ Builder pattern for everything

## New Components

### 1. HTML Elements (`html_elements.wj`)

Type-safe wrappers for common HTML elements:

```windjammer
// Div
Div::new()
    .class("container")
    .id("main")
    .child(H1::new("Title").render())
    .child(P::new().text("Paragraph").render())
    .render()

// Span
Span::new()
    .text("Inline text")
    .class("highlight")
    .render()

// Headings
H1::new("Main Title").class("title").render()
H2::new("Subtitle").style("color: blue;").render()
H3::new("Section").render()

// Paragraph
P::new()
    .text("First sentence. ")
    .child(Span::new().text("Bold part").render())
    .render()
```

### 2. Style Builder (`style.wj`)

Type-safe CSS property builder:

```windjammer
Style::new()
    // Layout
    .display("flex")
    .flex_direction("row")
    .gap("16px")
    .align_items("center")
    .justify_content("space-between")
    
    // Spacing
    .padding("20px")
    .margin("10px")
    
    // Sizing
    .width("100%")
    .height("auto")
    .max_width("1200px")
    
    // Colors
    .color("#333")
    .background_color("#f5f5f5")
    .border_color("#ddd")
    
    // Border
    .border("1px solid #ddd")
    .border_radius("8px")
    
    // Typography
    .font_size("16px")
    .font_weight("bold")
    .text_align("center")
    
    // Position
    .position("relative")
    .top("10px")
    
    // Effects
    .opacity("0.9")
    .box_shadow("0 2px 4px rgba(0,0,0,0.1)")
    .cursor("pointer")
    
    // Overflow
    .overflow("auto")
    .overflow_x("hidden")
    
    .to_string()
```

### 3. Layout Components

#### Row (Horizontal Layout)

```windjammer
Row::new()
    .child(Button::new("Button 1").render())
    .child(Button::new("Button 2").render())
    .child(Button::new("Button 3").render())
    .gap("16px")
    .align(RowAlign::Center)
    .justify(RowJustify::SpaceBetween)
    .wrap(true)
    .render()
```

**Alignment Options:**
- `RowAlign::Start` - Align to top
- `RowAlign::Center` - Center vertically
- `RowAlign::End` - Align to bottom
- `RowAlign::Stretch` - Stretch to fill

**Justify Options:**
- `RowJustify::Start` - Pack to start
- `RowJustify::Center` - Center horizontally
- `RowJustify::End` - Pack to end
- `RowJustify::SpaceBetween` - Space between items
- `RowJustify::SpaceAround` - Space around items
- `RowJustify::SpaceEvenly` - Even spacing

#### Column (Vertical Layout)

```windjammer
Column::new()
    .child(H1::new("Title").render())
    .child(P::new().text("Description").render())
    .child(Button::new("Action").render())
    .gap("12px")
    .align(ColumnAlign::Start)
    .justify(ColumnJustify::Start)
    .render()
```

## Complete Example: Card Component

```windjammer
use windjammer_ui::components::*

pub fn create_user_card(name: string, email: string) -> string {
    Column::new()
        .child(
            Row::new()
                .child(
                    Div::new()
                        .class("avatar")
                        .style(
                            Style::new()
                                .width("48px")
                                .height("48px")
                                .border_radius("50%")
                                .background_color("#3b82f6")
                                .to_string()
                        )
                        .render()
                )
                .child(
                    Column::new()
                        .child(H3::new(name).render())
                        .child(
                            P::new()
                                .text(email)
                                .style(
                                    Style::new()
                                        .color("#666")
                                        .font_size("14px")
                                        .to_string()
                                )
                                .render()
                        )
                        .gap("4px")
                        .render()
                )
                .gap("16px")
                .align(RowAlign::Center)
                .render()
        )
        .child(
            Row::new()
                .child(Button::new("View Profile").variant(ButtonVariant::Primary).render())
                .child(Button::new("Message").variant(ButtonVariant::Secondary).render())
                .gap("8px")
                .justify(RowJustify::End)
                .render()
        )
        .gap("20px")
        .style(
            Style::new()
                .padding("24px")
                .border_radius("12px")
                .border("1px solid #e5e7eb")
                .background_color("white")
                .box_shadow("0 1px 3px rgba(0,0,0,0.1)")
                .to_string()
        )
        .render()
}
```

## Migration Guide

### Before (v0.2.x)

```windjammer
Container::new()
    .child("<div class='item'>Content</div>")
    .render()
```

### After (v0.3.0)

```windjammer
Container::new()
    .child(
        Div::new()
            .class("item")
            .text("Content")
            .render()
    )
    .render()
```

## Benefits Summary

1. **Type Safety**: Catch errors at compile time
2. **Composability**: Mix and match components freely
3. **Maintainability**: Easier to refactor and update
4. **IDE Support**: Full autocomplete and documentation
5. **Consistency**: Uniform API across all components
6. **Zero Raw Strings**: No HTML/CSS syntax errors
7. **Better DX**: More pleasant developer experience

## Future Enhancements

- CSS classes builder (similar to Tailwind)
- Animation/transition builders
- Responsive design helpers
- Theme system integration
- Component variants system

---

**This is a major step forward in making Windjammer UI the best UI framework for Windjammer!** 🚀

