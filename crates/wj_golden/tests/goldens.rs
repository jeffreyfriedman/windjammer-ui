//! Golden-image conformance (VISION 6.4 item 4).
//!  * `cpu_painter_matches_stored_goldens`: the CPU painter against committed PNGs. Goldens change
//!    only through `WJ_UPDATE_GOLDENS=1 cargo test -p wj-golden --release cpu_painter`, then review the diff.
//!  * `gpu_painter_matches_cpu_painter`: Vello GPU against the CPU painter on all scenes.
//!  * `cpu_painter_agrees_with_an_unrelated_rasterizer`: tiny-skia (the rasterizer behind resvg) on vector scenes.
mod common;
use common::*;
use wj_golden::{corpus, GoldenScene};
use wj_paint::{DrawCmd, Image, Painter};
use wj_paint_cpu::CpuPainter;

fn render_cpu(s: &GoldenScene) -> Image {
    CpuPainter::new().render(&s.list, s.width, s.height, s.clear)
}

#[test]
fn corpus_has_at_least_200_balanced_scenes() {
    let c = corpus();
    assert!(c.len() >= 200, "{}", c.len());
    assert!(c.iter().all(|s| s.list.is_balanced()));
    assert!(c.iter().filter(|s| !s.vector_only).count() >= 40, "text scenes present");
    let names: std::collections::HashSet<_> = c.iter().map(|s| s.name.clone()).collect();
    assert_eq!(names.len(), c.len(), "unique names");
}

#[test]
fn cpu_painter_matches_stored_goldens() {
    let update = std::env::var("WJ_UPDATE_GOLDENS").is_ok();
    let mut bad = vec![];
    let mut written = 0;
    for s in corpus() {
        let img = render_cpu(&s);
        let path = golden_path(&s.name);
        if update {
            write_png(&path, &img);
            written += 1;
            continue;
        }
        let Some(gold) = read_png(&path) else {
            bad.push(format!("{}: no golden (run with WJ_UPDATE_GOLDENS=1 and review)", s.name));
            continue;
        };
        let (max, frac) = img.diff(&gold, 2);
        if max > 2 && frac > 0.0 {
            bad.push(format!("{}: max channel diff {max}, {:.2}% pixels over tolerance", s.name, frac * 100.0));
        }
    }
    if update {
        println!("wrote {written} goldens");
        return;
    }
    assert!(bad.is_empty(), "{} scenes differ:\n{}", bad.len(), bad.iter().take(10).cloned().collect::<Vec<_>>().join("\n"));
}

fn gpu() -> Option<wj_paint_vello::VelloPainter> {
    let p = wj_paint_vello::VelloPainter::headless();
    if p.is_none() {
        assert!(std::env::var("WJ_ALLOW_NO_GPU").is_ok(), "no GPU adapter; set WJ_ALLOW_NO_GPU=1 to skip explicitly");
        eprintln!("SKIPPED: no GPU adapter (WJ_ALLOW_NO_GPU set)");
    }
    p
}

/// Compare two renders. VISION 6.2 M1 asks for agreement within 1/255 everywhere except anti-aliased
/// edges. Measured here: pixels whose 3x3 neighbourhood is uniform in both images ("interior") and
/// the rest ("edge"). Returns (interior max diff, edge pixel count share, mean abs error over all
/// channels in 1/255 units, fraction of all pixels differing by more than 3/255).
fn compare(a: &Image, b: &Image) -> (u8, f64, f64, f64) {
    let (fa, fb) = (flat_mask(a), flat_mask(b));
    let (mut interior_max, mut edge_n, mut over3, mut sum) = (0u8, 0usize, 0usize, 0u64);
    for (i, (p, q)) in a.rgba.chunks(4).zip(b.rgba.chunks(4)).enumerate() {
        let d = p.iter().zip(q).map(|(x, y)| x.abs_diff(*y)).max().unwrap();
        sum += p.iter().zip(q).map(|(x, y)| x.abs_diff(*y) as u64).sum::<u64>();
        if d > 3 {
            over3 += 1;
        }
        if fa[i] && fb[i] {
            interior_max = interior_max.max(d);
        } else {
            edge_n += 1;
        }
    }
    let n = (a.width * a.height) as f64;
    (interior_max, edge_n as f64 / n, sum as f64 / (n * 4.0), over3 as f64 / n)
}

