//! Output escaping: the single place that decides what is safe to emit as HTML.
//!
//! - [`escape_text`] / [`escape_attr`] return [`Escaped`], a string known to be safe
//!   in text / quoted-attribute position.
//! - [`RawHtml`] is the explicit, greppable opt-out for trusted markup.
//! - [`sanitize_url`] neutralizes script-capable URL schemes.
//! - [`is_valid_tag`] / [`is_valid_attr_name`] reject names that could break out of markup.

use std::fmt;

/// A string that has already been escaped for HTML output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Escaped(String);

impl Escaped {
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for Escaped {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Trusted HTML that bypasses escaping. Only construct from markup you control.
///
/// There is deliberately no `From<String>` / `From<&str>`: components with `*_html`
/// slots take `impl Into<RawHtml>`, so passing a plain string is a compile error and
/// every raw-markup call site is greppable (`RawHtml::new`). For plain text use
/// [`RawHtml::text`], which escapes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct RawHtml(pub String);

impl RawHtml {
    /// Trusted markup, emitted verbatim.
    pub fn new(html: impl Into<String>) -> Self {
        RawHtml(html.into())
    }
    /// Plain text, escaped, as a (safe) `RawHtml` for slots that take markup.
    pub fn text(text: &str) -> Self {
        RawHtml(escape_text(text).into_string())
    }
    pub fn empty() -> Self {
        RawHtml(String::new())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Escaped> for RawHtml {
    fn from(e: Escaped) -> Self {
        RawHtml(e.into_string())
    }
}

impl fmt::Display for RawHtml {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn escape_into(input: &str, out: &mut String) {
    for c in input.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            '`' => out.push_str("&#96;"),
            _ => out.push(c),
        }
    }
}

/// Escape for element text content.
pub fn escape_text(input: &str) -> Escaped {
    let mut out = String::with_capacity(input.len());
    escape_into(input, &mut out);
    Escaped(out)
}

/// Escape for a double- or single-quoted attribute value.
pub fn escape_attr(input: &str) -> Escaped {
    escape_text(input)
}

/// Attributes whose value is a URL and must pass [`sanitize_url`].
pub fn is_url_attr(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "href" | "src" | "action" | "formaction" | "xlink:href" | "poster" | "data" | "cite"
            | "background" | "ping" | "srcset"
    )
}

const SAFE_SCHEMES: [&str; 4] = ["http", "https", "mailto", "tel"];
const BLOCKED_URL: &str = "#";

/// Return `url` if it is relative or uses an allowlisted scheme, otherwise `"#"`.
///
/// Browsers ignore ASCII whitespace/control characters inside a scheme
/// (`java\tscript:`), so those are stripped before the scheme check.
pub fn sanitize_url(url: &str) -> String {
    let probe: String = url
        .chars()
        .filter(|c| !c.is_control() && !c.is_whitespace())
        .collect::<String>()
        .to_ascii_lowercase();
    match probe.find(':') {
        None => url.to_string(),
        Some(i) => {
            let head = &probe[..i];
            // A '/', '?' or '#' before ':' means the colon is not a scheme separator.
            if head.contains(['/', '?', '#']) {
                return url.to_string();
            }
            if SAFE_SCHEMES.contains(&head) || is_safe_data_image(&probe) {
                url.to_string()
            } else {
                BLOCKED_URL.to_string()
            }
        }
    }
}

fn is_safe_data_image(probe: &str) -> bool {
    ["png", "gif", "jpeg", "jpg", "webp"]
        .iter()
        .any(|t| probe.starts_with(&format!("data:image/{t};")) || probe.starts_with(&format!("data:image/{t},")))
}

/// Tags that may never be created through a VNode element (use [`RawHtml`] deliberately).
const BLOCKED_TAGS: [&str; 6] = ["script", "iframe", "object", "embed", "base", "noscript"];

/// True for `[a-zA-Z][a-zA-Z0-9-]*` tags that are not blocked.
pub fn is_valid_tag(tag: &str) -> bool {
    let mut chars = tag.chars();
    let first_ok = chars.next().is_some_and(|c| c.is_ascii_alphabetic());
    first_ok
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '-')
        && !BLOCKED_TAGS.contains(&tag.to_ascii_lowercase().as_str())
}

/// True for attribute names that cannot terminate the tag or inject other attributes.
pub fn is_valid_attr_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | ':' | '.'))
}

/// Static attributes named `onclick`, `onerror`, ... are inline script; always rejected.
/// Event handlers must be `VAttr::Event` (`on_click`).
pub fn is_inline_handler_name(name: &str) -> bool {
    let n = name.to_ascii_lowercase();
    n.starts_with("on") && !n.starts_with("on_") && n.len() > 2
}

