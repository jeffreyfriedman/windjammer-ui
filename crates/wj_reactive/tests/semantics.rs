//! Behavioural tests ported from the signal-bench spike plus scope, batch and memory gates.
use std::cell::Cell;
use std::rc::Rc;
use wj_reactive::*;

fn counter() -> (Rc<Cell<i32>>, Rc<Cell<i32>>) {
    let c = Rc::new(Cell::new(0));
    (c.clone(), c)
}

#[test]
fn diamond_is_glitch_free_and_runs_once() {
    reset();
    let a = state(1i32);
    let b = derived(move || a.get() + 1);
    let c = derived(move || a.get() * 2);
    let (runs, r) = counter();
    let seen = Rc::new(Cell::new((0, 0)));
    let s = seen.clone();
    effect(move || {
        r.set(r.get() + 1);
        s.set((b.get(), c.get()));
    });
    assert_eq!(runs.get(), 1);
    a.set(5);
    assert_eq!(runs.get(), 2, "effect ran exactly once per update");
    assert_eq!(seen.get(), (6, 10));
}

#[test]
fn equality_cutoff_stops_propagation() {
    reset();
    let a = state(3i32);
    let parity = derived(move || a.get() % 2);
    let (runs, r) = counter();
    effect(move || {
        r.set(r.get() + 1);
        let _ = parity.get();
    });
    a.set(5);
    assert_eq!(runs.get(), 1, "parity unchanged => effect not re-run");
    a.set(6);
    assert_eq!(runs.get(), 2);
}

#[test]
fn setting_an_equal_value_is_a_no_op() {
    reset();
    let a = state(1i32);
    let (runs, r) = counter();
    effect(move || {
        r.set(r.get() + 1);
        let _ = a.get();
    });
    a.set(1);
    assert_eq!(runs.get(), 1);
}

#[test]
fn dynamic_dependencies_are_dropped_and_collected() {
    reset();
    let flag = state(true);
    let x = state(1i32);
    let y = state(100i32);
    let m = derived(move || if flag.get() { x.get() } else { y.get() });
    let (runs, r) = counter();
    effect(move || {
        r.set(r.get() + 1);
        let _ = m.get();
    });
    y.set(101);
    assert_eq!(runs.get(), 1, "y not yet a dependency");
    flag.set(false);
    assert_eq!(runs.get(), 2);
    x.set(2);
    assert_eq!(runs.get(), 2, "x dropped");
    y.set(102);
    assert_eq!(runs.get(), 3);
}

#[test]
fn reads_outside_tracking_scopes_do_not_subscribe() {
    reset();
    let a = state(1i32);
    let (runs, r) = counter();
    effect(move || {
        r.set(r.get() + 1);
        let _ = untrack(|| a.get());
        let _ = a.get_untracked();
    });
    a.set(2);
    assert_eq!(runs.get(), 1);
}

#[test]
fn batch_runs_each_effect_once() {
    reset();
    let a = state(1i32);
    let b = state(1i32);
    let (runs, r) = counter();
    effect(move || {
        r.set(r.get() + 1);
        let _ = a.get() + b.get();
    });
    batch(|| {
        a.set(2);
        b.set(2);
        a.set(3);
    });
    assert_eq!(runs.get(), 2);
}

#[test]
fn nested_batches_flush_at_the_outermost_end() {
    reset();
    let a = state(0i32);
    let (runs, r) = counter();
    effect(move || {
        r.set(r.get() + 1);
        let _ = a.get();
    });
    batch(|| {
        batch(|| a.set(1));
        assert_eq!(runs.get(), 1, "inner batch end must not flush");
        a.set(2);
    });
    assert_eq!(runs.get(), 2);
}

#[test]
fn effects_that_write_cascade_to_a_fixed_point() {
    reset();
    let a = state(1i32);
    let doubled = state(0i32);
    effect(move || doubled.set(a.get() * 2));
    let seen = Rc::new(Cell::new(0));
    let s = seen.clone();
    effect(move || s.set(doubled.get()));
    a.set(21);
    assert_eq!(seen.get(), 42);
}

