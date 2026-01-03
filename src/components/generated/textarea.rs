#[derive(Debug, Clone, PartialEq)]
pub struct Textarea {
    pub value: String,
    pub placeholder: String,
    pub rows: i32,
    pub disabled: bool,
    pub readonly: bool,
    pub max_length: i32,
    pub resize: TextareaResize,
    pub class: String,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TextareaResize {
    None,
    Vertical,
    Horizontal,
    Both,
}

impl Textarea {
    #[inline]
    pub fn new() -> Textarea {
        Textarea {
            value: String::new(),
            placeholder: String::new(),
            rows: 4,
            disabled: false,
            readonly: false,
            max_length: 0,
            resize: TextareaResize::Vertical,
            class: String::new(),
        }
    }
    #[inline]
    pub fn value(mut self, value: String) -> Textarea {
        self.value = value;
        self
    }
    #[inline]
    pub fn placeholder(mut self, placeholder: String) -> Textarea {
        self.placeholder = placeholder;
        self
    }
    #[inline]
    pub fn rows(mut self, rows: i32) -> Textarea {
        self.rows = rows;
        self
    }
    #[inline]
    pub fn disabled(mut self, disabled: bool) -> Textarea {
        self.disabled = disabled;
        self
    }
    #[inline]
    pub fn readonly(mut self, readonly: bool) -> Textarea {
        self.readonly = readonly;
        self
    }
    #[inline]
    pub fn max_length(mut self, max_length: i32) -> Textarea {
        self.max_length = max_length;
        self
    }
    #[inline]
    pub fn resize(mut self, resize: TextareaResize) -> Textarea {
        self.resize = resize;
        self
    }
    #[inline]
    pub fn class(mut self, class: String) -> Textarea {
        self.class = class;
        self
    }
    #[inline]
    pub fn render(&self) -> String {
        let resize_style = match self.resize {
            TextareaResize::None => "resize: none;".to_string(),
            TextareaResize::Vertical => "resize: vertical;".to_string(),
            TextareaResize::Horizontal => "resize: horizontal;".to_string(),
            TextareaResize::Both => "resize: both;".to_string(),
        };
        let disabled_attr = {
            if self.disabled {
                " disabled".to_string()
            } else {
                "".to_string()
            }
        };
        let readonly_attr = {
            if self.readonly {
                " readonly".to_string()
            } else {
                "".to_string()
            }
        };
        let mut html = String::new();
        html.push_str("<textarea class=\"wj-textarea ");
        html.push_str(self.class.as_str());
        html.push_str("\" rows=\"");
        html.push_str(self.rows.to_string().as_str());
        html.push('"');
        if !self.placeholder.is_empty() {
            html.push_str(" placeholder=\"");
            html.push_str(self.placeholder.as_str());
            html.push('"')
        }
        if self.max_length > 0 {
            html.push_str(" maxlength=\"");
            html.push_str(self.max_length.to_string().as_str());
            html.push('"')
        }
        html.push_str(&disabled_attr);
        html.push_str(&readonly_attr);
        html.push_str(" style=\"");
        html.push_str(&resize_style);
        html.push_str(" padding: 8px 12px; border: 1px solid #d1d5db; border-radius: 6px; font-size: 14px; font-family: inherit; width: 100%; box-sizing: border-box;\">");
        html.push_str(self.value.as_str());
        html.push_str("</textarea>");
        html
    }
}
