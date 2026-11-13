use super::a11y::*;
use super::vnode::VNode;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct ModalDialog {
    pub base: String,
    pub trigger_label: String,
    pub title: String,
    pub body: String,
    pub confirm_label: String,
    pub cancel_label: String,
    pub open: bool,
    pub confirmed: bool,
    pub focus: String,
}

impl ModalDialog {
#[inline]
pub fn new(base: String, title: String) -> ModalDialog {
        ModalDialog { base, trigger_label: String::from("Open"), title, body: String::new(), confirm_label: String::from("OK"), cancel_label: String::from("Cancel"), open: false, confirmed: false, focus: String::new() }
}
#[inline]
pub fn trigger(mut self, label: impl Into<String>) -> ModalDialog {
        self.trigger_label = label.into();
        self
}
#[inline]
pub fn body(mut self, text: impl Into<String>) -> ModalDialog {
        self.body = text.into();
        self
}
#[inline]
pub fn is_open(&self) -> bool {
        self.open
}
/// True after the dialog was closed with its confirm button.
#[inline]
pub fn was_confirmed(&self) -> bool {
        self.confirmed
}
#[inline]
pub fn open_dialog(&mut self) {
        self.open = true;
        self.confirmed = false;
        self.focus = part_id(&self.base, "btn", 0_i32);
}
#[inline]
pub fn close_dialog(&mut self, confirmed: bool) {
        self.open = false;
        self.confirmed = confirmed;
        self.focus = part_id(&self.base, "trigger", 0_i32);
}
}

