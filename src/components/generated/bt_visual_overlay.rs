#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
use super::bt_visual_canvas::bt_visual_canvas_styles;
use super::bt_visual_palette::bt_visual_palette_styles;
use super::bt_visual_properties::bt_visual_properties_styles;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct BtDebuggerOverlay;

impl Renderable for BtDebuggerOverlay {
#[inline]
fn render(self) -> String {
        let legend = "".to_string() + &"<ul class=\"bt-legend\">" + &"<li><span class=\"swatch bt-visual-running\"></span> Running</li>" + &"<li><span class=\"swatch bt-visual-success\"></span> Success</li>" + &"<li><span class=\"swatch bt-visual-failure\"></span> Failure</li>" + &"<li><span class=\"swatch bt-visual-idle\"></span> Idle</li>" + &"</ul>";
        let mut dbg = "".to_string();
        dbg = format!("{}{}", dbg, "<aside class=\"bt-debug-pane\">");
        dbg = format!("{}{}", dbg, "<strong>Visualizer</strong><p class=\"muted\">Shares color tokens with windjammer-game-core bt_visual_debug snapshots.</p>");
        dbg = format!("{}{}", dbg, legend);
        format!("{}{}", dbg, "</aside>")
}
}

#[inline]
pub fn bt_visual_debug_styles() -> String {
    String::from("\n    .bt-debug-pane {\n        border-top: 1px solid rgba(255,255,255,0.08);\n        padding: 10px 16px;\n        color: #d7ddfb;\n        background: rgba(6,12,42,0.9);\n        display: flex;\n        align-items: center;\n        gap: 18px;\n    }\n    .bt-debug-pane .muted {\n        margin: 0;\n        flex: 1;\n        color: #8b93c9;\n        font-size: 12px;\n    }\n    .bt-legend {\n        list-style: none;\n        display: flex;\n        gap: 12px;\n        margin: 0;\n        padding: 0;\n    }\n    .bt-legend li {\n        display: flex;\n        align-items: center;\n        gap: 6px;\n        font-size: 12px;\n    }\n    .bt-legend .swatch {\n        width: 16px;\n        height: 16px;\n        border-radius: 4px;\n        background: rgba(255,255,255,0.14);\n        display: inline-flex;\n        border: 1px dashed rgba(255,255,255,0.3);\n    }\n    .bt-shell {\n        display: flex;\n        flex-direction: column;\n        height: 100vh;\n        background: #060915;\n        color: #dfe5ff;\n    }\n    .bt-shell-body {\n        display: flex;\n        flex: 1;\n        min-height: 0;\n    }\n    ")
}

#[inline]
pub fn bt_behavior_editor_styles() -> String {
    let mut blob = "".to_string();
    blob = blob + &bt_visual_canvas_styles();
    blob = blob + &bt_visual_palette_styles();
    blob = blob + &bt_visual_properties_styles();
    blob + &bt_visual_debug_styles()
}

#[inline]
pub fn bt_editor_layout_stub() -> String {
    "".to_string() + &String::from("<div class=\"bt-shell\">") + &String::from("<header style=\"padding:14px;border-bottom:1px solid rgba(255,255,255,0.08)\">Windjammer · Visual Behavior Tree Shell</header>") + &String::from("<div class=\"bt-shell-body\">{{PALETTE}}</div>") + &String::from("{{DEBUG}}") + &String::from("</div>")
}

