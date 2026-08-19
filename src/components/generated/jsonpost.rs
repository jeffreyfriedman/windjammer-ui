//! JsonPost — Bearer JSON POST from a body selector.
//! Hand-maintained. Always SKIP_WJ_REGEN=1.
//! `.wj` source: `components_wj/jsonpost.wj`.

use super::traits::Renderable;

#[derive(Clone, Debug)]
pub struct JsonPost {
    pub id: String,
    pub label: String,
    pub path: String,
    pub body_sel: String,
    pub token_key: String,
    pub status_sel: String,
    pub class_name: String,
    pub success_message: String,
    pub refresh_sel: String,
    pub period_guard: bool,
}

impl JsonPost {
    pub fn new(path: impl Into<String>, body_sel: impl Into<String>) -> Self {
        Self {
            id: "jsonPost".to_string(),
            label: "Submit".to_string(),
            path: path.into(),
            body_sel: body_sel.into(),
            token_key: "ledgerkit_token".to_string(),
            status_sel: "#out".to_string(),
            class_name: "btn-secondary".to_string(),
            success_message: "OK".to_string(),
            refresh_sel: String::new(),
            period_guard: false,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn token_key(mut self, key: impl Into<String>) -> Self {
        self.token_key = key.into();
        self
    }

    pub fn status_sel(mut self, sel: impl Into<String>) -> Self {
        self.status_sel = sel.into();
        self
    }

    pub fn class_name(mut self, class_name: impl Into<String>) -> Self {
        self.class_name = class_name.into();
        self
    }

    pub fn success_message(mut self, msg: impl Into<String>) -> Self {
        self.success_message = msg.into();
        self
    }

    /// After a successful POST, AuthFetch the given button selector (e.g. `#loadPeriod`).
    pub fn refresh(mut self, sel: impl Into<String>) -> Self {
        self.refresh_sel = sel.into();
        self
    }

    /// Honor PeriodBadge lock (`data-wj-period-guard`) via `wjApplyPeriodWriteGuard`.
    pub fn period_guard(mut self, enabled: bool) -> Self {
        self.period_guard = enabled;
        self
    }
}

impl Renderable for JsonPost {
    fn render(&self) -> String {
        let refresh_attr = if self.refresh_sel.is_empty() {
            String::new()
        } else {
            format!(" data-wj-refresh-sel=\"{}\"", self.refresh_sel)
        };
        let guard_attr = if self.period_guard {
            " data-wj-period-guard=\"1\""
        } else {
            ""
        };
        format!(
            r##"<button type="button" id="{id}" class="{class} wj-json-post" data-wj-json-post data-wj-fetch-path="{path}" data-wj-body-sel="{body}" data-wj-token-key="{token}" data-wj-status-sel="{status}" data-wj-success-message="{ok}"{refresh}{guard}>{label}</button>"##,
            id = self.id,
            class = self.class_name,
            path = self.path,
            body = self.body_sel,
            token = self.token_key,
            status = self.status_sel,
            ok = self.success_message,
            refresh = refresh_attr,
            guard = guard_attr,
            label = self.label,
        )
    }
}

/// Framework runtime: Bearer POST body from selector → status slot.
/// On success, optional `data-wj-refresh-sel` AuthFetch + `lkAfterJsonPost` hook.
pub fn json_post_runtime_js() -> &'static str {
    r##"
(function () {
  if (window.__wjJsonPostBound) return;
  window.__wjJsonPostBound = true;
  window.wjApplyPeriodWriteGuard = function () {
    var badge = document.querySelector('[data-period-state]');
    var state = badge ? (badge.getAttribute('data-period-state') || 'open') : 'open';
    state = String(state).toLowerCase();
    var blocked = state === 'locked';
    var warn = blocked || state === 'closed';
    document.querySelectorAll('[data-wj-period-guard]').forEach(function (el) {
      el.disabled = blocked;
      if (blocked) {
        el.setAttribute('aria-disabled', 'true');
      } else {
        el.removeAttribute('aria-disabled');
      }
    });
    document.querySelectorAll('[data-wj-period-warn]').forEach(function (el) {
      el.hidden = !warn;
      if (!warn) return;
      var lockedMsg = el.getAttribute('data-wj-period-warn-locked')
        || 'This period is locked. Reopen it from Close to post.';
      var closedMsg = el.getAttribute('data-wj-period-warn-closed')
        || 'This period is soft-closed. Posting is still allowed.';
      el.textContent = blocked ? lockedMsg : closedMsg;
      el.classList.toggle('is-error', blocked);
    });
  };
  window.wjHttpErrorKind = function (status, data) {
    if (status !== 403) return 'http';
    var msg = String((data && (data.error || data.message)) || '').toLowerCase();
    if (msg.indexOf('creator cannot approve') >= 0) return 'sod';
    if (msg.indexOf('grant scope') >= 0) return 'scope';
    if (msg.indexOf('workflow') >= 0) return 'workflow';
    if (msg.indexOf('posting not allowed') >= 0) return 'period';
    return 'http';
  };
  window.wjHttpErrorMessage = function (status, data, fallback) {
    var kind = window.wjHttpErrorKind(status, data);
    if (kind === 'sod') return 'You can\'t approve your own entry (segregation of duties).';
    if (kind === 'workflow') return 'This entry needs approval before posting.';
    if (kind === 'scope') return 'This report is outside the current auditor grant.';
    var body = data || {};
    var msg = body.error || body.message || '';
    if (msg) return String(msg);
    return (fallback || 'Request failed') + ' (' + status + ')';
  };
  window.wjIsPeriodLockError = function (status, data) {
    return window.wjHttpErrorKind(status, data) === 'period';
  };
  window.wjHandleForbiddenHint = function (status, data) {
    var kind = window.wjHttpErrorKind(status, data);
    var showHint = kind === 'sod' || kind === 'workflow';
    document.querySelectorAll('[data-wj-forbidden-hint]').forEach(function (el) {
      el.hidden = !showHint;
    });
  };
  window.wjHandleScopeDeniedHint = function (status, data) {
    var kind = window.wjHttpErrorKind(status, data);
    var show = kind === 'scope';
    document.querySelectorAll('[data-wj-scope-denied]').forEach(function (el) {
      el.hidden = !show;
    });
  };
  window.wjHandlePeriodLockError = function (status, data) {
    window.wjHandleForbiddenHint(status, data);
    if (!window.wjIsPeriodLockError(status, data)) return;
    var periodBtn = document.querySelector('[data-wj-auth-fetch][data-wj-render-kind="period"]');
    if (periodBtn && typeof window.wjAuthFetch === 'function') {
      try { window.wjAuthFetch(periodBtn); } catch (e) {}
    } else if (typeof window.wjApplyPeriodWriteGuard === 'function') {
      try { window.wjApplyPeriodWriteGuard(); } catch (e) {}
    }
  };
  document.addEventListener('click', function (ev) {
    var t = ev.target;
    if (!t || !t.closest) return;
    var btn = t.closest('[data-wj-json-post]');
    if (!btn) return;
    if (btn.disabled) return;
    ev.preventDefault();
    var path = btn.getAttribute('data-wj-fetch-path') || '';
    var bodySel = btn.getAttribute('data-wj-body-sel') || '';
    var tokenKey = btn.getAttribute('data-wj-token-key') || 'ledgerkit_token';
    var statusSel = btn.getAttribute('data-wj-status-sel') || '#out';
    var okMsg = btn.getAttribute('data-wj-success-message') || 'OK';
    var out = document.querySelector(statusSel);
    var bodyEl = bodySel ? document.querySelector(bodySel) : null;
    var token = '';
    try { token = localStorage.getItem(tokenKey) || ''; } catch (e) {}
    function show(msg, err) {
      if (!out) return;
      out.hidden = false;
      out.textContent = msg;
      out.classList.toggle('is-error', !!err);
    }
    if (!path) { show('Missing path', true); return; }
    if (!token) { show('No token — sign in on Login first.', true); return; }
    if (!bodyEl) { show('Missing request body', true); return; }
    show('Working…', false);
    fetch((window.LEDGERKIT_API || '') + path, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: 'Bearer ' + token
      },
      body: bodyEl.value
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
        show(window.wjHttpErrorMessage(res.status, data, 'Could not post'), true);
        window.wjHandlePeriodLockError(res.status, data);
        window.wjHandleScopeDeniedHint(res.status, data);
        return;
      }
      show(okMsg, false);
      window.wjHandleForbiddenHint(res.status, data);
      window.wjHandleScopeDeniedHint(res.status, data);
      var refreshSel = btn.getAttribute('data-wj-refresh-sel') || '';
      if (refreshSel && typeof window.wjAuthFetch === 'function') {
        var refreshBtn = document.querySelector(refreshSel);
        if (refreshBtn) {
          try { window.wjAuthFetch(refreshBtn); } catch (e) {}
        }
      }
      if (typeof window.lkAfterJsonPost === 'function') {
        try { window.lkAfterJsonPost(path, btn); } catch (e) {}
      }
    }).catch(function (err) {
      show(String(err), true);
    });
  });
  if (typeof window.wjApplyPeriodWriteGuard === 'function') {
    try { window.wjApplyPeriodWriteGuard(); } catch (e) {}
  }
})();
"##
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_post_emits_data_attrs() {
        let html = JsonPost::new("/api/v1/journal-entries", "#jeBody")
            .id("postJe")
            .label("Post journal entry")
            .success_message("Journal posted")
            .render();
        assert!(html.contains("data-wj-json-post"));
        assert!(html.contains("data-wj-fetch-path=\"/api/v1/journal-entries\""));
        assert!(html.contains("data-wj-body-sel=\"#jeBody\""));
        assert!(html.contains("id=\"postJe\""));
        assert!(html.contains("Journal posted") || html.contains("data-wj-success-message"));
    }

    #[test]
    fn json_post_refresh_emits_data_attr() {
        let html = JsonPost::new("/api/v1/fiscal-periods/close", "#periodCloseBody")
            .id("softClose")
            .refresh("#loadPeriod")
            .render();
        assert!(
            html.contains("data-wj-refresh-sel=\"#loadPeriod\"")
                || html.contains("data-wj-refresh-sel='#loadPeriod'"),
            "refresh sel: {html}"
        );
        let plain = JsonPost::new("/x", "#b").render();
        assert!(
            !plain.contains("data-wj-refresh-sel=\"#loadPeriod\""),
            "default has no refresh: {plain}"
        );
    }

    #[test]
    fn json_post_runtime_js_refreshes_auth_fetch_and_hooks() {
        let js = json_post_runtime_js();
        assert!(
            js.contains("data-wj-refresh-sel") || js.contains("wj-refresh-sel"),
            "runtime reads refresh sel: {js}"
        );
        assert!(
            js.contains("wjAuthFetch"),
            "runtime calls AuthFetch after success: {js}"
        );
        assert!(
            js.contains("lkAfterJsonPost"),
            "runtime exposes after-success hook: {js}"
        );
    }
}
