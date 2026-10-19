# wj-reactive (L0)

Arena-backed reactive core. No platform imports; runs headless, in tests and in the interpreter.

## Contract
- `state(v)` -> `Signal<T>`; `derived(f)` -> `Memo<T>`; `effect(f)`; `batch(f)`; `untrack(f)`.
- Handles are `Copy` (`index + generation`). Reading a handle whose scope was disposed panics with a clear
  message; `try_get` / `try_with` / `is_alive` are the fallible forms. Slots are recycled under a new generation,
  so a stale handle never aliases a new node.
- Reads inside `derived` / `effect` bodies are tracked; reads anywhere else (handlers) are not.
- Writes with an equal value are dropped (`set` needs `PartialEq`); `update` / `set_always` always notify.
- Glitch-free: every effect runs at most once per update or `batch`, after all its sources settled.
- Scopes (`create_scope`, `Scope::run`, `Scope::dispose`) own the nodes created inside them and dispose them together.
- Single-threaded: the runtime is thread-local. `resource` (async) is not part of this crate yet.

## Files
- `runtime.rs`: arena, three-colour propagation (clean / check / dirty), batching, effect queue.
- `handles.rs`: `Signal` / `Memo` handle types and their read/write methods.
- `scope.rs`: scope arena and disposal.

## Tests
- `tests/semantics.rs`: ported spike cases, scopes, batching, stale handles, memory gate (create+dispose churn keeps
  the arena bounded).
- `tests/model.rs`: 10,000 random graphs (sources, memos with dynamic dependencies, effects) checked against a naive
  recompute-everything oracle: effect run counts, observed values (no glitches) and memo values.
  Mutation check: removing equality cutoff or unsubscribing from dropped sources makes this test fail.
