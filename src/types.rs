//! Generic UI types that are NOT tied to any specific backend
//!
//! These abstractions allow windjammer-ui to be implementation-agnostic.
//! egui, gtk, or any other backend can be used by converting to/from these types.

use serde::{Deserialize, Serialize};

/// Generic color representation (implementation-agnostic)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    /// Create an RGB color (fully opaque)
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create an RGBA color with alpha channel
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Get red component
    pub fn r(&self) -> u8 {
        self.r
    }

    /// Get green component
    pub fn g(&self) -> u8 {
        self.g
    }

    /// Get blue component
    pub fn b(&self) -> u8 {
        self.b
    }

    /// Get alpha component
    pub fn a(&self) -> u8 {
        self.a
    }

    /// Common colors
    pub const RED: Self = Self {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const GREEN: Self = Self {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const BLUE: Self = Self {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const TRANSPARENT: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
}

/// Convert from egui::Color32 (implementation detail, hidden from users)
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl From<egui::Color32> for Color {
    fn from(c: egui::Color32) -> Self {
        Self {
            r: c.r(),
            g: c.g(),
            b: c.b(),
            a: c.a(),
        }
    }
}

/// Convert to egui::Color32 (implementation detail, hidden from users)
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl From<Color> for egui::Color32 {
    fn from(c: Color) -> Self {
        egui::Color32::from_rgba_premultiplied(c.r, c.g, c.b, c.a)
    }
}

/// Generic 2D position (implementation-agnostic)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    x: f32,
    y: f32,
}

impl Position {
    /// Create a new position
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Get x coordinate
    pub fn x(&self) -> f32 {
        self.x
    }

    /// Get y coordinate
    pub fn y(&self) -> f32 {
        self.y
    }
}

/// Convert from egui::Pos2 (implementation detail)
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl From<egui::Pos2> for Position {
    fn from(p: egui::Pos2) -> Self {
        Self { x: p.x, y: p.y }
    }
}

/// Convert to egui::Pos2 (implementation detail)
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl From<Position> for egui::Pos2 {
    fn from(p: Position) -> Self {
        egui::Pos2::new(p.x, p.y)
    }
}

/// Generic 2D size (implementation-agnostic)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Size {
    width: f32,
    height: f32,
}

impl Size {
    /// Create a new size
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Get width
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Get height
    pub fn height(&self) -> f32 {
        self.height
    }
}

/// Convert from egui::Vec2 (implementation detail)
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl From<egui::Vec2> for Size {
    fn from(v: egui::Vec2) -> Self {
        Self {
            width: v.x,
            height: v.y,
        }
    }
}

/// Convert to egui::Vec2 (implementation detail)
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl From<Size> for egui::Vec2 {
    fn from(s: Size) -> Self {
        egui::Vec2::new(s.width, s.height)
    }
}

// ============================================================================
// Extension traits for ergonomic conversion in editor code
// These are PUBLIC so editor can convert easily, but they're still generic
// ============================================================================

/// Extension trait for converting egui types to generic abstractions
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
pub trait EguiConvert {
    type Output;
    fn to_generic(&self) -> Self::Output;
}

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl EguiConvert for egui::Color32 {
    type Output = Color;
    fn to_generic(&self) -> Color {
        Color::from(*self)
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl EguiConvert for egui::Pos2 {
    type Output = Position;
    fn to_generic(&self) -> Position {
        Position::from(*self)
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl EguiConvert for egui::Vec2 {
    type Output = Size;
    fn to_generic(&self) -> Size {
        Size::from(*self)
    }
}

/// Extension trait for converting generic abstractions to egui types
/// This is only for internal use within windjammer-ui components
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
pub trait ToEgui {
    type Output;
    fn to_egui(&self) -> Self::Output;
}

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl ToEgui for Color {
    type Output = egui::Color32;
    fn to_egui(&self) -> egui::Color32 {
        egui::Color32::from(*self)
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl ToEgui for Position {
    type Output = egui::Pos2;
    fn to_egui(&self) -> egui::Pos2 {
        egui::Pos2::from(*self)
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl ToEgui for Size {
    type Output = egui::Vec2;
    fn to_egui(&self) -> egui::Vec2 {
        egui::Vec2::from(*self)
    }
}
