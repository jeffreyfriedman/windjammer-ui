//! Realtime profiler + frame debugger foundation smoke tests (generated .wj components).

use windjammer_ui::components::generated::frame_debugger_panel::{
    FrameDebuggerPanel, FrameDebuggerViewState,
};
use windjammer_ui::components::generated::profiler_timeline_model::LiveProfilerSnapshot;
use windjammer_ui::components::generated::realtime_profiler_panel::RealtimeProfilerPanel;
use windjammer_ui::components::generated::traits::Renderable;

#[test]
fn test_live_profiler_snapshot_mock_budget() {
    let snap = LiveProfilerSnapshot::mock_runtime_demo();
    assert!(snap.budget_utilization_pct() > 0.0);
    assert!(snap.budget_utilization_pct() < 100.0);
    assert_eq!(snap.frame_index, 42);
}

#[test]
fn test_realtime_profiler_panel_renders_budget_and_scopes() {
    let html = RealtimeProfilerPanel::from_mock().render();
    assert!(html.contains("Realtime Profiler"));
    assert!(html.contains("Frame budget"));
    assert!(html.contains("raymarch"));
    assert!(html.contains("wj-progress"));
}

#[test]
fn test_frame_debugger_panel_step_controls() {
    let html = FrameDebuggerPanel::from_mock().render();
    assert!(html.contains("Frame Debugger"));
    assert!(html.contains("frame_debug_step_forward"));
    assert!(html.contains("Frame 12"));
}

#[test]
fn test_frame_debugger_empty_state() {
    let html = FrameDebuggerPanel::inactive().render();
    assert!(html.contains("No frames captured"));
    let state = FrameDebuggerViewState::empty();
    assert_eq!(state.total_frames, 0);
}
