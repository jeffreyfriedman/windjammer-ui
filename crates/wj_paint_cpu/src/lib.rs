//! `wj-paint-cpu`: [`wj_paint::Painter`] over `vello_cpu`. The fallback renderer on machines
//! without a usable GPU and the reference that GPU painters are compared against in golden tests.
use std::collections::HashMap;
use std::sync::Arc;

use vello_cpu::kurbo::{Rect, RoundedRect, Shape, Stroke};
use vello_cpu::peniko::{Blob, Color as VColor, FontData};
use vello_cpu::{Glyph, Pixmap, RenderContext, Resources};
use wj_paint::{DisplayList, DrawCmd, GlyphRun, Image, Painter};
use wj_scene::Color;

fn vcolor(c: Color) -> VColor {
    VColor::from_rgba8(c.r, c.g, c.b, c.a)
}

#[derive(Default)]
pub struct CpuPainter {
    resources: Option<Resources>,
    fonts: HashMap<(u64, u32), FontData>,
}

impl CpuPainter {
    pub fn new() -> Self {
        Self::default()
    }

    fn font(&mut self, run: &GlyphRun) -> FontData {
        self.fonts
            .entry((run.font.id, run.font.index))
            .or_insert_with(|| {
                let bytes = run.font.bytes.clone();
                FontData::new(Blob::new(Arc::new(bytes.as_ref().clone()) as Arc<dyn AsRef<[u8]> + Send + Sync>), run.font.index)
            })
            .clone()
    }
}

fn rrect(r: wj_scene::Rect, radius: f32) -> RoundedRect {
    let rad = (radius as f64).min(r.w as f64 / 2.0).min(r.h as f64 / 2.0).max(0.0);
    RoundedRect::new(r.x as f64, r.y as f64, (r.x + r.w) as f64, (r.y + r.h) as f64, rad)
}

impl Painter for CpuPainter {
    fn render(&mut self, list: &DisplayList, width: u32, height: u32, clear: Color) -> Image {
        assert!(list.is_balanced(), "unbalanced display list");
        let (w, h) = (width as u16, height as u16);
        let mut ctx = RenderContext::new(w, h);
        let mut resources = self.resources.take().unwrap_or_else(Resources::new);
        ctx.set_paint(vcolor(clear));
        ctx.fill_rect(&Rect::new(0.0, 0.0, width as f64, height as f64));
        for cmd in &list.cmds {
            match cmd {
                DrawCmd::PushClip { rect, radius } => ctx.push_clip_layer(&rrect(*rect, *radius).to_path(0.1)),
                DrawCmd::PopClip | DrawCmd::PopLayer => ctx.pop_layer(),
                DrawCmd::PushLayer { opacity } => ctx.push_opacity_layer(*opacity),
                DrawCmd::FillRRect { rect, radius, color } => {
                    ctx.set_paint(vcolor(*color));
                    if *radius <= 0.0 {
                        ctx.fill_rect(&Rect::new(rect.x as f64, rect.y as f64, (rect.x + rect.w) as f64, (rect.y + rect.h) as f64));
                    } else {
                        ctx.fill_path(&rrect(*rect, *radius).to_path(0.1));
                    }
                }
                DrawCmd::StrokeRRect { rect, radius, width, color } => {
                    ctx.set_paint(vcolor(*color));
                    ctx.set_stroke(Stroke::new(*width as f64));
                    let half = width / 2.0;
                    let inner = wj_scene::Rect::new(rect.x + half, rect.y + half, rect.w - width, rect.h - width);
                    ctx.stroke_path(&rrect(inner, (radius - half).max(0.0)).to_path(0.1));
                }
                DrawCmd::Text { run } => {
                    let font = self.font(run);
                    ctx.set_paint(vcolor(run.color));
                    ctx.set_transform(vello_cpu::kurbo::Affine::translate((run.x as f64, run.y as f64)));
                    let glyphs: Vec<Glyph> = run.glyphs.iter().map(|g| Glyph { id: g.id, x: g.x, y: g.y }).collect();
                    let _ = ctx.glyph_run(&mut resources, &font).font_size(run.size).fill_glyphs(glyphs.into_iter());
                    ctx.reset_transform();
                }
            }
        }
        ctx.flush();
        let mut pix = Pixmap::new(w, h);
        ctx.render(&mut pix, &mut resources);
        self.resources = Some(resources);
        let mut img = Image::new(width, height);
        for (dst, src) in img.rgba.chunks_mut(4).zip(pix.data()) {
            let a = src.a as u32;
            if a == 0 {
                continue;
            }
            let un = |c: u8| (((c as u32) * 255 + a / 2) / a).min(255) as u8;
            dst.copy_from_slice(&[un(src.r), un(src.g), un(src.b), src.a]);
        }
        img
    }
}
