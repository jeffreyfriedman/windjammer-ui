#![allow(clippy::all)]
#![allow(noop_method_call)]
//! Hand-maintained from `components_wj/authfetch.wj` — SKIP_WJ_REGEN=1.
//! Note: avoid `use super::*` (ambiguous glob imports under wasm deny).
use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct AuthFetch {
    pub id: String,
    pub label: String,
    pub path: String,
    pub kind: String,
    pub mount: String,
    pub class_name: String,
    pub auto: bool,
}

impl AuthFetch {
    #[inline]
    pub fn new(path: String, kind: String) -> AuthFetch {
        AuthFetch {
            id: "authFetch".to_string(),
            label: "Load".to_string(),
            path,
            kind,
            mount: "#tableMount".to_string(),
            class_name: "btn-secondary".to_string(),
            auto: false,
        }
    }
    #[inline]
    pub fn id(mut self, id: String) -> AuthFetch {
        self.id = id;
        self
    }
    #[inline]
    pub fn label(mut self, label: String) -> AuthFetch {
        self.label = label;
        self
    }
    #[inline]
    pub fn mount(mut self, mount_sel: String) -> AuthFetch {
        self.mount = mount_sel;
        self
    }
    #[inline]
    pub fn class_name(mut self, class_name: String) -> AuthFetch {
        self.class_name = class_name;
        self
    }
    /// Hidden auto-fire on bind (`data-auto="1"`).
    #[inline]
    pub fn auto(mut self, enabled: bool) -> AuthFetch {
        self.auto = enabled;
        self
    }
}

impl Renderable for AuthFetch {
    #[inline]
    fn render(&self) -> String {
        let auto_attr = if self.auto {
            " data-auto=\"1\""
        } else {
            ""
        };
        "<button type=\"button\" id=\"".to_string()
            + &self.id
            + "\" class=\""
            + &self.class_name
            + " wj-auth-fetch\" data-wj-auth-fetch data-wj-fetch-path=\""
            + &self.path
            + "\" data-wj-render-kind=\""
            + &self.kind
            + "\" data-wj-mount=\""
            + &self.mount
            + "\""
            + auto_attr
            + ">"
            + &self.label
            + "</button>"
    }
}

/// Framework runtime: Bearer GET → lkRender; drives #lkSyncBadge (D5).
/// Auto chrome (`data-auto="1"`) fires on bind and skips Sign-in/Loading clobber.
#[inline]
pub fn auth_fetch_runtime_js() -> &'static str {
    r##"
(function () {
  if (window.__wjAuthFetchBound) return;
  window.__wjAuthFetchBound = true;
  window.lkSetSyncStatus = function (state) {
    var el = document.getElementById('lkSyncBadge');
    if (!el) return;
    var s = state === 'syncing' ? 'syncing' : state === 'offline' ? 'offline' : 'synced';
    el.setAttribute('data-lk-sync', s);
    el.className = 'lk-sync-badge ' + (
      s === 'syncing' ? 'lk-sync-syncing' : s === 'offline' ? 'lk-sync-offline' : 'lk-sync-synced'
    );
    el.textContent = s === 'syncing' ? 'Syncing…' : s === 'offline' ? 'Offline' : 'Synced';
  };
  window.wjAuthFetch = async function (btn) {
    const path = btn.getAttribute('data-wj-fetch-path') || '';
    const kind = btn.getAttribute('data-wj-render-kind') || '';
    const mountSel = btn.getAttribute('data-wj-mount') || '#tableMount';
    const auto = btn.getAttribute('data-auto') === '1';
    const token = localStorage.getItem('ledgerkit_token') || '';
    const mount = document.querySelector(mountSel);
    if (!path || !kind) return;
    if (!token) {
      if (!auto && mount) mount.innerHTML = '<p class="err">Sign in first.</p>';
      window.lkSetSyncStatus('offline');
      return;
    }
    if (!auto && mount) mount.innerHTML = '<p class="muted">Loading…</p>';
    window.lkSetSyncStatus('syncing');
    try {
      const res = await fetch((window.LEDGERKIT_API || '') + path, {
        headers: { Authorization: 'Bearer ' + token }
      });
      const data = await res.json();
      if (!res.ok) {
        if (!auto && mount) mount.innerHTML = '<p class="err">' + (data.error || res.status) + '</p>';
        window.lkSetSyncStatus('offline');
        return;
      }
      if (mount && window.lkRender && window.lkRender[kind]) {
        mount.innerHTML = window.lkRender[kind](data);
        if (typeof window.lkAfterAuthFetch === 'function') {
          try { window.lkAfterAuthFetch(kind, mount); } catch (err) {}
        }
        if (typeof window.wjApplyPeriodWriteGuard === 'function') {
          try { window.wjApplyPeriodWriteGuard(); } catch (err) {}
        }
      } else if (mount) {
        mount.innerHTML = '<p class="err">Renderer unavailable for ' + kind + '</p>';
      }
      window.lkSetSyncStatus('synced');
    } catch (e) {
      if (!auto && mount) mount.innerHTML = '<p class="err">' + (e && e.message ? e.message : e) + '</p>';
      window.lkSetSyncStatus('offline');
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
  document.querySelectorAll('[data-wj-auth-fetch][data-auto="1"]').forEach(function (el) {
    window.wjAuthFetch(el);
  });
})();
"##
}
