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
        NodePin { id, name, pin_type, is_input: true, connected_to: None, default_value: "".to_string() }
}
#[inline]
pub fn output(id: String, name: String, pin_type: PinType) -> NodePin {
        NodePin { id, name, pin_type, is_input: false, connected_to: None, default_value: "".to_string() }
}
#[inline]
pub fn default_value(mut self, value: String) -> NodePin {
        self.default_value = value;
        self
}
#[inline]
pub fn connect(mut self, target: String) -> NodePin {
        self.connected_to = Some(target.to_string());
        self
}
#[inline]
pub fn get_color(&self) -> String {
        match self.pin_type {
            PinType::Flow => "#ffffff".to_string(),
            PinType::Bool => "#e94560".to_string(),
            PinType::Int => "#00d9ff".to_string(),
            PinType::Float => "#4ade80".to_string(),
            PinType::Vec2 => "#facc15".to_string(),
            PinType::Vec3 => "#f59e0b".to_string(),
            PinType::Vec4 => "#a855f7".to_string(),
            PinType::Color => "#ec4899".to_string(),
            PinType::Texture => "#fb923c".to_string(),
            PinType::Object => "#3b82f6".to_string(),
            PinType::Any => "#888888".to_string(),
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

#[derive(Debug, Clone)]
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
}

impl GraphNode {
#[inline]
pub fn new(id: String, title: String, category: NodeCategory) -> GraphNode {
        GraphNode { id, title, category, x: 0.0, y: 0.0, inputs: Vec::new(), outputs: Vec::new(), collapsed: false, preview_enabled: false }
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
            NodeCategory::Math => "#4ade80".to_string(),
            NodeCategory::Logic => "#e94560".to_string(),
            NodeCategory::Texture => "#fb923c".to_string(),
            NodeCategory::Color => "#ec4899".to_string(),
            NodeCategory::Vector => "#facc15".to_string(),
            NodeCategory::Flow => "#ffffff".to_string(),
            NodeCategory::Event => "#3b82f6".to_string(),
            NodeCategory::Variable => "#a855f7".to_string(),
            NodeCategory::Custom => "#888888".to_string(),
        }
}
}

