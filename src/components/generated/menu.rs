#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Menu {
    pub items: Vec<String>,
    pub trigger: String,
    pub class: String,
}

impl Menu {
    #[inline]
    pub fn new(trigger: String) -> Menu {
        Menu {
            items: Vec::new(),
            trigger,
            class: String::new(),
        }
    }
    #[inline]
    pub fn item(mut self, item: String) -> Menu {
        self.items.push(item);
        self
    }
    #[inline]
    pub fn class(mut self, class: String) -> Menu {
        self.class = class;
        self
    }
    #[inline]
    pub fn render(&self) -> String {
        let mut html = String::new();
        html.push_str("<div class=\"wj-menu ");
        html.push_str(self.class.as_str());
        html.push_str("\" style=\"position: relative; display: inline-block;\">");
        html.push_str(self.trigger.as_str());
        html.push_str("<div class=\"wj-menu-items\" style=\"position: absolute; top: 100%; left: 0; margin-top: 4px; background: white; border: 1px solid #e5e7eb; border-radius: 8px; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1); min-width: 200px; z-index: 1000; display: none;\">");
        for item in &self.items {
            html.push_str(item.as_str());
        }
        html.push_str("</div>");
        html.push_str("</div>");
        html.push_str("<style>.wj-menu:hover .wj-menu-items { display: block; }</style>");
        html
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct MenuItem {
    pub label: String,
    pub icon: String,
    pub href: String,
    pub disabled: bool,
    pub class: String,
}

impl MenuItem {
    #[inline]
    pub fn new(label: String) -> MenuItem {
        MenuItem {
            label,
            icon: String::new(),
            href: "#".to_string(),
            disabled: false,
            class: String::new(),
        }
    }
    #[inline]
    pub fn icon(mut self, icon: String) -> MenuItem {
        self.icon = icon;
        self
    }
    #[inline]
    pub fn href(mut self, href: String) -> MenuItem {
        self.href = href;
        self
    }
    #[inline]
    pub fn disabled(mut self, disabled: bool) -> MenuItem {
        self.disabled = disabled;
        self
    }
    #[inline]
    pub fn class(mut self, class: String) -> MenuItem {
        self.class = class;
        self
    }
    #[inline]
    pub fn render(&self) -> String {
        let disabled_style = {
            if self.disabled {
                " opacity: 0.5; cursor: not-allowed;".to_string()
            } else {
                " cursor: pointer;".to_string()
            }
        };
        let mut html = String::new();
        html.push_str("<a href=\"");
        html.push_str(self.href.as_str());
        html.push_str("\" class=\"wj-menu-item ");
        html.push_str(self.class.as_str());
        html.push_str("\" style=\"display: flex; align-items: center; gap: 8px; padding: 10px 16px; color: #374151; text-decoration: none;");
        html.push_str(&disabled_style);
        html.push_str(" transition: background-color 0.2s;\">");
        if !self.icon.is_empty() {
            html.push_str("<span>");
            html.push_str(self.icon.as_str());
            html.push_str("</span>")
        }
        html.push_str("<span>");
        html.push_str(self.label.as_str());
        html.push_str("</span>");
        html.push_str("</a>");
        html.push_str("<style>.wj-menu-item:hover:not([style*='cursor: not-allowed']) { background-color: #f3f4f6; }</style>");
        html
    }
}
