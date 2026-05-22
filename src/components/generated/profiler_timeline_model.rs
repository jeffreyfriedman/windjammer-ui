#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use crate::frame_trace_ffi;
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
        LiveProfilerSnapshot {
            frame_index,
            frame_time_ms,
            budget_ms,
            scopes: Vec::new(),
        }
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
    pub fn from_scope_rows(
        frame_index: u64,
        frame_time_ms: f32,
        budget_ms: f32,
        rows: Vec<ProfilerScopeRow>,
    ) -> LiveProfilerSnapshot {
        let mut snap = LiveProfilerSnapshot::new(frame_index, frame_time_ms, budget_ms);
        for row in rows {
            snap = snap.scope(row);
        }
        snap
    }
    #[inline]
    pub fn from_engine_live(budget_ms: f32) -> LiveProfilerSnapshot {
        if !frame_trace_ffi::has_live_data() {
            return LiveProfilerSnapshot::new(0_u64, 0.0_f32, budget_ms);
        }
        let frame_index = frame_trace_ffi::live_frame_index();
        let frame_time_ms = frame_trace_ffi::live_frame_time_ms();
        let mut snap = LiveProfilerSnapshot::new(frame_index, frame_time_ms, budget_ms);
        let count = frame_trace_ffi::live_scope_count();
        let mut i = 0;
        while i < count {
            let name = frame_trace_ffi::live_scope_name(i);
            let duration_ms = frame_trace_ffi::live_scope_duration_ms(i);
            let is_gpu = frame_trace_ffi::live_scope_is_gpu(i);
            let kind = {
                if is_gpu {
                    ProfilerScopeKind::Gpu
                } else {
                    ProfilerScopeKind::Cpu
                }
            };
            let pct = {
                if frame_time_ms > 0.0_f32 {
                    duration_ms / frame_time_ms * 100.0_f32
                } else {
                    0.0_f32
                }
            };
            snap = snap.scope(ProfilerScopeRow {
                name: name.clone(),
                duration_ms: duration_ms.clone(),
                percentage: pct,
                kind: kind.clone(),
            });
            i += 1;
        }
        snap
    }
    #[inline]
    pub fn mock_runtime_demo() -> LiveProfilerSnapshot {
        let budget = 16.67_f32;
        let mut snap = LiveProfilerSnapshot::new(42_u64, 14.2_f32, budget);
        snap = snap.scope(ProfilerScopeRow {
            name: "Update".to_string(),
            duration_ms: 2.1_f32,
            percentage: 14.8_f32,
            kind: ProfilerScopeKind::Cpu,
        });
        snap = snap.scope(ProfilerScopeRow {
            name: "Physics".to_string(),
            duration_ms: 1.4_f32,
            percentage: 9.9_f32,
            kind: ProfilerScopeKind::Cpu,
        });
        snap = snap.scope(ProfilerScopeRow {
            name: "Render".to_string(),
            duration_ms: 5.8_f32,
            percentage: 40.8_f32,
            kind: ProfilerScopeKind::Cpu,
        });
        snap = snap.scope(ProfilerScopeRow {
            name: "raymarch".to_string(),
            duration_ms: 3.2_f32,
            percentage: 22.5_f32,
            kind: ProfilerScopeKind::Gpu,
        });
        snap = snap.scope(ProfilerScopeRow {
            name: "lighting".to_string(),
            duration_ms: 1.7_f32,
            percentage: 12.0_f32,
            kind: ProfilerScopeKind::Gpu,
        });
        snap
    }
}
