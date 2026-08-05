//! BusinessWorkspace — single pane of glass + tabbed preference (ADR-002 / R1 / R1.1).
//! Hand-maintained. Always SKIP_WJ_REGEN=1.
//! `.wj` source: `components_wj/businessworkspace.wj` (structural API).

use super::traits::Renderable;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BusinessWorkspaceLayout {
    Pane,
    Tabs,
}

impl BusinessWorkspaceLayout {
    pub fn parse(raw: &str) -> Self {
        match raw.trim().to_ascii_lowercase().as_str() {
            "tabs" | "tab" | "tabbed" => Self::Tabs,
            _ => Self::Pane,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pane => "pane",
            Self::Tabs => "tabs",
        }
    }
}

#[derive(Clone, Debug)]
pub struct BusinessWorkspace {
    pub layout: BusinessWorkspaceLayout,
    pub rail_html: String,
    pub register_html: String,
    pub write_check_html: String,
    pub memorized_html: String,
    pub bank_html: String,
    pub active_tab: String,
}

impl BusinessWorkspace {
    pub fn new() -> Self {
        Self {
            layout: BusinessWorkspaceLayout::Pane,
            rail_html: String::new(),
            register_html: String::new(),
            write_check_html: String::new(),
            memorized_html: String::new(),
            bank_html: String::new(),
            active_tab: "register".to_string(),
        }
    }

    pub fn layout(mut self, layout: BusinessWorkspaceLayout) -> Self {
        self.layout = layout;
        self
    }

    /// String layout for Windjammer / hex adapters (`pane` | `tabs`).
    pub fn layout_key(mut self, key: impl Into<String>) -> Self {
        self.layout = BusinessWorkspaceLayout::parse(&key.into());
        self
    }

    pub fn rail_html(mut self, html: impl Into<String>) -> Self {
        self.rail_html = html.into();
        self
    }

    pub fn register_html(mut self, html: impl Into<String>) -> Self {
        self.register_html = html.into();
        self
    }

    pub fn write_check_html(mut self, html: impl Into<String>) -> Self {
        self.write_check_html = html.into();
        self
    }

    pub fn memorized_html(mut self, html: impl Into<String>) -> Self {
        self.memorized_html = html.into();
        self
    }

    pub fn bank_html(mut self, html: impl Into<String>) -> Self {
        self.bank_html = html.into();
        self
    }

    pub fn active_tab(mut self, id: impl Into<String>) -> Self {
        self.active_tab = id.into();
        self
    }
}

impl Default for BusinessWorkspace {
    fn default() -> Self {
        Self::new()
    }
}

fn layout_toggle_html(active: BusinessWorkspaceLayout) -> String {
    let pane_active = if active == BusinessWorkspaceLayout::Pane {
        " is-active"
    } else {
        ""
    };
    let tabs_active = if active == BusinessWorkspaceLayout::Tabs {
        " is-active"
    } else {
        ""
    };
    format!(
        r##"<div class="wj-bw-layout-toggle" data-wj-bw-layout-toggle role="group" aria-label="Business layout">
<button type="button" class="btn-secondary{pane}" data-wj-bw-layout="pane">Single pane</button>
<button type="button" class="btn-secondary{tabs}" data-wj-bw-layout="tabs">Tabbed</button>
</div>"##,
        pane = pane_active,
        tabs = tabs_active
    )
}

fn tab_active(active: &str, id: &str) -> &'static str {
    if active == id {
        " is-active"
    } else {
        ""
    }
}

fn tab_display(active: &str, id: &str) -> &'static str {
    if active == id {
        "block"
    } else {
        "none"
    }
}

