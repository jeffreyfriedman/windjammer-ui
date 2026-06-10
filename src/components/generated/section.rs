#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Section {
    pub icon: String,
    pub title: String,
    pub children: Vec<String>,
    pub collapsed: bool,
    pub accent_color: String,
    pub removable: bool,
    pub on_remove: String,
}

impl Section {
    #[inline]
    pub fn new(title: String) -> Section {
        Section {
            icon: "".to_string(),
            title,
            children: Vec::new(),
            collapsed: false,
            accent_color: "".to_string(),
            removable: false,
            on_remove: "".to_string(),
        }
    }
    #[inline]
    pub fn icon(mut self, icon: String) -> Section {
        self.icon = icon;
        self
    }
    #[inline]
    pub fn child(mut self, child: String) -> Section {
        self.children.push(child);
        self
    }
    #[inline]
    pub fn children(mut self, children: Vec<String>) -> Section {
        self.children = children;
        self
    }
    #[inline]
    pub fn collapsed(mut self, collapsed: bool) -> Section {
        self.collapsed = collapsed;
        self
    }
    #[inline]
    pub fn accent(mut self, color: String) -> Section {
        self.accent_color = color;
        self
    }
    #[inline]
    pub fn removable(mut self, on_remove: String) -> Section {
        self.removable = true;
        self.on_remove = on_remove;
        self
    }
}

impl Renderable for Section {
    #[inline]
    fn render(self) -> String {
        let collapse_icon: String = {
            if self.collapsed {
                String::from("▶")
            } else {
                String::from("▼")
            }
        };
        let content_class: String = {
            if self.collapsed {
                String::from("section-content collapsed")
            } else {
                String::from("section-content")
            }
        };
        let icon_html: String = {
            if self.icon != "" {
                format!("<span class='section-icon'>{}</span>", self.icon)
            } else {
                "".to_string()
            }
        };
        let accent_style: String = {
            if self.accent_color != "" {
                format!(" style='border-left: 3px solid {}'", self.accent_color)
            } else {
                "".to_string()
            }
        };
        let remove_btn: String = {
            if self.removable {
                format!(
                    "<button class='section-remove' onclick='{}'>×</button>",
                    self.on_remove
                )
            } else {
                "".to_string()
            }
        };
        let children_html = self.children.join("\n");
        format!("\n            <div class='wj-section'{}>\n                <div class='section-header'>\n                    <span class='collapse-arrow'>{}</span>\n                    {}\n                    <span class='section-title'>{}</span>\n                    {}\n                </div>\n                <div class='{}'>\n                    {}\n                </div>\n            </div>\n        ", accent_style, collapse_icon, icon_html, self.title, remove_btn, content_class, children_html)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct SectionGroup {
    pub sections: Vec<Section>,
    pub accordion: bool,
}

impl SectionGroup {
    #[inline]
    pub fn new() -> SectionGroup {
        SectionGroup {
            sections: Vec::new(),
            accordion: false,
        }
    }
    #[inline]
    pub fn section(mut self, section: Section) -> SectionGroup {
        self.sections.push(section);
        self
    }
    #[inline]
    pub fn accordion(mut self, accordion: bool) -> SectionGroup {
        self.accordion = accordion;
        self
    }
}

impl Renderable for SectionGroup {
    #[inline]
    fn render(self) -> String {
        let mut sections_html = "".to_string();
        for s in self.sections {
            sections_html = format!("{}{}{}", sections_html, s.clone().render(), "\n");
        }
        let class: String = {
            if self.accordion {
                String::from("section-group accordion")
            } else {
                String::from("section-group")
            }
        };
        format!("<div class='{}'>{}</div>", class, sections_html)
    }
}

#[inline]
pub fn section_styles() -> String {
    "\n    .wj-section {\n        background: #16213e;\n        border-radius: 8px;\n        margin-bottom: 8px;\n        overflow: hidden;\n        border-left: 3px solid transparent;\n    }\n    \n    .section-header {\n        display: flex;\n        align-items: center;\n        gap: 8px;\n        padding: 12px 16px;\n        cursor: pointer;\n        user-select: none;\n        transition: background 0.15s;\n    }\n    \n    .section-header:hover {\n        background: rgba(255,255,255,0.05);\n    }\n    \n    .collapse-arrow {\n        font-size: 10px;\n        color: #666;\n        width: 12px;\n        transition: transform 0.2s;\n    }\n    \n    .section-icon {\n        font-size: 16px;\n    }\n    \n    .section-title {\n        flex: 1;\n        font-weight: 500;\n        font-size: 13px;\n        color: #e0e0e0;\n    }\n    \n    .section-remove {\n        width: 20px;\n        height: 20px;\n        border: none;\n        background: transparent;\n        color: #666;\n        font-size: 16px;\n        cursor: pointer;\n        border-radius: 4px;\n        display: flex;\n        align-items: center;\n        justify-content: center;\n    }\n    \n    .section-remove:hover {\n        background: #e94560;\n        color: white;\n    }\n    \n    .section-content {\n        padding: 0 16px 16px 16px;\n        animation: section-expand 0.2s ease-out;\n    }\n    \n    .section-content.collapsed {\n        display: none;\n    }\n    \n    @keyframes section-expand {\n        from {\n            opacity: 0;\n            transform: translateY(-10px);\n        }\n        to {\n            opacity: 1;\n            transform: translateY(0);\n        }\n    }\n    \n    .section-group {\n        display: flex;\n        flex-direction: column;\n    }\n    \n    .section-group.accordion .wj-section:not(:first-child) {\n        margin-top: -1px;\n        border-radius: 0;\n    }\n    \n    .section-group.accordion .wj-section:first-child {\n        border-radius: 8px 8px 0 0;\n    }\n    \n    .section-group.accordion .wj-section:last-child {\n        border-radius: 0 0 8px 8px;\n    }\n    ".to_string()
}
