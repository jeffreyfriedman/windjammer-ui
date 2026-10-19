use wj_scene::{Align, Color, Dim, Direction, Edges, EventKind, Justify, NodeId, Paint, Role, Scene, Semantics, Style, TextProps};

use wj_reactive::Readable;

use crate::ctx::with_scene;

/// A node in the scene. `Copy`: capture it in closures freely.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Node {
    scene: Scene,
    id: NodeId,
}

fn make(f: impl FnOnce(Scene, NodeId)) -> Node {
    with_scene(|scene| {
        let id = scene.create_detached();
        f(scene, id);
        Node { scene, id }
    })
}

/// Vertical stack.
pub fn column() -> Node {
    make(|s, id| s.set_style(id, Style { direction: Direction::Column, align_items: Align::Start, ..Default::default() }))
}

/// Horizontal stack.
pub fn row() -> Node {
    make(|s, id| s.set_style(id, Style { direction: Direction::Row, align_items: Align::Center, ..Default::default() }))
}

/// Static text.
pub fn text(content: &str) -> Node {
    make(|s, id| {
        s.set_text(id, TextProps::new(content));
    })
}

/// Section heading (announced as a heading to assistive technology).
pub fn heading(content: &str) -> Node {
    make(|s, id| {
        let mut t = TextProps::new(content);
        t.font_size = 22.0;
        s.set_text(id, t);
        s.set_semantics(id, Semantics { role: Role::Heading, ..Default::default() });
    })
}

/// A button with a text label.
pub fn button(label: &str) -> Node {
    let b = make(|s, id| {
        s.set_semantics(id, Semantics { role: Role::Button, focusable: true, ..Default::default() });
        s.set_paint(id, Paint { background: Some(Color::rgb(30, 100, 220)), corner_radius: 8.0, ..Default::default() });
        s.set_style(id, Style { padding: Edges { left: 16.0, right: 16.0, top: 8.0, bottom: 8.0 }, ..Default::default() });
    });
    let t = text(label).color(255, 255, 255);
    b.child(t)
}

/// Text that shows a signal or memo (`text_of(count)`): no closure needed.
pub fn text_of<T: std::fmt::Display + Clone + 'static>(source: impl Readable<T>) -> Node {
    text("").text_fn(move || source.read().to_string())
}

/// A button whose label is reactive: `label` re-runs when the signals it reads change.
pub fn button_fn(label: impl FnMut() -> String + 'static) -> Node {
    let b = button("");
    let t = b.first_child().expect("button has a label child");
    t.text_fn(label);
    b
}

/// A checkbox with a visible label. `checked` is read reactively.
pub fn checkbox(label: &str, checked: impl Fn() -> bool + 'static) -> Node {
    let c = make(|s, id| s.set_style(id, Style { direction: Direction::Row, align_items: Align::Center, gap: 8.0, ..Default::default() }));
    let scene = c.scene;
    let id = c.id;
    let name = label.to_string();
    scene.bind(id, move || {
        scene.set_semantics(id, Semantics { role: Role::Checkbox, name: Some(name.clone()), checked: Some(checked()), focusable: true, ..Default::default() });
    });
    c.child(text(label))
}

/// A single-line text field whose value is read reactively. (Editing arrives with the text-input work.)
pub fn text_field(name: &str, value: impl Fn() -> String + 'static) -> Node {
    let f = make(|s, id| {
        s.set_paint(id, Paint { background: Some(Color::rgb(255, 255, 255)), corner_radius: 6.0, border_width: 1.0, border_color: Color::rgb(190, 195, 205), ..Default::default() });
        s.set_style(id, Style { padding: Edges::all(8.0), width: Dim::Px(220.0), ..Default::default() });
    });
    let scene = f.scene;
    let id = f.id;
    let nm = name.to_string();
    let value = std::rc::Rc::new(value);
    let for_semantics = value.clone();
    scene.bind(id, move || {
        scene.set_semantics(id, Semantics { role: Role::TextField, name: Some(nm.clone()), value: Some(for_semantics()), focusable: true, ..Default::default() });
    });
    f.child(text("").text_fn(move || value()))
}