impl Renderable for BusinessWorkspace {
    fn render(&self) -> String {
        let toggle = layout_toggle_html(self.layout);
        match self.layout {
            BusinessWorkspaceLayout::Tabs => {
                format!(
                    r##"<div class="wj-business-workspace" data-layout="tabs" data-wj-business-workspace>
{toggle}
<div class="wj-bw-tabbar" role="tablist">
<button type="button" class="wj-bw-tab{reg_a}" data-wj-bw-tab="register" role="tab">Checkbook</button>
<button type="button" class="wj-bw-tab{write_a}" data-wj-bw-tab="write" role="tab">Write check</button>
<button type="button" class="wj-bw-tab{mem_a}" data-wj-bw-tab="memorized" role="tab">Memorized</button>
<button type="button" class="wj-bw-tab{bank_a}" data-wj-bw-tab="bank" role="tab">Bank match</button>
</div>
<div class="wj-bw-tab-panes">
<div class="wj-bw-pane" data-tab="register" style="display:{reg}" role="tabpanel">{rail}{register}</div>
<div class="wj-bw-pane" data-tab="write" style="display:{write}" role="tabpanel">{write_html}</div>
<div class="wj-bw-pane" data-tab="memorized" style="display:{mem}" role="tabpanel">{memorized}</div>
<div class="wj-bw-pane" data-tab="bank" style="display:{bank}" role="tabpanel">{bank_html}</div>
</div>
</div>"##,
                    toggle = toggle,
                    reg_a = tab_active(&self.active_tab, "register"),
                    write_a = tab_active(&self.active_tab, "write"),
                    mem_a = tab_active(&self.active_tab, "memorized"),
                    bank_a = tab_active(&self.active_tab, "bank"),
                    reg = tab_display(&self.active_tab, "register"),
                    write = tab_display(&self.active_tab, "write"),
                    mem = tab_display(&self.active_tab, "memorized"),
                    bank = tab_display(&self.active_tab, "bank"),
                    rail = self.rail_html,
                    register = self.register_html,
                    write_html = self.write_check_html,
                    memorized = self.memorized_html,
                    bank_html = self.bank_html,
                )
            }
            BusinessWorkspaceLayout::Pane => format!(
                r##"<div class="wj-business-workspace" data-layout="pane" data-wj-business-workspace>
{toggle}
<aside class="wj-bw-rail">{rail}</aside>
<section class="wj-bw-register">{register}</section>
<section class="wj-bw-write">{write}</section>
<section class="wj-bw-memorized">{memorized}</section>
<section class="wj-bw-bank">{bank}</section>
</div>"##,
                toggle = toggle,
                rail = self.rail_html,
                register = self.register_html,
                write = self.write_check_html,
                memorized = self.memorized_html,
                bank = self.bank_html,
            ),
        }
    }
}

/// Persist layout preference and remount (product host wires hashchange / WASM).
pub fn business_workspace_runtime_js() -> &'static str {
    r##"
(function () {
  if (window.__wjBusinessWorkspaceBound) return;
  window.__wjBusinessWorkspaceBound = true;
  var KEY = 'ledgerkit_business_layout';
  document.addEventListener('click', function (ev) {
    var t = ev.target;
    if (!t || !t.closest) return;
    var layoutBtn = t.closest('[data-wj-bw-layout]');
    if (layoutBtn) {
      var layout = layoutBtn.getAttribute('data-wj-bw-layout') || 'pane';
      try { localStorage.setItem(KEY, layout); } catch (e) {}
      if (typeof location !== 'undefined') {
        if (location.hash !== '#/money') location.hash = '#/money';
        else if (typeof window.dispatchEvent === 'function') {
          window.dispatchEvent(new HashChangeEvent('hashchange'));
        } else { location.reload(); }
      }
      return;
    }
    var tabBtn = t.closest('[data-wj-bw-tab]');
    if (!tabBtn) return;
    var root = tabBtn.closest('[data-wj-business-workspace]');
    if (!root || root.getAttribute('data-layout') !== 'tabs') return;
    var id = tabBtn.getAttribute('data-wj-bw-tab') || 'register';
    root.querySelectorAll('[data-wj-bw-tab]').forEach(function (b) {
      b.classList.toggle('is-active', b.getAttribute('data-wj-bw-tab') === id);
    });
    root.querySelectorAll('.wj-bw-pane[data-tab]').forEach(function (p) {
      p.style.display = p.getAttribute('data-tab') === id ? 'block' : 'none';
    });
  });
})();
"##
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pane_renders_five_regions_including_bank() {
        let html = BusinessWorkspace::new()
            .rail_html("RAIL")
            .register_html("REG")
            .write_check_html("WC")
            .memorized_html("MEM")
            .bank_html("BANK")
            .render();
        assert!(html.contains("data-layout=\"pane\""));
        assert!(html.contains("wj-bw-rail") && html.contains("RAIL"));
        assert!(html.contains("wj-bw-register") && html.contains("REG"));
        assert!(html.contains("wj-bw-write") && html.contains("WC"));
        assert!(html.contains("wj-bw-memorized") && html.contains("MEM"));
        assert!(html.contains("wj-bw-bank") && html.contains("BANK"));
    }

    #[test]
    fn layout_key_parses_tabs_and_pane() {
        let tabs = BusinessWorkspace::new().layout_key("tabs").render();
        assert!(tabs.contains("data-layout=\"tabs\""), "{tabs}");
        let tabbed = BusinessWorkspace::new().layout_key("tabbed").render();
        assert!(tabbed.contains("data-layout=\"tabs\""), "{tabbed}");
        let pane = BusinessWorkspace::new().layout_key("nope").render();
        assert!(pane.contains("data-layout=\"pane\""), "{pane}");
    }

    #[test]
    fn tabs_include_bank_match() {
        let html = BusinessWorkspace::new()
            .layout(BusinessWorkspaceLayout::Tabs)
            .active_tab("bank")
            .bank_html("BANK")
            .render();
        assert!(html.contains("data-layout=\"tabs\""));
        assert!(html.contains("data-wj-bw-tab=\"bank\""));
        assert!(html.contains("display:block") && html.contains("BANK"));
    }
}
