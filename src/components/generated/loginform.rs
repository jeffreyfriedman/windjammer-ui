//! LoginForm — email/password session form.
//! Hand-maintained. Always SKIP_WJ_REGEN=1.
//! `.wj` source: `components_wj/loginform.wj`.

use super::form::FormField;
use super::traits::Renderable;

#[derive(Clone, Debug)]
pub struct LoginForm {
    pub path: String,
    pub token_key: String,
    pub email_value: String,
    pub password_value: String,
    pub lede: String,
}

impl LoginForm {
    pub fn new() -> Self {
        Self {
            path: "/auth/login".to_string(),
            token_key: "ledgerkit_token".to_string(),
            email_value: "owner@demo.local".to_string(),
            password_value: "dev".to_string(),
            lede: "Use the demo books account to load live KPIs and registers.".to_string(),
        }
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }

    pub fn token_key(mut self, key: impl Into<String>) -> Self {
        self.token_key = key.into();
        self
    }

    pub fn email_value(mut self, v: impl Into<String>) -> Self {
        self.email_value = v.into();
        self
    }

    pub fn password_value(mut self, v: impl Into<String>) -> Self {
        self.password_value = v.into();
        self
    }

    pub fn lede(mut self, v: impl Into<String>) -> Self {
        self.lede = v.into();
        self
    }
}

impl Default for LoginForm {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for LoginForm {
    fn render(&self) -> String {
        let email = FormField::new(
            "Email".to_string(),
            format!(
                r##"<input id="email" name="email" type="email" value="{v}" data-wj-login-email/>"##,
                v = self.email_value
            ),
        )
        .render();
        let password = FormField::new(
            "Password".to_string(),
            format!(
                r##"<input id="password" name="password" type="password" value="{v}" data-wj-login-password/>"##,
                v = self.password_value
            ),
        )
        .render();
        format!(
            r##"<div class="wj-login-form panel" data-wj-login-form data-wj-login-path="{path}" data-wj-token-key="{token}">
<p class="hub-kicker">Session</p>
<h2>Login</h2>
<p class="lede">{lede}</p>
{email}{password}
<div class="row"><button id="loginBtn" type="button" data-wj-login-submit>Sign in</button></div>
<p id="out" class="lk-status" role="status" hidden></p>
</div>"##,
            path = self.path,
            token = self.token_key,
            lede = self.lede,
            email = email,
            password = password,
        )
    }
}

/// Framework runtime: POST credentials → store access_token.
pub fn login_form_runtime_js() -> &'static str {
    r##"
(function () {
  if (window.__wjLoginFormBound) return;
  window.__wjLoginFormBound = true;
  document.addEventListener('click', function (ev) {
    var t = ev.target;
    if (!t || !t.closest) return;
    var btn = t.closest('[data-wj-login-submit]');
    if (!btn) return;
    var root = btn.closest('[data-wj-login-form]');
    if (!root) return;
    ev.preventDefault();
    var path = root.getAttribute('data-wj-login-path') || '/auth/login';
    var tokenKey = root.getAttribute('data-wj-token-key') || 'ledgerkit_token';
    var emailEl = root.querySelector('[data-wj-login-email]') || document.getElementById('email');
    var passEl = root.querySelector('[data-wj-login-password]') || document.getElementById('password');
    var out = root.querySelector('#out') || document.getElementById('out');
    var email = emailEl ? emailEl.value : '';
    var password = passEl ? passEl.value : '';
    if (out) { out.hidden = false; out.textContent = 'Signing in…'; out.classList.remove('is-error'); }
    fetch((window.LEDGERKIT_API || '') + path, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ email: email, password: password })
    }).then(function (res) { return res.json().then(function (data) { return { res: res, data: data }; }); })
      .then(function (r) {
        if (r.data && r.data.access_token) {
          try { localStorage.setItem(tokenKey, r.data.access_token); } catch (e) {}
          if (out) out.textContent = 'OK — token stored. Continue to Register or New journal.';
        } else if (out) {
          out.textContent = JSON.stringify(r.data);
        }
      }).catch(function (err) {
        if (out) { out.textContent = String(err); out.classList.add('is-error'); }
      });
  });
})();
"##
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_form_emits_markers_and_fields() {
        let html = LoginForm::new().render();
        assert!(html.contains("data-wj-login-form"));
        assert!(html.contains("data-wj-login-submit"));
        assert!(html.contains("data-wj-login-email"));
        assert!(html.contains("owner@demo.local"));
        assert!(html.contains("/auth/login"));
    }

    #[test]
    fn login_form_lede_is_overridable() {
        let html = LoginForm::new()
            .lede("Auditor scoped to trial balance.")
            .render();
        assert!(
            html.contains("Auditor scoped to trial balance."),
            "custom lede: {html}"
        );
        assert!(
            !html.contains("Use the demo books account"),
            "default lede must not remain when overridden: {html}"
        );
        assert!(html.contains("class=\"lede\""), "lede class: {html}");
    }
}
