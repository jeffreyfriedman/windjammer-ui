#[allow(unused_imports)]
use super::*;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ProfilerScopeKind {
    Cpu,
    Gpu,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ProfilerScopeRow {
    pub name: String,
    pub duration_ms: f32,
    pub percentage: f32,
    pub kind: ProfilerScopeKind,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct LiveProfilerSnapshot {
    pub frame_index: u64,
    pub frame_time_ms: f32,
    pub budget_ms: f32,
    pub scopes: Vec<ProfilerScopeRow>,
}

impl LiveProfilerSnapshot {
#[inline]
pub fn new(frame_index: u64, frame_time_ms: f32, budget_ms: f32) -> LiveProfilerSnapshot {
        LiveProfilerSnapshot { frame_index, frame_time_ms, budget_ms, scopes: Vec::new() }
}
#[inline]
pub fn scope(mut self, row: ProfilerScopeRow) -> LiveProfilerSnapshot {
        self.scopes.push(row);
        self
}
#[inline]
pub fn budget_utilization_pct(&self) -> f32 {
        if self.budget_ms <= 0.0_f32 {
            return 0.0_f32;
        }
        self.frame_time_ms / self.budget_ms * 100.0_f32
}
#[inline]
pub fn budget_band_class(&self) -> String {
        let util = self.budget_utilization_pct();
        if util > 100.0_f32 {
            "wj-progress-danger".to_string()
        } else {
            if util > 85.0_f32 {
                "wj-progress-warning".to_string()
            } else {
                "wj-progress-success".to_string()
            }
        }
}
#[inline]
pub fn budget_band_color(&self) -> String {
        let util = self.budget_utilization_pct();
        if util > 100.0_f32 {
            "#e74c3c".to_string()
        } else {
            if util > 85.0_f32 {
                "#f39c12".to_string()
            } else {
                "#2ecc71".to_string()
            }
        }
}
#[inline]
pub fn mock_runtime_demo() -> LiveProfilerSnapshot {
        let budget = 16.67_f32;
        let mut snap = LiveProfilerSnapshot::new(42_u64, 14.2_f32, budget);
        snap = snap.scope(ProfilerScopeRow { name: "Update".to_string(), duration_ms: 2.1_f32, percentage: 14.8_f32, kind: ProfilerScopeKind::Cpu });
        snap = snap.scope(ProfilerScopeRow { name: "Physics".to_string(), duration_ms: 1.4_f32, percentage: 9.9_f32, kind: ProfilerScopeKind::Cpu });
        snap = snap.scope(ProfilerScopeRow { name: "Render".to_string(), duration_ms: 5.8_f32, percentage: 40.8_f32, kind: ProfilerScopeKind::Cpu });
        snap = snap.scope(ProfilerScopeRow { name: "raymarch".to_string(), duration_ms: 3.2_f32, percentage: 22.5_f32, kind: ProfilerScopeKind::Gpu });
        snap = snap.scope(ProfilerScopeRow { name: "lighting".to_string(), duration_ms: 1.7_f32, percentage: 12.0_f32, kind: ProfilerScopeKind::Gpu });
        snap
}
}

