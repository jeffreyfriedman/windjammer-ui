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
        BtInspectorModel {
            title: "attack".to_string().to_string(),
            subtitle: "action".to_string().to_string(),
            hotspot: "Break when reached".to_string().to_string(),
            params_json: "{ \"dmg\":12 }".to_string().to_string(),
            breakpoint: true,
        }
    }
}

impl Renderable for BtInspectorModel {
    fn render(&self) -> String {
        let mut bp = "".to_string();
        if self.breakpoint {
            bp = "checked".to_string();
        }
        let mut panel = "".to_string();
        panel = format!("{}{}", panel, "<div class=\"bt-inspector-title\">");
        panel = format!("{}{}", panel, self.title.clone());
        panel = format!("{}{}", panel, "</div>");
        panel = format!("{}{}", panel, "<div class=\"bt-inspector-sub\">");
        panel = format!("{}{}", panel, self.subtitle.clone());
        panel = format!("{}{}", panel, "</div>");
        panel = format!(
            "{}{}",
            panel, "<label class=\"bt-toggle\"><input type=\"checkbox\" "
        );
        panel = format!("{}{}", panel, bp);
        panel = format!("{}{}", panel, " /> Breakpoint · ");
        panel = format!("{}{}", panel, self.hotspot.clone());
        panel = format!("{}{}", panel, "</label>");
        panel = format!(
            "{}{}",
            panel, "<label class=\"bt-field\"><span>Payload</span>"
        );
        panel = format!("{}{}", panel, "<textarea rows=\"6\">");
        panel = format!("{}{}", panel, self.params_json.clone());
        panel = format!("{}{}", panel, "</textarea></label>");
        let mut wrapped = "".to_string();
        wrapped = format!("{}{}", wrapped, "<section class=\"bt-properties-pane\">");
        wrapped = format!("{}{}", wrapped, panel);
        format!("{}{}", wrapped, "</section>".to_string())
    }
}

#[inline]
pub fn bt_visual_properties_styles() -> String {
    "\n    .bt-properties-pane {\n        width: 300px;\n        border-left: 1px solid rgba(255,255,255,0.08);\n        background: rgba(8,11,29,0.96);\n        color: #e5ecff;\n        padding: 16px;\n        display: flex;\n        flex-direction: column;\n        gap: 12px;\n    }\n    .bt-inspector-title {\n        font-size: 20px;\n        font-weight: 600;\n    }\n    .bt-inspector-sub {\n        color: #8492c7;\n        text-transform: uppercase;\n        font-size: 11px;\n        letter-spacing: 0.22em;\n    }\n    .bt-toggle {\n        display: flex;\n        gap: 8px;\n        font-size: 13px;\n    }\n    .bt-field textarea {\n        width: 100%;\n        margin-top: 6px;\n        border-radius: 8px;\n        border: 1px solid rgba(255,255,255,0.12);\n        background: rgba(5,10,38,0.88);\n        color: inherit;\n        font-family: JetBrains Mono, monospace;\n        font-size: 12px;\n        padding: 8px;\n    }\n    ".to_string()
}
