//! Scene -> display list. Walks the tree with absolute offsets, applies scroll, clip and opacity,
//! and culls subtrees outside the viewport/clip (virtualization at the paint level).
use wj_scene::{NodeId, Rect, Scene};

use crate::display_list::{DisplayList, DrawCmd};
use crate::text::TextLayouter;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BuildStats {
    pub visited: usize,
    pub culled: usize,
}

fn intersects(a: Rect, b: Rect) -> bool {
    a.x < b.x + b.w && a.x + a.w > b.x && a.y < b.y + b.h && a.y + a.h > b.y
}

pub fn build_display_list(scene: Scene, viewport: (f32, f32), text: &mut dyn TextLayouter) -> (DisplayList, BuildStats) {
    let mut list = DisplayList::new();
    let mut stats = BuildStats::default();
    let view = Rect::new(0.0, 0.0, viewport.0, viewport.1);
    walk(scene, scene.root(), 0.0, 0.0, view, text, &mut list, &mut stats);
    (list, stats)
}

#[allow(clippy::too_many_arguments)]
fn walk(scene: Scene, node: NodeId, ox: f32, oy: f32, clip: Rect, text: &mut dyn TextLayouter, out: &mut DisplayList, stats: &mut BuildStats) {
    stats.visited += 1;
    let r = scene.rect(node);
    let abs = Rect::new(r.x + ox, r.y + oy, r.w, r.h);
    let paint = scene.paint(node);
    let style_scroll = scene.style(node).scroll;
    let clips = paint.clip || style_scroll;
    let has_kids = scene.child_count(node) > 0;
    // Cull: nothing of this node or (when it clips) its subtree can be visible.
    if !intersects(abs, clip) && (clips || !has_kids) {
        stats.culled += 1;
        return;
    }
    let layer = paint.opacity < 1.0;
    if layer {
        out.push(DrawCmd::PushLayer { opacity: paint.opacity.max(0.0) });
    }
    if let Some(bg) = paint.background {
        if bg.a > 0 {
            out.push(DrawCmd::FillRRect { rect: abs, radius: paint.corner_radius, color: bg });
        }
    }
    if paint.border_width > 0.0 && paint.border_color.a > 0 {
        out.push(DrawCmd::StrokeRRect { rect: abs, radius: paint.corner_radius, width: paint.border_width, color: paint.border_color });
    }
    if let Some(tp) = scene.text(node) {
        if !tp.content.is_empty() {
            let layout = text.layout(&tp.content, tp.font_size, Some(abs.w));
            for run in &layout.runs {
                let mut run = run.clone();
                run.x += abs.x;
                run.y += abs.y;
                run.color = tp.color;
                out.push(DrawCmd::Text { run });
            }
        }
    }
    let child_clip = if clips {
        out.push(DrawCmd::PushClip { rect: abs, radius: paint.corner_radius });
        Rect::new(abs.x.max(clip.x), abs.y.max(clip.y), (abs.x + abs.w).min(clip.x + clip.w) - abs.x.max(clip.x), (abs.y + abs.h).min(clip.y + clip.h) - abs.y.max(clip.y))
    } else {
        clip
    };
    let (sx, sy) = scene.scroll(node);
    for c in scene.children(node) {
        walk(scene, c, abs.x - sx, abs.y - sy, child_clip, text, out, stats);
    }
    if clips {
        out.push(DrawCmd::PopClip);
    }
    if layer {
        out.push(DrawCmd::PopLayer);
    }
}
