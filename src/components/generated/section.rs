use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
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
        let collapse_icon = {
            if self.collapsed {
                "▶".to_string()
            } else {
                "▼".to_string()
            }
        };
        let content_class = {
            if self.collapsed {
                "section-content collapsed".to_string()
            } else {
                "section-content".to_string()
            }
        };
        let icon_html = {
            if self.icon != "" {
                format!("<span class='section-icon'>{}</span>", self.icon)
            } else {
                "".to_string()
            }
        };
        let accent_style = {
            if self.accent_color != "" {
                format!(" style='border-left: 3px solid {}'", self.accent_color)
            } else {
                "".to_string()
            }
        };
        let remove_btn = {
            if self.removable {
                format!(
                    "<button class='section-remove' onclick='{}'>×</button>",
                    self.on_remove
                )
            } else {
                "".to_string()
            }
        };
        let children_html = self.children.join(
            "
",
        );
        format!(
            "
            <div class='wj-section'{}>
                <div class='section-header'>
                    <span class='collapse-arrow'>{}</span>
                    {}
                    <span class='section-title'>{}</span>
                    {}
                </div>
                <div class='{}'>
                    {}
                </div>
            </div>
        ",
            accent_style,
            collapse_icon,
            icon_html,
            self.title,
            remove_btn,
            content_class,
            children_html
        )
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
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
        for s in &self.sections {
            sections_html = format!(
                "{}{}{}",
                sections_html,
                s.clone().render().as_str(),
                "
"
            );
        }
        let class = {
            if self.accordion {
                "section-group accordion".to_string()
            } else {
                "section-group".to_string()
            }
        };
        format!("<div class='{}'>{}</div>", class, sections_html)
    }
}

#[inline]
pub fn section_styles() -> String {
    "
    .wj-section {
        background: #16213e;
        border-radius: 8px;
        margin-bottom: 8px;
        overflow: hidden;
        border-left: 3px solid transparent;
    }
    
    .section-header {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 12px 16px;
        cursor: pointer;
        user-select: none;
        transition: background 0.15s;
    }
    
    .section-header:hover {
        background: rgba(255,255,255,0.05);
    }
    
    .collapse-arrow {
        font-size: 10px;
        color: #666;
        width: 12px;
        transition: transform 0.2s;
    }
    
    .section-icon {
        font-size: 16px;
    }
    
    .section-title {
        flex: 1;
        font-weight: 500;
        font-size: 13px;
        color: #e0e0e0;
    }
    
    .section-remove {
        width: 20px;
        height: 20px;
        border: none;
        background: transparent;
        color: #666;
        font-size: 16px;
        cursor: pointer;
        border-radius: 4px;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    
    .section-remove:hover {
        background: #e94560;
        color: white;
    }
    
    .section-content {
        padding: 0 16px 16px 16px;
        animation: section-expand 0.2s ease-out;
    }
    
    .section-content.collapsed {
        display: none;
    }
    
    @keyframes section-expand {
        from {
            opacity: 0;
            transform: translateY(-10px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }
    
    .section-group {
        display: flex;
        flex-direction: column;
    }
    
    .section-group.accordion .wj-section:not(:first-child) {
        margin-top: -1px;
        border-radius: 0;
    }
    
    .section-group.accordion .wj-section:first-child {
        border-radius: 8px 8px 0 0;
    }
    
    .section-group.accordion .wj-section:last-child {
        border-radius: 0 0 8px 8px;
    }
    "
    .to_string()
}
