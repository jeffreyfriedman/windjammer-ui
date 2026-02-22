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
    pub const YELLOW: Self = Self {
        r: 255,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const GRAY: Self = Self {
        r: 128,
        g: 128,
        b: 128,
        a: 255,
    };

    /// Create an RGBA color with premultiplied alpha (for compatibility with backends)
    pub fn rgba_premultiplied(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Convert to array [r, g, b, a]
    pub fn to_array(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Convert to f32 array [r, g, b, a] normalized to 0.0-1.0
    pub fn to_f32_array(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
    }
}

/// Convert from native UI::Color32 (implementation detail, hidden from users)
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

/// Convert to native UI::Color32 (implementation detail, hidden from users)
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl From<Color> for egui::Color32 {
    fn from(c: Color) -> Self {
        egui::Color32::from_rgba_premultiplied(c.r, c.g, c.b, c.a)
    }
}

/// Generic 2D position (implementation-agnostic)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
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

/// Arithmetic operations for Position
impl std::ops::Add for Position {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for Position {
    type Output = Size;
    fn sub(self, other: Self) -> Size {
        Size::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Sub<Size> for Position {
    type Output = Self;
    fn sub(self, size: Size) -> Self {
        Self::new(self.x - size.width, self.y - size.height)
    }
}

impl std::ops::Add<Size> for Position {
    type Output = Self;
    fn add(self, size: Size) -> Self {
        Self::new(self.x + size.width, self.y + size.height)
    }
}

/// Cross-type operations with native UI types
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl std::ops::Add<egui::Vec2> for Position {
    type Output = Self;
    fn add(self, v: egui::Vec2) -> Self {
        Self::new(self.x + v.x, self.y + v.y)
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl std::ops::Sub<egui::Pos2> for Position {
    type Output = Self;
    fn sub(self, p: egui::Pos2) -> Self {
        Self::new(self.x - p.x, self.y - p.y)
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl std::ops::Sub<Size> for egui::Pos2 {
    type Output = Self;
    fn sub(self, s: Size) -> Self {
        egui::Pos2::new(self.x - s.width, self.y - s.height)
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl std::ops::Sub<Position> for egui::Pos2 {
    type Output = Self;
    fn sub(self, p: Position) -> Self {
        egui::Pos2::new(self.x - p.x, self.y - p.y)
    }
}

/// Convert from native UI::Pos2 (implementation detail)
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl From<egui::Pos2> for Position {
    fn from(p: egui::Pos2) -> Self {
        Self { x: p.x, y: p.y }
    }
}

/// Convert to native UI::Pos2 (implementation detail)
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

/// Arithmetic operations for Size
impl std::ops::Add for Size {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.width + other.width, self.height + other.height)
    }
}

impl std::ops::Sub for Size {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.width - other.width, self.height - other.height)
    }
}

impl std::ops::Mul<f32> for Size {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self::new(self.width * scalar, self.height * scalar)
    }
}

impl std::ops::Div<f32> for Size {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Self::new(self.width / scalar, self.height / scalar)
    }
}

/// Convert from native UI::Vec2 (implementation detail)
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl From<egui::Vec2> for Size {
    fn from(v: egui::Vec2) -> Self {
        Self {
            width: v.x,
            height: v.y,
        }
    }
}

/// Convert to native UI::Vec2 (implementation detail)
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

/// Extension trait for converting native UI types to generic abstractions
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
pub trait FromNative {
    type Output;
    fn to_generic(&self) -> Self::Output;
}

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl FromNative for egui::Color32 {
    type Output = Color;
    fn to_generic(&self) -> Color {
        Color::from(*self)
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl FromNative for egui::Pos2 {
    type Output = Position;
    fn to_generic(&self) -> Position {
        Position::from(*self)
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl FromNative for egui::Vec2 {
    type Output = Size;
    fn to_generic(&self) -> Size {
        Size::from(*self)
    }
}

/// Extension trait for converting generic abstractions to native UI types
/// This is only for internal use within windjammer-ui components
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
pub trait ToNative {
    type Output;
    fn to_native(&self) -> Self::Output;
}

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl ToNative for Color {
    type Output = egui::Color32;
    fn to_native(&self) -> egui::Color32 {
        egui::Color32::from(*self)
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl ToNative for Position {
    type Output = egui::Pos2;
    fn to_native(&self) -> egui::Pos2 {
        egui::Pos2::from(*self)
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
impl ToNative for Size {
    type Output = egui::Vec2;
    fn to_native(&self) -> egui::Vec2 {
        egui::Vec2::from(*self)
    }
}
