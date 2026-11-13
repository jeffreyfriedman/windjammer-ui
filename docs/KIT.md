# Accessible component kit

The kit is a set of interactive components that are accessible by construction: each one has a role, an accessible
name, keyboard operation and focus management, and each is covered by a browser test that asserts its ARIA attributes,
tab order and key handling. Everything is written in Windjammer (`src/components_wj/`) and regenerated into
`src/components/generated/`.

| Area | Components | Source |
|------|-----------|--------|
| Disclosure | `TabStrip`, `AccordionStack` | `kit_disclosure.wj` |
| Overlays | `ModalDialog`, `MenuButton` | `kit_overlay.wj` |
| Choice | `CheckboxField`, `SwitchControl`, `ToggleButton`, `RadioChoice`, `SliderControl`, `PageNav`, `TooltipHost` | `kit_choice.wj` |
| Forms | `FormKit` with `FormField` (`Text`, `Email`, `Number`, `Date`, `Select`, `Checkbox`, `Radio`) | `form_kit.wj` |
| Data display | `DataGrid` (sort, filter, paging), `VirtualList` | `grid_kit.wj` |
| Theming | `tokens` (light, dark, high-contrast) | `tokens.wj` |
| Shared | `Interactive`, keyboard rules, ARIA helpers | `a11y.wj` |

## How a kit component works

A kit component is a model: it holds state, renders ARIA-complete markup from it (`view()`), and updates the state from
DOM events (`on_event`). Interactive elements carry `data-wj-act="name"` or `"name:index"`. The mount layer
(`windjammer_ui::kit_mount`) delivers `keydown`, `click`, `input`, `change`, `focusin`, `focusout` and `scroll` as
`(event, act, key, value)`, patches the new view into the DOM (focus, caret and typed text survive), cancels the native
default for events the model handled, and moves focus to the element named by `focus_id()`.

```rust
use windjammer_ui::components::generated::kit_disclosure::TabStrip;
use windjammer_ui::kit_mount::mount;

let tabs = TabStrip::new("docs".to_string())
    .tab("Overview".to_string(), "What it is".to_string())
    .tab("API".to_string(), "How to call it".to_string());
let kit = mount(document, container, tabs);
let selected = kit.with_model(|m| m.selected_index());
```

Because the models are plain Rust types, their logic (validation, sorting, windows) is also unit-tested on the host in
`tests/kit_logic_test.rs`.

## Components

Each section shows a minimal example. The `base` argument is an id prefix; ids are `<base>-<part>-<index>`.

### TabStrip

```rust
TabStrip::new("tabs".into()).tab("One".into(), "First".into()).tab("Two".into(), "Second".into())
```

`role=tablist/tab/tabpanel`, `aria-selected`, `aria-controls`/`aria-labelledby`. One tab stop (roving `tabindex`);
Left/Right move and select with wrap, Home/End jump. Tests: `tabs_*` in `tests/wasm_kit_a11y_test.rs`.

### AccordionStack

```rust
AccordionStack::new("faq".into()).item("Shipping".into(), "Two days.".into()).exclusive(true)
```

Header buttons with `aria-expanded` and `aria-controls`, regions labelled by their header. Enter/Space toggle, Up/Down/Home/End
move between headers. Tests: `accordion_*`, `exclusive_accordion_*`.

### ModalDialog

```rust
ModalDialog::new("confirm".into(), "Delete project".into()).trigger("Delete...".into()).body("This cannot be undone.".into())
```

`role=dialog`, `aria-modal`, labelled by its title and described by its body. Opening moves focus inside; Tab and Shift+Tab
cycle within; Escape, Cancel, OK or the backdrop close it and focus returns to the opener. `was_confirmed()` reports which
button closed it. Tests: `dialog_*`.

### MenuButton

```rust
MenuButton::new("actions".into(), "Actions".into()).item("Rename".into()).item("Delete".into())
```

`aria-haspopup=menu`, `aria-expanded`, `role=menu/menuitem`. Enter, Space or Down opens on the first item, Up on the last;
arrows move with wrap, Home/End jump, Enter/Space choose, Escape closes; focus returns to the button. Tests: `menu_*`.

### CheckboxField

```rust
CheckboxField::new("news".into(), "Subscribe to updates".into())
```

A native checkbox inside its label, so the browser supplies the role, name and Space key. Tests: `checkbox_*`.

### SwitchControl and ToggleButton

```rust
SwitchControl::new("wifi".into(), "Wi-Fi".into()).on(true)
ToggleButton::new("bold".into(), "Bold".into())
```

`role=switch` with `aria-checked`; the toggle button uses `aria-pressed`. Space and Enter toggle; a disabled switch leaves
the tab order. Tests: `switch_*`, `disabled_switch_*`, `toggle_button_*`.

### RadioChoice

