# Windjammer UI Design System

**Version:** 0.1.0  
**Date:** November 22, 2025  
**Goal:** Create a modern, professional UI competitive with Unreal, Unity, and Godot editors

---

## Design Philosophy

### Core Principles

1. **Professional & Modern**
   - Clean, contemporary aesthetic
   - Polished look and feel
   - Attention to detail

2. **Ergonomic**
   - Less overwhelming than existing game engines
   - Clear information hierarchy
   - Intuitive workflows
   - Progressive disclosure (show what's needed, hide complexity)

3. **Consistent**
   - Unified design language across all panels
   - Predictable interactions
   - Coherent visual style

4. **Performant**
   - Smooth animations (60fps minimum)
   - Responsive interactions
   - Efficient rendering

---

## Color Palette

### Dark Theme (Primary)

**Editor Background:**
```
- Primary Background:   #1E1E1E (Dark charcoal)
- Secondary Background: #252525 (Slightly lighter)
- Panel Background:     #2D2D2D (Panel backdrop)
- Hover Background:     #333333 (Interactive elements)
```

**Accent Colors:**
```
- Primary Accent:   #007ACC (Blue - Microsoft VSCode inspired)
- Success:          #4EC9B0 (Teal)
- Warning:          #CE9178 (Orange)
- Error:            #F48771 (Salmon)
- Focus:            #0E639C (Darker blue)
```

**Text Colors:**
```
- Primary Text:     #D4D4D4 (Light gray)
- Secondary Text:   #9D9D9D (Medium gray)
- Disabled Text:    #6E6E6E (Dark gray)
- Accent Text:      #007ACC (Blue)
```

**UI Elements:**
```
- Border:           #3E3E3E (Subtle borders)
- Divider:          #2A2A2A (Section dividers)
- Selection:        #264F78 (Selected items)
- Input Background: #3C3C3C (Text inputs)
```

### Light Theme (Optional)

```
- Primary Background:   #F5F5F5
- Panel Background:     #FFFFFF
- Primary Text:         #1E1E1E
- Primary Accent:       #0078D4
```

---

## Typography

### Font Families

**Primary:** `Inter`, `Segoe UI`, `Roboto`, `system-ui`
- Modern, professional, excellent readability

**Code/Monospace:** `Fira Code`, `JetBrains Mono`, `Consolas`, `monospace`
- For script editors, property values, etc.

### Font Sizes

```
- Heading 1:  24px (Panel titles)
- Heading 2:  18px (Section headers)
- Heading 3:  16px (Subsection headers)
- Body:       14px (Default text)
- Small:      12px (Hints, captions)
- Tiny:       10px (Labels)
```

### Font Weights

```
- Bold:       600 (Headings, emphasis)
- Medium:     500 (Labels, buttons)
- Regular:    400 (Body text)
- Light:      300 (Secondary text)
```

---

## Spacing

### Grid System

**Base Unit:** 8px (all spacing is a multiple of 8)

```
- XXS:  4px   (Tight spacing)
- XS:   8px   (Compact spacing)
- SM:   12px  (Small spacing)
- MD:   16px  (Default spacing)
- LG:   24px  (Large spacing)
- XL:   32px  (Extra large spacing)
- XXL:  48px  (Section spacing)
```

### Padding

```
- Panel Padding:       16px
- Section Padding:     12px
- Input Padding:       8px 12px
- Button Padding:      8px 16px
- Compact Padding:     4px 8px
```

---

## Components

### Buttons

#### Primary Button
```
- Background:      #007ACC
- Text:           #FFFFFF
- Hover:          #005A9E
- Active:         #004578
- Border Radius:  4px
- Padding:        8px 16px
- Font Weight:    500
```

#### Secondary Button
```
- Background:      Transparent
- Text:           #007ACC
- Border:         1px solid #007ACC
- Hover:          rgba(0, 122, 204, 0.1)
- Active:         rgba(0, 122, 204, 0.2)
```

#### Ghost Button
```
- Background:      Transparent
- Text:           #D4D4D4
- Hover:          #333333
- Active:         #3E3E3E
```

### Inputs

#### Text Input
```
- Background:      #3C3C3C
- Text:           #D4D4D4
- Border:         1px solid #3E3E3E
- Focus Border:   1px solid #007ACC
- Border Radius:  4px
- Padding:        8px 12px
- Height:         32px
```

#### Number Input (with Drag)
```
- Same as text input
- Drag to adjust value
- Show increment/decrement arrows on hover
```

### Sliders

```
- Track:          #3E3E3E (4px height)
- Fill:           #007ACC
- Thumb:          #007ACC (12px circle)
- Thumb Hover:    #0E639C (14px circle)
- Thumb Active:   #0E639C (16px circle)
```

### Color Picker

```
- Preview:        48px x 48px rounded square
- Picker:         256px x 256px saturation/value grid
- Hue Slider:     256px x 16px gradient
- Alpha Slider:   256px x 16px checkerboard + gradient
- Hex Input:      Below picker
```

### Dropdowns

```
- Background:      #3C3C3C
- Text:           #D4D4D4
- Border:         1px solid #3E3E3E
- Hover:          #333333
- Border Radius:  4px
- Height:         32px
- Padding:        8px 12px
- Arrow:          Right-aligned, 16px icon
```

### Checkboxes

```
- Size:           16px x 16px
- Background:     #3C3C3C
- Border:         1px solid #3E3E3E
- Checked BG:     #007ACC
- Checkmark:      #FFFFFF
- Border Radius:  2px
```

### Panels

```
- Background:     #2D2D2D
- Border:         1px solid #3E3E3E
- Border Radius:  8px
- Shadow:         0 2px 8px rgba(0, 0, 0, 0.3)
- Title Bar:      #252525
- Title Height:   40px
- Padding:        16px
```

### Collapsible Sections

```
- Header BG:      #252525
- Header Hover:   #2A2A2A
- Content BG:     #2D2D2D
- Border:         1px solid #3E3E3E
- Arrow:          12px chevron (rotates 90° when open)
- Animation:      200ms ease-in-out
```

---

## Layouts

### Panel Structure

```
┌─────────────────────────────────────┐
│ ■ Panel Title              [_][□][x]│ ← Title bar (40px)
├─────────────────────────────────────┤
│                                     │
│  Panel content with 16px padding    │ ← Content area
│                                     │
│  ┌───────────────────────────────┐ │
│  │ Section Header            ▼   │ │ ← Collapsible section
│  ├───────────────────────────────┤ │
│  │ Section content               │ │
│  │                               │ │
│  └───────────────────────────────┘ │
│                                     │
└─────────────────────────────────────┘
```

### Property Grid

```
┌──────────────────────────────────┐
│ Property Name:        [Value    ]│ ← 40% label, 60% value
│ Another Property:     [Value    ]│
│ Nested Section:            ▼     │
│   Sub Property:       [Value    ]│
│   Sub Property 2:     [Value    ]│
└──────────────────────────────────┘
```

### Three-Column Layout (Inspector-style)

```
┌──────────┬─────────────────┬──────────┐
│          │                 │          │
│  Scene   │  Main Viewport  │  Props   │
│  Tree    │                 │  Panel   │
│  (20%)   │     (60%)       │  (20%)   │
│          │                 │          │
└──────────┴─────────────────┴──────────┘
```

---

## Animations

### Transitions

```
- Hover:      100ms ease-out
- Click:      150ms ease-in-out
- Expand:     200ms ease-in-out
- Fade:       150ms ease-in-out
- Slide:      200ms ease-in-out
```

### Easing Functions

```
- ease-out:        cubic-bezier(0.0, 0.0, 0.2, 1)
- ease-in-out:     cubic-bezier(0.4, 0.0, 0.2, 1)
- ease-in:         cubic-bezier(0.4, 0.0, 1, 1)
```

---

## Icons

### Style
- **Outlined** style (not filled)
- **16px** for inline icons
- **24px** for toolbar icons
- **32px** for large icons

### Sources
- Material Design Icons (outlined variant)
- Lucide Icons
- Heroicons (outline)

### Common Icons
```
- File:           📄
- Folder:         📁
- Settings:       ⚙️
- Search:         🔍
- Close:          ✕
- Minimize:       −
- Maximize:       □
- Menu:           ☰
- Add:            +
- Remove:         −
- Edit:           ✎
- Delete:         🗑
- Save:           💾
- Undo:           ↶
- Redo:           ↷
```

---

## Responsive Behavior

### Breakpoints

```
- Small:      < 1280px  (Single column, stacked panels)
- Medium:     1280-1920px (Two-column layout)
- Large:      > 1920px  (Three-column layout)
```

### Panel Resizing

```
- Min Width:     200px
- Max Width:     600px
- Default:       300px
- Drag Handle:   4px wide, hover expands to 8px
```

---

## Comparison with Competitors

### Unreal Engine
**What we keep:**
- Dark theme
- Professional polish
- Clear information hierarchy

**What we improve:**
- Less overwhelming UI
- Simpler property editors
- Better use of whitespace
- Clearer visual grouping

### Unity
**What we keep:**
- Clean, modern aesthetic
- Good use of color accents
- Inspector-style layouts

**What we improve:**
- More consistent design language
- Better hover states
- Smoother animations
- Less visual noise

### Godot
**What we keep:**
- Open-source friendly
- Accessible design
- Good documentation

**What we improve:**
- More polished look
- Better color palette
- Professional typography
- Refined spacing

---

## Implementation Guide

### CSS/Style Variables (for web)

```css
:root {
  /* Colors */
  --bg-primary: #1E1E1E;
  --bg-secondary: #252525;
  --bg-panel: #2D2D2D;
  --bg-hover: #333333;
  
  --accent-primary: #007ACC;
  --accent-success: #4EC9B0;
  --accent-warning: #CE9178;
  --accent-error: #F48771;
  
  --text-primary: #D4D4D4;
  --text-secondary: #9D9D9D;
  --text-disabled: #6E6E6E;
  
  /* Spacing */
  --space-xs: 8px;
  --space-sm: 12px;
  --space-md: 16px;
  --space-lg: 24px;
  --space-xl: 32px;
  
  /* Typography */
  --font-size-h1: 24px;
  --font-size-h2: 18px;
  --font-size-body: 14px;
  --font-size-small: 12px;
  
  /* Borders */
  --border-radius: 4px;
  --border-radius-large: 8px;
  
  /* Shadows */
  --shadow-sm: 0 2px 4px rgba(0, 0, 0, 0.2);
  --shadow-md: 0 2px 8px rgba(0, 0, 0, 0.3);
  --shadow-lg: 0 4px 16px rgba(0, 0, 0, 0.4);
  
  /* Transitions */
  --transition-fast: 100ms ease-out;
  --transition-normal: 150ms ease-in-out;
  --transition-slow: 200ms ease-in-out;
}
```

### Rust Constants (for desktop)

```rust
// Colors
pub const BG_PRIMARY: Color32 = Color32::from_rgb(30, 30, 30);
pub const BG_PANEL: Color32 = Color32::from_rgb(45, 45, 45);
pub const ACCENT_PRIMARY: Color32 = Color32::from_rgb(0, 122, 204);
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(212, 212, 212);

// Spacing
pub const SPACE_XS: f32 = 8.0;
pub const SPACE_MD: f32 = 16.0;
pub const SPACE_LG: f32 = 24.0;

// Borders
pub const BORDER_RADIUS: f32 = 4.0;
pub const BORDER_RADIUS_LARGE: f32 = 8.0;
```

---

## Quality Checklist

Before shipping any UI component, ensure:

- ✅ Follows color palette
- ✅ Uses correct spacing (8px grid)
- ✅ Has appropriate hover states
- ✅ Has smooth transitions
- ✅ Proper focus indicators (keyboard navigation)
- ✅ Accessible contrast ratios (WCAG AA minimum)
- ✅ Consistent with other components
- ✅ Tested at multiple screen sizes
- ✅ Performant (no jank)
- ✅ Documented in style guide

---

**Status:** v0.1.0 - Initial design system  
**Next:** Implement PBR Material Editor panel following these guidelines

