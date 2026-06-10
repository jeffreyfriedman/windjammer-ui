#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
use crate::frame_trace_ffi;
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct FrameDebuggerViewState {
    pub current_frame_index: u64,
    pub total_frames: usize,
    pub frame_time_ms: f32,
    pub can_step_back: bool,
    pub can_step_forward: bool,
}
impl FrameDebuggerViewState {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut __bytes = Vec::with_capacity(28);
        __bytes.extend_from_slice(&self.current_frame_index.to_ne_bytes());
        __bytes.extend_from_slice(&self.total_frames.to_ne_bytes());
        __bytes.extend_from_slice(&self.frame_time_ms.to_ne_bytes());
        __bytes.extend_from_slice(&(if self.can_step_back { 1u32 } else { 0u32 }).to_ne_bytes());
        __bytes.extend_from_slice(&(if self.can_step_forward { 1u32 } else { 0u32 }).to_ne_bytes());
        __bytes
    }
}

impl FrameDebuggerViewState {
    #[inline]
    pub fn empty() -> FrameDebuggerViewState {
        FrameDebuggerViewState {
            current_frame_index: 0_u64,
            total_frames: 0_usize,
            frame_time_ms: 0.0_f32,
            can_step_back: false,
            can_step_forward: false,
        }
    }
    #[inline]
    pub fn from_trace(
        current_index: u64,
        total: usize,
        frame_time_ms: f32,
        can_back: bool,
        can_forward: bool,
    ) -> FrameDebuggerViewState {
        FrameDebuggerViewState {
            current_frame_index: current_index,
            total_frames: total,
            frame_time_ms,
            can_step_back: can_back,
            can_step_forward: can_forward,
        }
    }
    #[inline]
    pub fn mock_captured() -> FrameDebuggerViewState {
        FrameDebuggerViewState::from_trace(12_u64, 48_usize, 15.8_f32, true, true)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct FrameDebuggerPanel {
    pub state: FrameDebuggerViewState,
    pub step_back_handler: String,
    pub step_forward_handler: String,
    pub capture_handler: String,
}

impl FrameDebuggerPanel {
    #[inline]
    pub fn inactive() -> FrameDebuggerPanel {
        FrameDebuggerPanel {
            state: FrameDebuggerViewState::empty(),
            step_back_handler: String::from("frame_debug_step_back()"),
            step_forward_handler: String::from("frame_debug_step_forward()"),
            capture_handler: String::from("frame_debug_capture()"),
        }
    }
    #[inline]
    pub fn from_mock() -> FrameDebuggerPanel {
        FrameDebuggerPanel {
            state: FrameDebuggerViewState::mock_captured(),
            step_back_handler: String::from("frame_debug_step_back()"),
            step_forward_handler: String::from("frame_debug_step_forward()"),
            capture_handler: String::from("frame_debug_capture()"),
        }
    }
    #[inline]
    pub fn from_live() -> FrameDebuggerPanel {
        if !frame_trace_ffi::has_live_data() {
            return FrameDebuggerPanel::inactive();
        }
        let state = FrameDebuggerViewState::from_trace(
            frame_trace_ffi::live_frame_index(),
            frame_trace_ffi::live_frame_count() as usize,
            frame_trace_ffi::live_frame_time_ms(),
            frame_trace_ffi::live_can_step_back(),
            frame_trace_ffi::live_can_step_forward(),
        );
        FrameDebuggerPanel {
            state,
            step_back_handler: String::from("frame_debug_step_back()"),
            step_forward_handler: String::from("frame_debug_step_forward()"),
            capture_handler: String::from("frame_debug_capture()"),
        }
    }
    #[inline]
    pub fn state(mut self, view: FrameDebuggerViewState) -> FrameDebuggerPanel {
        self.state = view;
        self
    }
}

impl Renderable for FrameDebuggerPanel {
    #[inline]
    fn render(self) -> String {
        let current_frame_index = self.state.current_frame_index;
        let total_frames = self.state.total_frames;
        let frame_time_ms = self.state.frame_time_ms;
        let can_step_back = self.state.can_step_back;
        let can_step_forward = self.state.can_step_forward;
        let capture_handler = self.capture_handler.clone();
        let step_back_handler = self.step_back_handler.clone();
        let step_forward_handler = self.step_forward_handler;
        let styles = frame_debugger_panel_styles();
        let position_label = frame_position_label(current_frame_index, total_frames, frame_time_ms);
        let back_class = step_button_class(can_step_back);
        let forward_class = step_button_class(can_step_forward);
        format!("<style>{}</style>\n<div class='frame-debug-panel'>\n  <header>\n    <h3>Frame Debugger</h3>\n    <button type='button' onclick='{}'>Capture</button>\n  </header>\n  <div class='frame-debug-transport'>\n    <button type='button' class='{}' onclick='{}' title='Previous frame'>&larr; Prev</button>\n    <span class='frame-debug-position'>{}</span>\n    <button type='button' class='{}' onclick='{}' title='Next frame'>Next &rarr;</button>\n  </div>\n  <p class='frame-debug-hint'>Step through captured frame traces from the engine ring buffer.</p>\n</div>", styles, capture_handler, back_class, step_back_handler, position_label, forward_class, step_forward_handler)
    }
}

#[inline]
pub fn step_button_class(enabled: bool) -> String {
    if enabled {
        String::new()
    } else {
        String::from("disabled")
    }
}

#[inline]
pub fn frame_position_label(current_index: u64, total_frames: usize, frame_time_ms: f32) -> String {
    if total_frames == 0_usize {
        String::from("No frames captured")
    } else {
        format!(
            "Frame {} / {} ({:.2} ms)",
            current_index, total_frames, frame_time_ms
        )
    }
}

#[inline]
pub fn frame_debugger_panel_styles() -> String {
    String::from("\n.frame-debug-panel { background:#09090f; border-top:2px solid #60a5fa; color:#ecfdf5; padding:10px; font-family:monospace; }\n.frame-debug-panel header { display:flex; align-items:center; gap:10px; margin-bottom:8px; }\n.frame-debug-panel header h3 { margin:0; flex:1; color:#60a5fa; font-size:14px; }\n.frame-debug-panel header button { padding:6px 10px; border-radius:4px; border:none; background:#0f3460; color:#e5e7eb; cursor:pointer; }\n.frame-debug-transport { display:flex; align-items:center; gap:12px; margin-bottom:8px; }\n.frame-debug-transport button { padding:8px 14px; border-radius:4px; border:none; background:#16213e; color:#e5e7eb; cursor:pointer; }\n.frame-debug-transport button.disabled { opacity:0.4; cursor:not-allowed; }\n.frame-debug-position { flex:1; text-align:center; color:#e0e0e0; font-weight:600; }\n.frame-debug-hint { margin:0; color:#666; font-size:11px; }\n")
}
