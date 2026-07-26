//! AccountRail — Business workspace account list with balances (ADR-002 / R1.2).
//! Hand-maintained. Always SKIP_WJ_REGEN=1.
//! `.wj` source: `components_wj/accountrail.wj` (structural API).

use super::traits::Renderable;

#[derive(Clone, Debug)]
pub struct AccountRailItem {
    pub code: String,
    pub label: String,
    pub balance_html: String,
    pub active: bool,
}

impl AccountRailItem {
    pub fn new(
        code: impl Into<String>,
        label: impl Into<String>,
        balance_html: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            label: label.into(),
            balance_html: balance_html.into(),
            active: false,
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

#[derive(Clone, Debug)]
pub struct AccountRail {
    pub items: Vec<AccountRailItem>,
    pub hint: String,
}

impl AccountRail {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            hint: "Select an account to load its register.".to_string(),
        }
    }

    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = hint.into();
        self
    }

    pub fn item(mut self, item: AccountRailItem) -> Self {
        self.items.push(item);
        self
    }
}

impl Default for AccountRail {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for AccountRail {
    fn render(&self) -> String {
        let body = if self.items.is_empty() {
            r#"<li class="muted lk-empty">Load accounts to see balances.</li>"#.to_string()
        } else {
            self.items
                .iter()
                .map(|i| {
                    let cls = if i.active {
                        "wj-account-rail-item rail-item is-active"
                    } else {
                        "wj-account-rail-item rail-item"
                    };
                    format!(
                        r#"<li><button type="button" class="{cls}" data-code="{code}" data-wj-account-rail-item><span class="wj-account-rail-label">{label}</span><span class="wj-account-rail-bal lk-num">{bal}</span></button></li>"#,
                        cls = cls,
                        code = i.code,
                        label = i.label,
                        bal = i.balance_html,
                    )
                })
                .collect::<Vec<_>>()
                .join("")
        };
        format!(
            r##"<div class="wj-account-rail" data-wj-account-rail>
<ul class="rail-list" id="accountRailList">{body}</ul>
<p class="muted wj-account-rail-hint">{hint}</p>
</div>"##,
            body = body,
            hint = self.hint
        )
    }
}

/// Click rail item → mark active, scope checkbook/bank fetch to account, reload register.
/// Also re-syncs title after AuthFetch replaces the checkbook host (R1.3 / R2).
pub fn account_rail_runtime_js() -> &'static str {
    r##"
(function () {
  if (window.__wjAccountRailBound) return;
  window.__wjAccountRailBound = true;

  window.lkSyncCheckbookTitle = function () {
    var active = document.querySelector('[data-wj-account-rail-item].is-active .wj-account-rail-label');
    if (!active) return;
    var label = active.textContent.trim();
    if (!label) return;
    document.querySelectorAll('.checkbook-account, [data-wj-checkbook-account]').forEach(function (el) {
      el.textContent = label;
    });
  };

  /** R2: rewrite checkbook/bank AuthFetch paths + write-check bank code for selected account. */
  window.lkScopeRegisterAccount = function (code) {
    var acct = String(code || '').trim() || '1000';
    document.querySelectorAll('[data-wj-auth-fetch][data-wj-render-kind="checkbook"],[data-wj-auth-fetch][data-wj-render-kind="bank"]').forEach(function (btn) {
      var path = btn.getAttribute('data-wj-fetch-path') || '';
      if (!path) return;
      if (/[?&]account=/.test(path)) {
        path = path.replace(/([?&]account=)[^&]*/, '$1' + encodeURIComponent(acct));
      } else {
        path += (path.indexOf('?') >= 0 ? '&' : '?') + 'account=' + encodeURIComponent(acct);
      }
      btn.setAttribute('data-wj-fetch-path', path);
    });
    document.querySelectorAll('[data-wj-write-check]').forEach(function (root) {
      root.setAttribute('data-wj-bank-code', acct);
    });
  };

  var prevAfter = window.lkAfterAuthFetch;
  window.lkAfterAuthFetch = function (kind, mount) {
    if (typeof prevAfter === 'function') {
      try { prevAfter(kind, mount); } catch (e) {}
    }
    if (kind === 'checkbook') window.lkSyncCheckbookTitle();
  };

  document.addEventListener('click', function (ev) {
    var t = ev.target;
    if (!t || !t.closest) return;
    var btn = t.closest('[data-wj-account-rail-item]');
    if (!btn) return;
    var root = btn.closest('[data-wj-account-rail]');
    if (!root) return;
    root.querySelectorAll('[data-wj-account-rail-item]').forEach(function (b) {
      b.classList.toggle('is-active', b === btn);
    });
    var code = btn.getAttribute('data-code') || '1000';
    window.lkScopeRegisterAccount(code);
    window.lkSyncCheckbookTitle();
    var load = document.getElementById('loadCheckbook')
      || document.querySelector('[data-wj-render-kind="checkbook"]');
    if (load) { try { load.click(); } catch (e) {} }
    var loadBank = document.getElementById('loadBank')
      || document.querySelector('[data-wj-render-kind="bank"]');
    if (loadBank) { try { loadBank.click(); } catch (e) {} }
  });
})();
"##
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_balances_and_active() {
        let html = AccountRail::new()
            .item(
                AccountRailItem::new("1000", "1000 · Cash", "$9,802.00").active(true),
            )
            .item(AccountRailItem::new("2000", "2000 · AP", "−$1,806.00"))
            .render();
        assert!(html.contains("data-wj-account-rail"));
        assert!(html.contains("wj-account-rail-bal"));
        assert!(html.contains("$9,802.00"));
        assert!(html.contains("is-active"));
        assert!(html.contains("data-code=\"1000\""));
    }

    #[test]
    fn runtime_scopes_register_fetch_to_account() {
        let js = account_rail_runtime_js();
        assert!(js.contains("lkScopeRegisterAccount"));
        assert!(js.contains("data-wj-fetch-path"));
        assert!(js.contains("account="));
        assert!(js.contains("data-wj-bank-code"));
        assert!(js.contains("checkbook"));
        assert!(js.contains("bank"));
    }
}
