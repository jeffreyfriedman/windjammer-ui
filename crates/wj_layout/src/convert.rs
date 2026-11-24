//! Scene `Style` -> Taffy `Style`.
use taffy::prelude::*;
use taffy::{Overflow, Point};
use wj_scene::{Align, Dim, Direction, Justify, Style as S};

fn dim(d: Dim) -> Dimension {
    match d {
        Dim::Auto => auto(),
        Dim::Px(v) => length(v),
        Dim::Pct(p) => percent(p / 100.0),
    }
}

pub(crate) fn to_taffy(s: &S) -> Style {
    let mut t = Style {
        display: Display::Flex,
        flex_direction: match s.direction {
            Direction::Row => FlexDirection::Row,
            Direction::Column => FlexDirection::Column,
        },
        flex_wrap: if s.wrap { FlexWrap::Wrap } else { FlexWrap::NoWrap },
        size: Size { width: dim(s.width), height: dim(s.height) },
        padding: Rect { left: length(s.padding.left), right: length(s.padding.right), top: length(s.padding.top), bottom: length(s.padding.bottom) },
        margin: Rect { left: length(s.margin.left), right: length(s.margin.right), top: length(s.margin.top), bottom: length(s.margin.bottom) },
        gap: Size { width: length(s.gap), height: length(s.gap) },
        flex_grow: s.flex_grow,
        flex_shrink: s.flex_shrink,
        align_items: Some(match s.align_items {
            Align::Start => AlignItems::FLEX_START,
            Align::Center => AlignItems::CENTER,
            Align::End => AlignItems::FLEX_END,
            Align::Stretch => AlignItems::STRETCH,
        }),
        justify_content: Some(match s.justify {
            Justify::Start => JustifyContent::FLEX_START,
            Justify::Center => JustifyContent::CENTER,
            Justify::End => JustifyContent::FLEX_END,
            Justify::SpaceBetween => JustifyContent::SPACE_BETWEEN,
            Justify::SpaceAround => JustifyContent::SPACE_AROUND,
        }),
        ..Default::default()
    };
    if let Some((x, y)) = s.absolute {
        t.position = Position::Absolute;
        t.inset = Rect { left: length(x), top: length(y), right: auto(), bottom: auto() };
        // Absolute children of a list stretch to the parent's width unless they set one.
        if matches!(s.width, Dim::Auto) {
            t.inset.right = length(0.0);
        }
    }
    if s.scroll {
        t.overflow = Point { x: Overflow::Hidden, y: Overflow::Scroll };
        t.scrollbar_width = 0.0;
        t.min_size = Size { width: auto(), height: length(0.0) };
    }
    t
}
