# Escaping: remaining sites

Rust-side renderers escape by default (`src/escape.rs`, `render_to_html`, the patcher, and
the interpolations in the hand-maintained `src/components/generated/*.rs` listed in
`docs/EXTRACTION_PLAN.md` section B). The sites below cannot be fixed on the Rust side.

## `.wj` / generated builders (fix in `.wj` sources, then regenerate)

Every compiler-generated builder in `src/components/generated/*.rs` (and its
`src/components_wj/*.wj` source) concatenates field values into HTML strings without
escaping. This is a `.wj`-side fix (call an escape function exposed from this crate, e.g.
`windjammer_ui::escape::escape_text` / `escape_attr`, at each interpolation). Not touched
here because `.wj` sources are out of scope for this pass and the compiler is in flux.
Highest-risk first: `Text`, `Button` labels, `Alert`, `Input` (`value`, `placeholder`),
`Link`/`Image` (`href`/`src`: also pass through `sanitize_url`), `DataTable` cells, `Tabs`,
`Modal`, `CommandPalette`, `Tooltip`, `Badge`, `Card` title/body.

## Hand-maintained files: `*_html` fields are `RawHtml`

The pre-rendered HTML slots now have type `escape::RawHtml` (constructors take `impl Into<RawHtml>`; there is no
`From<&str>`/`From<String>`, so passing a plain string is a compile error). Use `RawHtml::new(markup)` for trusted markup
and `RawHtml::text(s)` for plain text (escaped). This breaks callers in downstream-screens, which must wrap values.

- `accountrail.rs`: `AccountRailItem.balance_html`
- `checkbookregister.rs`: `CheckbookRow.amount_html`, `balance_html`
- `bankmatch.rs`: `BankMatchRow.je_options_html`
- `businessworkspace.rs`: `rail_html`, `register_html`, `write_check_html`, `memorized_html`, `bank_html`

The `.wj` copies of accountrail, bankmatch and checkbookregister still take `string`; they leave with the extraction.

## Embedded JS runtimes (audited)

No runtime assigns `innerHTML`/`outerHTML`, calls `insertAdjacentHTML`, `document.write` or `eval`, except through
`window.wjDom` (`src/safe_dom_js.rs`): `setText`, `note`, `clear`, `setSafeHtml` (DOMParser, strips scripts/frames/`on*`/bad
URLs, executes nothing), and `setTrustedHtml`, the single audited sink used only by `hashrouter` for static app-authored
pages. `tests/embedded_js_test.rs` enforces this statically and runs the runtimes under node with a DOM stub.
Behaviour change: markup returned by `window.lkRender[kind]`, `__wjRenderReadModel` and `LK_NAVS` is now sanitised, so
inline `on*` handlers or `<script>` in those renderers no longer work (use `data-*` attributes plus delegation).

## Legacy `vdom` string renderers

`ssr.rs` now validates tag names, sanitises URL attributes (all of `is_url_attr`), escapes hydration state for
`<script type="application/json">`, and chunks streaming output on UTF-8 boundaries. `renderer.rs::vnode_to_html`
uses the shared escaper. Note `src/components/generated/ssr.rs` is a stale duplicate of `src/ssr.rs` without these fixes
(see DESIGN_DIRECTION.md D4: delete).
