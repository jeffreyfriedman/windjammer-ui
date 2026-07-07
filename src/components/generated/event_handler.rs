#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
//! Type aliases for event handlers

use std::cell::RefCell;
use std::rc::Rc;

/// Type alias for a mutable event handler with no arguments
pub type EventHandler = Rc<RefCell<dyn FnMut()>>;

/// Type alias for a mutable event handler with a String argument
pub type StringEventHandler = Rc<RefCell<dyn FnMut(String)>>;

/// Type alias for a mutable event handler with a bool argument
pub type BoolEventHandler = Rc<RefCell<dyn FnMut(bool)>>;

/// Type alias for a mutable event handler with an f64 argument
pub type F64EventHandler = Rc<RefCell<dyn FnMut(f64)>>;

/// Type alias for a mutable event handler with an [f32; 4] argument (RGBA color)
pub type ColorEventHandler = Rc<RefCell<dyn FnMut([f32; 4])>>;
