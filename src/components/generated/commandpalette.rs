#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;

/// One jump target in a command palette.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct CommandPaletteItem {
    pub label: String,
    pub href: String,
}

impl CommandPaletteItem {
    #[inline]
    pub fn new(label: impl Into<String>, href: impl Into<String>) -> CommandPaletteItem {
        CommandPaletteItem {
            label: label.into(),
            href: href.into(),
        }
    }
}

/// ⌘K-style command palette. Items baked into HTML; use [`command_palette_runtime_js`].
/// Source: `components_wj/commandpalette.wj`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct CommandPalette {
    pub id: String,
    pub items: Vec<CommandPaletteItem>,
    pub tip: String,
    pub open: bool,
}

impl CommandPalette {
    #[inline]
    pub fn new() -> CommandPalette {
        CommandPalette {
            id: "cmdPalette".to_string(),
            items: Vec::new(),
            tip: "Tip: ⌘K / Ctrl+K · Esc to close".to_string(),
            open: false,
        }
    }

    #[inline]
    pub fn id(mut self, id: impl Into<String>) -> CommandPalette {
        self.id = id.into();
        self
    }

    #[inline]
    pub fn tip(mut self, tip: impl Into<String>) -> CommandPalette {
        self.tip = tip.into();
        self
    }

    #[inline]
    pub fn open(mut self, open: bool) -> CommandPalette {
        self.open = open;
        self
    }

    #[inline]
    pub fn item(mut self, item: CommandPaletteItem) -> CommandPalette {
        self.items.push(item);
        self
    }
}

impl Renderable for CommandPalette {
    #[inline]
    fn render(&self) -> String {
        let mut list = String::new();
        for item in &self.items {
            list.push_str(&format!(
                "<button type=\"button\" class=\"cmd-item\" data-href=\"{}\" data-label=\"{}\">{}</button>",
                item.href, item.label, item.label
            ));
        }
        let hidden = if self.open { "" } else { " hidden" };
        format!(
            "<div id=\"{}\" class=\"cmd-palette wj-command-palette\"{} role=\"dialog\" aria-modal=\"true\" aria-label=\"Command palette\"><div class=\"cmd-backdrop\" data-wj-cmd-close></div><div class=\"cmd-panel\"><input id=\"cmdInput\" type=\"search\" placeholder=\"Jump to… (Esc to close)\" autocomplete=\"off\" data-wj-cmd-filter/><div id=\"cmdList\" class=\"cmd-list\">{}</div><p class=\"cmd-hint muted\">{}</p></div></div>",
            self.id, hidden, list, self.tip
        )
    }
}

/// Framework runtime for [`CommandPalette`] (open / close / filter / ⌘K).
pub fn command_palette_runtime_js() -> &'static str {
    r##"
(function () {
  function root() { return document.getElementById('cmdPalette'); }
  function input() { return document.getElementById('cmdInput'); }
  function items() {
    const list = document.getElementById('cmdList');
    return list ? Array.from(list.querySelectorAll('.cmd-item')) : [];
  }
  window.wjCmdClose = function () {
    const el = root();
    if (el) el.hidden = true;
  };
  window.wjCmdFilter = function (q) {
    const needle = String(q || '').toLowerCase();
    let visible = 0;
    items().forEach((btn) => {
      const label = (btn.getAttribute('data-label') || btn.textContent || '').toLowerCase();
      const show = !needle || label.includes(needle);
      btn.hidden = !show;
      if (show) visible++;
    });
    const list = document.getElementById('cmdList');
    if (!list) return;
    let empty = list.querySelector('.cmd-empty');
    if (visible === 0) {
      if (!empty) {
        empty = document.createElement('p');
        empty.className = 'muted cmd-empty';
        empty.textContent = 'No matches';
        list.appendChild(empty);
      }
    } else if (empty) {
      empty.remove();
    }
  };
  window.wjCmdOpen = function () {
    const el = root();
    const inp = input();
    if (!el) return;
    el.hidden = false;
    if (inp) { inp.value = ''; window.wjCmdFilter(''); inp.focus(); }
  };
  document.addEventListener('click', (e) => {
    const t = e.target;
    if (!(t instanceof Element)) return;
    if (t.closest('[data-wj-cmd-close]')) { window.wjCmdClose(); return; }
    if (t.closest('[data-wj-cmd-open]')) { e.preventDefault(); window.wjCmdOpen(); return; }
    const item = t.closest('.cmd-item');
    if (item && root() && !root().hidden) {
      const href = item.getAttribute('data-href');
      if (href) { location.hash = href; window.wjCmdClose(); }
    }
  });
  document.addEventListener('input', (e) => {
    const t = e.target;
    if (t instanceof HTMLInputElement && t.hasAttribute('data-wj-cmd-filter')) {
      window.wjCmdFilter(t.value);
    }
  });
  document.addEventListener('keydown', (e) => {
    if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
      e.preventDefault();
      const el = root();
      if (el && !el.hidden) window.wjCmdClose(); else window.wjCmdOpen();
    } else if (e.key === 'Escape') {
      window.wjCmdClose();
    }
  });
})();
"##
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_palette_renders_items_and_hooks() {
        let html = CommandPalette::new()
            .tip("Tip: ⌘K")
            .item(CommandPaletteItem::new("Go to Home", "#/"))
            .item(CommandPaletteItem::new(
                "Trial balance",
                "#/reports/trial-balance",
            ))
            .render();
        assert!(html.contains("wj-command-palette"));
        assert!(html.contains("cmdPalette"));
        assert!(html.contains("data-wj-cmd-close"));
        assert!(html.contains("data-wj-cmd-filter"));
        assert!(html.contains("data-href=\"#/\""));
        assert!(html.contains("Go to Home"));
        assert!(html.contains("hidden"));
    }

    #[test]
    fn runtime_js_exposes_wj_cmd_api() {
        let js = command_palette_runtime_js();
        assert!(js.contains("wjCmdOpen"));
        assert!(js.contains("wjCmdClose"));
        assert!(!js.contains("lkOpenPalette"));
    }
}
