//! `wj-shell-desktop` (layer L4): a winit window presenting `wj-shell`'s frame pipeline through
//! Vello on wgpu, with input forwarded to the scene and an AccessKit adapter exposing the
//! semantic tree to VoiceOver (macOS), UI Automation (Windows) and AT-SPI (Linux).
//!
//! [`run`] builds the app inside the event loop (so the reactive runtime lives on the UI thread),
//! then drives frames on demand. With `max_frames` set it redraws continuously and returns timing,
//! which is how the startup and frame-time numbers in the docs are produced.
use std::sync::Arc;
use std::time::Instant;

use accesskit_winit::{Adapter, Event as AkEvent, WindowEvent as AkWindowEvent};
use vello::util::{RenderContext, RenderSurface};
use vello::wgpu;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::Key;
use winit::window::{Window, WindowId};
use wj_paint::DisplayList;
use wj_paint_vello::VelloPainter;
use wj_scene::{Color, Event, NodeId};
use wj_shell::{App, Shell, ShellEvent, SystemClock};

pub struct RunOptions {
    pub title: String,
    pub width: f64,
    pub height: f64,
    /// Redraw continuously and exit after this many frames (measurement mode).
    pub max_frames: Option<usize>,
    pub clear: Color,
    /// Called before every frame in measurement mode with the frame index: drive state changes here
    /// so each frame has work to do (an idle frame is skipped and not counted).
    pub on_frame: Option<Box<dyn FnMut(usize)>>,
}

impl Default for RunOptions {
    fn default() -> Self {
        RunOptions { title: "Windjammer".into(), width: 480.0, height: 320.0, max_frames: None, clear: Color::rgb(250, 250, 252), on_frame: None }
    }
}

/// Timing collected in measurement mode (all in ms).
#[derive(Clone, Debug, Default)]
pub struct RunSummary {
    pub window_ready_ms: f64,
    pub first_frame_ms: f64,
    pub frame_times_ms: Vec<f64>,
}

impl RunSummary {
    pub fn percentile(&self, p: f64) -> f64 {
        let mut v = self.frame_times_ms.clone();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if v.is_empty() { 0.0 } else { v[((v.len() - 1) as f64 * p) as usize] }
    }
}

enum User {
    Ak(AkEvent),
}
impl From<AkEvent> for User {
    fn from(e: AkEvent) -> Self {
        User::Ak(e)
    }
}

struct Surface {
    window: Arc<Window>,
    ctx: RenderContext,
    surface: RenderSurface<'static>,
    painter: VelloPainter,
    adapter: Adapter,
}

struct DesktopShell {
    surface: Surface,
    clear: Color,
    events: Vec<ShellEvent>,
    pending_a11y: Vec<accesskit::TreeUpdate>,
    cursor: (f32, f32),
}

impl Shell for DesktopShell {
    fn size(&self) -> (u32, u32) {
        let s = self.surface.window.inner_size();
        (s.width.max(1), s.height.max(1))
    }
    fn scale(&self) -> f32 {
        self.surface.window.scale_factor() as f32
    }
    fn poll_events(&mut self) -> Vec<ShellEvent> {
        std::mem::take(&mut self.events)
    }
    fn present(&mut self, list: &DisplayList, width: u32, height: u32) {
        let s = &mut self.surface;
        if s.surface.config.width != width || s.surface.config.height != height {
            s.ctx.resize_surface(&mut s.surface, width, height);
        }
        s.painter.scale = s.window.scale_factor();
        s.painter.render_to_texture(list, &s.surface.target_view, width, height, self.clear);
        let frame = match s.surface.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            _ => return,
        };
        let view = frame.texture.create_view(&Default::default());
        let dev = &s.ctx.devices[s.surface.dev_id];
        let mut enc = dev.device.create_command_encoder(&Default::default());
        s.surface.blitter.copy(&dev.device, &mut enc, &s.surface.target_view, &view);
        dev.queue.submit([enc.finish()]);
        s.window.pre_present_notify();
        dev.queue.present(frame);
    }
    fn a11y_update(&mut self, update: accesskit::TreeUpdate) {
        self.pending_a11y.push(update);
    }
}

struct Runner<F: FnOnce(&mut App)> {
    build: Option<F>,
    opts: RunOptions,
    app: Option<App>,
    shell: Option<DesktopShell>,
    started: Instant,
    summary: RunSummary,
    frames: usize,
    proxy: winit::event_loop::EventLoopProxy<User>,
}

impl<F: FnOnce(&mut App)> Runner<F> {
    fn redraw(&mut self) {
        let (Some(app), Some(shell)) = (self.app.as_mut(), self.shell.as_mut()) else { return };
        let t = Instant::now();
        let clock = SystemClock::new();
        let rep = app.frame(shell, &clock);
        for u in shell.pending_a11y.drain(..) {
            shell.surface.adapter.update_if_active(|| u);
        }
        if !rep.skipped {
            if self.frames == 0 {
                self.summary.first_frame_ms = self.started.elapsed().as_secs_f64() * 1e3;
            }
            self.frames += 1;
            self.summary.frame_times_ms.push(t.elapsed().as_secs_f64() * 1e3);
        }
        if rep.close_requested {
            // handled by caller of exit below
        }
    }
}

