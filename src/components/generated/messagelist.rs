#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Debug, Clone, Default)]
pub struct MessageList {
    pub messages: Vec<String>,
    pub height: String,
    pub auto_scroll: bool,
}

impl MessageList {
    #[inline]
    pub fn new() -> MessageList {
        MessageList {
            messages: Vec::new(),
            height: String::from("600px"),
            auto_scroll: true,
        }
    }
    #[inline]
    pub fn message(mut self, message: String) -> MessageList {
        self.messages.push(message);
        self
    }
    #[inline]
    pub fn height(mut self, height: String) -> MessageList {
        self.height = height;
        self
    }
    #[inline]
    pub fn auto_scroll(mut self, auto_scroll: bool) -> MessageList {
        self.auto_scroll = auto_scroll;
        self
    }
}

impl Renderable for MessageList {
    #[inline]
    fn render(self) -> String {
        let scroll_script = {
            if self.auto_scroll {
                "onload='this.scrollTop = this.scrollHeight'".to_string()
            } else {
                "".to_string()
            }
        };
        format!(
            "<div class='wj-message-list' style='height: {}' {}>
                {}
            </div>",
            self.height,
            scroll_script,
            self.messages.join("")
        )
    }
}
