use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Panel {
    pub title: String,
    pub children: Vec<String>,
    pub collapsible: bool,
    pub collapsed: bool,
    pub padding: String,
}

impl Panel {
#[inline]
pub fn new(title: String) -> Panel {
        Panel { title, children: Vec::new(), collapsible: false, collapsed: false, padding: "16px".to_string() }
}
#[inline]
pub fn child(mut self, child: String) -> Panel {
        self.children.push(child);
        self
}
#[inline]
pub fn collapsible(mut self, collapsible: bool) -> Panel {
        self.collapsible = collapsible;
        self
}
#[inline]
pub fn collapsed(mut self, collapsed: bool) -> Panel {
        self.collapsed = collapsed;
        self
}
#[inline]
pub fn padding(mut self, padding: String) -> Panel {
        self.padding = padding;
        self
}
}

impl Renderable for Panel {
#[inline]
fn render(self) -> String {
        let header_class = {
            if self.collapsible {
                "wj-panel-header-collapsible".to_string()
            } else {
                "wj-panel-header".to_string()
            }
        };
        let icon = {
            if self.collapsible {
                if self.collapsed {
                    "▶".to_string()
                } else {
                    "▼".to_string()
                }
            } else {
                "".to_string()
            }
        };
        let content_style = {
            if self.collapsed {
                "display: none;".to_string()
            } else {
                "display: block;".to_string()
            }
        };
        let children_html = self.children.join("
");
        format!("<div class='wj-panel'>
  <div class='{}'>
    <span>{}</span>
    <h3>{}</h3>
  </div>
  <div class='wj-panel-content' style='{}padding: {};'>
    {}
  </div>
</div>", header_class, icon, self.title, content_style, self.padding, children_html)
}
}

