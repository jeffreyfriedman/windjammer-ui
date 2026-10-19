//! Behaviour of the Windjammer samples, driven through the real frame pipeline with FakeShell.
use wj_scene::*;
use wj_shell::*;

fn boot(build: impl FnOnce()) -> (App, FakeShell, FakeClock) {
    wj_reactive::reset();
    let mut app = App::new();
    wj_ui::use_app(&app);
    build();
    let mut shell = FakeShell::new(320, 240);
    let clock = FakeClock::new();
    app.frame(&mut shell, &clock);
    (app, shell, clock)
}

fn find(tree: &SemanticTree, role: Role, name: &str) -> SemanticNode {
    let mut found = None;
    tree.walk(&mut |n| {
        if n.role == role && n.name.as_deref() == Some(name) {
            found = Some(n.clone());
        }
    });
    found.unwrap_or_else(|| panic!("no {role:?} named {name:?} in {tree:#?}"))
}

fn click(shell: &mut FakeShell, n: &SemanticNode) {
    let (x, y) = (n.bounds.x + n.bounds.w / 2.0, n.bounds.y + n.bounds.h / 2.0);
    shell.push(ShellEvent::Input(Event::PointerDown { x, y }));
    shell.push(ShellEvent::Input(Event::PointerUp { x, y }));
}

#[test]
fn counter_sample_counts_clicks_and_announces_the_new_value() {
    let (mut app, mut shell, clock) = boot(wj_samples::counter::counter);
    assert_eq!(app.scene.audit_semantics(), Vec::<String>::new());
    let tree = app.scene.semantic_tree();
    find(&tree, Role::Text, "0");
    let add = find(&tree, Role::Button, "Add one");
    for expected in 1..=3 {
        click(&mut shell, &add);
        app.frame(&mut shell, &clock);
        find(&app.scene.semantic_tree(), Role::Text, &expected.to_string());
    }
    assert_eq!(shell.presented.len(), 4);
    let mut names = vec![];
    app.scene.semantic_tree().walk(&mut |n| names.push(n.name.clone().unwrap_or_default()));
    assert!(!names.contains(&"0".to_string()), "{names:?}");
}

#[test]
fn counter_sample_is_centred_by_the_layout_engine() {
    let (app, _shell, _clock) = boot(wj_samples::counter::counter);
    let tree = app.scene.semantic_tree();
    let add = find(&tree, Role::Button, "Add one");
    let cx = add.bounds.x + add.bounds.w / 2.0;
    assert!((cx - 160.0).abs() < 1.0, "button centred horizontally in a 320px window: {cx}");
    let label = find(&tree, Role::Text, "0");
    assert!(label.bounds.y < add.bounds.y, "label above the button (column order)");
}

#[test]
fn profile_card_sample_exposes_heading_text_and_two_buttons() {
    let (app, _s, _c) = boot(wj_samples::profile_card::profile_card);
    assert_eq!(app.scene.audit_semantics(), Vec::<String>::new());
    let tree = app.scene.semantic_tree();
    find(&tree, Role::Heading, "Ada Lovelace");
    find(&tree, Role::Text, "Analyst and writer");
    let follow = find(&tree, Role::Button, "Follow");
    let message = find(&tree, Role::Button, "Message");
    assert!(follow.bounds.x < message.bounds.x && (follow.bounds.y - message.bounds.y).abs() < 1.0, "buttons share a row");
    let html = wj_a11y::project(&tree).to_html();
    assert!(html.contains("<h2") && html.contains(">Follow</button>"));
}
