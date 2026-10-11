# Windjammer UI

A UI component library and reactive runtime for Windjammer. Components are written in
`.wj` as builders that render HTML strings; a Rust reactive core (signals, keyed DOM patcher) drives
the browser (WASM) loop.

**Status: experimental.** The web target works for the components and examples in this
repo. Several platform targets are stubs (below). Do not treat this as production ready.

## Example

```windjammer
use windjammer_ui::*

fn main() {
    let app = Container::new()
        .child(Text::new("Hello, World!").size(TextSize::XLarge).bold().render())
        .child(Button::new("Click Me").variant(ButtonVariant::Primary).render())
        .render()
    println!("{}", app)
}
```

## What is here

- `src/components_wj/`: about 120 `.wj` component sources (primitives, layout, forms,
  data display, editor/profiler panels). Some are ledger-specific and are scheduled to
  move out; see `docs/EXTRACTION_PLAN.md`.
- `src/*.rs`: reactive core, vdom, app loops, SSR, routing.
- `examples/`, `examples_wj/`: demos (some HTML demos predate the current API).

## Platform status

| Target | State |
|--------|-------|
| Web (WASM) | Supported; default feature |
| Desktop (eframe/egui) | Works for editor-style UIs, lightly tested |
| SSR | Basic string rendering |
| iOS, Android | Stubs only (no renderer) |
| Tauri | Legacy optional dependency, not exercised |
| Terminal | Not implemented |

## Known limitations

- 17 files in `src/components/generated/` are hand-maintained, not compiler output.
- Some interactive behaviour is JS embedded in Rust strings.
- Escape-by-default is enforced for the VNode renderer, the DOM patcher, SSR and the
  hand-maintained generated files; compiler-generated string builders still need a
  `.wj`-side audit (`docs/known-issues/escaping.md`).

## Docs

`docs/ARCHITECTURE.md`, `docs/MODULE_MAP.md`, `docs/EXTRACTION_PLAN.md`, plus
`docs/QUICKSTART.md`, `docs/GUIDE.md`, `docs/API_REFERENCE.md`, `docs/STYLE_GUIDE.md`
(the guides may lag the code).

## License

MIT OR Apache-2.0
