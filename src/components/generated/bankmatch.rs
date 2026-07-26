//! BankMatch — Match action for unmatched bank lines (ADR-002 / R2.3).
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
}

impl BankMatchRow {
    #[inline]
    pub fn new(line_id: impl Into<String>, unmatched: bool) -> BankMatchRow {
        BankMatchRow {
            line_id: line_id.into(),
            unmatched,
        }
    }
}

impl Renderable for BankMatchRow {
    #[inline]
    fn render(&self) -> String {
        if self.unmatched {
            format!(
                r#"<button type="button" class="btn-secondary" data-wj-bank-match data-line-id="{id}">Match</button>"#,
                id = self.line_id
            )
        } else {
            r#"<span class="muted">Matched</span>"#.to_string()
        }
    }
}

/// Click Match → POST /api/v1/bank-imports/lines/{id}/match → refresh bank + register.
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
      body: JSON.stringify({ journal_entry_id: 'seed-je-ops' })
    }).then(function (res) {
      if (!res.ok) {
        btn.disabled = false;
        btn.textContent = 'Retry';
        return;
      }
      // Seed adapter match is ephemeral (no WJ static mut) — keep optimistic Matched UI.
      btn.removeAttribute('data-wj-bank-match');
      btn.outerHTML = '<span class="muted">Matched</span>';
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
    fn unmatched_renders_match_button() {
        let html = BankMatchRow::new("bank~1000~demo-fit-01", true).render();
        assert!(html.contains("data-wj-bank-match"));
        assert!(html.contains("data-line-id=\"bank~1000~demo-fit-01\""));
        assert!(html.contains("Match"));
    }

    #[test]
    fn matched_renders_label() {
        let html = BankMatchRow::new("bank~1000~x", false).render();
        assert!(html.contains("Matched"));
        assert!(!html.contains("data-wj-bank-match"));
    }

    #[test]
    fn runtime_posts_match_path() {
        let js = bank_match_runtime_js();
        assert!(js.contains("bank-imports/lines/"));
        assert!(js.contains("/match"));
        assert!(js.contains("data-wj-bank-match"));
                assert!(js.contains("loadCheckbook"));
        assert!(js.contains("outerHTML") || js.contains("Matched"));
    }
}
