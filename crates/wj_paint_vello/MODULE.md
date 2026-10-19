# wj-paint-vello
`VelloPainter`: `Painter` over Vello 0.11 / wgpu 30. `headless()` makes its own device and reads pixels back;
`with_device` + `render_to_texture` is the windowed path (`scale` = device pixels per list unit). `encode` is public so shells can time it.
