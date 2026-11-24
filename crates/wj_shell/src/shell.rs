use accesskit::TreeUpdate;
use wj_paint::DisplayList;
use wj_scene::Event;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

/// Everything a platform can tell the app.
#[derive(Clone, Debug, PartialEq)]
pub enum ShellEvent {
    Input(Event),
    Resize { width: u32, height: u32, scale: f32 },
    Theme(Theme),
    CloseRequested,
}

/// One implementation per platform. Everything above this trait is shared.
pub trait Shell {
    /// Current drawable size in physical pixels and the device scale factor.
    fn size(&self) -> (u32, u32);
    fn scale(&self) -> f32;
    /// Drain pending events without blocking.
    fn poll_events(&mut self) -> Vec<ShellEvent>;
    /// Draw a frame.
    fn present(&mut self, list: &DisplayList, width: u32, height: u32);
    /// Forward an accessibility update to the platform (AccessKit adapter, or the DOM projection on web).
    fn a11y_update(&mut self, update: TreeUpdate);
}
