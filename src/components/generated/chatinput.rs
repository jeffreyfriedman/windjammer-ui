#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

pub struct ChatInput {
    placeholder: String,
    value: String,
    disabled: bool,
    multiline: bool,
    rows: i32,
}

impl ChatInput {
    #[inline]
    pub fn new() -> ChatInput {
        ChatInput {
            placeholder: String::from("Type a message...".to_string()),
            value: String::from("".to_string()),
            disabled: false,
            multiline: true,
            rows: 3,
        }
    }
    #[inline]
    pub fn placeholder(mut self, placeholder: String) -> ChatInput {
        self.placeholder = placeholder;
        self
    }
    #[inline]
    pub fn value(mut self, value: String) -> ChatInput {
        self.value = value;
        self
    }
    #[inline]
    pub fn disabled(mut self, disabled: bool) -> ChatInput {
        self.disabled = disabled;
        self
    }
    #[inline]
    pub fn multiline(mut self, multiline: bool) -> ChatInput {
        self.multiline = multiline;
        self
    }
    #[inline]
    pub fn rows(mut self, rows: i32) -> ChatInput {
        self.rows = rows;
        self
    }
}

impl Renderable for ChatInput {
    #[inline]
    fn render(self) -> String {
        let disabled_attr = {
            if self.disabled {
                " disabled"
            } else {
                ""
            }
        };
        let input_html = {
            if self.multiline {
                format!("<textarea class='wj-chat-input-field' placeholder='{}' rows='{}'{}>{}</textarea>", self.placeholder, self.rows, disabled_attr, self.value)
            } else {
                format!("<input type='text' class='wj-chat-input-field' placeholder='{}' value='{}'{}/>", self.placeholder, self.value, disabled_attr)
            }
        };
        format!(
            "<div class='wj-chat-input'>
                {}
                <button class='wj-chat-send-button'{}>
                    <span>➤</span>
                </button>
            </div>",
            input_html, disabled_attr
        )
    }
}
