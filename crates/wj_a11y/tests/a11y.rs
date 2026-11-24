use accesskit::{Action, Role as AkRole, Toggled};
use wj_a11y::*;
use wj_layout::{FixedAdvanceMeasure, LayoutEngine};
use wj_scene::{Role as SRole, *};

/// The five-widget demo from VISION M0: heading, text field, checkbox, button, list of two.
struct Demo {
    scene: Scene,
    button: NodeId,
    checkbox: NodeId,
    field: NodeId,
}

fn text(s: Scene, parent: NodeId, t: &str) -> NodeId {
    let n = s.create(parent);
    s.set_text(n, TextProps::new(t));
    n
}

fn demo() -> Demo {
    wj_reactive::reset();
    let s = Scene::new();
    let root = s.root();
    let h = s.create(root);
    s.set_semantics(h, Semantics { role: SRole::Heading, ..Default::default() });
    text(s, h, "Settings");
    let field = s.create(root);
    s.set_semantics(field, Semantics { role: SRole::TextField, name: Some("Display name".into()), value: Some("Ada".into()), focusable: true, ..Default::default() });
    let checkbox = s.create(root);
    s.set_semantics(checkbox, Semantics { role: SRole::Checkbox, name: Some("Dark mode".into()), checked: Some(false), focusable: true, ..Default::default() });
    s.on(checkbox, EventKind::Click, |_| {});
    let button = s.create(root);
    s.set_semantics(button, Semantics { role: SRole::Button, focusable: true, ..Default::default() });
    text(s, button, "Save");
    s.on(button, EventKind::Click, |_| {});
    let list = s.create(root);
    s.set_semantics(list, Semantics { role: SRole::List, name: Some("Recent".into()), ..Default::default() });
    for t in ["one", "two"] {
        let li = s.create(list);
        s.set_semantics(li, Semantics { role: SRole::ListItem, ..Default::default() });
        text(s, li, t);
    }
    let mut e = LayoutEngine::new();
    let d = s.drain_dirty();
    e.update(s, &d, (200.0, 200.0), &FixedAdvanceMeasure);
    Demo { scene: s, button, checkbox, field }
}


#[test]
fn demo_passes_the_semantics_audit() {
    assert_eq!(demo().scene.audit_semantics(), Vec::<String>::new());
}

#[test]
fn first_update_describes_the_whole_demo() {
    let d = demo();
    let mut ak = AccessKitTree::new();
    let u = ak.update(&d.scene.semantic_tree(), None).expect("first update");
    assert!(u.tree.is_some(), "first update carries tree info");
    assert_eq!(u.focus, ROOT_ID);
    let by_role = |r: AkRole| u.nodes.iter().filter(|(_, n)| n.role() == r).count();
    assert_eq!(by_role(AkRole::Window), 1);
    assert_eq!(by_role(AkRole::Heading), 1);
    assert_eq!(by_role(AkRole::TextInput), 1);
    assert_eq!(by_role(AkRole::CheckBox), 1);
    assert_eq!(by_role(AkRole::Button), 1);
    assert_eq!(by_role(AkRole::List), 1);
    assert_eq!(by_role(AkRole::ListItem), 2);
    let btn = u.nodes.iter().find(|(_, n)| n.role() == AkRole::Button).unwrap();
    assert_eq!(btn.1.label(), Some("Save"));
    assert!(btn.1.supports_action(Action::Click) && btn.1.supports_action(Action::Focus));
    let cb = u.nodes.iter().find(|(_, n)| n.role() == AkRole::CheckBox).unwrap();
    assert_eq!(cb.1.toggled(), Some(Toggled::False));
    assert_eq!(cb.1.label(), Some("Dark mode"));
    let tf = u.nodes.iter().find(|(_, n)| n.role() == AkRole::TextInput).unwrap();
    assert_eq!(tf.1.value(), Some("Ada"));
    // every non-root node is some node's child (a valid tree)
    for (id, _) in u.nodes.iter().filter(|(id, _)| *id != ROOT_ID) {
        assert!(u.nodes.iter().any(|(_, n)| n.children().contains(id)), "{id:?} orphaned");
    }
}

#[test]
fn unchanged_tree_sends_nothing_and_a_change_sends_only_that_node() {
    let d = demo();
    let mut ak = AccessKitTree::new();
    ak.update(&d.scene.semantic_tree(), None).unwrap();
    assert!(ak.update(&d.scene.semantic_tree(), None).is_none(), "no change, no update");
    d.scene.set_semantics(d.checkbox, Semantics { role: SRole::Checkbox, name: Some("Dark mode".into()), checked: Some(true), focusable: true, ..Default::default() });
    let u = ak.update(&d.scene.semantic_tree(), None).unwrap();
    assert_eq!(u.nodes.len(), 1, "only the toggled checkbox: {:?}", u.nodes.iter().map(|(i, _)| i).collect::<Vec<_>>());
    assert_eq!(u.nodes[0].1.toggled(), Some(Toggled::True));
    assert!(u.tree.is_none());
}

#[test]
fn focus_changes_are_sent_even_when_nodes_are_unchanged() {
    let d = demo();
    let mut ak = AccessKitTree::new();
    ak.update(&d.scene.semantic_tree(), None).unwrap();
    let u = ak.update(&d.scene.semantic_tree(), Some(d.field)).expect("focus change");
    assert!(u.nodes.is_empty());
    assert_ne!(u.focus, ROOT_ID);
}

#[test]
fn removed_nodes_disappear_from_the_known_set() {
    let d = demo();
    let mut ak = AccessKitTree::new();
    ak.update(&d.scene.semantic_tree(), None).unwrap();
    let before = ak.known();
    d.scene.remove(d.button);
    let u = ak.update(&d.scene.semantic_tree(), None).unwrap();
    assert_eq!(ak.known(), before - 1);
    assert!(u.nodes.iter().any(|(id, n)| *id == ROOT_ID && n.children().len() == 4), "window lost a child");
    assert!(!u.nodes.iter().any(|(_, n)| n.role() == AkRole::Button));
}

#[test]
fn dom_projection_is_semantic_positioned_and_escaped() {
    let d = demo();
    let html = project(&d.scene.semantic_tree()).to_html();
    for want in ["<h2", ">Settings</h2>", "<input", "aria-label=\"Display name\"", "value=\"Ada\"", "type=\"checkbox\"", "<button", ">Save</button>", "<ul", "<li"] {
        assert!(html.contains(want), "missing {want:?} in {html}");
    }
    assert!(html.contains("position:absolute;left:"), "mirror is positioned over the canvas");
}

#[test]
fn dom_projection_escapes_hostile_names() {
    wj_reactive::reset();
    let s = Scene::new();
    let b = s.create(s.root());
    s.set_semantics(b, Semantics { role: SRole::Button, name: Some("<img src=x onerror=alert(1)>\"'&".into()), ..Default::default() });
    let html = project(&s.semantic_tree()).to_html();
    assert!(!html.contains("<img"), "{html}");
    assert!(html.contains("&lt;img src=x onerror=alert(1)&gt;&quot;&#39;&amp;"), "{html}");
}
