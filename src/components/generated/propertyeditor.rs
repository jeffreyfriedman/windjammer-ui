#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Clone, Debug, PartialEq)]
pub enum PropertyType {
    Number { min: f32, max: f32, step: f32 },
    Integer { min: i32, max: i32 },
    Boolean,
    Text,
    Color,
    Dropdown { options: Vec<String> },
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub property_type: PropertyType,
    pub unit: String,
    pub tooltip: String,
    pub on_change: String,
}

impl Property {
    #[inline]
    pub fn number(name: String, value: f32, min: f32, max: f32) -> Property {
        Property {
            name,
            value: format!("{:.3}", value),
            property_type: PropertyType::Number {
                min,
                max,
                step: 0.1_f32,
            },
            unit: "".to_string(),
            tooltip: "".to_string(),
            on_change: "".to_string(),
        }
    }
    #[inline]
    pub fn integer(name: String, value: i32, min: i32, max: i32) -> Property {
        Property {
            name,
            value: format!("{}", value),
            property_type: PropertyType::Integer { min, max },
            unit: "".to_string(),
            tooltip: "".to_string(),
            on_change: "".to_string(),
        }
    }
    #[inline]
    pub fn boolean(name: String, value: bool) -> Property {
        Property {
            name,
            value: {
                if value {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            },
            property_type: PropertyType::Boolean,
            unit: "".to_string(),
            tooltip: "".to_string(),
            on_change: "".to_string(),
        }
    }
    #[inline]
    pub fn text(name: String, value: String) -> Property {
        Property {
            name,
            value,
            property_type: PropertyType::Text,
            unit: "".to_string(),
            tooltip: "".to_string(),
            on_change: "".to_string(),
        }
    }
    #[inline]
    pub fn color(name: String, value: String) -> Property {
        Property {
            name,
            value,
            property_type: PropertyType::Color,
            unit: "".to_string(),
            tooltip: "".to_string(),
            on_change: "".to_string(),
        }
    }
    #[inline]
    pub fn unit(mut self, unit: String) -> Property {
        self.unit = unit;
        self
    }
    #[inline]
    pub fn tooltip(mut self, tooltip: String) -> Property {
        self.tooltip = tooltip;
        self
    }
    #[inline]
    pub fn on_change(mut self, handler: String) -> Property {
        self.on_change = handler;
        self
    }
}

impl Renderable for Property {
    #[inline]
    fn render(self) -> String {
        let tooltip_attr: String = {
            if self.tooltip != "" {
                format!(" title='{}'", self.tooltip)
            } else {
                "".to_string()
            }
        };
        let unit_html: String = {
            if self.unit != "" {
                format!("<span class='prop-unit'>{}</span>", self.unit)
            } else {
                "".to_string()
            }
        };
        let input_html: String = match self.property_type.clone() {
            PropertyType::Number {
                min: mn,
                max: mx,
                step: st,
            } => {
                format!("\n                    <div class='prop-number'>\n                        <input type='number' class='prop-input' \n                               value='{}' min='{}' max='{}' step='{}'\n                               onchange='{}(this.value)'/>\n                        {}\n                    </div>\n                ", self.value, mn, mx, st, self.on_change, unit_html)
            }
            PropertyType::Integer { min: mn, max: mx } => {
                format!("\n                    <div class='prop-number'>\n                        <input type='number' class='prop-input' \n                               value='{}' min='{}' max='{}' step='1'\n                               onchange='{}(this.value)'/>\n                        {}\n                    </div>\n                ", self.value, mn, mx, self.on_change, unit_html)
            }
            PropertyType::Boolean => {
                let checked = {
                    if self.value == String::from("true") {
                        String::from("checked")
                    } else {
                        String::new()
                    }
                };
                format!("\n                    <label class='prop-toggle'>\n                        <input type='checkbox' {} onchange='{}(this.checked)'/>\n                        <span class='toggle-slider'></span>\n                    </label>\n                ", checked, self.on_change)
            }
            PropertyType::Text => {
                format!("\n                    <input type='text' class='prop-input prop-text' \n                           value='{}' onchange='{}(this.value)'/>\n                ", self.value, self.on_change)
            }
            PropertyType::Color => {
                format!("\n                    <div class='prop-color'>\n                        <input type='color' class='color-swatch' \n                               value='{}' onchange='{}(this.value)'/>\n                        <input type='text' class='color-hex' \n                               value='{}' onchange='{}(this.value)'/>\n                    </div>\n                ", self.value, self.on_change, self.value, self.on_change)
            }
            PropertyType::Dropdown { options: opts } => {
                let mut options_html = String::new().to_string();
                for o in opts {
                    let selected = {
                        if o == self.value {
                            String::from("selected")
                        } else {
                            String::new()
                        }
                    };
                    options_html = options_html
                        + &format!("<option value='{}' {}>{}</option>", o, selected, o);
                }
                format!("\n                    <select class='prop-select' onchange='{}(this.value)'>\n                        {}\n                    </select>\n                ", self.on_change, options_html)
            }
        };
        format!("\n            <div class='prop-row'{}>\n                <label class='prop-label'>{}</label>\n                <div class='prop-value'>\n                    {}\n                </div>\n            </div>\n        ", tooltip_attr, self.name, input_html)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct Vec3Editor {
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub on_change: String,
}

impl Vec3Editor {
    #[inline]
    pub fn new(label: String, x: f32, y: f32, z: f32) -> Vec3Editor {
        Vec3Editor {
            label,
            x,
            y,
            z,
            on_change: "".to_string(),
        }
    }
    #[inline]
    pub fn on_change(mut self, handler: String) -> Vec3Editor {
        self.on_change = handler;
        self
    }
}

impl Renderable for Vec3Editor {
    #[inline]
    fn render(self) -> String {
        format!("\n            <div class='vec3-editor'>\n                <label class='prop-label'>{}</label>\n                <div class='vec3-inputs'>\n                    <div class='vec3-axis'>\n                        <span class='axis-label x'>X</span>\n                        <input type='number' step='0.1' value='{:.3}' \n                               onchange='{}(\"x\", this.value)'/>\n                    </div>\n                    <div class='vec3-axis'>\n                        <span class='axis-label y'>Y</span>\n                        <input type='number' step='0.1' value='{:.3}' \n                               onchange='{}(\"y\", this.value)'/>\n                    </div>\n                    <div class='vec3-axis'>\n                        <span class='axis-label z'>Z</span>\n                        <input type='number' step='0.1' value='{:.3}' \n                               onchange='{}(\"z\", this.value)'/>\n                    </div>\n                </div>\n            </div>\n        ", self.label, self.x, self.on_change, self.y, self.on_change, self.z, self.on_change)
    }
}

#[inline]
pub fn property_editor_styles() -> String {
    "\n    .prop-row {\n        display: flex;\n        align-items: center;\n        padding: 6px 0;\n        border-bottom: 1px solid rgba(255,255,255,0.05);\n    }\n    \n    .prop-row:hover {\n        background: rgba(255,255,255,0.02);\n    }\n    \n    .prop-label {\n        width: 100px;\n        font-size: 12px;\n        color: #999;\n        flex-shrink: 0;\n    }\n    \n    .prop-value {\n        flex: 1;\n    }\n    \n    .prop-input {\n        width: 100%;\n        padding: 6px 10px;\n        border: 1px solid #333;\n        border-radius: 4px;\n        background: #1a1a2e;\n        color: #e0e0e0;\n        font-size: 12px;\n    }\n    \n    .prop-input:focus {\n        border-color: #e94560;\n        outline: none;\n        box-shadow: 0 0 0 2px rgba(233, 69, 96, 0.2);\n    }\n    \n    .prop-number {\n        display: flex;\n        align-items: center;\n        gap: 4px;\n    }\n    \n    .prop-unit {\n        font-size: 11px;\n        color: #666;\n    }\n    \n    /* Toggle switch */\n    .prop-toggle {\n        position: relative;\n        display: inline-block;\n        width: 44px;\n        height: 24px;\n    }\n    \n    .prop-toggle input {\n        opacity: 0;\n        width: 0;\n        height: 0;\n    }\n    \n    .toggle-slider {\n        position: absolute;\n        cursor: pointer;\n        top: 0;\n        left: 0;\n        right: 0;\n        bottom: 0;\n        background-color: #333;\n        transition: 0.3s;\n        border-radius: 24px;\n    }\n    \n    .toggle-slider:before {\n        position: absolute;\n        content: '';\n        height: 18px;\n        width: 18px;\n        left: 3px;\n        bottom: 3px;\n        background-color: white;\n        transition: 0.3s;\n        border-radius: 50%;\n    }\n    \n    .prop-toggle input:checked + .toggle-slider {\n        background-color: #e94560;\n    }\n    \n    .prop-toggle input:checked + .toggle-slider:before {\n        transform: translateX(20px);\n    }\n    \n    /* Color editor */\n    .prop-color {\n        display: flex;\n        gap: 8px;\n        align-items: center;\n    }\n    \n    .color-swatch {\n        width: 32px;\n        height: 32px;\n        border: none;\n        border-radius: 4px;\n        cursor: pointer;\n    }\n    \n    .color-hex {\n        width: 80px;\n        padding: 6px 8px;\n        border: 1px solid #333;\n        border-radius: 4px;\n        background: #1a1a2e;\n        color: #e0e0e0;\n        font-family: monospace;\n        font-size: 12px;\n    }\n    \n    /* Vec3 editor */\n    .vec3-editor {\n        display: flex;\n        align-items: center;\n        padding: 6px 0;\n    }\n    \n    .vec3-inputs {\n        display: flex;\n        gap: 4px;\n        flex: 1;\n    }\n    \n    .vec3-axis {\n        display: flex;\n        align-items: center;\n        flex: 1;\n    }\n    \n    .vec3-axis input {\n        width: 100%;\n        padding: 6px 8px;\n        border: 1px solid #333;\n        border-radius: 0 4px 4px 0;\n        background: #1a1a2e;\n        color: #e0e0e0;\n        font-size: 12px;\n    }\n    \n    .axis-label {\n        padding: 6px 8px;\n        font-size: 11px;\n        font-weight: 600;\n        border-radius: 4px 0 0 4px;\n    }\n    \n    .axis-label.x { background: #e94560; color: white; }\n    .axis-label.y { background: #4ade80; color: #1a1a2e; }\n    .axis-label.z { background: #60a5fa; color: white; }\n    \n    /* Select dropdown */\n    .prop-select {\n        width: 100%;\n        padding: 6px 10px;\n        border: 1px solid #333;\n        border-radius: 4px;\n        background: #1a1a2e;\n        color: #e0e0e0;\n        font-size: 12px;\n        cursor: pointer;\n    }\n    ".to_string()
}
