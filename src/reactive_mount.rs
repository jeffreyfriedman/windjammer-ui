//! Mount-target helpers for ReactiveApp (host-testable; used by WASM hybrid shell).

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
}
