#[allow(unused_imports)]
use super::*;

use super::profiler_timeline_model::{LiveProfilerSnapshot, ProfilerScopeKind, ProfilerScopeRow};
use super::traits::Renderable;
#[derive(Debug, Clone)]
#[repr(C)]
pub struct RealtimeProfilerPanel {
    pub snapshot: LiveProfilerSnapshot,
    pub refresh_handler: String,
}

impl RealtimeProfilerPanel {
#[inline]
pub fn from_mock() -> RealtimeProfilerPanel {
        RealtimeProfilerPanel { snapshot: LiveProfilerSnapshot::mock_runtime_demo(), refresh_handler: "profiler_refresh()".to_string() }
}
#[inline]
pub fn snapshot(mut self, snap: LiveProfilerSnapshot) -> RealtimeProfilerPanel {
        self.snapshot = snap;
        self
}
}

impl Renderable for RealtimeProfilerPanel {
#[inline]
fn render(self) -> String {
        let styles = realtime_profiler_styles();
        let frame_index = self.snapshot.frame_index;
        let frame_time_ms = self.snapshot.frame_time_ms;
        let budget_ms = self.snapshot.budget_ms;
        let scopes = self.snapshot.scopes;
        let budget_html = render_budget_bar(frame_time_ms, budget_ms);
        let scopes_html = render_scope_list(scopes);
        format!("<style>{}</style>\n<div class='rt-profiler-panel'>\n  <header class='rt-profiler-header'>\n    <h3>Realtime Profiler</h3>\n    <button type='button' onclick='{}'>Refresh</button>\n    <span class='rt-profiler-frame-tag'>Frame #{}</span>\n  </header>\n  {}\n  <section class='rt-profiler-scopes'>\n    <h4>Scopes</h4>\n    {}\n  </section>\n</div>", styles, self.refresh_handler, frame_index, budget_html, scopes_html)
}
}

#[inline]
pub fn render_budget_bar(frame_time_ms: f32, budget_ms: f32) -> String {
    let util = {
        if budget_ms <= 0.0_f32 {
            0.0_f32
        } else {
            frame_time_ms / budget_ms * 100.0_f32
        }
    };
    let clamped = {
        if util > 100.0_f32 {
            100.0_f32
        } else {
            util
        }
    };
    let vclass = {
        if util > 100.0_f32 {
            "wj-progress-danger".to_string()
        } else {
            if util > 85.0_f32 {
                "wj-progress-warning".to_string()
            } else {
                "wj-progress-success".to_string()
            }
        }
    };
    let color = {
        if util > 100.0_f32 {
            "#e74c3c".to_string()
        } else {
            if util > 85.0_f32 {
                "#f39c12".to_string()
            } else {
                "#2ecc71".to_string()
            }
        }
    };
    format!("<div class='rt-profiler-budget'>\n  <div class='rt-profiler-budget-label'>\n    <span>Frame budget</span>\n    <span>{:.2} / {:.2} ms ({:.0}%)</span>\n  </div>\n  <div class='wj-progress-container' style='width:100%;background:#1a1a2e;border-radius:4px;overflow:hidden;'>\n    <div class='wj-progress-bar {}' style='width:{:.0}%;height:20px;background-color:{};'></div>\n  </div>\n</div>", frame_time_ms, budget_ms, util, vclass, clamped, color)
}

#[inline]
pub fn render_scope_row(row: ProfilerScopeRow) -> String {
    let kind_label = match row.kind {
        ProfilerScopeKind::Cpu => "CPU".to_string(),
        ProfilerScopeKind::Gpu => "GPU".to_string(),
    };
    let bar_w = {
        if row.percentage > 100.0_f32 {
            100.0_f32
        } else {
            row.percentage
        }
    };
    format!("<div class='rt-profiler-scope'>\n  <div class='rt-profiler-scope-head'>\n    <span class='rt-profiler-scope-name'>{}</span>\n    <span class='rt-profiler-scope-kind'>{}</span>\n    <span class='rt-profiler-scope-ms'>{:.2} ms</span>\n    <span class='rt-profiler-scope-pct'>{:.1}%</span>\n  </div>\n  <div class='rt-profiler-scope-bar' style='width:{:.0}%;'></div>\n</div>", row.name, kind_label, row.duration_ms, row.percentage, bar_w)
}

#[inline]
pub fn render_scope_list(scopes: Vec<ProfilerScopeRow>) -> String {
    if scopes.is_empty() {
        return "<div class='rt-profiler-empty'>No scope metrics (wire engine snapshot)</div>".to_string();
    }
    let mut html = String::new();
    for row in scopes {
        html = html + &render_scope_row(row);
    }
    html
}

#[inline]
pub fn realtime_profiler_styles() -> String {
    "\n.rt-profiler-panel { background:#0f0f1a; color:#e5e7eb; padding:12px; font-family:monospace; font-size:12px; }\n.rt-profiler-header { display:flex; align-items:center; gap:12px; margin-bottom:12px; border-bottom:1px solid #333; padding-bottom:8px; }\n.rt-profiler-header h3 { margin:0; flex:1; color:#60a5fa; }\n.rt-profiler-header button { background:#0f3460; color:#e5e7eb; border:none; padding:6px 10px; border-radius:4px; cursor:pointer; }\n.rt-profiler-frame-tag { color:#888; }\n.rt-profiler-budget { margin-bottom:16px; }\n.rt-profiler-budget-label { display:flex; justify-content:space-between; margin-bottom:6px; color:#aaa; }\n.rt-profiler-scopes h4 { margin:0 0 8px 0; color:#888; font-size:11px; text-transform:uppercase; }\n.rt-profiler-scope { background:#16213e; border-radius:4px; padding:8px; margin-bottom:6px; position:relative; overflow:hidden; }\n.rt-profiler-scope-head { display:flex; gap:8px; position:relative; z-index:1; }\n.rt-profiler-scope-name { flex:1; color:#e0e0e0; }\n.rt-profiler-scope-kind { color:#888; font-size:10px; }\n.rt-profiler-scope-ms { color:#60a5fa; }\n.rt-profiler-scope-pct { color:#4ade80; min-width:48px; text-align:right; }\n.rt-profiler-scope-bar { position:absolute; left:0; top:0; bottom:0; background:rgba(96,165,250,0.15); z-index:0; }\n.rt-profiler-empty { color:#666; padding:16px; text-align:center; }\n".to_string()
}

