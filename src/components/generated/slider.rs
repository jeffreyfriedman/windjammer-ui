#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Slider {
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub value: f64,
    pub disabled: bool,
    pub label: String,
}

impl Slider {
    #[inline]
    pub fn new() -> Slider {
        Slider {
            min: 0.0,
            max: 100.0,
            step: 1.0,
            value: 50.0,
            disabled: false,
            label: "".to_string(),
        }
    }
    #[inline]
    pub fn min(mut self, min: f64) -> Slider {
        self.min = min;
        self
    }
    #[inline]
    pub fn max(mut self, max: f64) -> Slider {
        self.max = max;
        self
    }
    #[inline]
    pub fn step(mut self, step: f64) -> Slider {
        self.step = step;
        self
    }
    #[inline]
    pub fn value(mut self, value: f64) -> Slider {
        self.value = value;
        self
    }
    #[inline]
    pub fn disabled(mut self, disabled: bool) -> Slider {
        self.disabled = disabled;
        self
    }
    #[inline]
    pub fn label(mut self, label: String) -> Slider {
        self.label = label;
        self
    }
}

impl Renderable for Slider {
    #[inline]
    fn render(self) -> String {
        let disabled_attr = {
            if self.disabled {
                " disabled".to_string()
            } else {
                "".to_string()
            }
        };
        let label_html = {
            if self.label != "" {
                format!("{}{}{}", "<label>".to_string(), self.label, "</label>")
            } else {
                "".to_string()
            }
        };
        format!(
            "{}<input type='range' class='wj-slider' min='{}' max='{}' step='{}' value='{}'{}>",
            label_html, self.min, self.max, self.step, self.value, disabled_attr
        )
    }
}
