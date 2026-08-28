#![allow(clippy::all)]
#![allow(noop_method_call)]
//! Mirrors `components_wj/syncbadge.wj` — Windjammer is source of truth.

use super::traits::Renderable;

/// Desktop sync status chrome — `auth_fetch_runtime_js` updates via `lkSetSyncStatus`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct SyncBadge {
    pub state: String,
    pub id: String,
}

impl SyncBadge {
    #[inline]
    pub fn new() -> SyncBadge {
        SyncBadge {
            state: "synced".to_string(),
            id: "lkSyncBadge".to_string(),
        }
    }

    /// Wire state: `synced` | `syncing` | `offline` (matches `lkSetSyncStatus`).
    #[inline]
    pub fn state(mut self, state: impl Into<String>) -> SyncBadge {
        self.state = state.into();
        self
    }

    #[inline]
    pub fn id(mut self, id: impl Into<String>) -> SyncBadge {
        self.id = id.into();
        self
    }
}

#[inline]
pub fn label_for(state: &str) -> String {
    let s = state.to_lowercase();
    if s == "syncing" {
        "Syncing…".to_string()
    } else if s == "offline" {
        "Offline".to_string()
    } else {
        "Synced".to_string()
    }
}

#[inline]
pub fn class_for(state: &str) -> String {
    let s = state.to_lowercase();
    if s == "syncing" {
        "lk-sync-syncing".to_string()
    } else if s == "offline" {
        "lk-sync-offline".to_string()
    } else {
        "lk-sync-synced".to_string()
    }
}

impl Renderable for SyncBadge {
    #[inline]
    fn render(&self) -> String {
        let state = self.state.to_lowercase();
        let label = label_for(&state);
        let cls = class_for(&state);
        format!(
            "<span id=\"{}\" class=\"lk-sync-badge {}\" data-lk-sync=\"{}\" role=\"status\" aria-live=\"polite\" title=\"Sync status (desktop always visible)\">{}</span>",
            self.id, cls, state, label
        )
    }
}
