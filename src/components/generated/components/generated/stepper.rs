use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct StepperStep {
    pub label: String,
    pub description: String,
    pub completed: bool,
}

impl StepperStep {
#[inline]
pub fn new(label: String) -> StepperStep {
        StepperStep { label, description: String::new(), completed: false }
}
#[inline]
pub fn description(mut self, desc: String) -> StepperStep {
        self.description = desc;
        self
}
#[inline]
pub fn completed(mut self, completed: bool) -> StepperStep {
        self.completed = completed;
        self
}
}

#[derive(Debug, Clone)]
pub struct Stepper {
    pub steps: Vec<StepperStep>,
    pub current_step: i32,
}

impl Stepper {
#[inline]
pub fn new() -> Stepper {
        Stepper { steps: Vec::new(), current_step: 0 }
}
#[inline]
pub fn step(mut self, step: StepperStep) -> Stepper {
        self.steps.push(step);
        self
}
#[inline]
pub fn current_step(mut self, index: i32) -> Stepper {
        self.current_step = index;
        self
}
}

impl Renderable for Stepper {
#[inline]
fn render(self) -> String {
        let mut html = String::new();
        html.push_str("<div style='display: flex; align-items: center; justify-content: space-between; padding: 24px 0;'>");
        let total_steps = self.steps.len() as i32;
        for (step_index, step) in self.steps.iter().enumerate() {
            let step_index = step_index as i32;
            let is_current = step_index == self.current_step;
            let is_completed = step.completed || step_index < self.current_step;
            html.push_str("<div style='display: flex; flex-direction: column; align-items: center; flex: 1;'>");
            let bg_color = {
                if is_completed {
                    "#10b981".to_string()
                } else {
                    if is_current {
                        "#3b82f6".to_string()
                    } else {
                        "#e2e8f0".to_string()
                    }
                }
            };
            let text_color = {
                if is_completed || is_current {
                    "white".to_string()
                } else {
                    "#718096".to_string()
                }
            };
            html.push_str("<div style='width: 40px; height: 40px; border-radius: 50%; background: ");
            html.push_str(&bg_color);
            html.push_str("; color: ");
            html.push_str(&text_color);
            html.push_str("; display: flex; align-items: center; justify-content: center; font-weight: 600; font-size: 16px; margin-bottom: 8px;'>");
            if is_completed {
                html.push('✓')
            } else {
                html.push_str(&format!("{}", step_index + 1))
            }
            html.push_str("</div>");
            html.push_str("<div style='text-align: center;'>");
            html.push_str("<div style='font-weight: 600; font-size: 14px; color: ");
            if is_current {
                html.push_str("#1a202c")
            } else {
                html.push_str("#718096")
            }
            html.push_str("; margin-bottom: 4px;'>");
            html.push_str(&step.label);
            html.push_str("</div>");
            if step.description.len() > (0 as usize) {
                html.push_str("<div style='font-size: 12px; color: #a0aec0;'>");
                html.push_str(&step.description);
                html.push_str("</div>")
            }
            html.push_str("</div>");
            html.push_str("</div>");
            if step_index < total_steps - 1 {
                let line_color = {
                    if is_completed {
                        "#10b981".to_string()
                    } else {
                        "#e2e8f0".to_string()
                    }
                };
                html.push_str("<div style='flex: 1; height: 2px; background: ");
                html.push_str(&line_color);
                html.push_str("; margin: 0 8px; margin-bottom: 48px;'></div>")
            }
        }
        html.push_str("</div>");
        html
}
}

