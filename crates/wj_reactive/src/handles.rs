//! Copy handles. A handle is `(index, generation)`; using one after its scope was disposed panics
//! with a clear message instead of reading recycled state (see `try_get` for the fallible form).
use std::cell::RefCell;
use std::marker::PhantomData;

use crate::runtime::{free_node, read_value, with_rt, write_notify};

macro_rules! handle {
    ($name:ident) => {
        pub struct $name<T> {
            idx: u32,
            gen: u32,
            _p: PhantomData<fn() -> T>,
        }
        impl<T> Clone for $name<T> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<T> Copy for $name<T> {}
        impl<T> PartialEq for $name<T> {
            fn eq(&self, o: &Self) -> bool {
                self.idx == o.idx && self.gen == o.gen
            }
        }
        impl<T> Eq for $name<T> {}
        impl<T> std::fmt::Debug for $name<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({}v{})", stringify!($name), self.idx, self.gen)
            }
        }
        impl<T> $name<T> {
            pub(crate) fn from_raw(idx: u32, gen: u32) -> Self {
                $name { idx, gen, _p: PhantomData }
            }
            /// True while the owning scope has not been disposed.
            pub fn is_alive(&self) -> bool {
                with_rt(|r| r.alive(self.idx, self.gen))
            }
            /// Dispose just this node (scopes dispose everything they own).
            pub fn dispose(self) {
                with_rt(|r| if r.alive(self.idx, self.gen) { free_node(r, self.idx) });
            }
        }
        impl<T: 'static> $name<T> {
            /// Read by reference. Tracked when called inside `derived` / `effect`.
            pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
                let v = read_value(self.idx, self.gen, true);
                let cell = v.downcast_ref::<RefCell<T>>().expect("handle type");
                let b = cell.borrow();
                f(&b)
            }
            /// Read by reference without tracking.
            pub fn with_untracked<R>(&self, f: impl FnOnce(&T) -> R) -> R {
                let v = read_value(self.idx, self.gen, false);
                let cell = v.downcast_ref::<RefCell<T>>().expect("handle type");
                let b = cell.borrow();
                f(&b)
            }
            /// `None` if the handle's scope was disposed.
            pub fn try_with<R>(&self, f: impl FnOnce(&T) -> R) -> Option<R> {
                if self.is_alive() { Some(self.with(f)) } else { None }
            }
        }
        impl<T: Clone + 'static> $name<T> {
            pub fn get(&self) -> T {
                self.with(|v| v.clone())
            }
            pub fn get_untracked(&self) -> T {
                self.with_untracked(|v| v.clone())
            }
            pub fn try_get(&self) -> Option<T> {
                self.try_with(|v| v.clone())
            }
        }
    };
}

handle!(Signal);
handle!(Memo);

impl<T: PartialEq + 'static> Signal<T> {
    /// Write; a value equal to the current one is dropped (equality cutoff).
    pub fn set(&self, v: T) {
        let changed = {
            let cur = read_value(self.idx, self.gen, false);
            let cell = cur.downcast_ref::<RefCell<T>>().expect("signal type");
            let mut b = cell.borrow_mut();
            if *b == v {
                false
            } else {
                *b = v;
                true
            }
        };
        if changed {
            write_notify(self.idx);
        }
    }
}

impl<T: 'static> Signal<T> {
    /// Mutate in place and always notify (use for lists and other non-comparable data).
    pub fn update(&self, f: impl FnOnce(&mut T)) {
        {
            let cur = read_value(self.idx, self.gen, false);
            let cell = cur.downcast_ref::<RefCell<T>>().expect("signal type");
            f(&mut cell.borrow_mut());
        }
        write_notify(self.idx);
    }
    /// Replace and always notify, without requiring `PartialEq`.
    pub fn set_always(&self, v: T) {
        self.update(|slot| *slot = v);
    }
}
