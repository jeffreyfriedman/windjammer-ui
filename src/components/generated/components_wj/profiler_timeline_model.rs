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
        self.scopes.push(row.clone());
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
            String::from("wj-progress-danger")
        } else {
            if util > 85.0_f32 {
                String::from("wj-progress-warning")
            } else {
                String::from("wj-progress-success")
            }
        }
    }
    #[inline]
    pub fn budget_band_color(&self) -> String {
        let util = self.budget_utilization_pct();
        if util > 100.0_f32 {
            String::from("#e74c3c")
        } else {
            if util > 85.0_f32 {
                String::from("#f39c12")
            } else {
                String::from("#2ecc71")
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
            snap = snap.scope(row.clone());
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
        let mut snap = LiveProfilerSnapshot::new(frame_index, frame_time_ms.clone(), budget_ms);
        let count = frame_trace_ffi::live_scope_count();
        let mut i = 0;
        while i < count {
            let name = frame_trace_ffi::live_scope_name(i);
            let duration_ms = frame_trace_ffi::live_scope_duration_ms(i);
            let is_gpu = frame_trace_ffi::live_scope_is_gpu(i);
            let kind = {
                if is_gpu {
                    ProfilerScopeKind::Gpu.clone()
                } else {
                    ProfilerScopeKind::Cpu.clone()
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
                name,
                duration_ms: duration_ms.clone(),
                percentage: pct,
                kind,
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
            name: String::from("Update"),
            duration_ms: 2.1_f32,
            percentage: 14.8_f32,
            kind: ProfilerScopeKind::Cpu.clone(),
        });
        snap = snap.scope(ProfilerScopeRow {
            name: String::from("Physics"),
            duration_ms: 1.4_f32,
            percentage: 9.9_f32,
            kind: ProfilerScopeKind::Cpu.clone(),
        });
        snap = snap.scope(ProfilerScopeRow {
            name: String::from("Render"),
            duration_ms: 5.8_f32,
            percentage: 40.8_f32,
            kind: ProfilerScopeKind::Cpu.clone(),
        });
        snap = snap.scope(ProfilerScopeRow {
            name: String::from("raymarch"),
            duration_ms: 3.2_f32,
            percentage: 22.5_f32,
            kind: ProfilerScopeKind::Gpu.clone(),
        });
        snap = snap.scope(ProfilerScopeRow {
            name: String::from("lighting"),
            duration_ms: 1.7_f32,
            percentage: 12.0_f32,
            kind: ProfilerScopeKind::Gpu.clone(),
        });
        snap
    }
}
