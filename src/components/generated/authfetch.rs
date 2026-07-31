#![allow(clippy::all)]
#![allow(noop_method_call)]
//! Regenerated from `components_wj/authfetch.wj` — Windjammer is source of truth.

#[allow(unused_imports)]
use super::*;

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
}

impl AuthFetch {
#[inline]
pub fn new(path: String, kind: String) -> AuthFetch {
        AuthFetch { id: "authFetch".to_string(), label: "Load".to_string(), path, kind, mount: "#tableMount".to_string(), class_name: "btn-secondary".to_string() }
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
pub fn mount(mut self, mount: String) -> AuthFetch {
        self.mount = mount;
        self
}
#[inline]
pub fn class_name(mut self, class_name: String) -> AuthFetch {
        self.class_name = class_name;
        self
}
}

impl Renderable for AuthFetch {
#[inline]
fn render(&self) -> String {
        "<button type=\"button\" id=\"".to_string() + &self.id.clone() + &String::from("\" class=\"") + &self.class_name.clone() + &String::from(" wj-auth-fetch\" data-wj-auth-fetch data-wj-fetch-path=\"") + &self.path.clone() + &String::from("\" data-wj-render-kind=\"") + &self.kind.clone() + &String::from("\" data-wj-mount=\"") + &self.mount.clone() + &String::from("\">") + &self.label.clone() + &String::from("</button>")
}
}

/// Framework runtime: Bearer GET → lkRender; drives #lkSyncBadge (D5).
#[inline]
pub fn auth_fetch_runtime_js() -> &'static str {
    "\n(function () {\n  if (window.__wjAuthFetchBound) return;\n  window.__wjAuthFetchBound = true;\n  window.lkSetSyncStatus = function (state) {\n    var el = document.getElementById('lkSyncBadge');\n    if (!el) return;\n    var s = state === 'syncing' ? 'syncing' : state === 'offline' ? 'offline' : 'synced';\n    el.setAttribute('data-lk-sync', s);\n    el.className = 'lk-sync-badge ' + (\n      s === 'syncing' ? 'lk-sync-syncing' : s === 'offline' ? 'lk-sync-offline' : 'lk-sync-synced'\n    );\n    el.textContent = s === 'syncing' ? 'Syncing…' : s === 'offline' ? 'Offline' : 'Synced';\n  };\n  window.wjAuthFetch = async function (btn) {\n    const path = btn.getAttribute('data-wj-fetch-path') || '';\n    const kind = btn.getAttribute('data-wj-render-kind') || '';\n    const mountSel = btn.getAttribute('data-wj-mount') || '#tableMount';\n    const token = localStorage.getItem('ledgerkit_token') || '';\n    const mount = document.querySelector(mountSel);\n    if (!path || !kind) return;\n    if (!token) {\n      if (mount) mount.innerHTML = '<p class=\"err\">Sign in first.</p>';\n      window.lkSetSyncStatus('offline');\n      return;\n    }\n    if (mount) mount.innerHTML = '<p class=\"muted\">Loading…</p>';\n    window.lkSetSyncStatus('syncing');\n    try {\n      const res = await fetch((window.LEDGERKIT_API || '') + path, {\n        headers: { Authorization: 'Bearer ' + token }\n      });\n      const data = await res.json();\n      if (!res.ok) {\n        if (mount) mount.innerHTML = '<p class=\"err\">' + (data.error || res.status) + '</p>';\n        window.lkSetSyncStatus('offline');\n        return;\n      }\n      if (mount && window.lkRender && window.lkRender[kind]) {\n        mount.innerHTML = window.lkRender[kind](data);\n        if (typeof window.lkAfterAuthFetch === 'function') {\n          try { window.lkAfterAuthFetch(kind, mount); } catch (err) {}\n        }\n      } else if (mount) {\n        mount.innerHTML = '<p class=\"err\">Renderer unavailable for ' + kind + '</p>';\n      }\n      window.lkSetSyncStatus('synced');\n    } catch (e) {\n      if (mount) mount.innerHTML = '<p class=\"err\">' + (e && e.message ? e.message : e) + '</p>';\n      window.lkSetSyncStatus('offline');\n    }\n  };\n  document.addEventListener('click', (e) => {\n    const t = e.target;\n    if (!(t instanceof Element)) return;\n    const btn = t.closest('[data-wj-auth-fetch]');\n    if (btn) {\n      e.preventDefault();\n      window.wjAuthFetch(btn);\n    }\n  });\n})();\n"
}