impl Renderable for GraphNode {
#[inline]
fn render(self) -> String {
        let header_color = self.get_category_color();
        let mut inputs_html = "".to_string();
        for pin in &self.inputs {
            let color = pin.get_color();
            let connected_class = match pin.connected_to.clone() {
                Some(_) => "connected".to_string(),
                None => "".to_string(),
            };
            inputs_html += format!("
                <div class='node-pin input {}'>
                    <div class='pin-socket' style='background: {};' data-pin='{}'></div>
                    <span class='pin-name'>{}</span>
                </div>
            ", connected_class, color, pin.id, pin.name.clone()).as_str();
        }
        let mut outputs_html = "".to_string();
        for pin in &self.outputs {
            let color = pin.get_color();
            let connected_class = match pin.connected_to.clone() {
                Some(_) => "connected".to_string(),
                None => "".to_string(),
            };
            outputs_html += format!("
                <div class='node-pin output {}'>
                    <span class='pin-name'>{}</span>
                    <div class='pin-socket' style='background: {};' data-pin='{}'></div>
                </div>
            ", connected_class, pin.name.clone(), color, pin.id).as_str();
        }
        let preview_html = {
            if self.preview_enabled {
                "<div class='node-preview'><canvas class='preview-canvas'></canvas></div>".to_string()
            } else {
                "".to_string()
            }
        };
        format!("
            <div class='graph-node' id='{}' style='left: {}px; top: {}px;'>
                <div class='node-header' style='background: {};'>
                    <span class='node-title'>{}</span>
                    <div class='node-actions'>
                        <button class='node-btn preview' title='Preview'>👁</button>
                        <button class='node-btn collapse' title='Collapse'>−</button>
                    </div>
                </div>
                <div class='node-body'>
                    <div class='node-inputs'>
                        {}
                    </div>
                    <div class='node-outputs'>
                        {}
                    </div>
                </div>
                {}
            </div>
        ", self.id, self.x, self.y, header_color, self.title, inputs_html, outputs_html, preview_html)
}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct NodeConnection {
    pub from_node: String,
    pub from_pin: String,
    pub to_node: String,
    pub to_pin: String,
}

#[derive(Debug, Clone)]
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
}

impl NodeGraph {
#[inline]
pub fn new() -> NodeGraph {
        NodeGraph { width: 800, height: 600, nodes: Vec::new(), connections: Vec::new(), zoom: 1.0, pan_x: 0.0, pan_y: 0.0, show_grid: true, on_change: "".to_string() }
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
pub fn connect(mut self, from_node: String, from_pin: String, to_node: String, to_pin: String) -> NodeGraph {
        self.connections.push(NodeConnection { from_node, from_pin, to_node, to_pin });
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
fn render(self) -> String {
        let mut nodes_html = "".to_string();
        for n in &self.nodes {
            nodes_html = format!("{}{}{}", nodes_html, n.clone().render().as_str(), "
");
        }
        let mut connections_html = "".to_string();
        for c in &self.connections {
            connections_html += format!("
                <path class='node-connection' 
                      data-from='{}:{}' 
                      data-to='{}:{}'/>
            ", c.from_node.clone(), c.from_pin.clone(), c.to_node.clone(), c.to_pin.clone()).as_str();
        }
        let grid_class = {
            if self.show_grid {
                "show-grid".to_string()
            } else {
                "".to_string()
            }
        };
        format!("
            <div class='node-graph {}' style='width: {}px; height: {}px;'>
                <div class='graph-toolbar'>
                    <button onclick='addNode()'>+ Add Node</button>
                    <span class='toolbar-sep'></span>
                    <button onclick='zoomIn()'>🔍+</button>
                    <button onclick='zoomOut()'>🔍−</button>
                    <button onclick='fitAll()'>⊞</button>
                    <span class='zoom-level'>{:.0}%</span>
                </div>
                <div class='graph-canvas' 
                     style='transform: scale({}) translate({}px, {}px);'>
                    <svg class='connections-layer'>
                        {}
                    </svg>
                    <div class='nodes-layer'>
                        {}
                    </div>
                </div>
                <div class='graph-minimap'>
                    <div class='minimap-viewport'></div>
                </div>
            </div>
        ", grid_class, self.width, self.height, self.zoom * 100.0, self.zoom, self.pan_x, self.pan_y, connections_html, nodes_html)
}
}

#[inline]
pub fn node_graph_styles() -> String {
    "
    .node-graph {
        position: relative;
        background: #0a0a1a;
        border-radius: 8px;
        overflow: hidden;
    }
    
    .node-graph.show-grid {
        background-image: 
            linear-gradient(rgba(255,255,255,0.03) 1px, transparent 1px),
            linear-gradient(90deg, rgba(255,255,255,0.03) 1px, transparent 1px);
        background-size: 20px 20px;
    }
    
    .graph-toolbar {
        position: absolute;
        top: 8px;
        left: 8px;
        display: flex;
        gap: 4px;
        padding: 4px;
        background: rgba(22, 33, 62, 0.9);
        border-radius: 4px;
        z-index: 100;
    }
    
    .graph-toolbar button {
        padding: 4px 8px;
        border: none;
        border-radius: 4px;
        background: #0f3460;
        color: #888;
        cursor: pointer;
    }
    
    .graph-toolbar button:hover {
        background: #1a4a8a;
        color: #e0e0e0;
    }
    
    .toolbar-sep {
        width: 1px;
        background: #333;
    }
    
    .zoom-level {
        padding: 0 8px;
        font-size: 12px;
        color: #666;
    }
    
    .graph-canvas {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        transform-origin: center center;
    }
    
    .connections-layer {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        pointer-events: none;
    }
    
    .node-connection {
        fill: none;
        stroke: #666;
        stroke-width: 2;
    }
    
    .nodes-layer {
        position: absolute;
        top: 0;
        left: 0;
    }
    
    /* Graph Node */
    .graph-node {
        position: absolute;
        min-width: 180px;
        background: #16213e;
        border-radius: 8px;
        box-shadow: 0 4px 12px rgba(0,0,0,0.3);
        user-select: none;
    }
    
    .node-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 8px 12px;
        border-radius: 8px 8px 0 0;
        cursor: move;
    }
    
    .node-title {
        font-size: 12px;
        font-weight: 600;
        color: #1a1a2e;
    }
    
    .node-actions {
        display: flex;
        gap: 4px;
    }
    
    .node-btn {
        width: 20px;
        height: 20px;
        border: none;
        background: rgba(0,0,0,0.2);
        border-radius: 4px;
        font-size: 10px;
        cursor: pointer;
        color: rgba(0,0,0,0.6);
    }
    
    .node-btn:hover {
        background: rgba(0,0,0,0.4);
        color: rgba(0,0,0,0.8);
    }
    
    .node-body {
        display: flex;
        justify-content: space-between;
        padding: 8px 0;
    }
    
    .node-inputs, .node-outputs {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }
    
    .node-pin {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 4px 12px;
        cursor: pointer;
    }
    
    .node-pin.input {
        flex-direction: row;
    }
    
    .node-pin.output {
        flex-direction: row-reverse;
    }
    
    .pin-socket {
        width: 12px;
        height: 12px;
        border-radius: 50%;
        border: 2px solid rgba(255,255,255,0.3);
        transition: transform 0.15s;
    }
    
    .node-pin:hover .pin-socket {
        transform: scale(1.3);
        border-color: white;
    }
    
    .node-pin.connected .pin-socket {
        border-color: white;
    }
    
    .pin-name {
        font-size: 11px;
        color: #888;
    }
    
    .node-preview {
        padding: 8px;
        border-top: 1px solid rgba(255,255,255,0.1);
    }
    
    .preview-canvas {
        width: 100%;
        height: 60px;
        background: #0a0a1a;
        border-radius: 4px;
    }
    
    /* Minimap */
    .graph-minimap {
        position: absolute;
        bottom: 8px;
        right: 8px;
        width: 150px;
        height: 100px;
        background: rgba(22, 33, 62, 0.9);
        border-radius: 4px;
        border: 1px solid #333;
    }
    
    .minimap-viewport {
        position: absolute;
        border: 2px solid #e94560;
        background: rgba(233, 69, 96, 0.1);
    }
    ".to_string()
}

