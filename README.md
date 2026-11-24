# Windjammer UI

A UI component library and reactive runtime for Windjammer. Components are written in
`.wj` as builders that render HTML strings; a Rust reactive core (signals, keyed DOM patcher) drives
the browser (WASM) loop.

The web target is the primary, supported path. Desktop (egui) works for editor-style
UIs; iOS and Android are stubs (see the table below).

## Example

```windjammer
use windjammer_ui::*

fn main() {
    let app = Container::new()
        .child(Text::new("Hello, World!").size(TextSize::XLarge).bold().render())
        .child(Button::new("Click Me").variant(ButtonVariant::Primary).render())
        .render()
    println("${app}")
}
```

## What is here

- `src/components_wj/`: 100 `.wj` component sources (primitives, layout, forms,
  data display, editor and profiler panels).
- `src/components_wj/{a11y,kit_*,form_kit,grid_kit,tokens}.wj`: the accessible component kit (tabs, accordion, dialog, menu,
  switch, radio, slider, pagination, tooltip, forms with validation, sortable/filterable/paged table, virtualized list,
  light/dark/high-contrast tokens). Each component is covered by a browser test that asserts its ARIA attributes, tab order
  and key handling; see `docs/KIT.md`.
- `src/*.rs`: reactive core, vdom, app loops, SSR, routing.
- `crates/`: the next-generation scene renderer (reactive arena, layout, text, paint, accessibility, shells), in its own
  Cargo workspace; see `docs/VISION_2026.md` and `docs/adr/`.
- `examples/`, `examples_wj/`: demos.

## Platforms

| Target | State |
|--------|-------|
| Web (WASM) | Supported; default feature (`web`) |
| Desktop (eframe/egui) | Works for editor-style UIs (`desktop` feature) |
| SSR | Basic string rendering |
| iOS, Android | Stubs (`mobile-ios`, `mobile-android` features; no renderer yet) |
| Tauri | Optional dependency (`desktop-tauri` feature) |
| Terminal | Not implemented |

## Build

```bash
WJ_UI_REGEN=1 cargo test --test generated_fresh_test                                          # regenerate components
cargo test                                                                                    # host tests
./build-wasm.sh                                                                               # wasm-pack packages (web, bundler, nodejs)
```

`./wj-build.sh` transpiles `src/components_wj/` with the `wj` on your `PATH`.
`src/components/generated/` is compiler output; `tests/generated_fresh_test.rs` fails when it drifts from the `.wj` sources,
and `tests/wj_sources_check_test.rs` runs `wj check` on every component source and example.

## Security notes

Escape-by-default is enforced for the VNode renderer, the DOM patcher and SSR. See `docs/known-issues/escaping.md`
for the string-builder components that still interpolate field values directly.

## Docs

`docs/ARCHITECTURE.md`, `docs/MODULE_MAP.md`, `docs/KIT.md`, `docs/WEB_AND_WASM.md`, `docs/TESTING_GUIDE.md`,
`docs/VISION_2026.md` and the decision records in `docs/adr/`.

## License

MIT OR Apache-2.0
