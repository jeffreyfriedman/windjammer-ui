#![allow(clippy::all)]
#![allow(noop_method_call)]

use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ColorPicker {
    value: String,
    label: String,
}

impl ColorPicker {
    #[inline]
    pub fn new() -> ColorPicker {
        ColorPicker {
            value: "#000000".to_string(),
            label: "".to_string(),
        }
    }
    #[inline]
    pub fn value(mut self, value: String) -> ColorPicker {
        self.value = value;
        self
    }
    #[inline]
    pub fn label(mut self, label: String) -> ColorPicker {
        self.label = label;
        self
    }
}

impl Renderable for ColorPicker {
    #[inline]
    fn render(self) -> String {
        let label_html = {
            if self.label != "" {
                format!("<label>{}</label>", self.label)
            } else {
                "".to_string()
            }
        };
        format!(
            "<div class='wj-color-picker'>
  {}
  <input type='color' value='{}' class='wj-color-input'>
  <span class='wj-color-value'>{}</span>
</div>",
            label_html, self.value, self.value
        )
    }
}
