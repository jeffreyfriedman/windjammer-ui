# Agent Instructions (Windjammer UI)

**Read the windjammer-engineering skill before starting work:**

`/Users/jeffreyfriedman/src/wj/.cursor/skills/windjammer-engineering/SKILL.md`

When this repo is opened with the full `wj` workspace, the skill is also at `../.cursor/skills/windjammer-engineering/SKILL.md`.

## Build

Follow project README for UI build steps. Compiler or engine bugs → fix in `../windjammer/tests/` first (compiler-first protocol).



## Dogfood rules (apply to every Windjammer project repo)

1. **Compiler-first.** If `wj` generates wrong or non-compiling Rust, file a minimal, generic repro in the compiler repo's `tests/` and fix it there. Never add shims (`+ ""`, spurious `.clone()`, `.as_str()`), never hand-edit generated Rust, never restructure idiomatic code to dodge a codegen bug.
2. **No compiler bug tracking here.** Status/"tip RED/GREEN" commits, repro queues and compiler incident docs live in the compiler repo only.
3. **Pin the compiler.** Record the known-good `wj` commit in `WJ_PIN.md`; bump deliberately, in its own commit.
4. **Generated output is never source.** Don't commit `build/`, `gen/` or transpiled `.rs` unless this repo's section below says otherwise; never edit them by hand.
5. **No status-report files.** No `*_COMPLETE.md`, `SESSION_SUMMARY*`, `*_STATUS.md` in the repo; record decisions in `docs/` ADRs and progress in commit messages.
6. **Real tests only.** Each feature needs a behavior test; no padding with trivial predicate/getter functions or placeholder `-> true` stubs.
7. **Races.** Other agents may be working: run `git status` before committing, stage only your own paths, never `git add -A` blindly, never force-push.
8. **Disk.** Use the shared target dir (`export CARGO_TARGET_DIR="$(wj cache path)"`); run `wj cache prune` when free disk is low; tests must write only to temp dirs.
9. **Claims need evidence.** Don't write "working/complete" without a command output or screenshot that proves it.

## UI framework rules
- Output is **escaped by default**; raw HTML requires an explicit `RawHtml` type.
- Components are generic. Domain components (checkbook, period lock, SoD) live in the consuming app (`finance-screens`), not here.
- Components generate from `src/components_wj/*.wj`; `src/components/generated/` is never hand-maintained. Don't use `SKIP_WJ_REGEN` to hide divergence.
- Behavior is written in Windjammer (signals/closures), not as JS strings embedded in Rust.
- Every component ships a rendering test; interactive components need a wasm/DOM test. No `.disabled` tests committed.
- README claims (platforms, component counts, "production ready") must match reality.
