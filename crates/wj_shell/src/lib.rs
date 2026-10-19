//! `wj-shell` (layer L4, platform-independent half): everything an app needs to run frames
//! without a window. A [`Shell`] supplies events and a surface; [`App`] owns the scene and runs the
//! frame pipeline (events, layout, paint, accessibility). [`FakeShell`] and [`FakeClock`] make
//! whole-app tests deterministic; platform shells (`wj-shell-desktop`) implement the same trait.

mod app;
mod clock;
mod fake;
mod shell;

pub use app::{App, FrameReport, WindowInfo};
pub use clock::{Clock, FakeClock, SystemClock};
pub use fake::FakeShell;
pub use shell::{Shell, ShellEvent, Theme};
