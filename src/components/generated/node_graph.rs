#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum PinType {
    Flow,
    Bool,
    Int,
    Float,
    Vec2,
    Vec3,
    Vec4,
    Color,
    Texture,
    Object,
    Any,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodePin {
    pub id: String,
    pub name: String,
    pub pin_type: PinType,
    pub is_input: bool,
    pub connected_to: Option<String>,
    pub default_value: String,
}

impl NodePin {
    #[inline]
    pub fn input(id: String, name: String, pin_type: PinType) -> NodePin {
        NodePin {
            id,
            name,
            pin_type,
            is_input: true,
            connected_to: None,
            default_value: "".to_string(),
        }
    }
    #[inline]
    pub fn output(id: String, name: String, pin_type: PinType) -> NodePin {
        NodePin {
            id,
            name,
            pin_type,
            is_input: false,
            connected_to: None,
            default_value: "".to_string(),
        }
    }
    #[inline]
    pub fn default_value(mut self, value: String) -> NodePin {
        self.default_value = value;
        self
    }
    #[inline]
    pub fn connect(mut self, target: &String) -> NodePin {
        self.connected_to = Some(target.to_string());
        self
    }
    #[inline]
    pub fn get_color(&self) -> String {
        match self.pin_type {
            PinType::Flow => String::from("#ffffff").to_string(),
            PinType::Bool => String::from("#e94560").to_string(),
            PinType::Int => String::from("#00d9ff").to_string(),
            PinType::Float => String::from("#4ade80").to_string(),
            PinType::Vec2 => String::from("#facc15").to_string(),
            PinType::Vec3 => String::from("#f59e0b").to_string(),
            PinType::Vec4 => String::from("#a855f7").to_string(),
            PinType::Color => String::from("#ec4899").to_string(),
            PinType::Texture => String::from("#fb923c").to_string(),
            PinType::Object => String::from("#3b82f6").to_string(),
            PinType::Any => String::from("#888888").to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum NodeCategory {
    Math,
    Logic,
    Texture,
    Color,
    Vector,
    Flow,
    Event,
    Variable,
    Custom,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GraphNode {
    pub id: String,
    pub title: String,
    pub category: NodeCategory,
    pub x: f32,
    pub y: f32,
    pub inputs: Vec<NodePin>,
    pub outputs: Vec<NodePin>,
    pub collapsed: bool,
    pub preview_enabled: bool,
    pub trace_active: bool,
    pub pin_watch_text: String,
}

impl GraphNode {
    #[inline]
    pub fn new(id: String, title: String, category: NodeCategory) -> GraphNode {
        GraphNode {
            id,
            title,
            category,
            x: 0.0_f32,
            y: 0.0_f32,
            inputs: Vec::new(),
            outputs: Vec::new(),
            collapsed: false,
            preview_enabled: false,
            trace_active: false,
            pin_watch_text: "".to_string(),
        }
    }
    #[inline]
    pub fn trace_highlight(mut self, active: bool) -> GraphNode {
        self.trace_active = active;
        self
    }
    #[inline]
    pub fn pin_watch(mut self, label: String) -> GraphNode {
        self.pin_watch_text = label;
        self
    }
    #[inline]
    pub fn position(mut self, x: f32, y: f32) -> GraphNode {
        self.x = x;
        self.y = y;
        self
    }
    #[inline]
    pub fn input(mut self, pin: NodePin) -> GraphNode {
        self.inputs.push(pin);
        self
    }
    #[inline]
    pub fn output(mut self, pin: NodePin) -> GraphNode {
        self.outputs.push(pin);
        self
    }
    #[inline]
    pub fn collapsed(mut self, collapsed: bool) -> GraphNode {
        self.collapsed = collapsed;
        self
    }
    #[inline]
    pub fn preview(mut self, enabled: bool) -> GraphNode {
        self.preview_enabled = enabled;
        self
    }
    #[inline]
    pub fn get_category_color(&self) -> String {
        match self.category {
            NodeCategory::Math => String::from("#4ade80").to_string(),
            NodeCategory::Logic => String::from("#e94560").to_string(),
            NodeCategory::Texture => String::from("#fb923c").to_string(),
            NodeCategory::Color => String::from("#ec4899").to_string(),
            NodeCategory::Vector => String::from("#facc15").to_string(),
            NodeCategory::Flow => String::from("#ffffff").to_string(),
            NodeCategory::Event => String::from("#3b82f6").to_string(),
            NodeCategory::Variable => String::from("#a855f7").to_string(),
            NodeCategory::Custom => String::from("#888888").to_string(),
        }
    }
}

impl Renderable for GraphNode {
    #[inline]
    fn render(&self) -> String {
        let header_color = self.get_category_color();
        let mut inputs_html = "".to_string();
        for pin in &self.inputs {
            let color = pin.get_color();
            let connected_class: String = match pin.connected_to.clone() {
                Some(_) => String::from("connected"),
                None => String::new(),
            };
            inputs_html = inputs_html + &format!("\n                <div class='node-pin input {}'>\n                    <div class='pin-socket' style='background: {};' data-pin='{}'></div>\n                    <span class='pin-name'>{}</span>\n                </div>\n            ", connected_class, color, pin.id.clone(), pin.name.clone());
        }
        let mut outputs_html = "".to_string();
        for pin in &self.outputs {
            let color = pin.get_color();
            let connected_class: String = match pin.connected_to.clone() {
                Some(_) => String::from("connected"),
                None => String::new(),
            };
            outputs_html = outputs_html + &format!("\n                <div class='node-pin output {}'>\n                    <span class='pin-name'>{}</span>\n                    <div class='pin-socket' style='background: {};' data-pin='{}'></div>\n                </div>\n            ", connected_class, pin.name.clone(), color, pin.id.clone());
        }
        let preview_html = {
            if self.preview_enabled {
                String::from(
                    "<div class='node-preview'><canvas class='preview-canvas'></canvas></div>",
                )
            } else {
                String::new()
            }
        };
        let trace_class = {
            if self.trace_active {
                String::from(" trace-active")
            } else {
                String::new()
            }
        };
        let watch_html: String = {
            if !self.pin_watch_text.is_empty() {
                format!(
                    "<div class='pin-watch'>{}</div>",
                    self.pin_watch_text.clone()
                )
            } else {
                "".to_string()
            }
        };
        format!("\n            <div class='graph-node{}' id='{}' style='left: {}px; top: {}px;'>\n                <div class='node-header' style='background: {};'>\n                    <span class='node-title'>{}</span>\n                    <div class='node-actions'>\n                        <button class='node-btn preview' title='Preview'>👁</button>\n                        <button class='node-btn collapse' title='Collapse'>−</button>\n                    </div>\n                </div>\n                <div class='node-body'>\n                    <div class='node-inputs'>\n                        {}\n                    </div>\n                    <div class='node-outputs'>\n                        {}\n                    </div>\n                </div>\n                {}\n                {}\n            </div>\n        ", trace_class, self.id.clone(), self.x, self.y, header_color, self.title.clone(), inputs_html, outputs_html, preview_html, watch_html)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct NodeConnection {
    pub from_node: String,
    pub from_pin: String,
    pub to_node: String,
    pub to_pin: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct NodeGraph {
    pub width: i32,
    pub height: i32,
    pub nodes: Vec<GraphNode>,
    pub connections: Vec<NodeConnection>,
    pub zoom: f32,
    pub pan_x: f32,
    pub pan_y: f32,
    pub show_grid: bool,
    pub on_change: String,
    pub trace_node_ids: Vec<String>,
}

impl NodeGraph {
    #[inline]
    pub fn new() -> NodeGraph {
        NodeGraph {
            width: 800_i32,
            height: 600_i32,
            nodes: Vec::new(),
            connections: Vec::new(),
            zoom: 1.0_f32,
            pan_x: 0.0_f32,
            pan_y: 0.0_f32,
            show_grid: true,
            on_change: "".to_string(),
            trace_node_ids: Vec::new(),
        }
    }
    /// Highlight nodes on the active execution path (from graph_trace runtime).
    #[inline]
    pub fn apply_execution_trace(mut self, node_ids: Vec<String>) -> NodeGraph {
        self.trace_node_ids = node_ids;
        self
    }
    #[inline]
    pub fn size(mut self, width: i32, height: i32) -> NodeGraph {
        self.width = width;
        self.height = height;
        self
    }
    #[inline]
    pub fn node(mut self, node: GraphNode) -> NodeGraph {
        self.nodes.push(node);
        self
    }
    #[inline]
    pub fn connect(
        mut self,
        from_node: String,
        from_pin: String,
        to_node: String,
        to_pin: String,
    ) -> NodeGraph {
        self.connections.push(NodeConnection {
            from_node,
            from_pin,
            to_node,
            to_pin,
        });
        self
    }
    #[inline]
    pub fn zoom(mut self, zoom: f32) -> NodeGraph {
        self.zoom = zoom;
        self
    }
    #[inline]
    pub fn pan(mut self, x: f32, y: f32) -> NodeGraph {
        self.pan_x = x;
        self.pan_y = y;
        self
    }
}

impl Renderable for NodeGraph {
    #[inline]
    fn render(&self) -> String {
        let mut nodes_html = "".to_string();
        for n in &self.nodes {
            let mut node = n.clone();
            for tid in &self.trace_node_ids {
                if node.id == *tid {
                    node = node.trace_highlight(true);
                }
            }
            nodes_html = format!("{}{}{}", nodes_html, node.render(), "\n");
        }
        let mut connections_html = "".to_string();
        for c in &self.connections {
            connections_html = connections_html + &format!("\n                <path class='node-connection' \n                      data-from='{}:{}' \n                      data-to='{}:{}'/>\n            ", c.from_node.clone(), c.from_pin.clone(), c.to_node.clone(), c.to_pin.clone());
        }
        let grid_class = {
            if self.show_grid {
                String::from("show-grid")
            } else {
                String::new()
            }
        };
        format!("\n            <div class='node-graph {}' style='width: {}px; height: {}px;'>\n                <div class='graph-toolbar'>\n                    <button onclick='addNode()'>+ Add Node</button>\n                    <span class='toolbar-sep'></span>\n                    <button onclick='zoomIn()'>🔍+</button>\n                    <button onclick='zoomOut()'>🔍−</button>\n                    <button onclick='fitAll()'>⊞</button>\n                    <span class='zoom-level'>{:.0}%</span>\n                </div>\n                <div class='graph-canvas' \n                     style='transform: scale({}) translate({}px, {}px);'>\n                    <svg class='connections-layer'>\n                        {}\n                    </svg>\n                    <div class='nodes-layer'>\n                        {}\n                    </div>\n                </div>\n                <div class='graph-minimap'>\n                    <div class='minimap-viewport'></div>\n                </div>\n            </div>\n        ", grid_class, self.width, self.height, self.zoom * 100.0_f32, self.zoom, self.pan_x, self.pan_y, connections_html, nodes_html)
    }
}

#[inline]
pub fn node_graph_styles() -> String {
    "\n    .node-graph {\n        position: relative;\n        background: #0a0a1a;\n        border-radius: 8px;\n        overflow: hidden;\n    }\n    \n    .node-graph.show-grid {\n        background-image: \n            linear-gradient(rgba(255,255,255,0.03) 1px, transparent 1px),\n            linear-gradient(90deg, rgba(255,255,255,0.03) 1px, transparent 1px);\n        background-size: 20px 20px;\n    }\n    \n    .graph-toolbar {\n        position: absolute;\n        top: 8px;\n        left: 8px;\n        display: flex;\n        gap: 4px;\n        padding: 4px;\n        background: rgba(22, 33, 62, 0.9);\n        border-radius: 4px;\n        z-index: 100;\n    }\n    \n    .graph-toolbar button {\n        padding: 4px 8px;\n        border: none;\n        border-radius: 4px;\n        background: #0f3460;\n        color: #888;\n        cursor: pointer;\n    }\n    \n    .graph-toolbar button:hover {\n        background: #1a4a8a;\n        color: #e0e0e0;\n    }\n    \n    .toolbar-sep {\n        width: 1px;\n        background: #333;\n    }\n    \n    .zoom-level {\n        padding: 0 8px;\n        font-size: 12px;\n        color: #666;\n    }\n    \n    .graph-canvas {\n        position: absolute;\n        top: 0;\n        left: 0;\n        width: 100%;\n        height: 100%;\n        transform-origin: center center;\n    }\n    \n    .connections-layer {\n        position: absolute;\n        top: 0;\n        left: 0;\n        width: 100%;\n        height: 100%;\n        pointer-events: none;\n    }\n    \n    .node-connection {\n        fill: none;\n        stroke: #666;\n        stroke-width: 2;\n    }\n    \n    .nodes-layer {\n        position: absolute;\n        top: 0;\n        left: 0;\n    }\n    \n    /* Graph Node */\n    .graph-node {\n        position: absolute;\n        min-width: 180px;\n        background: #16213e;\n        border-radius: 8px;\n        box-shadow: 0 4px 12px rgba(0,0,0,0.3);\n        user-select: none;\n    }\n    \n    .node-header {\n        display: flex;\n        align-items: center;\n        justify-content: space-between;\n        padding: 8px 12px;\n        border-radius: 8px 8px 0 0;\n        cursor: move;\n    }\n    \n    .node-title {\n        font-size: 12px;\n        font-weight: 600;\n        color: #1a1a2e;\n    }\n    \n    .node-actions {\n        display: flex;\n        gap: 4px;\n    }\n    \n    .node-btn {\n        width: 20px;\n        height: 20px;\n        border: none;\n        background: rgba(0,0,0,0.2);\n        border-radius: 4px;\n        font-size: 10px;\n        cursor: pointer;\n        color: rgba(0,0,0,0.6);\n    }\n    \n    .node-btn:hover {\n        background: rgba(0,0,0,0.4);\n        color: rgba(0,0,0,0.8);\n    }\n    \n    .node-body {\n        display: flex;\n        justify-content: space-between;\n        padding: 8px 0;\n    }\n    \n    .node-inputs, .node-outputs {\n        display: flex;\n        flex-direction: column;\n        gap: 4px;\n    }\n    \n    .node-pin {\n        display: flex;\n        align-items: center;\n        gap: 8px;\n        padding: 4px 12px;\n        cursor: pointer;\n    }\n    \n    .node-pin.input {\n        flex-direction: row;\n    }\n    \n    .node-pin.output {\n        flex-direction: row-reverse;\n    }\n    \n    .pin-socket {\n        width: 12px;\n        height: 12px;\n        border-radius: 50%;\n        border: 2px solid rgba(255,255,255,0.3);\n        transition: transform 0.15s;\n    }\n    \n    .node-pin:hover .pin-socket {\n        transform: scale(1.3);\n        border-color: white;\n    }\n    \n    .node-pin.connected .pin-socket {\n        border-color: white;\n    }\n    \n    .pin-name {\n        font-size: 11px;\n        color: #888;\n    }\n    \n    .node-preview {\n        padding: 8px;\n        border-top: 1px solid rgba(255,255,255,0.1);\n    }\n    \n    .preview-canvas {\n        width: 100%;\n        height: 60px;\n        background: #0a0a1a;\n        border-radius: 4px;\n    }\n    \n    /* Minimap */\n    .graph-minimap {\n        position: absolute;\n        bottom: 8px;\n        right: 8px;\n        width: 150px;\n        height: 100px;\n        background: rgba(22, 33, 62, 0.9);\n        border-radius: 4px;\n        border: 1px solid #333;\n    }\n    \n    .minimap-viewport {\n        position: absolute;\n        border: 2px solid #e94560;\n        background: rgba(233, 69, 96, 0.1);\n    }\n\n    .graph-node.trace-active {\n        outline: 2px solid #e94560;\n        box-shadow: 0 0 16px rgba(233, 69, 96, 0.55);\n    }\n\n    .pin-watch {\n        font-size: 10px;\n        color: #fbbf24;\n        padding: 4px 8px;\n        border-top: 1px solid rgba(251, 191, 36, 0.25);\n    }\n    ".to_string()
}
