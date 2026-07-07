#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    pub avatar: String,
    pub timestamp: String,
}

impl ChatMessage {
    #[inline]
    pub fn new(content: String) -> ChatMessage {
        ChatMessage {
            role: MessageRole::User,
            content,
            avatar: String::new(),
            timestamp: String::new(),
        }
    }
    #[inline]
    pub fn role(mut self, role: MessageRole) -> ChatMessage {
        self.role = role;
        self
    }
    #[inline]
    pub fn avatar(mut self, avatar: String) -> ChatMessage {
        self.avatar = avatar;
        self
    }
    #[inline]
    pub fn timestamp(mut self, timestamp: String) -> ChatMessage {
        self.timestamp = timestamp;
        self
    }
}

impl Renderable for ChatMessage {
    #[inline]
    fn render(&mut self) -> String {
        let role_class: String = match self.role {
            MessageRole::User => String::from("wj-message-user"),
            MessageRole::Assistant => String::from("wj-message-assistant"),
            MessageRole::System => String::from("wj-message-system"),
        };
        let avatar_html: String = {
            if !self.avatar.is_empty() {
                format!(
                    "<div class='wj-message-avatar'><img src='{}' alt='avatar'/></div>",
                    self.avatar
                )
            } else {
                let default_icon: String = match self.role {
                    MessageRole::User => String::from("👤"),
                    MessageRole::Assistant => String::from("🤖"),
                    MessageRole::System => String::from("⚙️"),
                };
                format!("<div class='wj-message-avatar'>{}</div>", default_icon)
            }
        };
        let timestamp_html = {
            if !self.timestamp.is_empty() {
                format!("<div class='wj-message-timestamp'>{}</div>", self.timestamp)
            } else {
                String::new()
            }
        };
        format!("<div class='wj-chat-message {}'>\n                {}\n                <div class='wj-message-content-wrapper'>\n                    <div class='wj-message-content'>{}</div>\n                    {}\n                </div>\n            </div>", role_class, avatar_html, self.content, timestamp_html)
    }
}
