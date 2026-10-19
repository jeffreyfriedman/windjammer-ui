# wj-shell (L4, portable half)
`Shell` trait (size, scale, events, present, a11y_update), `Clock`/`FakeClock`, `FakeShell` (scripted input, records frames and
a11y updates, keeps a replayable event log) and `App`, the frame pipeline: events -> layout -> display list -> present -> a11y.
Idle frames are skipped. `WindowInfo` exposes size/scale/theme as signals. Replaying a recorded event log reproduces every frame exactly.
