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
}

impl Renderable for JsonPost {
    fn render(&self) -> String {
        format!(
            r##"<button type="button" id="{id}" class="{class} wj-json-post" data-wj-json-post data-wj-fetch-path="{path}" data-wj-body-sel="{body}" data-wj-token-key="{token}" data-wj-status-sel="{status}" data-wj-success-message="{ok}">{label}</button>"##,
            id = self.id,
            class = self.class_name,
            path = self.path,
            body = self.body_sel,
            token = self.token_key,
            status = self.status_sel,
            ok = self.success_message,
            label = self.label,
        )
    }
}

/// Framework runtime: Bearer POST body from selector → status slot.
pub fn json_post_runtime_js() -> &'static str {
    r##"
(function () {
  if (window.__wjJsonPostBound) return;
  window.__wjJsonPostBound = true;
  document.addEventListener('click', function (ev) {
    var t = ev.target;
    if (!t || !t.closest) return;
    var btn = t.closest('[data-wj-json-post]');
    if (!btn) return;
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
      if (!res.ok) {
        show('Could not post (' + res.status + ')', true);
        return;
      }
      show(okMsg, false);
    }).catch(function (err) {
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
}