#[test]
fn gpu_painter_matches_cpu_painter() {
    let Some(mut gpu) = gpu() else { return };
    let (mut worst_interior, mut worst_mae, mut worst_over3) = (0u8, 0f64, 0f64);
    let mut worst_name = String::new();
    let mut failures = vec![];
    let scenes = corpus();
    for s in &scenes {
        let cpu = render_cpu(s);
        let g = gpu.render(&s.list, s.width, s.height, s.clear);
        let (imax, _edge, mae, over3) = compare(&cpu, &g);
        worst_interior = worst_interior.max(imax);
        worst_mae = worst_mae.max(mae);
        if over3 > worst_over3 {
            worst_name = s.name.clone();
        }
        worst_over3 = worst_over3.max(over3);
        // Interior pixels: rounding differences only (alpha layers round differently on CPU and GPU).
        // Overall: mean error under 1/255 and under 12% of pixels visibly different (edges of small scenes).
        if imax > 2 || mae > 1.5 || over3 > 0.12 {
            failures.push(format!("{}: interior max {imax}/255, mean abs error {mae:.2}/255, {:.2}% px > 3/255", s.name, over3 * 100.0));
        }
    }
    println!(
        "GPU vs CPU over {} scenes: worst interior diff {worst_interior}/255; worst mean abs error {worst_mae:.3}/255; worst fraction of pixels > 3/255: {:.2}% ({worst_name})",
        scenes.len(), worst_over3 * 100.0
    );
    assert!(failures.is_empty(), "{} of {} scenes outside tolerance:\n{}", failures.len(), scenes.len(), failures.iter().take(10).cloned().collect::<Vec<_>>().join("\n"));
}

