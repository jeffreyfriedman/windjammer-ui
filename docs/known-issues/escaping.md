# Escaping: string-builder components

Rust-side renderers escape by default (`src/escape.rs`, `render_to_html` and the patcher). The sites below are fixed in the `.wj` sources.

## `.wj` / generated builders (fix in `.wj` sources, then regenerate)

Every compiler-generated builder in `src/components/generated/*.rs` (and its
`src/components_wj/*.wj` source) concatenates field values into HTML strings without
escaping. The fix is on the `.wj` side (call an escape function exposed from this crate, e.g.
`windjammer_ui::escape::escape_text` / `escape_attr`, at each interpolation).
Highest-risk first: `Text`, `Button` labels, `Alert`, `Input` (`value`, `placeholder`),
`Link`/`Image` (`href`/`src`: also pass through `sanitize_url`), `DataTable` cells, `Tabs`,
`Modal`, `CommandPalette`, `Tooltip`, `Badge`, `Card` title/body.

## Raw markup

Fields that carry trusted markup use
`escape::RawHtml` (`RawHtml::new(markup)` for trusted markup, `RawHtml::text(s)` for escaped plain text).
Shared DOM helpers remain in `src/safe_dom_js.rs` (`window.wjDom`: `setText`, `note`, `clear`, `setSafeHtml`,
`setTrustedHtml`); no runtime in this crate assigns `innerHTML`/`outerHTML` or calls `eval`.

## Legacy `vdom` string renderers

`ssr.rs` validates tag names, sanitises URL attributes (all of `is_url_attr`), escapes hydration state for
`<script type="application/json">`, and chunks streaming output on UTF-8 boundaries. `renderer.rs::vnode_to_html`
uses the shared escaper.
