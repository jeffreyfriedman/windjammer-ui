#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
// Auto-generated mod.rs by Windjammer CLI
// This file declares all generated Windjammer modules

pub mod accordion;
pub mod advancedcodeeditor;
pub mod alert;
pub mod avatar;
pub mod badge;
pub mod icon;
pub mod statuschip;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod center;
pub mod chart;
pub mod chatinput;
pub mod chatmessage;
pub mod checkbox;
pub mod chip;
pub mod codeblock;
pub mod codeeditor;
pub mod collapsible;
pub mod colorpicker;
pub mod column;
pub mod container;
pub mod contextmenu;
pub mod curve_editor;
pub mod datatable;
pub mod datepicker;
pub mod dialog;
pub mod divider;
pub mod drawer;
pub mod dropdown;
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
pub mod moneydisplay;
pub mod kpitile;
pub mod navbar;
pub mod node_graph;
pub mod pagination;
pub mod panel;
pub mod popover;
pub mod progress;
pub mod propertyeditor;
pub mod radio;
pub mod rating;
pub mod row;
pub mod scroll;
pub mod scrollarea;
pub mod section;
pub mod select;
pub mod sidebar;
pub mod skeleton;
pub mod slider;
pub mod spacer;
pub mod spinner;
pub mod splitpanel;
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
pub mod toast;
pub mod toolbar;
pub mod tooltip;
pub mod traits;
pub mod treeview;
pub mod typingindicator;
pub mod vnode;

// Re-export all public items
pub use accordion::*;
pub use advancedcodeeditor::*;
pub use alert::*;
pub use avatar::*;
pub use badge::*;
pub use icon::*;
pub use statuschip::*;
pub use breadcrumb::*;
pub use button::*;
pub use card::*;
pub use center::*;
pub use chart::*;
pub use chatinput::*;
pub use chatmessage::*;
pub use checkbox::*;
pub use chip::*;
pub use codeblock::*;
pub use codeeditor::*;
pub use collapsible::*;
pub use colorpicker::*;
pub use column::*;
pub use container::*;
pub use contextmenu::*;
pub use curve_editor::*;
pub use datatable::*;
pub use datepicker::*;
pub use dialog::*;
pub use divider::*;
pub use drawer::*;
pub use dropdown::*;
pub use filetree::*;
pub use flex::*;
pub use form::*;
pub use grid::*;
pub use hamburgermenu::*;
pub use html_elements::*;
pub use input::*;
pub use label::*;
pub use list::*;
pub use loading::*;
pub use menu::*;
pub use messagelist::*;
pub use modal::*;
pub use moneydisplay::*;
pub use kpitile::*;
pub use navbar::*;
pub use node_graph::*;
pub use pagination::*;
pub use panel::*;
pub use popover::*;
pub use progress::*;
pub use propertyeditor::*;
pub use radio::*;
pub use rating::*;
pub use row::*;
pub use scroll::*;
pub use scrollarea::*;
pub use section::*;
pub use select::*;
pub use sidebar::*;
pub use skeleton::*;
pub use slider::*;
pub use spacer::*;
pub use spinner::*;
pub use splitpanel::*;
pub use stack::*;
pub use stepper::*;
pub use style::*;
pub use switch::*;
pub use table::*;
pub use tabpanel::*;
pub use tabs::*;
pub use text::*;
pub use textarea::*;
pub use theme::*;
pub use timeline::*;
pub use toast::*;
pub use toolbar::*;
pub use tooltip::*;
pub use traits::*;
pub use treeview::*;
pub use typingindicator::*;
// vnode::* overlaps vdom::VNode — use `vnode::` paths or vdom re-exports below.

// bt_visual_* omitted until Renderable trait impl codegen matches object-safe &self (E0053)
pub mod profiler_timeline_model;
pub use profiler_timeline_model::*;

pub mod realtime_profiler_panel;
pub use realtime_profiler_panel::*;

pub mod frame_debugger_panel;
pub use frame_debugger_panel::*;

// bt_visual_* WIP — sources in src/components/bt_visual_*.wj

pub mod ecs_inspector_model;
pub use ecs_inspector_model::*;

pub mod ecs_inspector_panel;
pub use ecs_inspector_panel::*;

pub mod bt_visual_canvas;
pub use bt_visual_canvas::*;
pub mod bt_visual_overlay;
pub use bt_visual_overlay::*;
pub mod bt_visual_palette;
pub use bt_visual_palette::*;
pub mod bt_visual_properties;
pub use bt_visual_properties::*;

#[cfg(test)]
pub mod graph_trace_test;

pub mod app;
pub use app::*;
#[cfg(feature = "desktop")]
pub mod app_docking;
#[cfg(feature = "desktop")]
pub use app_docking::*;
pub mod app_reactive;
pub use app_reactive::*;
#[cfg(feature = "desktop")]
pub mod app_reactive_eframe;
#[cfg(feature = "desktop")]
pub use app_reactive_eframe::*;
pub mod component;
pub use component::*;
pub mod component_runtime;
pub use component_runtime::*;
#[cfg(feature = "desktop")]
pub mod desktop_app_context;
#[cfg(feature = "desktop")]
pub use desktop_app_context::*;
#[cfg(feature = "desktop")]
pub mod desktop_renderer;
#[cfg(feature = "desktop")]
pub use desktop_renderer::*;
#[cfg(feature = "desktop")]
pub mod desktop_renderer_v2;
#[cfg(feature = "desktop")]
pub use desktop_renderer_v2::*;
pub mod event_handler;
pub use event_handler::*;
pub mod frame_trace_ffi;
pub use frame_trace_ffi::*;
pub mod reactivity;
pub use reactivity::*;
pub mod reactivity_optimized;
// Do not glob-reexport: overlaps Signal/Effect/create_* with reactivity (ambiguous glob warnings).
pub mod renderer;
pub use renderer::*;
pub mod routing;
pub use routing::*;
pub mod runtime;
pub use runtime::*;
pub mod signal;
// signal::* overlaps reactivity::* — keep module available without glob re-export.
pub mod signal_ffi;
pub use signal_ffi::*;
pub mod simple_renderer;
pub use simple_renderer::*;
pub mod simple_vnode;
pub use simple_vnode::*;
pub mod ssr;
pub use ssr::*;
pub mod to_vnode;
pub use to_vnode::*;
pub mod undo_redo;
pub use undo_redo::*;
pub mod vdom;
// Prefer vdom::VNode in public API — do not also glob-export vnode::* (ambiguous VNode).
pub use vdom::{VElement, VNode, VText};
pub mod vnode_ffi;
pub use vnode_ffi::*;
pub mod wasm_events;
pub use wasm_events::*;

// examples_wasm lives at crate root (`src/examples_wasm.rs`) — do not also
// compile the generated copy or wasm_bindgen symbols collide.
// pub mod examples_wasm;
// #[cfg(target_arch = "wasm32")]
// pub use examples_wasm::*;

