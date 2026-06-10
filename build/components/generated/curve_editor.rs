#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct CurvePoint {
    pub time: f32,
    pub value: f32,
    pub in_tangent: f32,
    pub out_tangent: f32,
}
impl CurvePoint {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut __bytes = Vec::with_capacity(16);
        __bytes.extend_from_slice(&self.time.to_ne_bytes());
        __bytes.extend_from_slice(&self.value.to_ne_bytes());
        __bytes.extend_from_slice(&self.in_tangent.to_ne_bytes());
        __bytes.extend_from_slice(&self.out_tangent.to_ne_bytes());
        __bytes
    }
}

impl CurvePoint {
    #[inline]
    pub fn new(time: f32, value: f32) -> CurvePoint {
        CurvePoint {
            time,
            value,
            in_tangent: 0.0_f32,
            out_tangent: 0.0_f32,
        }
    }
    #[inline]
    pub fn tangents(mut self, in_tan: f32, out_tan: f32) -> CurvePoint {
        self.in_tangent = in_tan;
        self.out_tangent = out_tan;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum EasingPreset {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Elastic,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct CurveEditor {
    pub width: i32,
    pub height: i32,
    pub points: Vec<CurvePoint>,
    pub min_value: f32,
    pub max_value: f32,
    pub grid_enabled: bool,
    pub on_change: String,
}

impl CurveEditor {
    #[inline]
    pub fn new() -> CurveEditor {
        let mut points: Vec<CurvePoint> = Vec::new();
        points.push(CurvePoint::new(0.0_f32, 0.0_f32));
        points.push(CurvePoint::new(1.0_f32, 1.0_f32));
        CurveEditor {
            width: 300_i32,
            height: 200_i32,
            points,
            min_value: 0.0_f32,
            max_value: 1.0_f32,
            grid_enabled: true,
            on_change: "".to_string(),
        }
    }
    #[inline]
    pub fn size(mut self, width: i32, height: i32) -> CurveEditor {
        self.width = width;
        self.height = height;
        self
    }
    #[inline]
    pub fn range(mut self, min: f32, max: f32) -> CurveEditor {
        self.min_value = min;
        self.max_value = max;
        self
    }
    #[inline]
    pub fn points(mut self, points: Vec<CurvePoint>) -> CurveEditor {
        self.points = points;
        self
    }
    #[inline]
    pub fn add_point(mut self, point: CurvePoint) -> CurveEditor {
        self.points.push(point);
        self
    }
    #[inline]
    pub fn grid(mut self, enabled: bool) -> CurveEditor {
        self.grid_enabled = enabled;
        self
    }
    #[inline]
    pub fn on_change(mut self, handler: String) -> CurveEditor {
        self.on_change = handler;
        self
    }
    #[inline]
    pub fn apply_preset(&mut self, preset: EasingPreset) {
        self.points.clear();
        match preset {
            EasingPreset::Linear => {
                self.points.push(CurvePoint::new(0.0_f32, 0.0_f32));
                self.points.push(CurvePoint::new(1.0_f32, 1.0_f32));
            }
            EasingPreset::EaseIn => {
                self.points
                    .push(CurvePoint::new(0.0_f32, 0.0_f32).tangents(0.0_f32, 0.5_f32));
                self.points
                    .push(CurvePoint::new(1.0_f32, 1.0_f32).tangents(0.5_f32, 0.0_f32));
            }
            EasingPreset::EaseOut => {
                self.points
                    .push(CurvePoint::new(0.0_f32, 0.0_f32).tangents(0.0_f32, 2.0_f32));
                self.points
                    .push(CurvePoint::new(1.0_f32, 1.0_f32).tangents(2.0_f32, 0.0_f32));
            }
            EasingPreset::EaseInOut => {
                self.points
                    .push(CurvePoint::new(0.0_f32, 0.0_f32).tangents(0.0_f32, 0.5_f32));
                self.points.push(CurvePoint::new(0.5_f32, 0.5_f32));
                self.points
                    .push(CurvePoint::new(1.0_f32, 1.0_f32).tangents(0.5_f32, 0.0_f32));
            }
            _ => {
                self.points.push(CurvePoint::new(0.0_f32, 0.0_f32));
                self.points.push(CurvePoint::new(1.0_f32, 1.0_f32));
            }
        }
    }
}

impl Renderable for CurveEditor {
    #[inline]
    fn render(self) -> String {
        let mut path_d = "".to_string();
        let range = self.max_value - self.min_value;
        for i in 0_usize..self.points.len() {
            let p = self.points.get(i);
            match p {
                Some(point) => {
                    let x = point.time as f32 * self.width as f32;
                    let y = self.height as f32
                        - (((point.value - self.min_value) / range) as f32 * self.height as f32)
                            as f32;
                    if i == 0 {
                        path_d = format!("M {} {}", x, y);
                    } else {
                        path_d = format!("{} L {} {}", path_d, x, y);
                    }
                }
                None => {}
            }
        }
        let mut points_html = "".to_string();
        for i in 0_usize..self.points.len() {
            let p = self.points.get(i);
            match p {
                Some(point) => {
                    let x = point.time as f32 * self.width as f32;
                    let y = self.height as f32
                        - (((point.value - self.min_value) / range) as f32 * self.height as f32)
                            as f32;
                    points_html = format!(
                        "{}<circle cx='{}' cy='{}' r='6' class='curve-point' data-index='{}'/>",
                        points_html, x, y, i
                    );
                }
                None => {}
            }
        }
        let grid_html: String = {
            if self.grid_enabled {
                format!("\n                <line x1='0' y1='{}' x2='{}' y2='{}' class='grid-line'/>\n                <line x1='0' y1='{}' x2='{}' y2='{}' class='grid-line'/>\n                <line x1='{}' y1='0' x2='{}' y2='{}' class='grid-line'/>\n                <line x1='{}' y1='0' x2='{}' y2='{}' class='grid-line'/>\n            ", self.height / 4_i32, self.width, self.height / 4_i32, self.height * 3_i32 / 4_i32, self.width, self.height * 3_i32 / 4_i32, self.width / 4_i32, self.width / 4_i32, self.height, self.width * 3_i32 / 4_i32, self.width * 3_i32 / 4_i32, self.height)
            } else {
                "".to_string()
            }
        };
        format!("\n            <div class='curve-editor'>\n                <div class='curve-toolbar'>\n                    <button onclick='setCurvePreset(\"linear\")'>Linear</button>\n                    <button onclick='setCurvePreset(\"easeIn\")'>Ease In</button>\n                    <button onclick='setCurvePreset(\"easeOut\")'>Ease Out</button>\n                    <button onclick='setCurvePreset(\"easeInOut\")'>Ease In/Out</button>\n                </div>\n                <svg class='curve-canvas' width='{}' height='{}' viewBox='0 0 {} {}'>\n                    <rect class='curve-bg' width='100%' height='100%'/>\n                    {}\n                    <path d='{}' class='curve-line'/>\n                    {}\n                </svg>\n                <div class='curve-values'>\n                    <span>{:.2}</span>\n                    <span>{:.2}</span>\n                </div>\n            </div>\n        ", self.width, self.height, self.width, self.height, grid_html, path_d, points_html, self.max_value, self.min_value)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct GradientStop {
    pub position: f32,
    pub color: String,
}

impl GradientStop {
    #[inline]
    pub fn new(position: f32, color: String) -> GradientStop {
        GradientStop { position, color }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct GradientEditor {
    pub width: i32,
    pub height: i32,
    pub stops: Vec<GradientStop>,
    pub on_change: String,
}

impl GradientEditor {
    #[inline]
    pub fn new() -> GradientEditor {
        let mut stops: Vec<GradientStop> = Vec::new();
        stops.push(GradientStop::new(0.0_f32, "#000000".to_string()));
        stops.push(GradientStop::new(1.0_f32, "#ffffff".to_string()));
        GradientEditor {
            width: 300_i32,
            height: 40_i32,
            stops,
            on_change: "".to_string(),
        }
    }
    #[inline]
    pub fn add_stop(mut self, stop: GradientStop) -> GradientEditor {
        self.stops.push(stop);
        self
    }
    #[inline]
    pub fn on_change(mut self, handler: String) -> GradientEditor {
        self.on_change = handler;
        self
    }
}

impl Renderable for GradientEditor {
    #[inline]
    fn render(self) -> String {
        let mut gradient_stops = "".to_string();
        for i in 0_usize..self.stops.len() {
            let s = self.stops.get(i);
            match s {
                Some(stop) => {
                    if i > 0 {
                        gradient_stops = format!("{}{}", gradient_stops, ", ");
                    }
                    gradient_stops = gradient_stops
                        + &stop.color.clone()
                        + &String::from(" ")
                        + &format!("{}%", (stop.position * 100.0_f32) as i32);
                }
                None => {}
            }
        }
        let mut markers_html = "".to_string();
        for i in 0_usize..self.stops.len() {
            let s = self.stops.get(i);
            match s {
                Some(stop) => {
                    let left = (stop.position * 100.0_f32) as i32;
                    markers_html = format!("{}\n                        <div class='gradient-stop' style='left: {}%; background: {};' \n                             data-index='{}'></div>\n                    ", markers_html, left, stop.color.clone(), i);
                }
                None => {}
            }
        }
        format!("\n            <div class='gradient-editor'>\n                <div class='gradient-bar' style='background: linear-gradient(to right, {});'>\n                    {}\n                </div>\n                <div class='gradient-controls'>\n                    <button onclick='addGradientStop()'>+ Add Stop</button>\n                </div>\n            </div>\n        ", gradient_stops, markers_html)
    }
}

#[inline]
pub fn curve_editor_styles() -> String {
    "\n    .curve-editor {\n        background: #16213e;\n        border-radius: 8px;\n        padding: 12px;\n    }\n    \n    .curve-toolbar {\n        display: flex;\n        gap: 4px;\n        margin-bottom: 12px;\n    }\n    \n    .curve-toolbar button {\n        padding: 4px 8px;\n        border: none;\n        border-radius: 4px;\n        background: #0f3460;\n        color: #888;\n        font-size: 11px;\n        cursor: pointer;\n    }\n    \n    .curve-toolbar button:hover {\n        background: #1a4a8a;\n        color: #e0e0e0;\n    }\n    \n    .curve-canvas {\n        display: block;\n        border-radius: 4px;\n        overflow: hidden;\n    }\n    \n    .curve-bg {\n        fill: #0a0a1a;\n    }\n    \n    .grid-line {\n        stroke: #1a1a3a;\n        stroke-width: 1;\n    }\n    \n    .curve-line {\n        fill: none;\n        stroke: #e94560;\n        stroke-width: 2;\n    }\n    \n    .curve-point {\n        fill: #e94560;\n        stroke: white;\n        stroke-width: 2;\n        cursor: move;\n    }\n    \n    .curve-point:hover {\n        fill: #ff6b8a;\n        r: 8;\n    }\n    \n    .curve-values {\n        display: flex;\n        justify-content: space-between;\n        margin-top: 4px;\n        font-size: 10px;\n        color: #666;\n    }\n    \n    /* Gradient editor */\n    .gradient-editor {\n        background: #16213e;\n        border-radius: 8px;\n        padding: 12px;\n    }\n    \n    .gradient-bar {\n        position: relative;\n        height: 32px;\n        border-radius: 4px;\n        cursor: crosshair;\n    }\n    \n    .gradient-stop {\n        position: absolute;\n        top: 100%;\n        width: 12px;\n        height: 12px;\n        margin-left: -6px;\n        margin-top: 4px;\n        border-radius: 50%;\n        border: 2px solid white;\n        cursor: move;\n        box-shadow: 0 2px 4px rgba(0,0,0,0.3);\n    }\n    \n    .gradient-controls {\n        margin-top: 16px;\n    }\n    \n    .gradient-controls button {\n        padding: 6px 12px;\n        border: 1px dashed #333;\n        border-radius: 4px;\n        background: transparent;\n        color: #888;\n        cursor: pointer;\n    }\n    \n    .gradient-controls button:hover {\n        border-color: #e94560;\n        color: #e94560;\n    }\n    ".to_string()
}
