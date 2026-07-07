#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
//! Read-side bridge for live engine frame trace snapshots (profiler / frame debugger UI).
//!
//! Standalone tests inject snapshots via `test_set_live_snapshot`. The game editor /
//! runtime host registers a provider that reads the host `FrameTraceReader` ring.

use std::sync::Mutex;

#[derive(Clone, Debug, PartialEq)]
pub struct LiveScopeRow {
    pub name: String,
    pub duration_ms: f32,
    pub is_gpu: bool,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LiveFrameSnapshot {
    pub frame_index: u64,
    pub frame_time_ms: f32,
    pub scopes: Vec<LiveScopeRow>,
    pub frame_count: u32,
    pub can_step_back: bool,
    pub can_step_forward: bool,
}

type SnapshotProvider = fn() -> Option<LiveFrameSnapshot>;

static TEST_SNAPSHOT: Mutex<Option<LiveFrameSnapshot>> = Mutex::new(None);
static LIVE_PROVIDER: Mutex<Option<SnapshotProvider>> = Mutex::new(None);

/// Register a host-backed reader (called from editor / runtime-host init).
pub fn register_live_provider(provider: SnapshotProvider) {
    *LIVE_PROVIDER.lock().expect("live provider lock") = Some(provider);
}

pub fn clear_live_provider() {
    *LIVE_PROVIDER.lock().expect("live provider lock") = None;
}

/// Inject a snapshot for unit tests (no runtime-host required).
pub fn test_set_live_snapshot(snap: LiveFrameSnapshot) {
    *TEST_SNAPSHOT.lock().expect("test snapshot lock") = Some(snap);
}

pub fn test_clear_live_snapshot() {
    *TEST_SNAPSHOT.lock().expect("test snapshot lock") = None;
}

/// Read the current live snapshot from test injection or registered host provider.
pub fn read_live_snapshot() -> Option<LiveFrameSnapshot> {
    if let Some(test) = TEST_SNAPSHOT.lock().expect("test snapshot lock").clone() {
        return Some(test);
    }
    if let Some(provider) = *LIVE_PROVIDER.lock().expect("live provider lock") {
        return provider();
    }
    None
}

pub fn has_live_data() -> bool {
    read_live_snapshot().is_some()
}

// Windjammer-callable accessors (profiler_timeline_model.wj)

pub fn live_frame_index() -> u64 {
    read_live_snapshot().map(|s| s.frame_index).unwrap_or(0)
}

pub fn live_frame_time_ms() -> f32 {
    read_live_snapshot().map(|s| s.frame_time_ms).unwrap_or(0.0)
}

pub fn live_scope_count() -> u32 {
    read_live_snapshot()
        .map(|s| s.scopes.len() as u32)
        .unwrap_or(0)
}

pub fn live_scope_name(index: u32) -> String {
    read_live_snapshot()
        .and_then(|s| s.scopes.get(index as usize).map(|r| r.name.clone()))
        .unwrap_or_default()
}

pub fn live_scope_duration_ms(index: u32) -> f32 {
    read_live_snapshot()
        .and_then(|s| s.scopes.get(index as usize).map(|r| r.duration_ms))
        .unwrap_or(0.0)
}

pub fn live_scope_is_gpu(index: u32) -> bool {
    read_live_snapshot()
        .and_then(|s| s.scopes.get(index as usize).map(|r| r.is_gpu))
        .unwrap_or(false)
}

pub fn live_frame_count() -> u32 {
    read_live_snapshot().map(|s| s.frame_count).unwrap_or(0)
}

pub fn live_can_step_back() -> bool {
    read_live_snapshot()
        .map(|s| s.can_step_back)
        .unwrap_or(false)
}

pub fn live_can_step_forward() -> bool {
    read_live_snapshot()
        .map(|s| s.can_step_forward)
        .unwrap_or(false)
}
