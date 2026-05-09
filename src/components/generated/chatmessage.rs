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
        ChatMessage { role: MessageRole::User, content: content.to_string(), avatar: String::from(""), timestamp: String::from("") }
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
fn render(&self) -> String {
        let role_class = match self.role {
            MessageRole::User => "wj-message-user".to_string(),
            MessageRole::Assistant => "wj-message-assistant".to_string(),
            MessageRole::System => "wj-message-system".to_string(),
        };
        let avatar_html = {
            if !self.avatar.is_empty() {
                format!("<div class='wj-message-avatar'><img src='{}' alt='avatar'/></div>", self.avatar.clone())
            } else {
                let default_icon = match self.role {
                    MessageRole::User => "👤".to_string(),
                    MessageRole::Assistant => "🤖".to_string(),
                    MessageRole::System => "⚙️".to_string(),
                };
                format!("<div class='wj-message-avatar'>{}</div>", default_icon)
            }
        };
        let timestamp_html = {
            if !self.timestamp.is_empty() {
                format!("<div class='wj-message-timestamp'>{}</div>", self.timestamp.clone())
            } else {
                String::from("")
            }
        };
        format!("<div class='wj-chat-message {}'>\n                {}\n                <div class='wj-message-content-wrapper'>\n                    <div class='wj-message-content'>{}</div>\n                    {}\n                </div>\n            </div>", role_class, avatar_html, self.content.clone(), timestamp_html)
}
}

