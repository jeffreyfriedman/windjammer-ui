#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct BtCanvasNode {
    pub editor_id: i32,
    pub title: String,
    pub subtitle: String,
    pub accent: String,
    pub cx: i32,
    pub cy: i32,
    pub debug_class: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct BtWire {
    pub from_id: i32,
    pub to_id: i32,
}
impl BtWire {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut __bytes = Vec::with_capacity(8);
        __bytes.extend_from_slice(&self.from_id.to_ne_bytes());
        __bytes.extend_from_slice(&self.to_id.to_ne_bytes());
        __bytes
    }
}


#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct BtCanvasModel {
    pub nodes: Vec<BtCanvasNode>,
    pub wires: Vec<BtWire>,
    pub zoom: f32,
    pub pan_x: f32,
    pub pan_y: f32,
}

impl BtCanvasModel {

pub fn patrol_demo() -> BtCanvasModel {
        let mut nodes: Vec<BtCanvasNode> = Vec::new();
        nodes.push(BtCanvasNode { editor_id: 1_i32, title: String::from("Root"), subtitle: String::from("sequence"), accent: String::from("#4c6ef5"), cx: 360_i32, cy: 48_i32, debug_class: String::from("bt-visual-success") });
        nodes.push(BtCanvasNode { editor_id: 2_i32, title: String::from("patrol"), subtitle: String::from("action"), accent: String::from("#2f9e44"), cx: 140_i32, cy: 168_i32, debug_class: String::from("bt-visual-success") });
        nodes.push(BtCanvasNode { editor_id: 3_i32, title: String::from("is_enemy_near"), subtitle: String::from("condition"), accent: String::from("#f08c00"), cx: 360_i32, cy: 168_i32, debug_class: String::from("bt-visual-success") });
        nodes.push(BtCanvasNode { editor_id: 4_i32, title: String::from("chase"), subtitle: String::from("action"), accent: String::from("#2f9e44"), cx: 580_i32, cy: 168_i32, debug_class: String::from("bt-visual-success") });
        nodes.push(BtCanvasNode { editor_id: 5_i32, title: String::from("attack"), subtitle: String::from("action"), accent: String::from("#2f9e44"), cx: 800_i32, cy: 168_i32, debug_class: String::from("bt-visual-running") });
        let mut wires: Vec<BtWire> = Vec::new();
        wires.push(BtWire { from_id: 1_i32, to_id: 2_i32 });
        wires.push(BtWire { from_id: 1_i32, to_id: 3_i32 });
        wires.push(BtWire { from_id: 1_i32, to_id: 4_i32 });
        wires.push(BtWire { from_id: 1_i32, to_id: 5_i32 });
        BtCanvasModel { nodes, wires, zoom: 1.0_f32, pan_x: 0.0_f32, pan_y: 12.0_f32 }
}
}

impl Renderable for BtCanvasModel {

fn render(&self) -> String {
        let mut body = String::new();
        let mut i = 0_usize;
        while i < self.nodes.len() {
            let n = &self.nodes[i];
            let frag = String::new();
            let frag = format!("{}{}", frag, "<div class='bt-node-card ");
            let frag = format!("{}{} {}", frag, n.debug_class, "'");
            let frag = format!("{}{}", frag, " data-bt-node='");
            let frag = format!("{}{}", frag, n.editor_id);
            let frag = format!("{}{}", frag, "' style=\"left:");
            let frag = format!("{}{}", frag, n.cx);
            let frag = format!("{}{}", frag, "px;top:");
            let frag = format!("{}{}", frag, n.cy);
            let frag = format!("{}{}", frag, "px;border-color:");
            let frag = format!("{}{}", frag, n.accent);
            let frag = format!("{}{}", frag, ";\">");
            let frag = format!("{}{}", frag, "<div class='bt-node-head'>");
            let frag = format!("{}{}", frag, n.title);
            let frag = format!("{}{}", frag, "</div><div class='bt-node-meta'>");
            let frag = format!("{}{}", frag, n.subtitle);
            let frag = format!("{}{}", frag, "</div></div>");
            { use std::fmt::Write as _; write!(body, "{}", frag).expect("a formatting trait implementation returned an error"); };
            i = i.wrapping_add(1);
        }
        let mut svg_paths = String::new();
        let mut w = 0_usize;
        while w < self.wires.len() {
            let edge = self.wires[w];
            { use std::fmt::Write as _; write!(svg_paths, "<path class='bt-wire' data-from=\"{}\" data-to=\"{}\" />", edge.from_id, edge.to_id).expect("a formatting trait implementation returned an error"); };
            w = w.wrapping_add(1);
        }
        let z = format!("{}{}", "", "<div class='bt-canvas'>");
        let z = z + "<div class='bt-canvas-inner' style=\"transform: scale(" + &format!("{}", self.zoom) + ") translate(" + &format!("{}", self.pan_x) + "px," + &format!("{}", self.pan_y) + "px);\">";
        let z = format!("{}{}", z, "<svg class='bt-wire-layer'>");
        let z = format!("{}{}", z, svg_paths);
        let z = format!("{}{}", z, "</svg><div class='bt-node-layer'>");
        let z = format!("{}{}", z, body);
        format!("{}{}", z, "</div></div></div>")
}
}


pub fn bt_visual_canvas_styles() -> String {
    String::from("\n    .bt-canvas {\n        flex: 1;\n        background: radial-gradient(circle at 25% 20%, rgba(99,141,246,0.14), transparent 45%), #090b16;\n        position: relative;\n        overflow: hidden;\n    }\n    .bt-canvas-inner {\n        width: 100%;\n        height: 100%;\n        position: absolute;\n        left: 0;\n        top: 0;\n        transform-origin: 0 0;\n    }\n    .bt-wire-layer {\n        position: absolute;\n        inset: 0;\n        stroke: rgba(255,255,255,0.28);\n        fill: none;\n    }\n    .bt-node-layer {\n        position: absolute;\n        inset: 0;\n        pointer-events: auto;\n    }\n    .bt-node-card {\n        position: absolute;\n        min-width: 160px;\n        padding: 10px;\n        border-radius: 12px;\n        border: 2px solid transparent;\n        background: rgba(20,26,54,0.92);\n        box-shadow: 0 14px 32px rgba(0,0,0,0.45);\n        color: #e8eaf5;\n        font-family: 'Inter',system-ui,sans-serif;\n    }\n    .bt-visual-running {\n        outline: 2px solid rgba(250,229,116,0.95);\n        box-shadow: 0 0 18px rgba(250,229,116,0.35);\n    }\n    .bt-visual-success {\n        outline: 2px solid rgba(78,226,154,0.65);\n    }\n    .bt-visual-failure {\n        outline: 2px solid rgba(247,134,143,0.82);\n        box-shadow: 0 0 18px rgba(247,134,143,0.42);\n    }\n    .bt-visual-idle {\n        opacity: 0.86;\n        filter: grayscale(0.2);\n    }\n    .bt-node-head {\n        font-weight: 600;\n        letter-spacing: 0.01em;\n    }\n    .bt-node-meta {\n        font-size: 11px;\n        color: #aab2d9;\n        margin-top: 4px;\n        text-transform: uppercase;\n        letter-spacing: 0.14em;\n    }\n    ")
}

