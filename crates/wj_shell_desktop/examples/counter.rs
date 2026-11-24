//! The counter, in plain Rust against the M1 API. Run: `cargo run --release -p wj-shell-desktop --example counter`
//! Measurement mode: `... --example counter -- --frames 120` prints startup and frame-time numbers.
use wj_reactive::state;
use wj_scene::*;
use wj_shell_desktop::{run, RunOptions};

fn main() {
    let frames = std::env::args().skip_while(|a| a != "--frames").nth(1).and_then(|n| n.parse().ok());
    let bump = std::rc::Rc::new(std::cell::RefCell::new(None::<wj_reactive::Signal<i32>>));
    let b2 = bump.clone();
    let on_frame: Option<Box<dyn FnMut(usize)>> = frames.map(|_| Box::new(move |_i: usize| if let Some(c) = *b2.borrow() { c.set(c.get_untracked() + 1) }) as Box<dyn FnMut(usize)>);
    let summary = run(RunOptions { title: "Counter".into(), max_frames: frames, on_frame, ..Default::default() }, move |app| {
        let s = app.scene;
        s.set_style(s.root(), Style { align_items: Align::Center, justify: Justify::Center, ..Default::default() });
        let count = state(0);
        *bump.borrow_mut() = Some(count);
        let button = s.create(s.root());
        s.set_semantics(button, Semantics { role: Role::Button, focusable: true, ..Default::default() });
        s.set_paint(button, Paint { background: Some(Color::rgb(30, 100, 220)), corner_radius: 8.0, ..Default::default() });
        s.set_style(button, Style { padding: Edges { left: 18.0, right: 18.0, top: 10.0, bottom: 10.0 }, ..Default::default() });
        let label = s.create(button);
        s.set_text(label, TextProps { content: String::new(), font_size: 18.0, color: Color::WHITE });
        s.bind_text(label, move || format!("Clicked {} times", count.get()));
        s.on(button, EventKind::Click, move |_| count.set(count.get_untracked() + 1));
    });
    if frames.is_some() {
        println!(
            "window ready {:.1} ms; first frame {:.1} ms; frames {}; p50 {:.2} ms; p95 {:.2} ms; max {:.2} ms",
            summary.window_ready_ms,
            summary.first_frame_ms,
            summary.frame_times_ms.len(),
            summary.percentile(0.5),
            summary.percentile(0.95),
            summary.percentile(1.0)
        );
    }
}
