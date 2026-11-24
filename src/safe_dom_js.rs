//! Safe DOM helper shared by the embedded JS runtimes.
//!
//! Embedded runtimes must not assign `innerHTML` directly. They go through `window.wjDom`:
//! text via `setText`/`textContent`, untrusted-ish markup via `setSafeHtml` (parsed with
//! `DOMParser`, so nothing executes; scripts, frames, `on*` attributes and non-allowlisted
//! URL schemes are removed), and the single audited `setTrustedHtml` sink for app-authored
//! static pages whose inline scripts are re-executed on purpose.
//!
//! The helper text is embedded into each runtime with `concat!(safe_dom_js!(), ...)` so every
//! runtime stays standalone; `tests/embedded_js_test.rs` checks all copies are identical.

/// The helper as a string literal (usable inside `concat!`).
#[macro_export]
macro_rules! safe_dom_js {
    () => {
        r##"
(function () {
  if (window.wjDom) return;
  var BLOCKED = /^(script|iframe|frame|frameset|object|embed|base|link|meta|noscript|template)$/i;
  var URL_ATTRS = /^(href|src|action|formaction|xlink:href|poster|data|cite|background|ping|srcset)$/i;
  function safeUrl(u) {
    var probe = String(u).replace(/[\u0000- \u007f-\u009f]+/g, '').toLowerCase();
    var i = probe.indexOf(':');
    if (i < 0) return u;
    if (/[\/?#]/.test(probe.slice(0, i))) return u;
    if (/^(https?|mailto|tel)$/.test(probe.slice(0, i))) return u;
    if (/^data:image\/(png|gif|jpe?g|webp)[;,]/.test(probe)) return u;
    return '#';
  }
  function sanitize(node) {
    var kids = Array.prototype.slice.call(node.childNodes || []);
    kids.forEach(function (c) {
      if (c.nodeType !== 1) return;
      if (BLOCKED.test(c.nodeName)) { node.removeChild(c); return; }
      Array.prototype.slice.call(c.attributes || []).forEach(function (a) {
        var n = a.name.toLowerCase();
        if (n.indexOf('on') === 0 || n === 'style' && /expression|url\(|@import/i.test(a.value)) {
          c.removeAttribute(a.name);
        } else if (URL_ATTRS.test(n)) {
          c.setAttribute(a.name, safeUrl(a.value));
        }
      });
      sanitize(c);
    });
    return node;
  }
  function setText(el, text) { el.textContent = text == null ? '' : String(text); }
  function clear(el) { el.textContent = ''; }
  // One <p class=cls> holding plain text (status/error lines).
  function note(el, cls, text) {
    var p = document.createElement('p');
    p.className = cls;
    p.textContent = text == null ? '' : String(text);
    el.textContent = '';
    el.appendChild(p);
    return p;
  }
  function setSafeHtml(el, html) {
    var doc = new DOMParser().parseFromString('<!doctype html><body>' + (html == null ? '' : String(html)), 'text/html');
    sanitize(doc.body);
    el.replaceChildren.apply(el, Array.prototype.slice.call(doc.body.childNodes));
  }
  // Audited sink: app-authored static pages only (never server or user data).
  function setTrustedHtml(el, html) { el.innerHTML = html; }
  window.wjDom = { setText: setText, note: note, clear: clear, setSafeHtml: setSafeHtml, sanitize: sanitize, safeUrl: safeUrl, setTrustedHtml: setTrustedHtml };
})();
"##
    };
}

/// The shared helper on its own (host pages may include it once up front).
pub fn safe_dom_runtime_js() -> &'static str {
    safe_dom_js!()
}
