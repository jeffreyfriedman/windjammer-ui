#[allow(unused_imports)]
use super::*;
#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct Style {
    pub properties: Vec<StyleProperty>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct StyleProperty {
    pub name: String,
    pub value: String,
}

impl Style {
#[inline]
pub fn new() -> Style {
        Style { properties: Vec::new() }
}
#[inline]
pub fn display(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("display"), value });
        self
}
#[inline]
pub fn flex_direction(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("flex-direction"), value });
        self
}
#[inline]
pub fn gap(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("gap"), value });
        self
}
#[inline]
pub fn align_items(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("align-items"), value });
        self
}
#[inline]
pub fn justify_content(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("justify-content"), value });
        self
}
#[inline]
pub fn margin(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("margin"), value });
        self
}
#[inline]
pub fn padding(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("padding"), value });
        self
}
#[inline]
pub fn width(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("width"), value });
        self
}
#[inline]
pub fn height(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("height"), value });
        self
}
#[inline]
pub fn min_width(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("min-width"), value });
        self
}
#[inline]
pub fn max_width(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("max-width"), value });
        self
}
#[inline]
pub fn color(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("color"), value });
        self
}
#[inline]
pub fn background_color(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("background-color"), value });
        self
}
#[inline]
pub fn border_color(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("border-color"), value });
        self
}
#[inline]
pub fn border(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("border"), value });
        self
}
#[inline]
pub fn border_radius(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("border-radius"), value });
        self
}
#[inline]
pub fn font_size(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("font-size"), value });
        self
}
#[inline]
pub fn font_weight(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("font-weight"), value });
        self
}
#[inline]
pub fn text_align(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("text-align"), value });
        self
}
#[inline]
pub fn position(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("position"), value });
        self
}
#[inline]
pub fn top(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("top"), value });
        self
}
#[inline]
pub fn right(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("right"), value });
        self
}
#[inline]
pub fn bottom(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("bottom"), value });
        self
}
#[inline]
pub fn left(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("left"), value });
        self
}
#[inline]
pub fn opacity(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("opacity"), value });
        self
}
#[inline]
pub fn box_shadow(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("box-shadow"), value });
        self
}
#[inline]
pub fn cursor(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("cursor"), value });
        self
}
#[inline]
pub fn overflow(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("overflow"), value });
        self
}
#[inline]
pub fn overflow_x(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("overflow-x"), value });
        self
}
#[inline]
pub fn overflow_y(mut self, value: String) -> Style {
        self.properties.push(StyleProperty { name: String::from("overflow-y"), value });
        self
}
#[inline]
pub fn to_string(&self) -> String {
        let mut result = String::new();
        for (i, prop) in self.properties.iter().enumerate() {
            if i > 0_usize {
                let _ = result.push_str("; ");
            }
            result.push_str(&prop.name);
            result.push_str(": ");
            result.push_str(&prop.value);
        }
        result
}
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&Style::to_string(self))
    }
}

