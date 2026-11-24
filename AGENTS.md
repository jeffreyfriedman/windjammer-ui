# Agent Instructions (Windjammer UI)

## Build

See the README for build steps. Compiler bugs are fixed in the compiler repository, with a minimal repro in its `tests/`
(compiler-first protocol).

## Dogfood rules (apply to every Windjammer project repo)

1. **Compiler-first.** If `wj` generates wrong or non-compiling Rust, file a minimal, generic repro in the compiler repo's `tests/` and fix it there. Never add shims (`+ ""`, spurious `.clone()`, `.as_str()`), never hand-edit generated Rust, never restructure idiomatic code to dodge a codegen bug.
2. **No compiler bug tracking here.** Status commits, repro queues and compiler incident docs live in the compiler repo only.
3. **Pin the compiler.** Record the known-good `wj` version in a pin file; bump deliberately, in its own commit.
4. **Generated output is never source.** Don't commit `build/`, `gen/` or transpiled `.rs` unless this repo's section below says otherwise; never edit them by hand.
5. **No status-report files.** No `*_COMPLETE.md`, `SESSION_SUMMARY*`, `*_STATUS.md` in the repo; record decisions in `docs/` ADRs and progress in commit messages.
6. **Real tests only.** Each feature needs a behavior test; no padding with trivial predicate/getter functions or `-> true` stubs.
7. **Parallel work.** Other contributors may be working: check `git status` before committing, stage only your own paths, never `git add -A` blindly, never force-push.
8. **Disk.** Use a shared Cargo target dir (`export CARGO_TARGET_DIR="$(wj cache path)"`); run `wj cache prune` when free disk is low; tests write only to temp dirs.
9. **Claims need evidence.** Don't write "working/complete" without a command output or screenshot that proves it.

## UI framework rules
- Output is **escaped by default**; raw HTML requires an explicit `RawHtml` type.
- Components are generic. Domain components (for example accounting or game-HUD specific) live in the consuming app, not here.
- Components generate from `src/components_wj/*.wj`; `src/components/generated/` is never hand-maintained. Don't use `SKIP_WJ_REGEN` to hide divergence.
- Behavior is written in Windjammer (signals/closures), not as JS strings embedded in Rust.
- Every component ships a rendering test; interactive components need a wasm/DOM test. No `.disabled` tests committed.
- README claims (platforms, component counts) must match reality.
