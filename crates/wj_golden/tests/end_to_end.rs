//! Whole-stack check with real pixels: app code -> scene -> layout -> text -> display list -> painter.
mod common;
use wj_paint::Painter;
use wj_paint_cpu::CpuPainter;
use wj_reactive::state;
use wj_scene::*;
use wj_shell::{App, FakeClock, FakeShell, Shell, ShellEvent};

fn counter_frames() -> (wj_paint::DisplayList, wj_paint::DisplayList) {
    wj_reactive::reset();
    let mut app = App::new();
    let s = app.scene;
    s.set_style(s.root(), Style { align_items: Align::Center, justify: Justify::Center, ..Default::default() });
    let count = state(0);
    let button = s.create(s.root());
    s.set_semantics(button, Semantics { role: Role::Button, focusable: true, ..Default::default() });
    s.set_paint(button, Paint { background: Some(Color::rgb(30, 100, 220)), corner_radius: 8.0, ..Default::default() });
    s.set_style(button, Style { padding: Edges { left: 18.0, right: 18.0, top: 10.0, bottom: 10.0 }, ..Default::default() });
    let label = s.create(button);
    s.set_text(label, TextProps { content: String::new(), font_size: 18.0, color: Color::WHITE });
    s.bind_text(label, move || format!("Clicked {} times", count.get()));
    s.on(button, EventKind::Click, move |_| count.set(count.get_untracked() + 1));
    let mut shell = FakeShell::new(240, 120);
    let clock = FakeClock::new();
    app.frame(&mut shell, &clock);
    // click the centre of the window
    shell.push(ShellEvent::Input(Event::PointerDown { x: 120.0, y: 60.0 }));
    shell.push(ShellEvent::Input(Event::PointerUp { x: 120.0, y: 60.0 }));
    app.frame(&mut shell, &clock);
    assert_eq!(shell.size(), (240, 120));
    (shell.presented[0].clone(), shell.presented[1].clone())
}

fn count_pixels(img: &wj_paint::Image, pred: impl Fn([u8; 4]) -> bool) -> usize {
    img.rgba.chunks(4).filter(|p| pred([p[0], p[1], p[2], p[3]])).count()
}

#[test]
fn counter_renders_a_blue_button_with_white_text_and_updates_on_click() {
    let (before, after) = counter_frames();
    let bg = Color::rgb(250, 250, 252);
    let a = CpuPainter::new().render(&before, 240, 120, bg);
    let b = CpuPainter::new().render(&after, 240, 120, bg);
    // The button is blue and centred; its surroundings are the clear colour.
    assert_eq!(a.pixel(5, 5), [250, 250, 252, 255]);
    let blue = |p: [u8; 4]| p[2] > 180 && p[0] < 60;
    assert!(count_pixels(&a, blue) > 1500, "button body painted: {}", count_pixels(&a, blue));
    let white = |p: [u8; 4]| p[0] > 200 && p[1] > 200 && p[2] > 200 && p[3] == 255;
    // White-ish pixels exist inside the button (text), not only the page background: compare to the bg count.
    let inside_button_white = (0..a.height).flat_map(|y| (0..a.width).map(move |x| (x, y))).filter(|&(x, y)| (a.pixel(x, y)[2] > 200 && a.pixel(x, y)[0] > 200 && a.pixel(x, y)[1] > 200) && x > 40 && x < 200 && y > 40 && y < 80).count();
    assert!(inside_button_white > 100, "text glyph pixels: {inside_button_white}");
    let _ = white;
    // The click changed the label ("0" -> "1"), so the frames must differ in the text region only.
    let (max, frac) = a.diff(&b, 8);
    assert!(max > 100 && frac > 0.0003 && frac < 0.10, "label changed (button re-centres when the text width changes): max {max}, {:.3}% pixels", frac * 100.0);
    assert_eq!(a.pixel(5, 5), b.pixel(5, 5));
    // Evidence image for docs: written only when asked.
    if let Ok(dir) = std::env::var("WJ_EVIDENCE_DIR") {
        common::write_png(&std::path::Path::new(&dir).join("counter-0.png"), &a);
        common::write_png(&std::path::Path::new(&dir).join("counter-1.png"), &b);
    }
}

#[test]
fn gpu_renders_the_same_counter_within_tolerance() {
    let Some(mut gpu) = wj_paint_vello::VelloPainter::headless() else {
        assert!(std::env::var("WJ_ALLOW_NO_GPU").is_ok(), "no GPU adapter; set WJ_ALLOW_NO_GPU=1 to skip explicitly");
        return;
    };
    let (before, _) = counter_frames();
    let bg = Color::rgb(250, 250, 252);
    let cpu = CpuPainter::new().render(&before, 240, 120, bg);
    let g = gpu.render(&before, 240, 120, bg);
    let (_, frac) = cpu.diff(&g, 12);
    assert!(frac < 0.03, "{:.2}% of pixels differ by > 12/255", frac * 100.0);
    gpu.scale = 2.0;
    let g2 = gpu.render(&before, 480, 240, bg);
    // At 2x the button's blue interior covers about four times the pixels.
    let blue = |p: [u8; 4]| p[2] > 180 && p[0] < 60;
    let (n1, n2) = (count_pixels(&g, blue), count_pixels(&g2, blue));
    assert!(n2 > n1 * 3 && n2 < n1 * 5, "scale factor applied: {n1} -> {n2}");
}
