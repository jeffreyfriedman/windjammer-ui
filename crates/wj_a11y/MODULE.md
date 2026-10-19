# wj-a11y (L3)
`SemanticTree` -> AccessKit `TreeUpdate`s (`AccessKitTree` keeps what was sent and emits minimal updates; `reset()` when the
platform adapter reactivates) and -> `ProjectedNode` DOM mirror (`project`, `to_html`, escape-by-default) for the web host.
`scene_parts` maps AccessKit ids back to scene nodes for action requests. Tests use the five-widget M0 demo.
