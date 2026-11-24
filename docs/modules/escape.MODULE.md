# escape (`src/escape.rs`)

**Purpose.** The single place that decides what is safe to emit as HTML.

**Inputs.** Arbitrary untrusted strings, tag names, attribute names/values.
**Outputs.** `Escaped` (safe string), sanitized URLs, validity verdicts, `safe_attr`.

**Invariants.**
- `escape_text`/`escape_attr` escape `& < > " ' \``; unicode untouched; not idempotent.
- `sanitize_url` strips control/whitespace before reading the scheme; only
  http, https, mailto, tel, relative URLs and `data:image/{png,gif,jpeg,jpg,webp}` pass;
  everything else becomes `#`.
- Tags must match `[A-Za-z][A-Za-z0-9-]*` and not be script/iframe/object/embed/base/noscript.
- Attribute names `[A-Za-z0-9_:.-]+`; static `on*` (inline handler) attributes are rejected.
- `RawHtml` is the only opt-out; it is never produced by this module.

**Owns.** Escaping and URL/name policy. **Never decides.** Rendering, diffing, structure.
**Allowed imports.** std only.
**Entry points.** `escape_text`, `escape_attr`, `sanitize_url`, `safe_attr`, `is_valid_tag`,
`is_valid_attr_name`, `RawHtml`, `Escaped`.
**Tests.** Unit tests in `src/escape.rs`; renderer-level corpus in `src/simple_renderer.rs`
(`xss_tests`); patch-level in `tests/dom_patch_test.rs`.
