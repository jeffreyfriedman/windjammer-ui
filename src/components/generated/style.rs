#[derive(Debug, Clone, Default)]
pub struct Style {
    pub properties: Vec<StyleProperty>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct StyleProperty {
    pub name: String,
    pub value: String,
}

impl Style {
    #[inline]
    pub fn new() -> Style {
        Style {
            properties: Vec::new(),
        }
    }
    #[inline]
    pub fn display(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "display".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn flex_direction(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "flex-direction".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn gap(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "gap".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn align_items(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "align-items".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn justify_content(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "justify-content".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn margin(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "margin".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn padding(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "padding".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn width(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "width".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn height(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "height".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn min_width(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "min-width".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn max_width(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "max-width".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn color(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "color".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn background_color(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "background-color".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn border_color(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "border-color".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn border(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "border".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn border_radius(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "border-radius".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn font_size(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "font-size".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn font_weight(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "font-weight".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn text_align(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "text-align".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn position(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "position".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn top(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "top".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn right(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "right".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn bottom(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "bottom".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn left(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "left".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn opacity(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "opacity".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn box_shadow(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "box-shadow".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn cursor(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "cursor".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn overflow(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "overflow".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn overflow_x(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "overflow-x".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn overflow_y(mut self, value: String) -> Style {
        self.properties.push(StyleProperty {
            name: "overflow-y".to_string(),
            value,
        });
        self
    }
    #[inline]
    pub fn to_css(&self) -> String {
        let mut result = String::new();
        for (i, prop) in self.properties.iter().enumerate() {
            if i > 0 {
                result.push_str("; ")
            }
            result.push_str(prop.name.as_str());
            result.push_str(": ");
            result.push_str(prop.value.as_str());
        }
        result
    }
}
