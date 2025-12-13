#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChatMessage {
    role: MessageRole,
    content: String,
    avatar: String,
    timestamp: String,
}

impl ChatMessage {
    #[inline]
    pub fn new(content: String) -> ChatMessage {
        ChatMessage {
            role: MessageRole::User,
            content,
            avatar: String::from("".to_string()),
            timestamp: String::from("".to_string()),
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
    fn render(self) -> String {
        let role_class = match self.role {
            MessageRole::User => "wj-message-user",
            MessageRole::Assistant => "wj-message-assistant",
            MessageRole::System => "wj-message-system",
        };
        let avatar_html = {
            if (self.avatar.len() as i32) > 0 {
                format!(
                    "<div class='wj-message-avatar'><img src='{}' alt='avatar'/></div>",
                    self.avatar
                )
            } else {
                let default_icon = match self.role {
                    MessageRole::User => "👤",
                    MessageRole::Assistant => "🤖",
                    MessageRole::System => "⚙️",
                };
                format!("<div class='wj-message-avatar'>{}</div>", default_icon)
            }
        };
        let timestamp_html = {
            if (self.timestamp.len() as i32) > 0 {
                format!("<div class='wj-message-timestamp'>{}</div>", self.timestamp)
            } else {
                String::from("".to_string())
            }
        };
        format!(
            "<div class='wj-chat-message {}'>
                {}
                <div class='wj-message-content-wrapper'>
                    <div class='wj-message-content'>{}</div>
                    {}
                </div>
            </div>",
            role_class, avatar_html, self.content, timestamp_html
        )
    }
}
