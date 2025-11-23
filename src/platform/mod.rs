// Platform abstraction layer for windjammer-ui
//
// This module provides platform-specific implementations for:
// - Desktop (egui)
// - Web (WASM)
// - iOS (UIKit/SwiftUI) - Future
// - Android (Jetpack Compose) - Future

pub mod desktop;
pub mod web;

// Mobile platforms (coming soon)
#[cfg(target_os = "ios")]
pub mod ios;

#[cfg(target_os = "android")]
pub mod android;

/// Platform target for code generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// Desktop (egui backend)
    Desktop,
    /// Web (WASM backend)
    Web,
    /// iOS (UIKit/SwiftUI backend)
    IOS,
    /// Android (Jetpack Compose backend)
    Android,
}

impl Target {
    /// Get target from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "desktop" => Some(Target::Desktop),
            "web" | "wasm" => Some(Target::Web),
            "ios" | "iphone" => Some(Target::IOS),
            "android" => Some(Target::Android),
            _ => None,
        }
    }

    /// Check if target is mobile
    pub fn is_mobile(&self) -> bool {
        matches!(self, Target::IOS | Target::Android)
    }

    /// Get target name
    pub fn name(&self) -> &'static str {
        match self {
            Target::Desktop => "desktop",
            Target::Web => "web",
            Target::IOS => "ios",
            Target::Android => "android",
        }
    }
}

/// Renderer trait that all platforms must implement
pub trait Renderer {
    /// Render a virtual DOM node
    fn render(&mut self, vnode: &crate::vdom::VNode);

    /// Handle an event
    fn handle_event(&mut self, event: Event);

    /// Update the UI (call after state changes)
    fn update(&mut self);
}

/// Platform-agnostic event
#[derive(Debug, Clone)]
pub enum Event {
    /// Click/Tap event
    Click { x: f32, y: f32 },
    /// Key press
    KeyPress { key: String },
    /// Text input
    TextInput { text: String },
    /// Gesture (mobile)
    Gesture(GestureEvent),
}

/// Mobile gesture events
#[derive(Debug, Clone)]
pub enum GestureEvent {
    /// Swipe gesture
    Swipe {
        direction: SwipeDirection,
        velocity: f32,
    },
    /// Pinch gesture (zoom)
    Pinch { scale: f32 },
    /// Long press
    LongPress { x: f32, y: f32, duration: f32 },
    /// Pan/Drag
    Pan { dx: f32, dy: f32 },
}

#[derive(Debug, Clone, Copy)]
pub enum SwipeDirection {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_from_str() {
        assert_eq!(Target::from_str("desktop"), Some(Target::Desktop));
        assert_eq!(Target::from_str("web"), Some(Target::Web));
        assert_eq!(Target::from_str("ios"), Some(Target::IOS));
        assert_eq!(Target::from_str("android"), Some(Target::Android));
        assert_eq!(Target::from_str("invalid"), None);
    }

    #[test]
    fn test_is_mobile() {
        assert!(!Target::Desktop.is_mobile());
        assert!(!Target::Web.is_mobile());
        assert!(Target::IOS.is_mobile());
        assert!(Target::Android.is_mobile());
    }

    #[test]
    fn test_target_name() {
        assert_eq!(Target::Desktop.name(), "desktop");
        assert_eq!(Target::Web.name(), "web");
        assert_eq!(Target::IOS.name(), "ios");
        assert_eq!(Target::Android.name(), "android");
    }
}
