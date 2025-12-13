#![allow(clippy::all)]
#![allow(noop_method_call)]

use super::traits::Renderable;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurvePoint {
    pub time: f32,
    pub value: f32,
    pub in_tangent: f32,
    pub out_tangent: f32,
}

impl CurvePoint {
    #[inline]
    pub fn new(time: f32, value: f32) -> CurvePoint {
        CurvePoint {
            time,
            value,
            in_tangent: 0.0,
            out_tangent: 0.0,
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

#[derive(Debug, Clone, PartialEq)]
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
    pub fn new(&mut self) -> CurveEditor {
        let mut points = Vec::new();
        self.points.push(CurvePoint::new(0.0, 0.0));
        self.points.push(CurvePoint::new(1.0, 1.0));
        CurveEditor {
            width: 300,
            height: 200,
            points,
            min_value: 0.0,
            max_value: 1.0,
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
                self.points.push(CurvePoint::new(0.0, 0.0));
                self.points.push(CurvePoint::new(1.0, 1.0))
            }
            EasingPreset::EaseIn => {
                self.points
                    .push(CurvePoint::new(0.0, 0.0).tangents(0.0, 0.5));
                self.points
                    .push(CurvePoint::new(1.0, 1.0).tangents(0.5, 0.0))
            }
            EasingPreset::EaseOut => {
                self.points
                    .push(CurvePoint::new(0.0, 0.0).tangents(0.0, 2.0));
                self.points
                    .push(CurvePoint::new(1.0, 1.0).tangents(2.0, 0.0))
            }
            EasingPreset::EaseInOut => {
                self.points
                    .push(CurvePoint::new(0.0, 0.0).tangents(0.0, 0.5));
                self.points.push(CurvePoint::new(0.5, 0.5));
                self.points
                    .push(CurvePoint::new(1.0, 1.0).tangents(0.5, 0.0))
            }
            _ => {
                self.points.push(CurvePoint::new(0.0, 0.0));
                self.points.push(CurvePoint::new(1.0, 1.0))
            }
        }
    }
}

impl Renderable for CurveEditor {
    fn render(self) -> String {
        let mut path_d = "".to_string();
        let range = self.max_value - self.min_value;
        for i in 0..self.points.len() {
            let p = self.points.get(i);
            match p {
                Some(point) => {
                    let x = point.time * self.width as f32;
                    let y = self.height as f32
                        - (point.value - self.min_value) / range * self.height as f32;
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
        for i in 0..self.points.len() {
            let p = self.points.get(i);
            match p {
                Some(point) => {
                    let x = point.time * self.width as f32;
                    let y = self.height as f32
                        - (point.value - self.min_value) / range * self.height as f32;
                    points_html = format!(
                        "{}<circle cx='{}' cy='{}' r='6' class='curve-point' data-index='{}'/>",
                        points_html, x, y, i
                    );
                }
                None => {}
            }
        }
        let grid_html = {
            if self.grid_enabled {
                format!(
                    "
                <line x1='0' y1='{}' x2='{}' y2='{}' class='grid-line'/>
                <line x1='0' y1='{}' x2='{}' y2='{}' class='grid-line'/>
                <line x1='{}' y1='0' x2='{}' y2='{}' class='grid-line'/>
                <line x1='{}' y1='0' x2='{}' y2='{}' class='grid-line'/>
            ",
                    self.height / 4,
                    self.width,
                    self.height / 4,
                    self.height * 3 / 4,
                    self.width,
                    self.height * 3 / 4,
                    self.width / 4,
                    self.width / 4,
                    self.height,
                    self.width * 3 / 4,
                    self.width * 3 / 4,
                    self.height
                )
            } else {
                "".to_string()
            }
        };
        format!(
            "
            <div class='curve-editor'>
                <div class='curve-toolbar'>
                    <button onclick='setCurvePreset(\"linear\")'>Linear</button>
                    <button onclick='setCurvePreset(\"easeIn\")'>Ease In</button>
                    <button onclick='setCurvePreset(\"easeOut\")'>Ease Out</button>
                    <button onclick='setCurvePreset(\"easeInOut\")'>Ease In/Out</button>
                </div>
                <svg class='curve-canvas' width='{}' height='{}' viewBox='0 0 {} {}'>
                    <rect class='curve-bg' width='100%' height='100%'/>
                    {}
                    <path d='{}' class='curve-line'/>
                    {}
                </svg>
                <div class='curve-values'>
                    <span>{:.2}</span>
                    <span>{:.2}</span>
                </div>
            </div>
        ",
            self.width,
            self.height,
            self.width,
            self.height,
            grid_html,
            path_d,
            points_html,
            self.max_value,
            self.min_value
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct GradientEditor {
    pub width: i32,
    pub height: i32,
    pub stops: Vec<GradientStop>,
    pub on_change: String,
}

impl GradientEditor {
    #[inline]
    pub fn new(&mut self) -> GradientEditor {
        let mut stops = Vec::new();
        self.stops
            .push(GradientStop::new(0.0, "#000000".to_string()));
        self.stops
            .push(GradientStop::new(1.0, "#ffffff".to_string()));
        GradientEditor {
            width: 300,
            height: 40,
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
        for i in 0..self.stops.len() {
            let s = self.stops.get(i);
            match s {
                Some(stop) => {
                    if i > 0 {
                        gradient_stops = format!("{}{}", gradient_stops, ", ");
                    }
                    gradient_stops = format!(
                        "{}{}{}{}",
                        gradient_stops,
                        stop.color.as_str(),
                        " ",
                        format!("{}%", (stop.position * 100.0) as i32).as_str()
                    );
                }
                None => {}
            }
        }
        let mut markers_html = "".to_string();
        for i in 0..self.stops.len() {
            let s = self.stops.get(i);
            match s {
                Some(stop) => {
                    let left = (stop.position * 100.0) as i32;
                    markers_html = format!(
                        "{}
                        <div class='gradient-stop' style='left: {}%; background: {};' 
                             data-index='{}'></div>
                    ",
                        markers_html, left, stop.color, i
                    );
                }
                None => {}
            }
        }
        format!(
            "
            <div class='gradient-editor'>
                <div class='gradient-bar' style='background: linear-gradient(to right, {});'>
                    {}
                </div>
                <div class='gradient-controls'>
                    <button onclick='addGradientStop()'>+ Add Stop</button>
                </div>
            </div>
        ",
            gradient_stops, markers_html
        )
    }
}

#[inline]
pub fn curve_editor_styles() -> String {
    "
    .curve-editor {
        background: #16213e;
        border-radius: 8px;
        padding: 12px;
    }
    
    .curve-toolbar {
        display: flex;
        gap: 4px;
        margin-bottom: 12px;
    }
    
    .curve-toolbar button {
        padding: 4px 8px;
        border: none;
        border-radius: 4px;
        background: #0f3460;
        color: #888;
        font-size: 11px;
        cursor: pointer;
    }
    
    .curve-toolbar button:hover {
        background: #1a4a8a;
        color: #e0e0e0;
    }
    
    .curve-canvas {
        display: block;
        border-radius: 4px;
        overflow: hidden;
    }
    
    .curve-bg {
        fill: #0a0a1a;
    }
    
    .grid-line {
        stroke: #1a1a3a;
        stroke-width: 1;
    }
    
    .curve-line {
        fill: none;
        stroke: #e94560;
        stroke-width: 2;
    }
    
    .curve-point {
        fill: #e94560;
        stroke: white;
        stroke-width: 2;
        cursor: move;
    }
    
    .curve-point:hover {
        fill: #ff6b8a;
        r: 8;
    }
    
    .curve-values {
        display: flex;
        justify-content: space-between;
        margin-top: 4px;
        font-size: 10px;
        color: #666;
    }
    
    /* Gradient editor */
    .gradient-editor {
        background: #16213e;
        border-radius: 8px;
        padding: 12px;
    }
    
    .gradient-bar {
        position: relative;
        height: 32px;
        border-radius: 4px;
        cursor: crosshair;
    }
    
    .gradient-stop {
        position: absolute;
        top: 100%;
        width: 12px;
        height: 12px;
        margin-left: -6px;
        margin-top: 4px;
        border-radius: 50%;
        border: 2px solid white;
        cursor: move;
        box-shadow: 0 2px 4px rgba(0,0,0,0.3);
    }
    
    .gradient-controls {
        margin-top: 16px;
    }
    
    .gradient-controls button {
        padding: 6px 12px;
        border: 1px dashed #333;
        border-radius: 4px;
        background: transparent;
        color: #888;
        cursor: pointer;
    }
    
    .gradient-controls button:hover {
        border-color: #e94560;
        color: #e94560;
    }
    "
    .to_string()
}
