#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#[derive(Debug, Clone)]
pub struct Select {
    pub options: Vec<SelectOption>,
    pub selected: String,
    pub placeholder: String,
    pub disabled: bool,
    pub size: SelectSize,
    pub class: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum SelectSize {
    Small,
    Medium,
    Large,
}

impl Select {
    #[inline]
    pub fn new() -> Select {
        Select {
            options: Vec::new(),
            selected: String::new(),
            placeholder: "Select an option".to_string(),
            disabled: false,
            size: SelectSize::Medium,
            class: String::new(),
        }
    }
    #[inline]
    pub fn option(mut self, value: String, label: String) -> Select {
        self.options.push(SelectOption { value, label });
        self
    }
    #[inline]
    pub fn selected(mut self, selected: String) -> Select {
        self.selected = selected;
        self
    }
    #[inline]
    pub fn placeholder(mut self, placeholder: String) -> Select {
        self.placeholder = placeholder;
        self
    }
    #[inline]
    pub fn disabled(mut self, disabled: bool) -> Select {
        self.disabled = disabled;
        self
    }
    #[inline]
    pub fn size(mut self, size: SelectSize) -> Select {
        self.size = size;
        self
    }
    #[inline]
    pub fn class(mut self, class: String) -> Select {
        self.class = class;
        self
    }
    #[inline]
    pub fn render(&self) -> String {
        let size_style = match self.size {
            SelectSize::Small => "padding: 4px 8px; font-size: 12px;".to_string(),
            SelectSize::Medium => "padding: 8px 12px; font-size: 14px;".to_string(),
            SelectSize::Large => "padding: 12px 16px; font-size: 16px;".to_string(),
        };
        let disabled_attr = {
            if self.disabled {
                " disabled".to_string()
            } else {
                "".to_string()
            }
        };
        let mut html = String::new();
        html.push_str("<select class=\"wj-select ");
        html.push_str(&self.class.as_str());
        html.push_str("\" style=\"");
        html.push_str(&size_style);
        html.push_str(
            " border: 1px solid #d1d5db; border-radius: 6px; background: white; cursor: pointer;\"",
        );
        html.push_str(&disabled_attr);
        html.push('>');
        if !self.placeholder.is_empty() {
            html.push_str("<option value=\"\" disabled");
            if self.selected.is_empty() {
                html.push_str(" selected")
            }
            html.push_str(">");
            html.push_str(&self.placeholder.as_str());
            html.push_str("</option>")
        }
        for opt in &self.options {
            html.push_str("<option value=\"");
            html.push_str(&opt.value.clone().as_str());
            html.push('"');
            if opt.value.clone() == self.selected {
                html.push_str(" selected")
            }
            html.push('>');
            html.push_str(&opt.label.clone().as_str());
            html.push_str("</option>");
        }
        html.push_str("</select>");
        html
    }
}
