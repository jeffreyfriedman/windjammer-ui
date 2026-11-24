# wj-shell-desktop (L4, macOS first)
winit 0.30 window, Vello on wgpu via `vello::util`, AccessKit winit adapter (actions: click, focus). `run(RunOptions, build)`
builds the app on the UI thread; `max_frames` + `on_frame` give measurement mode. Example: `examples/counter.rs`.
Windows and Linux use the same code path but are not exercised yet.
