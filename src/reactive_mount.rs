//! Mount-target helpers for ReactiveApp (host-testable; used by WASM hybrid shell).

use crate::simple_vnode::VNode;

/// Default DOM mount when no `.mount_target(...)` is set (legacy full-page shell).
pub fn default_mount_selector() -> &'static str {
    "#app"
}

/// Builder state for where ReactiveApp remounts HTML (LedgerKit: `#main`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MountTarget {
    selector: String,
}

impl Default for MountTarget {
    fn default() -> Self {
        Self {
            selector: default_mount_selector().to_string(),
        }
    }
}

impl MountTarget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mount_target(mut self, selector: impl Into<String>) -> Self {
        self.selector = selector.into();
        self
    }

    pub fn selector(&self) -> &str {
        &self.selector
    }
}

/// One-shot HTML for a VNode (Home pilot — no RAF loop).
pub fn paint_once_html(vnode: &VNode) -> String {
    crate::simple_renderer::render_to_html(vnode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_mount_selector_is_app() {
        assert_eq!(default_mount_selector(), "#app");
        assert_eq!(MountTarget::new().selector(), "#app");
    }

    #[test]
    fn mount_target_main_overrides_default() {
        let m = MountTarget::new().mount_target("#main");
        assert_eq!(m.selector(), "#main");
    }

    #[test]
    fn paint_once_html_uses_raw_html_for_main_pilot() {
        let html = paint_once_html(&VNode::raw_html(
            "<div class=\"panel home-hero\"><div class=\"kpi-grid\"></div></div>",
        ));
        assert!(html.contains("home-hero"));
        assert!(html.contains("kpi-grid"));
    }
}
