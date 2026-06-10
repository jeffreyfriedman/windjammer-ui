#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct MessageList {
    messages: Vec<String>,
    height: String,
    auto_scroll: bool,
}

impl MessageList {
    #[inline]
    pub fn new() -> MessageList {
        MessageList {
            messages: Vec::new(),
            height: String::from("600px".to_string()),
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
                String::from("onload='this.scrollTop = this.scrollHeight'")
            } else {
                String::new()
            }
        };
        format!("<div class='wj-message-list' style='height: {}' {}>\n                {}\n            </div>", self.height, scroll_script, self.messages.join(""))
    }
}
