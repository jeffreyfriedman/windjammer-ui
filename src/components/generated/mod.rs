#![allow(clippy::all)]
#![allow(noop_method_call)]
// Auto-generated mod.rs by Windjammer CLI
// This file declares all generated Windjammer modules

pub mod accordion;
pub mod advancedcodeeditor;
pub mod alert;
// COMPILER BUG WORKAROUND: app is a top-level module in src/lib.rs, not a generated component
// pub mod app;
#[cfg(feature = "desktop")]
pub mod app_docking;
pub mod app_reactive;
#[cfg(feature = "desktop")]
pub mod app_reactive_eframe;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod center;
pub mod chatinput;
pub mod chatmessage;
pub mod checkbox;
pub mod chip;
pub mod codeblock;
pub mod codeeditor;
pub mod collapsible;
pub mod colorpicker;
pub mod column;
pub mod component;
pub mod component_runtime;
pub mod container;
pub mod contextmenu;
pub mod curve_editor;
#[cfg(feature = "desktop")]
pub mod desktop_app_context;
#[cfg(feature = "desktop")]
pub mod desktop_renderer;
#[cfg(feature = "desktop")]
pub mod desktop_renderer_v2;
pub mod dialog;
pub mod divider;
pub mod drawer;
pub mod dropdown;
pub mod event_handler;
// COMPILER BUG WORKAROUND: examples_wasm is a top-level module in src/lib.rs, not a generated component
// pub mod examples_wasm;
pub mod filetree;
pub mod flex;
pub mod form;
pub mod grid;
pub mod hamburgermenu;
pub mod html_elements;
pub mod input;
pub mod label;
pub mod list;
pub mod loading;
pub mod menu;
pub mod messagelist;
pub mod modal;
pub mod navbar;
pub mod node_graph;
pub mod pagination;
pub mod panel;
pub mod platform;
pub mod popover;
pub mod progress;
pub mod propertyeditor;
pub mod radio;
pub mod rating;
pub mod reactivity;
pub mod reactivity_optimized;
pub mod reactivity_tests;
pub mod renderer;
pub mod routing;
pub mod row;
pub mod runtime;
pub mod scroll;
pub mod scrollarea;
pub mod section;
pub mod select;
pub mod sidebar;
pub mod signal;
pub mod simple_renderer;
pub mod simple_vnode;
pub mod skeleton;
pub mod slider;
pub mod spacer;
pub mod spinner;
pub mod splitpanel;
pub mod ssr;
pub mod stack;
pub mod stepper;
pub mod style;
pub mod switch;
pub mod table;
pub mod tabpanel;
pub mod tabs;
pub mod text;
pub mod textarea;
pub mod theme;
pub mod timeline;
pub mod to_vnode;
pub mod toast;
pub mod toolbar;
pub mod tooltip;
pub mod traits;
pub mod treeview;
pub mod typingindicator;
pub mod undo_redo;
pub mod vdom;
pub mod vnode;
pub mod vnode_ffi;
pub mod wasm_events;

// Note: Glob re-exports skipped due to symbol conflicts
// Use explicit imports: use your_crate::module_name::SymbolName;
