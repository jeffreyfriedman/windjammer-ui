#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct BtInspectorModel {
    pub title: String,
    pub subtitle: String,
    pub hotspot: String,
    pub params_json: String,
    pub breakpoint: bool,
}

impl BtInspectorModel {
#[inline]
pub fn demo() -> BtInspectorModel {
        BtInspectorModel { title: String::from("attack"), subtitle: String::from("action"), hotspot: String::from("Break when reached"), params_json: String::from("{ \"dmg\":12 }"), breakpoint: true }
}
}

impl Renderable for BtInspectorModel {

fn render(&self) -> String {
        let mut bp = String::new();
        if self.breakpoint {
            bp = String::from("checked");
        }
        let mut panel = String::new();
        { use std::fmt::Write as _; write!(panel, "{}", "<div class=\"bt-inspector-title\">").expect("a formatting trait implementation returned an error"); };
        { use std::fmt::Write as _; write!(panel, "{}", self.title).expect("a formatting trait implementation returned an error"); };
        { use std::fmt::Write as _; write!(panel, "{}", "</div>").expect("a formatting trait implementation returned an error"); };
        { use std::fmt::Write as _; write!(panel, "{}", "<div class=\"bt-inspector-sub\">").expect("a formatting trait implementation returned an error"); };
        { use std::fmt::Write as _; write!(panel, "{}", self.subtitle).expect("a formatting trait implementation returned an error"); };
        { use std::fmt::Write as _; write!(panel, "{}", "</div>").expect("a formatting trait implementation returned an error"); };
        { use std::fmt::Write as _; write!(panel, "{}", "<label class=\"bt-toggle\"><input type=\"checkbox\" ").expect("a formatting trait implementation returned an error"); };
        { use std::fmt::Write as _; write!(panel, "{}", bp).expect("a formatting trait implementation returned an error"); };
        { use std::fmt::Write as _; write!(panel, "{}", " /> Breakpoint · ").expect("a formatting trait implementation returned an error"); };
        { use std::fmt::Write as _; write!(panel, "{}", self.hotspot).expect("a formatting trait implementation returned an error"); };
        { use std::fmt::Write as _; write!(panel, "{}", "</label>").expect("a formatting trait implementation returned an error"); };
        { use std::fmt::Write as _; write!(panel, "{}", "<label class=\"bt-field\"><span>Payload</span>").expect("a formatting trait implementation returned an error"); };
        { use std::fmt::Write as _; write!(panel, "{}", "<textarea rows=\"6\">").expect("a formatting trait implementation returned an error"); };
        { use std::fmt::Write as _; write!(panel, "{}", self.params_json).expect("a formatting trait implementation returned an error"); };
        { use std::fmt::Write as _; write!(panel, "{}", "</textarea></label>").expect("a formatting trait implementation returned an error"); };
        let mut wrapped = String::new();
        { use std::fmt::Write as _; write!(wrapped, "{}", "<section class=\"bt-properties-pane\">").expect("a formatting trait implementation returned an error"); };
        { use std::fmt::Write as _; write!(wrapped, "{}", panel).expect("a formatting trait implementation returned an error"); };
        format!("{}{}", wrapped, "</section>")
}
}


pub fn bt_visual_properties_styles() -> String {
    String::from("\n    .bt-properties-pane {\n        width: 300px;\n        border-left: 1px solid rgba(255,255,255,0.08);\n        background: rgba(8,11,29,0.96);\n        color: #e5ecff;\n        padding: 16px;\n        display: flex;\n        flex-direction: column;\n        gap: 12px;\n    }\n    .bt-inspector-title {\n        font-size: 20px;\n        font-weight: 600;\n    }\n    .bt-inspector-sub {\n        color: #8492c7;\n        text-transform: uppercase;\n        font-size: 11px;\n        letter-spacing: 0.22em;\n    }\n    .bt-toggle {\n        display: flex;\n        gap: 8px;\n        font-size: 13px;\n    }\n    .bt-field textarea {\n        width: 100%;\n        margin-top: 6px;\n        border-radius: 8px;\n        border: 1px solid rgba(255,255,255,0.12);\n        background: rgba(5,10,38,0.88);\n        color: inherit;\n        font-family: JetBrains Mono, monospace;\n        font-size: 12px;\n        padding: 8px;\n    }\n    ")
}