impl Node {
    pub fn id(&self) -> NodeId {
        self.id
    }

    /// The root of the scene (the window content).
    pub fn root() -> Node {
        with_scene(|scene| Node { scene, id: scene.root() })
    }

    fn style(self, f: impl FnOnce(&mut Style)) -> Node {
        let mut st = self.scene.style(self.id);
        f(&mut st);
        self.scene.set_style(self.id, st);
        self
    }

    fn paint(self, f: impl FnOnce(&mut Paint)) -> Node {
        let mut p = self.scene.paint(self.id);
        f(&mut p);
        self.scene.set_paint(self.id, p);
        self
    }

    pub fn gap(self, v: f64) -> Node {
        self.style(|s| s.gap = v as f32)
    }
    pub fn padding(self, v: f64) -> Node {
        self.style(|s| s.padding = Edges::all(v as f32))
    }
    pub fn width(self, v: f64) -> Node {
        self.style(|s| s.width = Dim::Px(v as f32))
    }
    pub fn height(self, v: f64) -> Node {
        self.style(|s| s.height = Dim::Px(v as f32))
    }
    pub fn grow(self, v: f64) -> Node {
        self.style(|s| s.flex_grow = v as f32)
    }
    /// Centre children on both axes.
    pub fn center(self) -> Node {
        self.style(|s| {
            s.align_items = Align::Center;
            s.justify = Justify::Center;
        })
    }
    pub fn background(self, r: i64, g: i64, b: i64) -> Node {
        self.paint(|p| p.background = Some(Color::rgb(r as u8, g as u8, b as u8)))
    }
    pub fn radius(self, v: f64) -> Node {
        self.paint(|p| p.corner_radius = v as f32)
    }
    /// Text colour.
    pub fn color(self, r: i64, g: i64, b: i64) -> Node {
        let mut t = self.scene.text(self.id).unwrap_or_else(|| TextProps::new(""));
        t.color = Color::rgb(r as u8, g as u8, b as u8);
        self.scene.set_text(self.id, t);
        self
    }
    pub fn text_size(self, v: f64) -> Node {
        let mut t = self.scene.text(self.id).unwrap_or_else(|| TextProps::new(""));
        t.font_size = v as f32;
        self.scene.set_text(self.id, t);
        self
    }
    /// Accessible name (overrides text-derived names).
    pub fn label(self, name: &str) -> Node {
        let mut s = self.scene.semantics(self.id);
        s.name = Some(name.to_string());
        self.scene.set_semantics(self.id, s);
        self
    }

    pub(crate) fn first_child(self) -> Option<Node> {
        self.scene.children(self.id).first().map(|id| Node { scene: self.scene, id: *id })
    }

    pub fn child(self, c: Node) -> Node {
        self.scene.append(self.id, c.id);
        self
    }

    /// Reactive text: `f` re-runs when the signals it reads change.
    pub fn text_fn(self, f: impl FnMut() -> String + 'static) -> Node {
        if self.scene.text(self.id).is_none() {
            self.scene.set_text(self.id, TextProps::new(""));
        }
        self.scene.bind_text(self.id, f);
        self
    }

    /// Reactive background colour (`None` clears it).
    pub fn background_fn(self, f: impl FnMut() -> Option<(i64, i64, i64)> + 'static) -> Node {
        let mut f = f;
        self.scene.bind_background(self.id, move || f().map(|(r, g, b)| Color::rgb(r as u8, g as u8, b as u8)));
        self
    }

    /// Run `f` when the node is clicked (or activated by assistive technology). Reads inside do not subscribe.
    pub fn on_click(self, f: impl Fn() + 'static) -> Node {
        self.scene.on(self.id, EventKind::Click, move |_| f());
        self
    }

    /// Attach this node to the window root.
    pub fn mount(self) -> Node {
        let root = Node::root();
        root.child(self)
    }
}
