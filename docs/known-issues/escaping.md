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

## Hand-maintained files: intentionally raw `*_html` fields

These fields are documented as pre-rendered HTML and are NOT escaped (changing their
type would break consumers in downstream-screens). Callers must pass trusted markup or
pre-escape. Proper fix: change the field type to `escape::RawHtml` when the components
move out (EXTRACTION_PLAN section A).

- `accountrail.rs`: `AccountRailItem.balance_html`
- `checkbookregister.rs`: `CheckbookRow.amount_html`, `balance_html`
- `bankmatch.rs`: `BankMatchRow.je_options_html`
- `businessworkspace.rs`: `rail_html`, `register_html`, `write_check_html`,
  `memorized_html`, `bank_html`

Escaped in this pass (attribute and text interpolations): accountrail, authfetch, bankmatch
(line id), checkbookregister, jsonpost, loginform, memorizedlist. `hashrouter`,
`ledgertheme`, `readmodelbridge`, `reconcileactions`, `shellchrome`, `shellruntimes`,
`shortcutchord`, `lib.rs`, `mod.rs` have no dynamic interpolation (static markup/JS/CSS).

## Embedded JS runtimes

The `*_runtime_js()` strings assign server/user data with `innerHTML`
(`bank_match_runtime_js` copies `mount.innerHTML` into selects; auth-fetch `lkRender`
paths build tables from JSON). They need an audit and a move to Windjammer closures;
JS embedded in Rust strings is itself a rule violation (AGENTS.md).

## Legacy `vdom` string renderers

`ssr.rs` escapes text/attributes (now also `'`), but not URL-bearing attributes.
`renderer.rs::vnode_to_html` (desktop/Tauri placeholder) was unescaped and now uses the
shared escaper.
