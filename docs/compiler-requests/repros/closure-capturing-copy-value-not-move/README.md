# Two closures capture the same `Copy` value; only the last gets `move`, the first fails `'static`

```
run_later(|| println("value ${h.get()}"))   // emitted without `move`  -> E0373 (borrows `h`, needs 'static)
run_later(|| h.set(h.get() + 1))            // emitted with `move`
```
`Handle` is `#[derive(Clone, Copy)]` in the dependency, and `run_later` requires `'static`. Expected: both closures `move`
(copying the handle). Any UI/callback-registry API hits this as soon as one handle is used by two callbacks (this is the
`@handle` / Copy-like handle requirement, R1 in docs/VISION_2026.md).

Run: `WJ_BIN=/path/to/wj ./check.sh`. Compiler: `next/v0.50.0` at 8f70969e.
