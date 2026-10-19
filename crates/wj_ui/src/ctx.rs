use std::cell::Cell;

use wj_scene::Scene;
use wj_shell::App;

thread_local! {
    static CURRENT: Cell<Option<Scene>> = const { Cell::new(None) };
}

/// Make `app`'s scene the target of `column()`, `text()`, ... on this thread.
pub fn use_app(app: &App) {
    CURRENT.with(|c| c.set(Some(app.scene)));
}

/// The scene `column()` and friends build into. Panics with a clear message when `use_app` was not called.
pub fn with_scene<R>(f: impl FnOnce(Scene) -> R) -> R {
    let s = CURRENT.with(|c| c.get()).expect("wj_ui: call use_app(&app) before building views");
    f(s)
}
