#![allow(clippy::all)]
#![allow(noop_method_call)]
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
        Chart {
            title: "".to_string(),
            bars: Vec::new(),
            width: 320_i64,
            height: 160_i64,
        }
    }
    #[inline]
    pub fn title(mut self, title: String) -> Chart {
        self.title = title;
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
    #[inline]
    fn render(&self) -> String {
        let mut max_v = 1;
        let mut i = 0;
        while i < self.bars.len() {
            let v = self.bars[i].value;
            if v > max_v {
                max_v = v;
            }
            i += 1;
        }
        let mut bars_html = "".to_string();
        let mut bi = 0;
        let segment = 40;
        while bi < self.bars.len() {
            let bar = &self.bars[bi];
            let h = {
                if max_v == 0 {
                    0
                } else {
                    bar.value * (self.height as i64 - 40_i64) / max_v
                }
            };
            let x = 20 + bi * segment;
            let y = self.height as i64 - 20_i64 - h;
            bars_html = format!("{}<rect class='wj-chart-bar' x='{}' y='{}' width='24' height='{}' data-label='{}' data-value='{}'/>", bars_html, x, y, h, bar.label.clone(), bar.value);
            bi += 1;
        }
        let title_html: String = {
            if !self.title.is_empty() {
                format!(
                    "<text class='wj-chart-title' x='12' y='16'>{}</text>",
                    self.title.clone()
                )
            } else {
                "".to_string()
            }
        };
        format!("<div class='wj-chart'><svg class='wj-chart-svg' width='{}' height='{}' viewBox='0 0 {} {}' role='img'>{}{}</svg></div>", self.width, self.height, self.width, self.height, title_html, bars_html)
    }
}
