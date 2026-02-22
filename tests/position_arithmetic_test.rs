/// TDD Test: Position and Size should support arithmetic operations
use windjammer_ui::types::{Position, Size};

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
use egui::{Pos2, Vec2};

#[test]
fn test_position_addition() {
    let p1 = Position::new(10.0, 20.0);
    let p2 = Position::new(5.0, 15.0);
    let result = p1 + p2;

    assert_eq!(result.x(), 15.0);
    assert_eq!(result.y(), 35.0);
}

#[test]
fn test_position_subtraction() {
    let p1 = Position::new(10.0, 20.0);
    let p2 = Position::new(5.0, 15.0);
    let result = p1 - p2;

    assert_eq!(result.x(), 5.0);
    assert_eq!(result.y(), 5.0);
}

#[test]
fn test_size_addition() {
    let s1 = Size::new(10.0, 20.0);
    let s2 = Size::new(5.0, 15.0);
    let result = s1 + s2;

    assert_eq!(result.width(), 15.0);
    assert_eq!(result.height(), 35.0);
}

#[test]
fn test_size_multiplication() {
    let s = Size::new(10.0, 20.0);
    let result = s * 2.0;

    assert_eq!(result.width(), 20.0);
    assert_eq!(result.height(), 40.0);
}

#[test]
fn test_size_division() {
    let s = Size::new(10.0, 20.0);
    let result = s / 2.0;

    assert_eq!(result.width(), 5.0);
    assert_eq!(result.height(), 10.0);
}

#[test]
fn test_position_add_size() {
    let p = Position::new(10.0, 20.0);
    let s = Size::new(5.0, 15.0);
    let result = p + s;

    assert_eq!(result.x(), 15.0);
    assert_eq!(result.y(), 35.0);
}

// Tests for egui interoperability
#[test]
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
fn test_position_add_vec2() {
    let p = Position::new(10.0, 20.0);
    let v = Vec2::new(5.0, 15.0);
    let result = p + v;

    assert_eq!(result.x(), 15.0);
    assert_eq!(result.y(), 35.0);
}

#[test]
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
fn test_position_sub_pos2() {
    let p = Position::new(10.0, 20.0);
    let pos2 = Pos2::new(5.0, 15.0);
    let result = p - pos2;

    assert_eq!(result.x(), 5.0);
    assert_eq!(result.y(), 5.0);
}

#[test]
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
fn test_pos2_sub_size() {
    let pos2 = Pos2::new(10.0, 20.0);
    let s = Size::new(5.0, 15.0);
    let result = pos2 - s;

    assert_eq!(result.x, 5.0);
    assert_eq!(result.y, 5.0);
}
