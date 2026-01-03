use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AccordionItem {
    pub title: String,
    pub content: String,
    pub open: bool,
}

impl AccordionItem {
    #[inline]
    pub fn new(title: String, content: String) -> AccordionItem {
        AccordionItem {
            title,
            content,
            open: false,
        }
    }
    #[inline]
    pub fn open(mut self, open: bool) -> AccordionItem {
        self.open = open;
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct Accordion {
    pub items: Vec<AccordionItem>,
    pub allow_multiple: bool,
}

impl Accordion {
    #[inline]
    pub fn new() -> Accordion {
        Accordion {
            items: Vec::new(),
            allow_multiple: false,
        }
    }
    #[inline]
    pub fn item(mut self, item: AccordionItem) -> Accordion {
        self.items.push(item);
        self
    }
    #[inline]
    pub fn allow_multiple(mut self, allow: bool) -> Accordion {
        self.allow_multiple = allow;
        self
    }
}

impl Renderable for Accordion {
    #[inline]
    fn render(self) -> String {
        let mut html = "<div class='wj-accordion'>".to_string();
        let mut i = 0;
        while i < (self.items.len() as i64) {
            let item = &self.items[i as usize];
            let open_attr = {
                if item.open {
                    " open".to_string()
                } else {
                    "".to_string()
                }
            };
            html = format!(
                "{}<details class='wj-accordion-item'{}>
  <summary class='wj-accordion-title'>{}</summary>
  <div class='wj-accordion-content'>
    {}
  </div>
</details>",
                html, open_attr, item.title, item.content
            );
            i += 1;
        }
        format!("{}</div>", html)
    }
}
