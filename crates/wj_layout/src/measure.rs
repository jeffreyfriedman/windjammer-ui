/// Measures text for layout. The real implementation wraps Parley; layout only needs sizes.
pub trait TextMeasure {
    /// Size of `text` at `font_size`, wrapped to `max_width` when given.
    fn measure(&self, text: &str, font_size: f32, max_width: Option<f32>) -> (f32, f32);
}

/// Deterministic measurer for tests: every char advances `0.5 * font_size`, lines are `1.2 * font_size` tall,
/// and text wraps at spaces when it exceeds `max_width`. Not a font; an independent, simple oracle.
#[derive(Clone, Copy, Debug, Default)]
pub struct FixedAdvanceMeasure;

impl TextMeasure for FixedAdvanceMeasure {
    fn measure(&self, text: &str, font_size: f32, max_width: Option<f32>) -> (f32, f32) {
        let adv = font_size * 0.5;
        let line_h = font_size * 1.2;
        let total = text.chars().count() as f32 * adv;
        match max_width {
            Some(w) if total > w && w > 0.0 => {
                let mut lines = 1.0f32;
                let mut cur = 0.0f32;
                let mut widest = 0.0f32;
                for word in text.split(' ') {
                    let ww = word.chars().count() as f32 * adv;
                    let with_space = if cur == 0.0 { ww } else { cur + adv + ww };
                    if with_space > w && cur > 0.0 {
                        widest = widest.max(cur);
                        lines += 1.0;
                        cur = ww;
                    } else {
                        cur = with_space;
                    }
                }
                widest = widest.max(cur);
                (widest.min(w.max(widest)), lines * line_h)
            }
            _ => (total, line_h),
        }
    }
}
