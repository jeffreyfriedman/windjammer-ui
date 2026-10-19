# Integer literals get a concrete suffix before the target type is known

```
let h = new_handle(0)          // emitted `new_handle(0_i32)`
let doubled = h.get() * 2      // emitted `h.get() * 2_i64`   (E0277: i32 * i64)
let r = Record { id: h.get() } // field is i64, value is i32  (E0308)
```
Return types of methods on dependency types are not visible to the literal inference, so the compiler picks i32 in one place
and i64 in another. Expected: leave unconstrained literals unsuffixed (`2`) so rustc infers them, or read the dependency's
signatures (they are available through the path dependency in `wj.toml`).

Run: `WJ_BIN=/path/to/wj ./check.sh`. Compiler: `next/v0.50.0` at 8f70969e.
