//! BankMatch — Match action + JE picker (ADR-002 / R2.4).
//! Hand-maintained. Always SKIP_WJ_REGEN=1 (regen from bankmatch.wj drops runtime).

#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct BankMatchRow {
    pub line_id: String,
    pub unmatched: bool,
    pub je_options_html: String,
}

impl BankMatchRow {
    #[inline]
    pub fn new(
        line_id: impl Into<String>,
        unmatched: bool,
        je_options_html: impl Into<String>,
    ) -> BankMatchRow {
        BankMatchRow {
            line_id: line_id.into(),
            unmatched,
            je_options_html: je_options_html.into(),
        }
    }
}

impl Renderable for BankMatchRow {
    #[inline]
    fn render(&self) -> String {
        if self.unmatched {
            format!(
                r#"<span class="wj-bank-match-action" data-wj-bank-match-cell><select data-wj-bank-match-je aria-label="Journal entry">{opts}</select><button type="button" class="btn-secondary" data-wj-bank-match data-line-id="{id}">Match</button></span>"#,
                opts = self.je_options_html,
                id = self.line_id
            )
        } else {
            r#"<span class="muted">Matched</span>"#.to_string()
        }
    }
}

/// Click Match → read JE select → POST match → optimistic Matched UI.
pub fn bank_match_runtime_js() -> &'static str {
    r##"
(function () {
  if (window.__wjBankMatchBound) return;
  window.__wjBankMatchBound = true;

  document.addEventListener('click', function (ev) {
    var t = ev.target;
    if (!t || !t.closest) return;
    var btn = t.closest('[data-wj-bank-match]');
    if (!btn) return;
    ev.preventDefault();
    var lineId = btn.getAttribute('data-line-id') || '';
    if (!lineId) return;
    var cell = btn.closest('[data-wj-bank-match-cell]') || btn.parentElement;
    var sel = cell ? cell.querySelector('[data-wj-bank-match-je]') : null;
    var jeId = (sel && sel.value) ? sel.value : 'seed-je-ops';
    var token = localStorage.getItem('ledgerkit_token') || '';
    if (!token) {
      btn.textContent = 'Sign in';
      return;
    }
    btn.disabled = true;
    btn.textContent = 'Matching…';
    var api = window.LEDGERKIT_API || '';
    fetch(api + '/api/v1/bank-imports/lines/' + encodeURIComponent(lineId) + '/match', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json', Authorization: 'Bearer ' + token },
      body: JSON.stringify({ journal_entry_id: jeId })
    }).then(function (res) {
      if (!res.ok) {
        btn.disabled = false;
        btn.textContent = 'Retry';
        return;
      }
      // Seed adapter match is ephemeral (no WJ static mut) — keep optimistic Matched UI.
      if (cell) {
        cell.outerHTML = '<span class="muted">Matched</span>';
      } else {
        btn.removeAttribute('data-wj-bank-match');
        btn.outerHTML = '<span class="muted">Matched</span>';
      }
      var loadReg = document.getElementById('loadCheckbook')
        || document.querySelector('[data-wj-render-kind="checkbook"]');
      if (loadReg) { try { loadReg.click(); } catch (e) {} }
    }).catch(function () {
      btn.disabled = false;
      btn.textContent = 'Retry';
    });
  });
})();
"##
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unmatched_renders_match_button_and_je_select() {
        let html = BankMatchRow::new(
            "bank~1000~demo-fit-01",
            true,
            r#"<option value="seed-je-ops">Ops</option><option value="seed-je-inv-1001">INV-1001</option>"#,
        )
        .render();
        assert!(html.contains("data-wj-bank-match"));
        assert!(html.contains("data-wj-bank-match-je"));
        assert!(html.contains("data-line-id=\"bank~1000~demo-fit-01\""));
        assert!(html.contains("seed-je-inv-1001"));
        assert!(html.contains("Match"));
    }

    #[test]
    fn matched_renders_label() {
        let html = BankMatchRow::new("bank~1000~x", false, "").render();
        assert!(html.contains("Matched"));
        assert!(!html.contains("data-wj-bank-match"));
    }

    #[test]
    fn runtime_posts_selected_journal_entry() {
        let js = bank_match_runtime_js();
        assert!(js.contains("bank-imports/lines/"));
        assert!(js.contains("/match"));
        assert!(js.contains("data-wj-bank-match-je"));
        assert!(js.contains("journal_entry_id"));
        assert!(js.contains("loadCheckbook"));
        assert!(!js.contains("journal_entry_id: 'seed-je-ops'"));
    }
}
