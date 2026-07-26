//! AuthFetch — declarative auth-gated fetch button (LedgerKit F3).
//! Hand-maintained until Windjammer owned-string / compose codegen is green.
//! Source: `src/components_wj/authfetch.wj`. Always build with SKIP_WJ_REGEN=1.

use super::traits::Renderable;

#[derive(Clone, Debug)]
pub struct AuthFetch {
    pub id: String,
    pub label: String,
    pub path: String,
    pub kind: String,
    pub mount: String,
    pub class_name: String,
}

impl AuthFetch {
    pub fn new(path: impl Into<String>, kind: impl Into<String>) -> Self {
        Self {
            id: "authFetch".to_string(),
            label: "Load".to_string(),
            path: path.into(),
            kind: kind.into(),
            mount: "#tableMount".to_string(),
            class_name: "btn-secondary".to_string(),
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

    pub fn mount(mut self, mount: impl Into<String>) -> Self {
        self.mount = mount.into();
        self
    }

    pub fn class_name(mut self, class_name: impl Into<String>) -> Self {
        self.class_name = class_name.into();
        self
    }
}

impl Renderable for AuthFetch {
    fn render(&self) -> String {
        format!(
            "<button type=\"button\" id=\"{}\" class=\"{} wj-auth-fetch\" data-wj-auth-fetch data-wj-fetch-path=\"{}\" data-wj-render-kind=\"{}\" data-wj-mount=\"{}\">{}</button>",
            self.id, self.class_name, self.path, self.kind, self.mount, self.label
        )
    }
}

/// Framework runtime: click `[data-wj-auth-fetch]` → Bearer GET → `lkRender[kind]`.
pub fn auth_fetch_runtime_js() -> &'static str {
    r##"
(function () {
  if (window.__wjAuthFetchBound) return;
  window.__wjAuthFetchBound = true;
  window.wjAuthFetch = async function (btn) {
    const path = btn.getAttribute('data-wj-fetch-path') || '';
    const kind = btn.getAttribute('data-wj-render-kind') || '';
    const mountSel = btn.getAttribute('data-wj-mount') || '#tableMount';
    const token = localStorage.getItem('ledgerkit_token') || '';
    const mount = document.querySelector(mountSel);
    if (!path || !kind) return;
    if (!token) {
      if (mount) mount.innerHTML = '<p class="err">Sign in first.</p>';
      return;
    }
    if (mount) mount.innerHTML = '<p class="muted">Loading…</p>';
    try {
      const res = await fetch((window.LEDGERKIT_API || '') + path, {
        headers: { Authorization: 'Bearer ' + token }
      });
      const data = await res.json();
      if (!res.ok) {
        if (mount) mount.innerHTML = '<p class="err">' + (data.error || res.status) + '</p>';
        return;
      }
      if (mount && window.lkRender && window.lkRender[kind]) {
        mount.innerHTML = window.lkRender[kind](data);
        if (typeof window.lkAfterAuthFetch === 'function') {
          try { window.lkAfterAuthFetch(kind, mount); } catch (err) {}
        }
      } else if (mount) {
        mount.innerHTML = '<p class="err">Renderer unavailable for ' + kind + '</p>';
      }
    } catch (e) {
      if (mount) mount.innerHTML = '<p class="err">' + (e && e.message ? e.message : e) + '</p>';
    }
  };
  document.addEventListener('click', (e) => {
    const t = e.target;
    if (!(t instanceof Element)) return;
    const btn = t.closest('[data-wj-auth-fetch]');
    if (btn) {
      e.preventDefault();
      window.wjAuthFetch(btn);
    }
  });
})();
"##
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_fetch_bakes_data_attrs() {
        let html = AuthFetch::new("/api/v1/parties", "parties")
            .id("loadParties")
            .label("Load parties")
            .render();
        assert!(html.contains("wj-auth-fetch"));
        assert!(html.contains("data-wj-auth-fetch"));
        assert!(html.contains("data-wj-fetch-path=\"/api/v1/parties\""));
    }

    #[test]
    fn auth_fetch_runtime_exposes_wj_auth_fetch() {
        assert!(auth_fetch_runtime_js().contains("wjAuthFetch"));
    }
}
