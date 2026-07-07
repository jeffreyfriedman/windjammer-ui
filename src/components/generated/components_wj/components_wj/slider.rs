use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
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
            min: 0.0_f64,
            max: 100.0_f64,
            step: 1.0_f64,
            value: 50.0_f64,
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
    fn render(&self) -> String {
        let disabled_attr = {
            if self.disabled {
                String::from(" disabled")
            } else {
                String::new()
            }
        };
        let label_html = {
            if self.label != "" {
                "<label>".to_string() + &self.label.clone() + &String::from("</label>")
            } else {
                "".to_string()
            }
        };
        format!(
            "{}<input type='range' class='wj-slider' min='{}' max='{}' step='{}' value='{}'{}>",
            label_html,
            self.min.to_string(),
            self.max.to_string(),
            self.step.to_string(),
            self.value.to_string(),
            disabled_attr
        )
    }
}

fn main() {
    let slider1 = Slider::new().min(0.0_f64).max(100.0_f64).value(75.0_f64);
    let slider2 = Slider::new()
        .min(0.0_f64)
        .max(10.0_f64)
        .step(0.5_f64)
        .value(5.5_f64)
        .label("Volume".to_string());
    println!("{}", slider1.render());
    println!("{}", slider2.render());
}
