use wj_paint::DrawCmd;
use wj_reactive::{reset, state, Signal};
use wj_scene::*;
use wj_shell::*;

struct Counter {
    app: App,
    count: Signal<i32>,
}

fn counter() -> Counter {
    reset();
    let app = App::new();
    let s = app.scene;
    s.set_style(s.root(), Style { align_items: Align::Start, padding: Edges::all(8.0), ..Default::default() });
    let count = state(0);
    let button = s.create(s.root());
    s.set_semantics(button, Semantics { role: Role::Button, focusable: true, ..Default::default() });
    s.set_paint(button, Paint { background: Some(Color::rgb(30, 100, 220)), corner_radius: 6.0, ..Default::default() });
    s.set_style(button, Style { padding: Edges::all(6.0), ..Default::default() });
    let label = s.create(button);
    s.bind_text(label, move || format!("Clicked {} times", count.get()));
    s.on(button, EventKind::Click, move |_| count.set(count.get_untracked() + 1));
    Counter { app, count }
}

fn glyphs(list: &wj_paint::DisplayList) -> usize {
    list.cmds.iter().filter_map(|c| if let DrawCmd::Text { run } = c { Some(run.glyphs.len()) } else { None }).sum()
}

#[test]
fn counter_app_runs_frames_headlessly() {
    let mut c = counter();
    let mut shell = FakeShell::new(320, 200);
    let clock = FakeClock::new();
    let r0 = c.app.frame(&mut shell, &clock);
    assert!(!r0.skipped && r0.display_list_len > 0 && r0.a11y_nodes_sent > 0);
    assert_eq!(shell.presented.len(), 1);
    let g0 = glyphs(&shell.presented[0]);
    assert_eq!(g0, "Clicked 0 times".chars().count());

    // idle frame: nothing to do
    let r1 = c.app.frame(&mut shell, &clock);
    assert!(r1.skipped);
    assert_eq!(shell.presented.len(), 1);

    // click the button (it sits at the root's padding origin)
    shell.push(ShellEvent::Input(Event::PointerDown { x: 12.0, y: 12.0 }));
    shell.push(ShellEvent::Input(Event::PointerUp { x: 12.0, y: 12.0 }));
    let r2 = c.app.frame(&mut shell, &clock);
    assert_eq!(c.count.get(), 1);
    assert!(!r2.skipped);
    assert_eq!(shell.presented.len(), 2);
    assert_eq!(glyphs(&shell.presented[1]), "Clicked 1 times".chars().count());
    // the button's accessible name changed: exactly that node (the button) is re-sent
    assert_eq!(shell.a11y.len(), 2);
    assert_eq!(r2.a11y_nodes_sent, 1, "{:?}", shell.a11y[1].nodes.iter().map(|(i, _)| i).collect::<Vec<_>>());
}

#[test]
fn replaying_the_event_log_reproduces_every_frame_exactly() {
    let run = |script: Option<Vec<ShellEvent>>| {
        let mut c = counter();
        let mut shell = FakeShell::new(320, 200);
        let clock = FakeClock::new();
        c.app.frame(&mut shell, &clock);
        let frames: Vec<Vec<ShellEvent>> = match script {
            Some(log) => log.chunks(2).map(|c| c.to_vec()).collect(),
            None => (0..5).map(|i| vec![ShellEvent::Input(Event::PointerDown { x: 12.0, y: 12.0 }), ShellEvent::Input(Event::PointerUp { x: 12.0 + i as f32 * 0.0, y: 12.0 })]).collect(),
        };
        for evs in frames {
            for e in evs {
                shell.push(e);
            }
            c.app.frame(&mut shell, &clock);
            clock.advance(16.0);
        }
        (shell.log.clone(), shell.presented.clone(), shell.a11y.iter().map(|u| u.nodes.len()).collect::<Vec<_>>())
    };
    let (log, frames_a, a11y_a) = run(None);
    assert_eq!(log.len(), 10);
    let (_, frames_b, a11y_b) = run(Some(log));
    assert_eq!(frames_a, frames_b, "display lists identical on replay");
    assert_eq!(a11y_a, a11y_b);
    assert_eq!(frames_a.len(), 6);
}

#[test]
fn resize_and_theme_flow_into_window_signals() {
    let mut c = counter();
    let mut shell = FakeShell::new(200, 100);
    let clock = FakeClock::new();
    c.app.frame(&mut shell, &clock);
    assert_eq!((c.app.window.width.get(), c.app.window.height.get()), (200.0, 100.0));
    let seen = std::rc::Rc::new(std::cell::Cell::new(0.0f32));
    let s2 = seen.clone();
    let w = c.app.window;
    wj_reactive::effect(move || s2.set(w.width.get() * w.scale.get()));
    shell.push(ShellEvent::Resize { width: 800, height: 600, scale: 2.0 });
    shell.push(ShellEvent::Theme(Theme::Dark));
    let r = c.app.frame(&mut shell, &clock);
    assert!(!r.skipped, "a resize repaints even when no node changed");
    assert_eq!((c.app.window.width.get(), c.app.window.height.get(), c.app.window.scale.get()), (400.0, 300.0, 2.0));
    assert!(c.app.window.dark.get());
    assert_eq!(seen.get(), 800.0);
}

#[test]
fn close_request_is_reported() {
    let mut c = counter();
    let mut shell = FakeShell::new(100, 100);
    shell.push(ShellEvent::CloseRequested);
    assert!(c.app.frame(&mut shell, &FakeClock::new()).close_requested);
}

#[test]
fn fake_clock_only_moves_when_told() {
    let c = FakeClock::new();
    assert_eq!(c.now_ms(), 0.0);
    c.advance(16.5);
    c.advance(0.5);
    assert_eq!(c.now_ms(), 17.0);
}
