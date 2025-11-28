#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

pub struct Form {
    id: String,
    action: String,
    method: String,
    children: Vec<String>,
    on_submit: String,
}

impl Form {
    #[inline]
    pub fn new(id: String) -> Form {
        Form {
            id,
            action: "#".to_string(),
            method: "POST".to_string(),
            children: Vec::new(),
            on_submit: "return false;".to_string(),
        }
    }
    #[inline]
    pub fn action(mut self, action: String) -> Form {
        self.action = action;
        self
    }
    #[inline]
    pub fn method(mut self, method: String) -> Form {
        self.method = method;
        self
    }
    #[inline]
    pub fn on_submit(mut self, handler: String) -> Form {
        self.on_submit = handler;
        self
    }
    #[inline]
    pub fn child(mut self, child: String) -> Form {
        self.children.push(child);
        self
    }
}

pub struct FormField {
    label: String,
    input: String,
    error: String,
    required: bool,
    help_text: String,
}

impl FormField {
    #[inline]
    pub fn new(label: String, input: String) -> FormField {
        FormField {
            label,
            input,
            error: String::new(),
            required: false,
            help_text: String::new(),
        }
    }
    #[inline]
    pub fn required(mut self, required: bool) -> FormField {
        self.required = required;
        self
    }
    #[inline]
    pub fn error(mut self, error: String) -> FormField {
        self.error = error;
        self
    }
    #[inline]
    pub fn help_text(mut self, text: String) -> FormField {
        self.help_text = text;
        self
    }
    pub fn render(&self) -> String {
        let mut html = String::new();
        html.push_str("<div style='margin-bottom: 16px;'>");
        html.push_str(
            "<label style='display: block; margin-bottom: 4px; font-weight: 500; color: #333;'>",
        );
        html.push_str(&self.label);
        if self.required {
            html.push_str(" <span style='color: #e53e3e;'>*</span>")
        }
        html.push_str("</label>");
        html.push_str(&self.input);
        if self.help_text.len() > 0 {
            html.push_str("<div style='margin-top: 4px; font-size: 12px; color: #718096;'>");
            html.push_str(&self.help_text);
            html.push_str("</div>")
        }
        if self.error.len() > 0 {
            html.push_str("<div style='margin-top: 4px; font-size: 12px; color: #e53e3e;'>");
            html.push_str(&self.error);
            html.push_str("</div>")
        }
        html.push_str("</div>");
        html
    }
}

impl Renderable for Form {
    fn render(self) -> String {
        let mut html = String::new();
        html.push_str("<form id='");
        html.push_str(&self.id);
        html.push_str("' action='");
        html.push_str(&self.action);
        html.push_str("' method='");
        html.push_str(&self.method);
        html.push_str("' onsubmit='");
        html.push_str(&self.on_submit);
        html.push_str("'>");
        for child in &self.children {
            html.push_str(child);
        }
        html.push_str("</form>");
        html
    }
}
