//! `wj-paint-vello`: [`wj_paint::Painter`] over Vello on wgpu.
//!
//! [`VelloPainter::headless`] creates its own device (tests, thumbnails, server-side rendering).
//! A windowed shell passes its device and queue to [`VelloPainter::with_device`] and draws into a
//! surface texture with [`VelloPainter::render_to_texture`].
use std::collections::HashMap;
use std::sync::Arc;

use vello::kurbo::{Affine, Rect, RoundedRect, Stroke};
use vello::peniko::{BlendMode, Blob, Color as VColor, Compose, Fill, FontData, Mix};
use vello::{wgpu, AaConfig, AaSupport, Glyph, RenderParams, Renderer, RendererOptions, Scene};
use wj_paint::{DisplayList, DrawCmd, Image, Painter};
use wj_scene::Color;

fn vcolor(c: Color) -> VColor {
    VColor::from_rgba8(c.r, c.g, c.b, c.a)
}

fn rrect(r: wj_scene::Rect, radius: f32) -> RoundedRect {
    let rad = (radius as f64).min(r.w as f64 / 2.0).min(r.h as f64 / 2.0).max(0.0);
    RoundedRect::new(r.x as f64, r.y as f64, (r.x + r.w) as f64, (r.y + r.h) as f64, rad)
}

/// Encode a display list into a Vello scene. Public so shells can time encode and render separately.
pub fn encode(list: &DisplayList, scene: &mut Scene, fonts: &mut HashMap<(u64, u32), FontData>) {
    assert!(list.is_balanced(), "unbalanced display list");
    scene.reset();
    for cmd in &list.cmds {
        match cmd {
            DrawCmd::PushClip { rect, radius } => scene.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &rrect(*rect, *radius)),
            DrawCmd::PopClip | DrawCmd::PopLayer => scene.pop_layer(),
            DrawCmd::PushLayer { opacity } => {
                // Whole-surface group at the given alpha.
                scene.push_layer(Fill::NonZero, BlendMode::new(Mix::Normal, Compose::SrcOver), *opacity, Affine::IDENTITY, &Rect::new(-1e6, -1e6, 1e6, 1e6));
            }
            DrawCmd::FillRRect { rect, radius, color } => {
                if *radius <= 0.0 {
                    scene.fill(Fill::NonZero, Affine::IDENTITY, vcolor(*color), None, &Rect::new(rect.x as f64, rect.y as f64, (rect.x + rect.w) as f64, (rect.y + rect.h) as f64));
                } else {
                    scene.fill(Fill::NonZero, Affine::IDENTITY, vcolor(*color), None, &rrect(*rect, *radius));
                }
            }
            DrawCmd::StrokeRRect { rect, radius, width, color } => {
                let half = width / 2.0;
                let inner = wj_scene::Rect::new(rect.x + half, rect.y + half, rect.w - width, rect.h - width);
                scene.stroke(&Stroke::new(*width as f64), Affine::IDENTITY, vcolor(*color), None, &rrect(inner, (radius - half).max(0.0)));
            }
            DrawCmd::Text { run } => {
                let font = fonts
                    .entry((run.font.id, run.font.index))
                    .or_insert_with(|| FontData::new(Blob::new(Arc::new(run.font.bytes.as_ref().clone()) as Arc<dyn AsRef<[u8]> + Send + Sync>), run.font.index))
                    .clone();
                scene
                    .draw_glyphs(&font)
                    .transform(Affine::translate((run.x as f64, run.y as f64)))
                    .font_size(run.size)
                    .brush(vcolor(run.color))
                    .draw(Fill::NonZero, run.glyphs.iter().map(|g| Glyph { id: g.id, x: g.x, y: g.y }));
            }
        }
    }
}

pub struct VelloPainter {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    renderer: Renderer,
    scene: Scene,
    fonts: HashMap<(u64, u32), FontData>,
}

impl VelloPainter {
    /// Create a device on the default adapter. `None` when no adapter is available.
    pub fn headless() -> Option<Self> {
        let instance = wgpu::Instance::default();
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default())).ok()?;
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).ok()?;
        Some(Self::with_device(device, queue))
    }

    pub fn with_device(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        let renderer = Renderer::new(&device, RendererOptions { antialiasing_support: AaSupport::area_only(), ..Default::default() }).expect("vello renderer");
        VelloPainter { device, queue, renderer, scene: Scene::new(), fonts: HashMap::new() }
    }

    /// Render into a texture created with `STORAGE_BINDING` (usually `Rgba8Unorm`).
    pub fn render_to_texture(&mut self, list: &DisplayList, view: &wgpu::TextureView, width: u32, height: u32, clear: Color) {
        encode(list, &mut self.scene, &mut self.fonts);
        let params = RenderParams { base_color: vcolor(clear), width, height, antialiasing_method: AaConfig::Area };
        self.renderer.render_to_texture(&self.device, &self.queue, &self.scene, view, &params).expect("vello render");
    }
}

impl Painter for VelloPainter {
    fn render(&mut self, list: &DisplayList, width: u32, height: u32, clear: Color) -> Image {
        let tex = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("wj-paint-vello target"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = tex.create_view(&Default::default());
        self.render_to_texture(list, &view, width, height, clear);
        // Read back with 256-byte aligned rows.
        let bytes_per_row = (width * 4).div_ceil(256) * 256;
        let buf = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("wj-paint-vello readback"),
            size: (bytes_per_row * height) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut enc = self.device.create_command_encoder(&Default::default());
        enc.copy_texture_to_buffer(
            tex.as_image_copy(),
            wgpu::TexelCopyBufferInfo { buffer: &buf, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(bytes_per_row), rows_per_image: Some(height) } },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );
        self.queue.submit([enc.finish()]);
        let slice = buf.slice(..);
        slice.map_async(wgpu::MapMode::Read, |r| r.expect("map readback"));
        let _ = self.device.poll(wgpu::PollType::wait_indefinitely());
        let data = slice.get_mapped_range().expect("mapped range");
        let mut img = Image::new(width, height);
        for y in 0..height as usize {
            let src = &data[y * bytes_per_row as usize..y * bytes_per_row as usize + width as usize * 4];
            img.rgba[y * width as usize * 4..(y + 1) * width as usize * 4].copy_from_slice(src);
        }
        drop(data);
        buf.unmap();
        img
    }
}
