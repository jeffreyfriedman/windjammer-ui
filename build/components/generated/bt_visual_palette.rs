#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct BtPalettePanel;

impl Renderable for BtPalettePanel {
    #[inline]
    fn render(self) -> String {
        let mut mk = "".to_string();
        mk = format!("{}{}", mk, "<div class=\"bt-pane-label\">Compositor</div>");
        mk = format!("{}{}", mk, "<button class=\"bt-drag-chip\" draggable=\"true\" data-bt-kind=\"sequence\">Sequence</button>");
        mk = format!("{}{}", mk, "<button class=\"bt-drag-chip\" draggable=\"true\" data-bt-kind=\"selector\">Selector</button>");
        mk = format!("{}{}", mk, "<button class=\"bt-drag-chip\" draggable=\"true\" data-bt-kind=\"parallel\">Parallel</button>");
        mk = format!("{}{}", mk, "<div class=\"bt-pane-label\">Decorators</div>");
        mk = format!("{}{}", mk, "<button class=\"bt-drag-chip\" draggable=\"true\" data-bt-kind=\"inverter\">Inverter</button>");
        mk = format!("{}{}", mk, "<button class=\"bt-drag-chip\" draggable=\"true\" data-bt-kind=\"repeater\">Repeater</button>");
        mk = format!("{}{}", mk, "<button class=\"bt-drag-chip\" draggable=\"true\" data-bt-kind=\"until_fail\">Until Fail</button>");
        mk = format!("{}{}", mk, "<div class=\"bt-pane-label\">Leaves</div>");
        mk = format!("{}{}", mk, "<button class=\"bt-drag-chip\" draggable=\"true\" data-bt-kind=\"condition\">Condition</button>");
        mk = format!("{}{}", mk, "<button class=\"bt-drag-chip\" draggable=\"true\" data-bt-kind=\"action\">Action</button>");
        mk = format!("{}{}", mk, "<p class=\"bt-hint\">Drag onto the canvas grid. Host tooling wires callbacks to Rust/Windjammer delegates.</p>");
        let mut shell = "".to_string();
        shell = format!("{}{}", shell, "<aside class=\"bt-palette-pane\">");
        shell = format!("{}{}", shell, mk);
        format!("{}{}", shell, "</aside>")
    }
}

#[inline]
pub fn bt_visual_palette_styles() -> String {
    String::from("\n    .bt-palette-pane {\n        width: 236px;\n        border-right: 1px solid rgba(255,255,255,0.08);\n        background: rgba(9,13,34,0.96);\n        color: #e5e9ff;\n        padding: 14px;\n        overflow-y: auto;\n    }\n    .bt-pane-label {\n        font-size: 11px;\n        letter-spacing: 0.2em;\n        text-transform: uppercase;\n        color: #7580b3;\n        margin: 14px 0 8px 0;\n    }\n    .bt-drag-chip {\n        display: flex;\n        width: 100%;\n        margin-bottom: 6px;\n        padding: 8px;\n        border-radius: 10px;\n        border: 1px solid rgba(118,146,255,0.4);\n        background: rgba(32,43,103,0.55);\n        color: inherit;\n        cursor: grab;\n        text-align: left;\n    }\n    .bt-hint {\n        font-size: 11px;\n        line-height: 1.35;\n        color: #7580b3;\n        margin-top: 16px;\n    }\n    ")
}
