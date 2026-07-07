#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ProgressVariant {
    Default,
    Success,
    Warning,
    Danger,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Progress {
    pub value: f64,
    pub max: f64,
    pub variant: ProgressVariant,
    pub show_label: bool,
}

impl Progress {
    #[inline]
    pub fn new(value: f64) -> Progress {
        Progress {
            value,
            max: 100.0_f64,
            variant: ProgressVariant::Default,
            show_label: true,
        }
    }
    #[inline]
    pub fn max(mut self, max: f64) -> Progress {
        self.max = max;
        self
    }
    #[inline]
    pub fn variant(mut self, variant: ProgressVariant) -> Progress {
        self.variant = variant;
        self
    }
    #[inline]
    pub fn show_label(mut self, show: bool) -> Progress {
        self.show_label = show;
        self
    }
}

impl Renderable for Progress {
    #[inline]
    fn render(&mut self) -> String {
        let percentage = (self.value / self.max * 100.0_f64).clamp(0.0_f64, 100.0_f64);
        let variant_class: String = match self.variant {
            ProgressVariant::Default => String::from("wj-progress-default"),
            ProgressVariant::Success => String::from("wj-progress-success"),
            ProgressVariant::Warning => String::from("wj-progress-warning"),
            ProgressVariant::Danger => String::from("wj-progress-danger"),
        };
        let color: String = match self.variant {
            ProgressVariant::Default => String::from("#3498db"),
            ProgressVariant::Success => String::from("#2ecc71"),
            ProgressVariant::Warning => String::from("#f39c12"),
            ProgressVariant::Danger => String::from("#e74c3c"),
        };
        let label_html: String = {
            if self.show_label {
                format!("{:.0}%", percentage)
            } else {
                "".to_string()
            }
        };
        format!("<div class='wj-progress-container' style='width: 100%; background-color: #e0e0e0; border-radius: 4px; overflow: hidden;'>\n  <div class='wj-progress-bar {}' style='width: {}%; height: 24px; background-color: {}; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; transition: width 0.3s ease;'>\n    {}\n  </div>\n</div>", variant_class, percentage, color, label_html)
    }
}