#[test]
fn update_mutates_in_place_and_always_notifies() {
    reset();
    let list = state(vec![1, 2]);
    let len = derived(move || list.with(|l| l.len()));
    list.update(|l| l.push(3));
    assert_eq!(len.get(), 3);
}

#[test]
fn handles_are_copy_and_capturable_without_clone() {
    reset();
    let n = state(0i32);
    let inc = move || n.set(n.get_untracked() + 1);
    let twice = move || {
        inc();
        inc();
    };
    twice();
    assert_eq!(n.get(), 2);
}

#[test]
fn scope_dispose_stops_effects_and_invalidates_handles() {
    reset();
    let a = state(1i32);
    let (runs, r) = counter();
    let (inner, scope) = create_scope(|| {
        effect(move || {
            r.set(r.get() + 1);
            let _ = a.get();
        });
        state(7i32)
    });
    a.set(2);
    assert_eq!(runs.get(), 2);
    assert!(inner.is_alive());
    scope.dispose();
    assert!(!inner.is_alive());
    assert_eq!(inner.try_get(), None);
    a.set(3);
    assert_eq!(runs.get(), 2, "disposed effect no longer runs");
}

#[test]
#[should_panic(expected = "after its scope was disposed")]
fn using_a_disposed_handle_panics_with_a_clear_message() {
    reset();
    let (s, scope) = create_scope(|| state(1i32));
    scope.dispose();
    let _ = s.get();
}

#[test]
fn nested_scopes_dispose_together() {
    reset();
    let ((child_sig, _), outer) = create_scope(|| create_scope(|| state(1i32)));
    assert!(child_sig.is_alive());
    outer.dispose();
    assert!(!child_sig.is_alive());
}

#[test]
fn dispose_removes_subscriptions_from_surviving_sources() {
    reset();
    let a = state(1i32);
    let (_, scope) = create_scope(|| {
        effect(move || {
            let _ = a.get();
        });
    });
    scope.dispose();
    let base = stats().live_nodes;
    assert_eq!(base, 1, "only the source remains");
    a.set(2); // must not touch freed slots
}

#[test]
fn stale_handle_does_not_alias_a_recycled_slot() {
    reset();
    let (old, s1) = create_scope(|| state(1i32));
    s1.dispose();
    let (_new, _s2) = create_scope(|| state(99i32)); // reuses the freed slot
    assert!(!old.is_alive(), "generation check rejects the stale handle");
}

/// Memory gate: churn of create+dispose keeps the arena bounded (slots are recycled).
#[test]
fn arena_is_bounded_under_create_dispose_churn() {
    reset();
    let src = state(0i32);
    for i in 0..20_000 {
        let (_, scope) = create_scope(|| {
            let m = derived(move || src.get() + i);
            effect(move || {
                let _ = m.get();
            });
        });
        scope.dispose();
    }
    let s = stats();
    assert_eq!(s.live_nodes, 1);
    assert!(s.slots <= 4, "slots recycled, got {}", s.slots);
    assert_eq!(s.live_scopes, 1);
}

#[test]
fn deep_chain_updates_in_one_pass() {
    reset();
    let a = state(0i64);
    let mut last = derived(move || a.get());
    for _ in 0..1000 {
        let prev = last;
        last = derived(move || prev.get() + 1);
    }
    let out = Rc::new(Cell::new(0i64));
    let o = out.clone();
    effect(move || o.set(last.get()));
    a.set(10);
    assert_eq!(out.get(), 1010);
}

#[test]
fn scope_run_adopts_nodes_created_later() {
    reset();
    let (_, scope) = create_scope(|| {});
    let m = scope.run(|| state(5i32));
    assert!(m.is_alive());
    scope.dispose();
    assert!(!m.is_alive());
}

/// Pinned behaviour: a batch that writes a signal and then restores its value still re-runs
/// direct dependents (dirty marking cannot know the value came back); memo cutoff does not apply
/// to the source itself.
#[test]
fn restoring_a_value_inside_a_batch_still_reruns_direct_readers() {
    reset();
    let a = state(1i32);
    let (runs, r) = counter();
    effect(move || {
        r.set(r.get() + 1);
        let _ = a.get();
    });
    batch(|| {
        a.set(2);
        a.set(1);
    });
    assert_eq!(runs.get(), 2);
}
