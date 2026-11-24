//! `Selector`: O(1) "is this key the selected one" reads. Changing the selection re-runs only the
//! readers of the previously and newly selected keys, not every row.
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use crate::{batch, effect, state, untrack, Signal};

struct Inner<K> {
    flags: HashMap<K, Signal<bool>>,
    current: Option<K>,
    purge_at: usize,
}

pub struct Selector<K> {
    inner: Rc<RefCell<Inner<K>>>,
}

impl<K> Clone for Selector<K> {
    fn clone(&self) -> Self {
        Selector { inner: self.inner.clone() }
    }
}

impl<K: Hash + Eq + Clone + 'static> Selector<K> {
    /// `source` is tracked; the selector follows it.
    pub fn new(mut source: impl FnMut() -> Option<K> + 'static) -> Self {
        let inner = Rc::new(RefCell::new(Inner { flags: HashMap::new(), current: None, purge_at: 64 }));
        let i2 = inner.clone();
        effect(move || {
            let next = source();
            let (prev, ps, ns) = {
                let mut b = i2.borrow_mut();
                if b.current == next {
                    return;
                }
                let prev = std::mem::replace(&mut b.current, next.clone());
                if b.flags.len() >= b.purge_at {
                    b.flags.retain(|_, s| s.is_alive());
                    b.purge_at = (b.flags.len() * 2).max(64);
                }
                let ps = prev.as_ref().and_then(|k| b.flags.get(k).copied());
                let ns = next.as_ref().and_then(|k| b.flags.get(k).copied());
                (prev, ps, ns)
            };
            let _ = prev;
            untrack(|| {
                batch(|| {
                    if let Some(s) = ps.filter(|s| s.is_alive()) {
                        s.set(false);
                    }
                    if let Some(s) = ns.filter(|s| s.is_alive()) {
                        s.set(true);
                    }
                })
            });
        });
        Selector { inner }
    }

    /// Tracked read: true while `key` is the selected key.
    pub fn is_selected(&self, key: &K) -> bool {
        let sig = {
            let b = self.inner.borrow();
            match b.flags.get(key).copied().filter(|s| s.is_alive()) {
                Some(s) => s,
                None => {
                    let init = b.current.as_ref() == Some(key);
                    drop(b);
                    let s = state(init);
                    self.inner.borrow_mut().flags.insert(key.clone(), s);
                    s
                }
            }
        };
        sig.get()
    }
}
