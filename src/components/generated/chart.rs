#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct ChartBar {
    pub label: String,
    pub value: i64,
}

impl ChartBar {
#[inline]
pub fn new(label: String, value: i64) -> ChartBar {
        ChartBar { label, value }
}
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct Chart {
    pub title: String,
    pub bars: Vec<ChartBar>,
    pub width: i64,
    pub height: i64,
}

impl Chart {
#[inline]
pub fn new() -> Chart {
        Chart { title: String::new(), bars: Vec::new(), width: 320_i64, height: 160_i64 }
}
#[inline]
pub fn title(mut self, title: impl Into<String>) -> Chart {
        self.title = title.into();
        self
}
#[inline]
pub fn bar(mut self, bar: ChartBar) -> Chart {
        self.bars.push(bar);
        self
}
#[inline]
pub fn size(mut self, width: i64, height: i64) -> Chart {
        self.width = width;
        self.height = height;
        self
}
}

impl Renderable for Chart {

fn render(&self) -> String {
        let mut max_v = 1_i64;
        let mut i = 0_usize;
        while i < self.bars.len() {
            let v = self.bars[i].value;
            if v > max_v {
                max_v = v;
            }
            i += 1;
        }
        let mut bars_html = String::new();
        let mut bi = 0_usize;
        let segment = 40_usize;
        while bi < self.bars.len() {
            let bar = &self.bars[bi];
            let h = {
                if max_v == 0_i64 {
                    0_i64
                } else {
                    bar.value * (self.height - 40_i64) / max_v
                }
            };
            let x = 20_usize + bi * segment;
            let y = self.height - 20_i64 - h;
            { use std::fmt::Write as _; write!(bars_html, "<rect class='wj-chart-bar' x='{}' y='{}' width='24' height='{}' data-label='{}' data-value='{}'/>", x, y, h, bar.label, bar.value).expect("a formatting trait implementation returned an error"); };
            bi += 1;
        }
        let title_html = {
            if !self.title.is_empty() {
                format!("<text class='wj-chart-title' x='12' y='16'>{}</text>", self.title)
            } else {
                String::new()
            }
        };
        format!("<div class='wj-chart'><svg class='wj-chart-svg' width='{}' height='{}' viewBox='0 0 {} {}' role='img'>{}{}</svg></div>", self.width, self.height, self.width, self.height, title_html, bars_html)
}
}