/// Decide the value to emit for an attribute, or `None` to drop it.
pub fn safe_attr(name: &str, value: &str) -> Option<String> {
    if !is_valid_attr_name(name) || is_inline_handler_name(name) {
        return None;
    }
    if is_url_attr(name) {
        Some(sanitize_url(value))
    } else {
        Some(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAYLOADS: [&str; 7] = [
        "<script>alert(1)</script>",
        "\" onmouseover=\"alert(1)",
        "' onfocus='alert(1)",
        "</div><img src=x onerror=alert(1)>",
        "&lt;already&gt;",
        "`backtick`",
        "<svg/onload=alert(1)>",
    ];

    #[test]
    fn text_escape_neutralizes_markup_characters() {
        for p in PAYLOADS {
            let e = escape_text(p).into_string();
            assert!(!e.contains('<') && !e.contains('>') && !e.contains('"') && !e.contains('\''), "{e}");
        }
    }

    #[test]
    fn escape_is_not_idempotent_so_double_escaping_is_visible() {
        assert_eq!(escape_text("&lt;").as_str(), "&amp;lt;");
    }

    #[test]
    fn unicode_is_preserved() {
        assert_eq!(escape_text("héllo 日本語 🚀 <b>").as_str(), "héllo 日本語 🚀 &lt;b&gt;");
    }

    #[test]
    fn attr_breakout_quotes_are_escaped() {
        assert_eq!(escape_attr("\" onmouseover=\"x").as_str(), "&quot; onmouseover=&quot;x");
        assert_eq!(escape_attr("' onfocus='x").as_str(), "&#39; onfocus=&#39;x");
    }

    #[test]
    fn dangerous_url_schemes_are_neutralized() {
        for u in [
            "javascript:alert(1)",
            "JaVaScRiPt:alert(1)",
            "  javascript:alert(1)",
            "java\tscript:alert(1)",
            "java\nscript:alert(1)",
            "\u{0001}javascript:alert(1)",
            "vbscript:msgbox(1)",
            "data:text/html,<script>alert(1)</script>",
            "data:image/svg+xml;base64,AAAA",
            "file:///etc/passwd",
        ] {
            assert_eq!(sanitize_url(u), "#", "{u:?}");
        }
    }

    #[test]
    fn safe_urls_pass_through() {
        for u in [
            "https://example.com/a?b=c:d",
            "http://x.test",
            "mailto:a@b.c",
            "tel:+15551234",
            "/path:with:colons",
            "relative/path",
            "#frag:ment",
            "?q=a:b",
            "data:image/png;base64,AAAA",
            "",
        ] {
            assert_eq!(sanitize_url(u), u);
        }
    }

    #[test]
    fn entity_encoded_scheme_is_not_decoded_here_but_is_escaped_on_output() {
        let u = sanitize_url("javascript&colon;alert(1)");
        assert_eq!(escape_attr(&u).as_str(), "javascript&amp;colon;alert(1)");
    }

    #[test]
    fn tag_and_attr_name_validation() {
        assert!(is_valid_tag("div") && is_valid_tag("my-element") && is_valid_tag("H1"));
        for t in ["", "1a", "div onclick=x", "a>b", "script", "SCRIPT", "iframe", "a/b"] {
            assert!(!is_valid_tag(t), "{t}");
        }
        assert!(is_valid_attr_name("data-x") && is_valid_attr_name("aria-label") && is_valid_attr_name("xlink:href"));
        for n in ["", "a b", "a=b", "a\"b", "a>b", "a/b"] {
            assert!(!is_valid_attr_name(n), "{n}");
        }
    }

    #[test]
    fn safe_attr_drops_inline_handlers_and_sanitizes_urls() {
        assert_eq!(safe_attr("onclick", "alert(1)"), None);
        assert_eq!(safe_attr("OnError", "x"), None);
        assert_eq!(safe_attr("on_click", "x"), Some("x".into()));
        assert_eq!(safe_attr("href", "javascript:x"), Some("#".into()));
        assert_eq!(safe_attr("HREF", "javascript:x"), Some("#".into()));
        assert_eq!(safe_attr("class", "a b"), Some("a b".into()));
        assert_eq!(safe_attr("a b", "x"), None);
    }

    #[test]
    fn raw_html_text_escapes_and_display_is_verbatim() {
        assert_eq!(RawHtml::text("<b>&\"x\"</b>").as_str(), "&lt;b&gt;&amp;&quot;x&quot;&lt;/b&gt;");
        assert_eq!(RawHtml::new("<b>ok</b>").to_string(), "<b>ok</b>");
        assert!(RawHtml::empty().is_empty());
        assert_eq!(RawHtml::from(escape_text("<")).as_str(), "&lt;");
    }
}
