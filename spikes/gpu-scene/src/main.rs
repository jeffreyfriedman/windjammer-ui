//! Spike: Vello (GPU 2D vector renderer on wgpu) + Parley (shaping/layout), headless.
//! Measures: renderer init, text layout cost, scene-encode time, GPU render time for
//! a "10k-row list" (all rows drawn = worst case) and a virtualized 40-row window.
use parley::{FontContext, Layout, LayoutContext, PositionedLayoutItem, StyleProperty};
use std::time::Instant;
use vello::kurbo::{Affine, RoundedRect};
use vello::peniko::{Color, Fill};
use vello::{wgpu, AaConfig, Glyph, RenderParams, Renderer, RendererOptions, Scene};

const W: u32 = 1280;
const H: u32 = 720;
const ROW_H: f64 = 36.0;

type Brush = [u8; 4];

fn layout_text(fcx: &mut FontContext, lcx: &mut LayoutContext<Brush>, s: &str, size: f32) -> Layout<Brush> {
    let mut b = lcx.ranged_builder(fcx, s, 1.0, true);
    b.push_default(StyleProperty::FontSize(size));
    let mut l = b.build(s);
    l.break_all_lines(Some(600.0));
    l
}

fn draw_layout(scene: &mut Scene, layout: &Layout<Brush>, tx: Affine) {
    for line in layout.lines() {
        for item in line.items() {
            if let PositionedLayoutItem::GlyphRun(run) = item {
                let r = run.run();
                let font = r.font();
                scene.draw_glyphs(font).transform(tx).font_size(r.font_size()).brush(Color::from_rgba8(20, 20, 30, 255))
                    .draw(Fill::NonZero, run.positioned_glyphs().map(|g| Glyph { id: g.id as u32, x: g.x, y: g.y }));
            }
        }
    }
}

fn build_scene(scene: &mut Scene, layouts: &[Layout<Brush>], first: usize, last: usize, scroll: f64) {
    scene.reset();
    for i in first..last {
        let y = i as f64 * ROW_H - scroll;
        let shade = if i % 2 == 0 { 245 } else { 235 };
        scene.fill(Fill::NonZero, Affine::IDENTITY, Color::from_rgba8(shade, shade, 250, 255), None,
            &RoundedRect::new(8.0, y + 2.0, W as f64 - 8.0, y + ROW_H - 2.0, 6.0));
        draw_layout(scene, &layouts[i], Affine::translate((16.0, y + 8.0)));
    }
}

fn stats(v: &mut Vec<f64>) -> (f64, f64, f64) { v.sort_by(|a, b| a.partial_cmp(b).unwrap()); (v[v.len() / 2], v[(v.len() as f64 * 0.95) as usize], v[0]) }

fn main() {
    let t0 = Instant::now();
    let instance = wgpu::Instance::default();
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default())).expect("adapter");
    let info = adapter.get_info();
    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).expect("device");
    let t_dev = t0.elapsed().as_secs_f64() * 1e3;
    let t1 = Instant::now();
    let mut renderer = Renderer::new(&device, RendererOptions { antialiasing_support: vello::AaSupport::area_only(), ..Default::default() }).expect("renderer");
    let t_rend = t1.elapsed().as_secs_f64() * 1e3;
    println!("adapter: {} ({:?}); device init {:.1} ms; vello Renderer::new (shader pipelines) {:.1} ms", info.name, info.backend, t_dev, t_rend);

    let tex = device.create_texture(&wgpu::TextureDescriptor { label: None, size: wgpu::Extent3d { width: W, height: H, depth_or_array_layers: 1 },
        mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC, view_formats: &[] });
    let view = tex.create_view(&Default::default());

    // text: shape 10k short strings
    let t2 = Instant::now();
    let mut fcx = FontContext::new(); let mut lcx = LayoutContext::<Brush>::new();
    let _warm = layout_text(&mut fcx, &mut lcx, "warm up", 15.0);
    let t_font = t2.elapsed().as_secs_f64() * 1e3;
    let t3 = Instant::now();
    let layouts: Vec<_> = (0..10_000).map(|i| layout_text(&mut fcx, &mut lcx, &format!("Row {i}: invoice #{} - Acme Corp - $1,234.56", i * 7), 15.0)).collect();
    let t_shape = t3.elapsed().as_secs_f64() * 1e3;
    println!("font context + first layout {:.1} ms; shaping+layout of 10,000 rows (~40 chars) {:.1} ms ({:.1} us/row)", t_font, t_shape, t_shape * 1e3 / 10_000.0);
    // bidi / non-latin smoke test
    let mixed = layout_text(&mut fcx, &mut lcx, "Hello \u{05E9}\u{05DC}\u{05D5}\u{05DD} world \u{0645}\u{0631}\u{062D}\u{0628}\u{0627} \u{65E5}\u{672C}\u{8A9E} \u{1F600}", 15.0);
    println!("bidi/CJK/emoji smoke test: {} line(s), {} glyph runs, width {:.1}", mixed.len(), mixed.lines().flat_map(|l| l.items()).count(), mixed.width());

    let params = RenderParams { base_color: Color::WHITE, width: W, height: H, antialiasing_method: AaConfig::Area };
    let mut scene = Scene::new();
    for (label, first, last, n) in [("virtualized 40 rows visible", 0usize, 40usize, 200), ("ALL 10,000 rows submitted (no culling)", 0, 10_000, 30)] {
        let (mut enc, mut gpu) = (vec![], vec![]);
        for f in 0..n {
            let scroll = (f as f64 * 13.0) % 4000.0;
            let (a, b) = if last == 40 { let a = (scroll / ROW_H) as usize; (a, a + 40) } else { (first, last) };
            let t = Instant::now(); build_scene(&mut scene, &layouts, a, b, scroll); enc.push(t.elapsed().as_secs_f64() * 1e3);
            let t = Instant::now();
            renderer.render_to_texture(&device, &queue, &scene, &view, &params).unwrap();
            let _ = device.poll(wgpu::PollType::wait_indefinitely());
            gpu.push(t.elapsed().as_secs_f64() * 1e3);
        }
        gpu.drain(0..2.min(gpu.len()));
        let (em, e95, _) = stats(&mut enc); let (gm, g95, gmin) = stats(&mut gpu);
        println!("{label:<42} scene encode median {em:.3} ms (p95 {e95:.3}); render+GPU wait median {gm:.2} ms (p95 {g95:.2}, min {gmin:.2})");
    }
}
