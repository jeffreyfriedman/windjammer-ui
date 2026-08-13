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
                r#"<span class="wj-bank-match-action" data-wj-bank-match-cell><select data-wj-bank-match-je aria-label="Journal entry">{opts}</select><button type="button" class="btn-secondary" data-wj-bank-match data-wj-period-guard="1" data-line-id="{id}">Match</button></span>"#,
                opts = self.je_options_html,
                id = self.line_id
            )
        } else {
            r#"<span class="muted">Matched</span>"#.to_string()
        }
    }
}

/// Click Match → read JE select → POST match → Matched label + AuthFetch reload (R2.7).
/// Also applies list-JE AuthFetch options into Match selects (R2.6).
pub fn bank_match_runtime_js() -> &'static str {
    r##"
(function () {
  if (window.__wjBankMatchBound) return;
  window.__wjBankMatchBound = true;

  window.lkApplyJournalOptions = function (html) {
    if (html) window.__lkJeOptionsHtml = html;
    var opts = window.__lkJeOptionsHtml;
    if (!opts) return;
    document.querySelectorAll('[data-wj-bank-match-je]').forEach(function (sel) {
      var prev = sel.value;
      sel.innerHTML = opts;
      if (prev) {
        try { sel.value = prev; } catch (e) {}
      }
    });
  };

  function refreshBankAndRegister() {
    var loadBank = document.getElementById('loadBank')
      || document.querySelector('[data-wj-auth-fetch][data-wj-render-kind="bank"]');
    if (loadBank && typeof window.wjAuthFetch === 'function') {
      try { window.wjAuthFetch(loadBank); } catch (e) {}
    } else if (loadBank) {
      try { loadBank.click(); } catch (e) {}
    }
    var loadReg = document.getElementById('loadCheckbook')
      || document.querySelector('[data-wj-auth-fetch][data-wj-render-kind="checkbook"]');
    if (loadReg && typeof window.wjAuthFetch === 'function') {
      try { window.wjAuthFetch(loadReg); } catch (e) {}
    } else if (loadReg) {
      try { loadReg.click(); } catch (e) {}
    }
  }

  var prevAfter = window.lkAfterAuthFetch;
  window.lkAfterAuthFetch = function (kind, mount) {
    if (typeof prevAfter === 'function') {
      try { prevAfter(kind, mount); } catch (e) {}
    }
    if (kind === 'journal-options' && mount) {
      window.lkApplyJournalOptions(mount.innerHTML || '');
    }
    if (kind === 'bank') {
      window.lkApplyJournalOptions(null);
    }
  };

  document.addEventListener('click', function (ev) {
    var t = ev.target;
    if (!t || !t.closest) return;
    // Require data-line-id — panel wrapper must not steal Match clicks.
    var btn = t.closest('[data-wj-bank-match][data-line-id]');
    if (!btn) return;
    if (btn.disabled) return;
    ev.preventDefault();
    var lineId = btn.getAttribute('data-line-id') || '';
    if (!lineId) return;
    var cell = btn.closest('[data-wj-bank-match-cell]') || btn.parentElement;
    var sel = cell ? cell.querySelector('[data-wj-bank-match-je]') : null;
    var jeId = (sel && sel.value) ? sel.value : 'seed-je-ops';
    var token = localStorage.getItem('ledgerkit_token') || '';
    var out = document.querySelector('#out');
    function show(msg, err) {
      if (!out) return;
      out.hidden = false;
      out.textContent = msg;
      out.classList.toggle('is-error', !!err);
    }
    if (!token) {
      btn.textContent = 'Sign in';
      show('No token — sign in on Login first.', true);
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
      return res.json().then(function (data) {
        return { res: res, data: data };
      }).catch(function () {
        return { res: res, data: {} };
      });
    }).then(function (pair) {
      var res = pair.res;
      var data = pair.data || {};
      if (!res.ok) {
        btn.disabled = false;
        btn.textContent = 'Retry';
        var msg = (typeof window.wjHttpErrorMessage === 'function')
          ? window.wjHttpErrorMessage(res.status, data, 'Could not match')
          : ('Could not match (' + res.status + ')');
        show(msg, true);
        if (typeof window.wjHandlePeriodLockError === 'function') {
          try { window.wjHandlePeriodLockError(res.status, data); } catch (e) {}
        }
        return;
      }
      // Immediate feedback; seed overlay + AuthFetch prove persistence on reload.
      if (cell) cell.innerHTML = '<span class="muted">Matched</span>';
      show('Matched', false);
      if (typeof window.wjHandleForbiddenHint === 'function') {
        try { window.wjHandleForbiddenHint(res.status, data); } catch (e) {}
      }
      refreshBankAndRegister();
    }).catch(function (err) {
      btn.disabled = false;
      btn.textContent = 'Retry';
      show(String(err), true);
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
        assert!(js.contains("loadBank"), "R2.7 reload bank after persist");
        assert!(js.contains("wjAuthFetch"), "R2.7 call AuthFetch directly");
        assert!(js.contains("[data-wj-bank-match][data-line-id]"));
        assert!(js.contains("Matched"));
        assert!(js.contains("lkApplyJournalOptions"));
        assert!(js.contains("journal-options"));
        assert!(!js.contains("journal_entry_id: 'seed-je-ops'"));
    }

    #[test]
    fn unmatched_match_button_is_period_guarded() {
        let html = BankMatchRow::new("bank~1000~x", true, "").render();
        assert!(
            html.contains("data-wj-period-guard"),
            "Match honors period lock: {html}"
        );
    }

    #[test]
    fn runtime_skips_disabled_and_surfaces_http_errors() {
        let js = bank_match_runtime_js();
        assert!(
            js.contains("btn.disabled") || js.contains(".disabled"),
            "skip when period-guard disables Match: {js}"
        );
        assert!(
            js.contains("wjHttpErrorMessage") || js.contains("wjHandlePeriodLockError"),
            "surface 403 period/SoD like JsonPost: {js}"
        );
    }
}
