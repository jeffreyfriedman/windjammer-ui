#[derive(Debug, Clone, Default)]
pub struct Breadcrumb {
    pub items: Vec<BreadcrumbItem>,
    pub separator: String,
    pub class: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: String,
    pub active: bool,
}

impl Breadcrumb {
#[inline]
pub fn new() -> Breadcrumb {
        Breadcrumb { items: Vec::new(), separator: "/".to_string(), class: String::new() }
}
#[inline]
pub fn item(mut self, label: String, href: String, active: bool) -> Breadcrumb {
        self.items.push(BreadcrumbItem { label, href, active });
        self
}
#[inline]
pub fn separator(mut self, separator: String) -> Breadcrumb {
        self.separator = separator;
        self
}
#[inline]
pub fn class(mut self, class: String) -> Breadcrumb {
        self.class = class;
        self
}
#[inline]
pub fn render(&self) -> String {
        let mut html = String::new();
        html.push_str("<nav class=\"wj-breadcrumb ");
        html.push_str(&self.class.as_str());
        html.push_str("\" style=\"display: flex; align-items: center; gap: 8px; font-size: 14px;\">");
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                html.push_str("<span style=\"color: #9ca3af;\">");
                html.push_str(&self.separator.as_str());
                html.push_str("</span>")
            }
            if item.active {
                html.push_str("<span style=\"color: #3b82f6; font-weight: 500;\">");
                html.push_str(&item.label.as_str());
                html.push_str("</span>")
            } else {
                html.push_str("<a href=\"");
                html.push_str(&item.href.as_str());
                html.push_str("\" style=\"color: #6b7280; text-decoration: none; hover: color: #3b82f6;\">");
                html.push_str(&item.label.as_str());
                html.push_str("</a>")
            }
        }
        html.push_str("</nav>");
        html
}
}

