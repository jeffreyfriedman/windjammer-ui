//! What the handler that is running is being told about its DOM event.
//!
//! Handlers take no event argument, so the web adapter publishes the details of the event it is delivering here
//! (like `stop_propagation`, a thread-local scoped to one dispatch). Kit components read it through
//! `current_event()` and receive it as `(event, act, key, value)`.

use std::cell::RefCell;

/// The parts of a DOM event the kit components act on.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EventInfo {
    /// `KeyboardEvent.key` (`"ArrowDown"`, `"Enter"`, `" "`), empty for other events.
    pub key: String,
    /// `data-wj-act` of the nearest element at or above the event target, empty when there is none.
    pub act: String,
    /// The target's value: text for inputs and selects, `"true"`/`"false"` for checkboxes and switches.
    pub value: String,
    pub shift: bool,
}

thread_local! {
    static CURRENT: RefCell<EventInfo> = RefCell::new(EventInfo::default());
}

/// The event being delivered to the running handler (default value outside a dispatch).
pub fn current_event() -> EventInfo {
    CURRENT.with(|c| c.borrow().clone())
}

/// Publish `info` for the duration of a dispatch; returns the previous value so the caller can restore it.
pub fn set_current_event(info: EventInfo) -> EventInfo {
    CURRENT.with(|c| c.replace(info))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_returns_previous_and_current_reads_back() {
        let first = EventInfo {
            key: "Enter".into(),
            ..Default::default()
        };
        let prev = set_current_event(first.clone());
        assert_eq!(prev, EventInfo::default());
        assert_eq!(current_event(), first);
        set_current_event(prev);
        assert_eq!(current_event(), EventInfo::default());
    }
}