impl<F: FnOnce(&mut App)> ApplicationHandler<User> for Runner<F> {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.shell.is_some() {
            return;
        }
        let attrs = Window::default_attributes().with_title(self.opts.title.clone()).with_inner_size(LogicalSize::new(self.opts.width, self.opts.height)).with_visible(false);
        let window = Arc::new(el.create_window(attrs).expect("window"));
        let adapter = Adapter::with_event_loop_proxy(el, &window, self.proxy.clone());
        window.set_visible(true);
        let size = window.inner_size();
        let mut ctx = RenderContext::new();
        let surface = pollster::block_on(ctx.create_surface(window.clone(), size.width.max(1), size.height.max(1), wgpu::PresentMode::AutoVsync)).expect("surface");
        let dev = &ctx.devices[surface.dev_id];
        let painter = VelloPainter::with_device(dev.device.clone(), dev.queue.clone());
        self.summary.window_ready_ms = self.started.elapsed().as_secs_f64() * 1e3;
        let mut app = App::new();
        (self.build.take().expect("build once"))(&mut app);
        self.app = Some(app);
        self.shell = Some(DesktopShell { surface: Surface { window, ctx, surface, painter, adapter }, clear: self.opts.clear, events: vec![], pending_a11y: vec![], cursor: (0.0, 0.0) });
        if self.opts.max_frames.is_some() {
            el.set_control_flow(ControlFlow::Poll);
        }
        self.shell.as_ref().unwrap().surface.window.request_redraw();
    }

    fn user_event(&mut self, _el: &ActiveEventLoop, ev: User) {
        let User::Ak(ev) = ev;
        let (Some(app), Some(shell)) = (self.app.as_mut(), self.shell.as_mut()) else { return };
        match ev.window_event {
            AkWindowEvent::InitialTreeRequested => {
                app.reset_a11y();
            }
            AkWindowEvent::ActionRequested(req) => {
                if let Some((idx, gen)) = wj_a11y::scene_parts(req.target_node) {
                    let node = NodeId::from_parts(idx, gen);
                    match req.action {
                        accesskit::Action::Click => {
                            app.scene.invoke(node, Event::Click { x: 0.0, y: 0.0 });
                        }
                        accesskit::Action::Focus => app.scene.set_focus(Some(node)),
                        _ => {}
                    }
                }
            }
            AkWindowEvent::AccessibilityDeactivated => {}
        }
        shell.surface.window.request_redraw();
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(shell) = self.shell.as_mut() else { return };
        shell.surface.adapter.process_event(&shell.surface.window.clone(), &event);
        let scale = shell.surface.window.scale_factor() as f32;
        match event {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::Resized(s) => {
                shell.events.push(ShellEvent::Resize { width: s.width.max(1), height: s.height.max(1), scale });
                shell.surface.window.request_redraw();
            }
            WindowEvent::ScaleFactorChanged { .. } => shell.surface.window.request_redraw(),
            WindowEvent::CursorMoved { position, .. } => {
                shell.cursor = (position.x as f32 / scale, position.y as f32 / scale);
                shell.events.push(ShellEvent::Input(Event::PointerMove { x: shell.cursor.0, y: shell.cursor.1 }));
                shell.surface.window.request_redraw();
            }
            WindowEvent::MouseInput { state, button: MouseButton::Left, .. } => {
                let (x, y) = shell.cursor;
                shell.events.push(ShellEvent::Input(if state == ElementState::Pressed { Event::PointerDown { x, y } } else { Event::PointerUp { x, y } }));
                shell.surface.window.request_redraw();
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let (dx, dy) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (x * 40.0, y * 40.0),
                    MouseScrollDelta::PixelDelta(p) => (p.x as f32 / scale, p.y as f32 / scale),
                };
                shell.events.push(ShellEvent::Input(Event::Wheel { x: shell.cursor.0, y: shell.cursor.1, dx, dy }));
                shell.surface.window.request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let key = match &event.logical_key {
                    Key::Character(c) => c.to_string(),
                    Key::Named(n) => format!("{n:?}"),
                    _ => String::new(),
                };
                shell.events.push(ShellEvent::Input(Event::Key { key, pressed: event.state == ElementState::Pressed }));
                shell.surface.window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                if let Some(f) = self.opts.on_frame.as_mut() {
                    f(self.frames);
                }
                self.redraw();
                if let Some(max) = self.opts.max_frames {
                    if self.frames >= max {
                        el.exit();
                    } else if let Some(s) = self.shell.as_ref() {
                        s.surface.window.request_redraw();
                    }
                }
            }
            _ => {}
        }
    }
}

/// Open a window, build the app on the UI thread with `build`, and run until closed (or until
/// `max_frames` frames were drawn). Returns the collected timings.
pub fn run<F: FnOnce(&mut App) + 'static>(opts: RunOptions, build: F) -> RunSummary {
    let started = Instant::now();
    let event_loop = EventLoop::<User>::with_user_event().build().expect("event loop");
    let proxy = event_loop.create_proxy();
    let mut runner = Runner { build: Some(build), opts, app: None, shell: None, started, summary: RunSummary::default(), frames: 0, proxy };
    event_loop.run_app(&mut runner).expect("run");
    runner.summary
}