```rust
RadioChoice::new("plan".into(), "Plan".into()).option("free".into(), "Free".into()).option("pro".into(), "Pro".into())
```

`role=radiogroup` named by its label, `role=radio` with `aria-checked`. One tab stop (the selected radio, or the first when
none is selected); arrows move and select with wrap; Space selects. Tests: `radio_*`.

### SliderControl

```rust
SliderControl::new("vol".into(), "Volume".into(), 0, 100).step(5).value(50)
```

`role=slider` with `aria-valuemin/max/now` and `aria-label`. Arrows step, PageUp/PageDown move ten steps, Home/End jump;
values clamp. Tests: `slider_*`.

### PageNav

```rust
PageNav::new("pages".into(), "Results pages".into(), 12)
```

A labelled `<nav>`; the current page has `aria-current=page`, the ends disable Prev/Next, page buttons carry
`aria-label="Page n"`. Tests: `pagination_*`.

### TooltipHost

```rust
TooltipHost::new("save".into(), "Save".into(), "Saves your work".into())
```

The trigger points at a `role=tooltip` with `aria-describedby`. It shows on focus and hides on blur or Escape. Tests: `tooltip_*`.

### FormKit

```rust
FormKit::new("signup".into(), "Sign up".into())
    .field(FormField::new("name".into(), "Name".into(), FieldKind::Text).required(true).min_len(2))
    .field(FormField::new("age".into(), "Age".into(), FieldKind::Number).min("18".into()))
    .field(FormField::new("plan".into(), "Plan".into(), FieldKind::Radio).option("free".into(), "Free".into()))
```

Fields: text, email, number, date, select, checkbox, radio. Each field has a label bound by `for`, `aria-required`, and,
once the user has left it or tried to submit, `aria-invalid` plus an error message referenced by `aria-describedby`. Submit
validates everything, announces "n fields need attention" in a polite live region and focuses the first invalid field; a
valid submit records `name=value` pairs (`submitted()`). Reset restores the initial values and hides errors. Enter in a text
field submits. Validation is a pure function (`FormField::error_message`). Tests: `form_*`, `error_*`, `each_field_kind_*`,
`failed_submit_*`, `valid_submit_*`, `enter_in_a_text_field_*`, `typed_text_and_focus_*`.

### DataGrid

```rust
DataGrid::new("team".into(), "Team".into())
    .column(GridColumn::text("Name".into())).column(GridColumn::number("Age".into()))
    .row(vec!["Mia".into(), "31".into()]).page_size(10)
```

A native `<table>` with caption, `th scope=col`, `aria-sort` and a button per sortable header (click, Enter or Space; the
third activation clears the sort; numeric columns sort numerically; ties keep their order). A filter box matches any cell
case-insensitively, a live status reads "Showing a-b of n", and Prev/Next page. Tests: `grid_*`.

### VirtualList

```rust
VirtualList::new("log".into(), "Log lines".into(), 10_000, 20, 200)
```

Renders the rows in the viewport plus an overscan inside a full-height spacer, so ten thousand rows cost about twenty DOM
nodes. The container is a focusable `listbox` (`aria-activedescendant`, rows with `aria-posinset`/`aria-setsize`); arrows,
Page keys, Home and End move the active row and scroll it into view. Tests: `ten_thousand_rows_*`, `scrolling_swaps_*`,
`virtual_list_keys_*`.

## Theming tokens

`tokens.wj` declares one token set with a value for each of light, dark and high-contrast. `tokens::theme_css()` emits the
custom properties (`--wj-color-bg`, `--wj-color-text`, `--wj-color-primary`, `--wj-focus-width`, ...) and `tokens::kit_css()`
styles the kit using only `var(--wj-...)`.

```rust
let css = format!("{}\n{}", tokens::theme_css(), tokens::kit_css()); // put in a <style> element
// <html data-wj-theme="dark"> selects a mode explicitly.
```

Without an explicit `data-wj-theme`, the user's system preference decides (`prefers-contrast: more` selects high-contrast,
`prefers-color-scheme: dark` selects dark). Overriding a single custom property restyles every component at once. Text and
status colors meet WCAG AA (4.5:1) in light and dark and AAA (7:1) in high-contrast; borders and focus rings meet 3:1.
Tests: `tests/kit_logic_test.rs` (coverage and contrast) and `components_read_the_same_token_set_*`,
`a_token_change_in_one_place_*` in `tests/wasm_kit_data_test.rs`.

## Running the browser tests

```text
CHROMEDRIVER=/path/to/chromedriver CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
  cargo test --target wasm32-unknown-unknown --no-default-features --features web --test wasm_kit_a11y_test --test wasm_kit_data_test
```

Chrome and Edge share the Chromium engine. For Edge, unset `CHROMEDRIVER` and put a `msedgedriver` that matches
the installed Edge on `PATH` (see `docs/TESTING_GUIDE.md`).
