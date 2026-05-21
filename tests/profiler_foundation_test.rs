//! Realtime profiler + frame debugger foundation smoke tests (generated .wj components).

use windjammer_ui::components::generated::frame_debugger_panel::{
    FrameDebuggerPanel, FrameDebuggerViewState,
};
use windjammer_ui::components::generated::profiler_timeline_model::{
    LiveProfilerSnapshot, ProfilerScopeKind, ProfilerScopeRow,
};
use windjammer_ui::components::generated::realtime_profiler_panel::RealtimeProfilerPanel;
use windjammer_ui::components::generated::traits::Renderable;
use windjammer_ui::frame_trace_ffi::{test_clear_live_snapshot, test_set_live_snapshot, LiveFrameSnapshot, LiveScopeRow};

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

#[test]
fn test_live_profiler_snapshot_from_scope_rows() {
    let rows = vec![
        ProfilerScopeRow {
            name: "Update".to_string(),
            duration_ms: 2.0,
            percentage: 12.5,
            kind: ProfilerScopeKind::Cpu,
        },
        ProfilerScopeRow {
            name: "raymarch".to_string(),
            duration_ms: 4.0,
            percentage: 25.0,
            kind: ProfilerScopeKind::Gpu,
        },
    ];
    let snap = LiveProfilerSnapshot::from_scope_rows(7, 16.0, 16.67, rows);
    assert_eq!(snap.frame_index, 7);
    assert_eq!(snap.scopes.len(), 2);
    assert!(snap.budget_utilization_pct() > 95.0);
}

#[test]
fn test_realtime_profiler_panel_from_live_engine_snapshot() {
    test_set_live_snapshot(LiveFrameSnapshot {
        frame_index: 99,
        frame_time_ms: 14.5,
        scopes: vec![
            LiveScopeRow {
                name: "ecs_update_systems".to_string(),
                duration_ms: 1.2,
                is_gpu: false,
            },
            LiveScopeRow {
                name: "voxel_do_render_frame".to_string(),
                duration_ms: 8.4,
                is_gpu: false,
            },
        ],
        frame_count: 48,
        can_step_back: true,
        can_step_forward: false,
    });

    let html = RealtimeProfilerPanel::from_live(16.67).render();
    assert!(html.contains("Frame #99"));
    assert!(html.contains("ecs_update_systems"));
    assert!(html.contains("voxel_do_render_frame"));
    assert!(!html.contains("raymarch"), "mock demo must not appear when live data set");

    test_clear_live_snapshot();
}

#[test]
fn test_frame_debugger_panel_from_live_trace() {
    test_set_live_snapshot(LiveFrameSnapshot {
        frame_index: 12,
        frame_time_ms: 15.8,
        scopes: vec![],
        frame_count: 48,
        can_step_back: true,
        can_step_forward: true,
    });

    let html = FrameDebuggerPanel::from_live().render();
    assert!(html.contains("Frame 12 / 48"));
    assert!(html.contains("15.80 ms"));

    test_clear_live_snapshot();
}