impl Interactive for ModalDialog {

fn on_event(&mut self, event: String, act: String, key: String, value: String) -> bool {
        if event == "click" {
            if act == "open" {
                let _ = self.open_dialog();
                return true;
            }
            if act == "backdrop" || act == "dlg:0" {
                let _ = self.close_dialog(false);
                return true;
            }
            if act == "dlg:1" {
                let _ = self.close_dialog(true);
                return true;
            }
            return false;
        }
        if event == "keydown" && self.open {
            if key == "Escape" {
                let _ = self.close_dialog(false);
                return true;
            }
            let index = act_index(&act, "dlg");
            if index >= 0_i32 && (key == "Tab" || key == "Shift+Tab") {
                let last = 1_i32;
                let mut next = index;
                if key == "Tab" {
                    next = {
                        if index >= last {
                            0_i32
                        } else {
                            index + 1_i32
                        }
                    };
                } else {
                    next = {
                        if index <= 0_i32 {
                            last
                        } else {
                            index - 1_i32
                        }
                    };
                }
                self.focus = part_id(&self.base, "btn", next);
                return true;
            }
        }
        false
}
#[inline]
fn focus_id(&self) -> String {
        self.focus.clone()
}

fn view(&self) -> VNode {
        let trigger = VNode::button().add_class("wj-button".to_string()).set_id(part_id(&self.base, "trigger", 0_i32)).add_attr("type".to_string(), "button".to_string()).add_attr("aria-haspopup".to_string(), "dialog".to_string()).add_attr("aria-expanded".to_string(), aria_bool(self.open)).add_attr("data-wj-act".to_string(), "open".to_string()).add_text(self.trigger_label.clone());
        let mut root = VNode::div().add_class("wj-modal-host".to_string()).child(trigger);
        if self.open {
            let title_id = part_id(&self.base, "title", 0_i32);
            let body_id = part_id(&self.base, "body", 0_i32);
            let cancel = VNode::button().add_class("wj-button".to_string()).set_id(part_id(&self.base, "btn", 0_i32)).add_attr("type".to_string(), "button".to_string()).add_attr("data-wj-act".to_string(), "dlg:0".to_string()).add_text(self.cancel_label.clone());
            let confirm = VNode::button().add_class("wj-button wj-button-primary".to_string()).set_id(part_id(&self.base, "btn", 1_i32)).add_attr("type".to_string(), "button".to_string()).add_attr("data-wj-act".to_string(), "dlg:1".to_string()).add_text(self.confirm_label.clone());
            let dialog = VNode::div().add_class("wj-modal".to_string()).add_attr("role".to_string(), "dialog".to_string()).add_attr("aria-modal".to_string(), "true".to_string()).add_attr("data-wj-act".to_string(), "dialog".to_string()).add_attr("aria-labelledby".to_string(), title_id).add_attr("aria-describedby".to_string(), body_id).child(VNode::element("h2".to_string()).set_id(part_id(&self.base, "title", 0_i32)).add_text(self.title.clone())).child(VNode::element("p".to_string()).set_id(part_id(&self.base, "body", 0_i32)).add_text(self.body.clone())).child(VNode::div().add_class("wj-modal-actions".to_string()).child(cancel).child(confirm));
            let backdrop = VNode::div().add_class("wj-modal-backdrop".to_string()).add_attr("data-wj-act".to_string(), "backdrop".to_string()).child(dialog);
            root = root.child(backdrop);
        }
        root
}
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct MenuButton {
    pub base: String,
    pub label: String,
    pub items: Vec<String>,
    pub open: bool,
    pub active: i32,
    pub chosen: String,
    pub focus: String,
}

impl MenuButton {
#[inline]
pub fn new(base: String, label: String) -> MenuButton {
        MenuButton { base, label, items: Vec::new(), open: false, active: 0_i32, chosen: String::new(), focus: String::new() }
}
#[inline]
pub fn item(mut self, label: impl Into<String>) -> MenuButton {
        self.items.push(label.into());
        self
}
#[inline]
pub fn is_open(&self) -> bool {
        self.open
}
/// Label of the last item chosen, empty before any choice.
#[inline]
pub fn chosen(&self) -> String {
        self.chosen.clone()
}
#[inline]
pub fn open_at(&mut self, index: i32) {
        self.open = true;
        self.active = index;
        self.focus = part_id(&self.base, "item", index);
}
#[inline]
pub fn close_to_trigger(&mut self) {
        self.open = false;
        self.focus = part_id(&self.base, "trigger", 0_i32);
}
#[inline]
pub fn choose(&mut self, index: i32) {
        self.chosen = self.items[index as usize].clone();
        self.close_to_trigger();
}
}

impl Interactive for MenuButton {

fn on_event(&mut self, event: String, act: String, key: String, value: String) -> bool {
        let count = self.items.len() as i32;
        if act == "trigger" {
            if event == "click" {
                if self.open {
                    let _ = self.close_to_trigger();
                } else {
                    let _ = self.open_at(0_i32);
                }
                return true;
            }
            if event == "keydown" {
                if key == "Enter" || key == " " || key == "ArrowDown" {
                    let _ = self.open_at(0_i32);
                    return true;
                }
                if key == "ArrowUp" {
                    let _ = self.open_at(count - 1_i32);
                    return true;
                }
            }
            return false;
        }
        let index = act_index(&act, "item");
        if index < 0_i32 {
            return false;
        }
        if event == "click" {
            let _ = self.choose(index);
            return true;
        }
        if event == "keydown" {
            if key == "Escape" {
                let _ = self.close_to_trigger();
                return true;
            }
            if key == "Tab" || key == "Shift+Tab" {
                self.open = false;
                return false;
            }
            if is_activation_key(&key) {
                let _ = self.choose(index);
                return true;
            }
            let next = roving_next(&key, index, count, true);
            if next >= 0_i32 {
                let _ = self.open_at(next);
                return true;
            }
        }
        false
}
#[inline]
fn focus_id(&self) -> String {
        self.focus.clone()
}

fn view(&self) -> VNode {
        let trigger = VNode::button().add_class("wj-button".to_string()).set_id(part_id(&self.base, "trigger", 0_i32)).add_attr("type".to_string(), "button".to_string()).add_attr("aria-haspopup".to_string(), "menu".to_string()).add_attr("aria-expanded".to_string(), aria_bool(self.open)).add_attr("aria-controls".to_string(), part_id(&self.base, "menu", 0_i32)).add_attr("data-wj-act".to_string(), "trigger".to_string()).add_text(self.label.clone());
        let mut menu = VNode::div().add_class("wj-menu".to_string()).set_id(part_id(&self.base, "menu", 0_i32)).add_attr("role".to_string(), "menu".to_string()).add_attr("aria-labelledby".to_string(), part_id(&self.base, "trigger", 0_i32));
        if !self.open {
            menu = menu.add_attr("hidden".to_string(), "hidden".to_string());
        }
        let mut i = 0_i32;
        while i < (self.items.len() as i32) {
            let entry = { let _temp0 = format!("item:{}", i); VNode::button().add_class("wj-menu-item".to_string()).set_id(part_id(&self.base, "item", i)).add_attr("type".to_string(), "button".to_string()).add_attr("role".to_string(), "menuitem".to_string()).add_attr("tabindex".to_string(), "-1".to_string()).add_attr("data-wj-act".to_string(), _temp0) }.add_text(self.items[i as usize].clone());
            menu = menu.child(entry);
            i += 1;
        }
        VNode::div().add_class("wj-menu-host".to_string()).child(trigger).child(menu)
}
}

