use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EventKind {
    Click,
    PointerDown,
    PointerUp,
    PointerMove,
    Wheel,
    Key,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    PointerDown { x: f32, y: f32 },
    PointerUp { x: f32, y: f32 },
    PointerMove { x: f32, y: f32 },
    Wheel { x: f32, y: f32, dx: f32, dy: f32 },
    Key { key: String, pressed: bool },
    /// Synthesized by the scene when down and up land on the same node.
    Click { x: f32, y: f32 },
}

impl Event {
    pub fn kind(&self) -> EventKind {
        match self {
            Event::PointerDown { .. } => EventKind::PointerDown,
            Event::PointerUp { .. } => EventKind::PointerUp,
            Event::PointerMove { .. } => EventKind::PointerMove,
            Event::Wheel { .. } => EventKind::Wheel,
            Event::Key { .. } => EventKind::Key,
            Event::Click { .. } => EventKind::Click,
        }
    }
}

/// Handlers run outside any tracking scope (reads do not subscribe) and inside a batch.
pub type Handler = Rc<dyn Fn(&Event)>;