// ---- unrelated rasterizer reference (tiny-skia) ----
fn skia_render(s: &GoldenScene) -> Image {
    use tiny_skia::*;
    fn rr(r: wj_scene::Rect, radius: f32) -> Path {
        let rad = radius.min(r.w / 2.0).min(r.h / 2.0).max(0.0);
        let mut pb = PathBuilder::new();
        if rad <= 0.0 {
            pb.push_rect(tiny_skia::Rect::from_xywh(r.x, r.y, r.w, r.h).unwrap());
        } else {
            let k = 0.552_284_75 * rad;
            let (x0, y0, x1, y1) = (r.x, r.y, r.x + r.w, r.y + r.h);
            pb.move_to(x0 + rad, y0);
            pb.line_to(x1 - rad, y0);
            pb.cubic_to(x1 - rad + k, y0, x1, y0 + rad - k, x1, y0 + rad);
            pb.line_to(x1, y1 - rad);
            pb.cubic_to(x1, y1 - rad + k, x1 - rad + k, y1, x1 - rad, y1);
            pb.line_to(x0 + rad, y1);
            pb.cubic_to(x0 + rad - k, y1, x0, y1 - rad + k, x0, y1 - rad);
            pb.line_to(x0, y0 + rad);
            pb.cubic_to(x0, y0 + rad - k, x0 + rad - k, y0, x0 + rad, y0);
            pb.close();
        }
        pb.finish().unwrap()
    }
    fn paint(c: wj_scene::Color) -> Paint<'static> {
        let mut p = Paint::default();
        p.set_color_rgba8(c.r, c.g, c.b, c.a);
        p.anti_alias = true;
        p
    }
    // Each layer is its own pixmap; clips are masks applied to everything drawn while active.
    struct Layer {
        pix: Pixmap,
        opacity: f32,
        is_clip: bool,
        mask: Option<Mask>,
    }
    let (w, h) = (s.width, s.height);
    let mut base = Pixmap::new(w, h).unwrap();
    base.fill(tiny_skia::Color::from_rgba8(s.clear.r, s.clear.g, s.clear.b, s.clear.a));
    let mut stack = vec![Layer { pix: base, opacity: 1.0, is_clip: false, mask: None }];
    fn current_mask(stack: &[Layer]) -> Option<&Mask> {
        stack.iter().rev().find_map(|l| l.mask.as_ref())
    }
    for cmd in &s.list.cmds {
        match cmd {
            DrawCmd::PushClip { rect, radius } => {
                let mut m = Mask::new(w, h).unwrap();
                m.fill_path(&rr(*rect, *radius), FillRule::Winding, true, Transform::identity());
                if let Some(prev) = current_mask(&stack) {
                    // intersect with the enclosing clip
                    let mut inter = prev.clone();
                    inter.intersect_path(&rr(*rect, *radius), FillRule::Winding, true, Transform::identity());
                    m = inter;
                }
                stack.push(Layer { pix: Pixmap::new(1, 1).unwrap(), opacity: 1.0, is_clip: true, mask: Some(m) });
            }
            DrawCmd::PopClip => {
                stack.pop();
            }
            DrawCmd::PushLayer { opacity } => {
                stack.push(Layer { pix: Pixmap::new(w, h).unwrap(), opacity: *opacity, is_clip: false, mask: None });
            }
            DrawCmd::PopLayer => {
                let l = stack.pop().unwrap();
                let target = stack.iter_mut().rev().find(|l| !l.is_clip).unwrap();
                let pp = PixmapPaint { opacity: l.opacity, blend_mode: BlendMode::SourceOver, quality: FilterQuality::Nearest };
                target.pix.draw_pixmap(0, 0, l.pix.as_ref(), &pp, Transform::identity(), None);
            }
            DrawCmd::FillRRect { rect, radius, color } => {
                let mask = current_mask(&stack).cloned();
                let t = stack.iter_mut().rev().find(|l| !l.is_clip).unwrap();
                t.pix.fill_path(&rr(*rect, *radius), &paint(*color), FillRule::Winding, Transform::identity(), mask.as_ref());
            }
            DrawCmd::StrokeRRect { rect, radius, width, color } => {
                let half = width / 2.0;
                let inner = wj_scene::Rect::new(rect.x + half, rect.y + half, rect.w - width, rect.h - width);
                let mask = current_mask(&stack).cloned();
                let t = stack.iter_mut().rev().find(|l| !l.is_clip).unwrap();
                let st = Stroke { width: *width, ..Default::default() };
                t.pix.stroke_path(&rr(inner, (radius - half).max(0.0)), &paint(*color), &st, Transform::identity(), mask.as_ref());
            }
            DrawCmd::Text { .. } => unreachable!("vector scenes only"),
        }
    }
    let pix = stack.pop().unwrap().pix;
    let mut img = Image::new(w, h);
    for (dst, src) in img.rgba.chunks_mut(4).zip(pix.pixels()) {
        let c = src.demultiply();
        dst.copy_from_slice(&[c.red(), c.green(), c.blue(), c.alpha()]);
    }
    img
}

#[test]
fn cpu_painter_agrees_with_an_unrelated_rasterizer() {
    let scenes: Vec<_> = corpus().into_iter().filter(|s| s.vector_only).collect();
    let mut failures = vec![];
    let mut worst_flat = 0u8;
    for s in &scenes {
        let ours = render_cpu(s);
        let skia = skia_render(s);
        let flat = flat_mask(&ours);
        let mut flat_max = 0u8;
        for (i, (a, b)) in ours.rgba.chunks(4).zip(skia.rgba.chunks(4)).enumerate() {
            if flat[i] {
                flat_max = flat_max.max(a.iter().zip(b).map(|(x, y)| x.abs_diff(*y)).max().unwrap());
            }
        }
        let (_, frac) = ours.diff(&skia, 12);
        worst_flat = worst_flat.max(flat_max);
        // Different anti-aliasing and rounded-corner approximations: interior must agree, edges loosely.
        if flat_max > 3 || frac > 0.07 {
            failures.push(format!("{}: interior max diff {flat_max}, {:.2}% pixels differ by > 12/255", s.name, frac * 100.0));
        }
    }
    println!("CPU painter vs tiny-skia over {} vector scenes: worst interior diff {worst_flat}/255", scenes.len());
    assert!(failures.is_empty(), "{} scenes disagree:\n{}", failures.len(), failures.iter().take(10).cloned().collect::<Vec<_>>().join("\n"));
}
