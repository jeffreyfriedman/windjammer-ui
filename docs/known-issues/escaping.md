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

## Domain-specific runtimes moved out

The hand-maintained domain components (register/matching tables, bearer-token fetch and shell runtimes) and their
embedded-JS tests were removed from this crate and now live in the consuming application. Their `*_html` fields use
`escape::RawHtml` (`RawHtml::new(markup)` for trusted markup, `RawHtml::text(s)` for escaped plain text).
Shared DOM helpers remain in `src/safe_dom_js.rs` (`window.wjDom`: `setText`, `note`, `clear`, `setSafeHtml`,
`setTrustedHtml`); no runtime in this crate assigns `innerHTML`/`outerHTML` or calls `eval`.

## Legacy `vdom` string renderers

`ssr.rs` now validates tag names, sanitises URL attributes (all of `is_url_attr`), escapes hydration state for
`<script type="application/json">`, and chunks streaming output on UTF-8 boundaries. `renderer.rs::vnode_to_html`
uses the shared escaper. Note `src/components/generated/ssr.rs` is a stale duplicate of `src/ssr.rs` without these fixes
(see DESIGN_DIRECTION.md D4: delete).
