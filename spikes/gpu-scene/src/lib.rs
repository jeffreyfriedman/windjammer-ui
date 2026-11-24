//! wasm size probe: links the same vello + parley + wgpu(WebGPU/WebGL) code paths as main.rs.
#![cfg(target_arch = "wasm32")]
use parley::{FontContext, LayoutContext, StyleProperty};
use vello::{wgpu, kurbo::{Affine, RoundedRect}, peniko::{Color, Fill}, RenderParams, Renderer, RendererOptions, Scene, AaConfig};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn run() -> Result<f64, JsValue> {
    console_error_panic_hook::set_once();
    let instance = wgpu::Instance::default();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.map_err(|e| JsValue::from_str(&format!("{e:?}")))?;
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await.map_err(|e| JsValue::from_str(&format!("{e:?}")))?;
    let mut renderer = Renderer::new(&device, RendererOptions::default()).map_err(|e| JsValue::from_str(&format!("{e:?}")))?;
    let mut fcx = FontContext::new(); let mut lcx = LayoutContext::<[u8; 4]>::new();
    let text = "Hello wasm";
    let mut b = lcx.ranged_builder(&mut fcx, text, 1.0, true); b.push_default(StyleProperty::FontSize(16.0));
    let mut l = b.build(text); l.break_all_lines(Some(300.0));
    let mut scene = Scene::new();
    scene.fill(Fill::NonZero, Affine::IDENTITY, Color::WHITE, None, &RoundedRect::new(0.0, 0.0, 100.0, 30.0, 4.0));
    let tex = device.create_texture(&wgpu::TextureDescriptor { label: None, size: wgpu::Extent3d { width: 256, height: 256, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
        dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm, usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC, view_formats: &[] });
    renderer.render_to_texture(&device, &queue, &scene, &tex.create_view(&Default::default()),
        &RenderParams { base_color: Color::WHITE, width: 256, height: 256, antialiasing_method: AaConfig::Area }).map_err(|e| JsValue::from_str(&format!("{e:?}")))?;
    Ok(l.width() as f64)
}
