#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
pub struct Center {
    child: String,
    width: String,
    height: String,
    class: String,
}

impl Center {
    #[inline]
    pub fn new(child: String) -> Center {
        Center {
            child,
            width: "100%".to_string(),
            height: "100%".to_string(),
            class: String::new(),
        }
    }
    #[inline]
    pub fn width(mut self, width: String) -> Center {
        self.width = width;
        self
    }
    #[inline]
    pub fn height(mut self, height: String) -> Center {
        self.height = height;
        self
    }
    #[inline]
    pub fn class(mut self, class: String) -> Center {
        self.class = class;
        self
    }
    pub fn render(&self) -> String {
        let mut html = String::new();
        html.push_str("<div class=\"wj-center ");
        html.push_str(self.class.as_str());
        html.push_str(
            "\" style=\"display: flex; align-items: center; justify-content: center; width: ",
        );
        html.push_str(self.width.as_str());
        html.push_str("; height: ");
        html.push_str(self.height.as_str());
        html.push_str(";\">");
        html.push_str(self.child.as_str());
        html.push_str("</div>");
        html
    }
}
