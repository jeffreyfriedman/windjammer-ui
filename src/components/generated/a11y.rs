#[allow(unused_imports)]
use super::*;

use super::vnode::VNode;
pub trait Interactive {
    fn on_event(&mut self, event: String, act: String, key: String, value: String) -> bool;
    fn view(&self) -> VNode;
    fn focus_id(&self) -> String;
}

/// "true" / "false" as ARIA expects them.
#[inline]
pub fn aria_bool(b: bool) -> String {
    if b {
        String::from("true")
    } else {
        String::from("false")
    }
}

/// Roving tabindex: the active item is in the tab order, the rest are reached with arrow keys.
#[inline]
pub fn roving_tabindex(is_active: bool) -> String {
    if is_active {
        String::from("0")
    } else {
        String::from("-1")
    }
}

/// Enter and Space activate a control.
#[inline]
pub fn is_activation_key(key: &str) -> bool {
    key == "Enter" || key == " " || key == "Spacebar"
}

/// Where arrow keys move inside a group of `count` items. Returns the new index, or -1 when `key` is not a navigation
/// key for this orientation. Movement wraps; Home and End jump to the ends.

pub fn roving_next(key: &str, current: i32, count: i32, vertical: bool) -> i32 {
    if count <= 0_i32 {
        return -1_i32;
    }
    if key == "Home" {
        return 0_i32;
    }
    if key == "End" {
        return count - 1_i32;
    }
    let forward: String = {
        if vertical {
            String::from("ArrowDown")
        } else {
            String::from("ArrowRight")
        }
    };
    let backward: String = {
        if vertical {
            String::from("ArrowUp")
        } else {
            String::from("ArrowLeft")
        }
    };
    if key == forward {
        return ((current + 1_i32) % count) as i32;
    }
    if key == backward {
        return ((current + count - 1_i32) % count) as i32;
    }
    -1_i32
}

/// The index in an act such as "tab:3" given its prefix "tab", or -1 when the act has another name.
#[inline]
pub fn act_index(act: &str, prefix: &str) -> i32 {
    let head = format!("{}:", prefix);
    if !act.starts_with(&head) {
        return -1_i32;
    }
    let rest = act.replace(&head, "");
    match rest.parse::<i32>() {
        Ok(n) => n,
        Err(_) => -1_i32,
    }
}

/// Stable element id for part `index` of a component: "<base>-<part>-<index>".
#[inline]
pub fn part_id(base: &str, part: &str, index: i32) -> String {
    format!("{}-{}-{}", base, part, index)
}

