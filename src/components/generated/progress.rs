#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ProgressVariant {
    Default,
    Success,
    Warning,
    Danger,
}

#[derive(Debug, Clone, PartialEq)]
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
            max: 100.0,
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
    fn render(self) -> String {
        let percentage = (self.value / self.max * 100.0).clamp(0.0, 100.0);
        let variant_class = match self.variant {
            ProgressVariant::Default => "wj-progress-default".to_string(),
            ProgressVariant::Success => "wj-progress-success".to_string(),
            ProgressVariant::Warning => "wj-progress-warning".to_string(),
            ProgressVariant::Danger => "wj-progress-danger".to_string(),
        };
        let color = match self.variant {
            ProgressVariant::Default => "#3498db".to_string(),
            ProgressVariant::Success => "#2ecc71".to_string(),
            ProgressVariant::Warning => "#f39c12".to_string(),
            ProgressVariant::Danger => "#e74c3c".to_string(),
        };
        let label_html = {
            if self.show_label {
                format!("{:.0}%", percentage)
            } else {
                "".to_string()
            }
        };
        format!("<div class='wj-progress-container' style='width: 100%; background-color: #e0e0e0; border-radius: 4px; overflow: hidden;'>
  <div class='wj-progress-bar {}' style='width: {}%; height: 24px; background-color: {}; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; transition: width 0.3s ease;'>
    {}
  </div>
</div>", variant_class, percentage, color, label_html)
    }
}
