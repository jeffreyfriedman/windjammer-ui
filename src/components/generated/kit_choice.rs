use super::a11y::*;
use super::vnode::VNode;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct CheckboxField {
    pub base: String,
    pub label: String,
    pub checked: bool,
}

impl CheckboxField {
#[inline]
pub fn new(base: String, label: String) -> CheckboxField {
        CheckboxField { base, label, checked: false }
}
#[inline]
pub fn checked(mut self, on: bool) -> CheckboxField {
        self.checked = on;
        self
}
#[inline]
pub fn is_checked(&self) -> bool {
        self.checked
}
}

impl Interactive for CheckboxField {
#[inline]
fn on_event(&mut self, event: String, act: String, key: String, value: String) -> bool {
        if act == "check" && event == "change" {
            self.checked = value == "true";
            return true;
        }
        false
}
#[inline]
fn focus_id(&self) -> String {
        String::new()
}
#[inline]
fn view(&self) -> VNode {
        let id = part_id(&self.base, "input", 0_i32);
        let mut input = VNode::input().set_id(id).set_type("checkbox".to_string()).add_attr("data-wj-act".to_string(), "check".to_string());
        if self.checked {
            input = input.add_attr("checked".to_string(), "true".to_string());
        }
        VNode::element("label".to_string()).add_class("wj-checkbox".to_string()).add_attr("for".to_string(), part_id(&self.base, "input", 0_i32)).child(input).child(VNode::span().add_text(self.label.clone()))
}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct SwitchControl {
    pub base: String,
    pub label: String,
    pub on: bool,
    pub disabled: bool,
}

impl SwitchControl {
#[inline]
pub fn new(base: String, label: String) -> SwitchControl {
        SwitchControl { base, label, on: false, disabled: false }
}
#[inline]
pub fn on(mut self, state: bool) -> SwitchControl {
        self.on = state;
        self
}
#[inline]
pub fn disabled(mut self, state: bool) -> SwitchControl {
        self.disabled = state;
        self
}
#[inline]
pub fn is_on(&self) -> bool {
        self.on
}
}

impl Interactive for SwitchControl {
#[inline]
fn on_event(&mut self, event: String, act: String, key: String, value: String) -> bool {
        if act != "switch" || self.disabled {
            return false;
        }
        if event == "click" || event == "keydown" && is_activation_key(&key) {
            self.on = !self.on;
            return true;
        }
        false
}
#[inline]
fn focus_id(&self) -> String {
        String::new()
}
#[inline]
fn view(&self) -> VNode {
        let mut node = VNode::button().add_class("wj-switch".to_string()).set_id(part_id(&self.base, "switch", 0_i32)).add_attr("type".to_string(), "button".to_string()).add_attr("role".to_string(), "switch".to_string()).add_attr("aria-checked".to_string(), aria_bool(self.on)).add_attr("data-wj-act".to_string(), "switch".to_string()).add_text(self.label.clone());
        if self.disabled {
            node = node.add_attr("disabled".to_string(), "true".to_string()).add_attr("aria-disabled".to_string(), "true".to_string());
        }
        node
}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct ToggleButton {
    pub base: String,
    pub label: String,
    pub pressed: bool,
}

impl ToggleButton {
#[inline]
pub fn new(base: String, label: String) -> ToggleButton {
        ToggleButton { base, label, pressed: false }
}
#[inline]
pub fn is_pressed(&self) -> bool {
        self.pressed
}
}

impl Interactive for ToggleButton {
#[inline]
fn on_event(&mut self, event: String, act: String, key: String, value: String) -> bool {
        if act != "toggle" {
            return false;
        }
        if event == "click" || event == "keydown" && is_activation_key(&key) {
            self.pressed = !self.pressed;
            return true;
        }
        false
}
#[inline]
fn focus_id(&self) -> String {
        String::new()
}
#[inline]
fn view(&self) -> VNode {
        VNode::button().add_class("wj-button".to_string()).set_id(part_id(&self.base, "toggle", 0_i32)).add_attr("type".to_string(), "button".to_string()).add_attr("aria-pressed".to_string(), aria_bool(self.pressed)).add_attr("data-wj-act".to_string(), "toggle".to_string()).add_text(self.label.clone())
}
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct RadioChoice {
    pub base: String,
    pub label: String,
    pub values: Vec<String>,
    pub labels: Vec<String>,
    pub selected: i32,
    pub focus: String,
}

impl RadioChoice {
#[inline]
pub fn new(base: String, label: String) -> RadioChoice {
        RadioChoice { base, label, values: Vec::new(), labels: Vec::new(), selected: -1_i32, focus: String::new() }
}
#[inline]
pub fn option(mut self, value: impl Into<String>, label: impl Into<String>) -> RadioChoice {
        self.values.push(value.into());
        self.labels.push(label.into());
        self
}
#[inline]
pub fn select(mut self, index: i32) -> RadioChoice {
        self.selected = index;
        self
}
/// Value of the selected option, empty when nothing is selected.
#[inline]
pub fn value(&self) -> String {
        if self.selected < 0_i32 {
            return String::new();
        }
        self.values[self.selected as usize].clone()
}
#[inline]
pub fn pick(&mut self, index: i32) {
        self.selected = index;
        self.focus = part_id(&self.base, "radio", index);
}
}

impl Interactive for RadioChoice {

fn on_event(&mut self, event: String, act: String, key: String, value: String) -> bool {
        let count = self.values.len() as i32;
        let index = act_index(&act, "radio");
        if index < 0_i32 {
            return false;
        }
        if event == "click" {
            let _ = self.pick(index);
            return true;
        }
        if event == "keydown" {
            if key == " " {
                let _ = self.pick(index);
                return true;
            }
            let mut next = roving_next(&key, index, count, false);
            if next < 0_i32 {
                next = roving_next(&key, index, count, true);
            }
            if next >= 0_i32 {
                let _ = self.pick(next);
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
        let label_id = part_id(&self.base, "label", 0_i32);
        let mut group = VNode::div().add_class("wj-radiogroup".to_string()).add_attr("role".to_string(), "radiogroup".to_string()).add_attr("aria-labelledby".to_string(), label_id).child(VNode::span().set_id(part_id(&self.base, "label", 0_i32)).add_text(self.label.clone()));
        let mut i = 0_i32;
        while i < (self.values.len() as i32) {
            let entry = {
                if self.selected < 0_i32 {
                    i == 0_i32
                } else {
                    i == self.selected
                }
            };
            let on = i == self.selected;
            let radio = { let _temp0 = format!("radio:{}", i); VNode::div().add_class("wj-radio".to_string()).set_id(part_id(&self.base, "radio", i)).add_attr("role".to_string(), "radio".to_string()).add_attr("aria-checked".to_string(), aria_bool(on)).add_attr("tabindex".to_string(), roving_tabindex(entry)).add_attr("data-wj-act".to_string(), _temp0) }.add_text(self.labels[i as usize].clone());
            group = group.child(radio);
            i += 1;
        }
        group
}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct SliderControl {
    pub base: String,
    pub label: String,
    pub min: i32,
    pub max: i32,
    pub step: i32,
    pub now: i32,
}

impl SliderControl {
#[inline]
pub fn new(base: String, label: String, min: i32, max: i32) -> SliderControl {
        SliderControl { base, label, min, max, step: 1_i32, now: min }
}
#[inline]
pub fn step(mut self, step: i32) -> SliderControl {
        self.step = step;
        self
}
#[inline]
pub fn value(mut self, v: i32) -> SliderControl {
        self.now = v;
        self
}
#[inline]
pub fn current(&self) -> i32 {
        self.now
}
#[inline]
pub fn set(&mut self, v: i32) {
        let mut next = v;
        if next < self.min {
            next = self.min;
        }
        if next > self.max {
            next = self.max;
        }
        self.now = next;
}
}

impl Interactive for SliderControl {

fn on_event(&mut self, event: String, act: String, key: String, value: String) -> bool {
        if act != "slider" || event != "keydown" {
            return false;
        }
        if key == "ArrowRight" || key == "ArrowUp" {
            let _ = self.set(self.now + self.step);
            return true;
        }
        if key == "ArrowLeft" || key == "ArrowDown" {
            let _ = self.set(self.now - self.step);
            return true;
        }
        if key == "PageUp" {
            let _ = self.set(self.now + self.step * 10_i32);
            return true;
        }
        if key == "PageDown" {
            let _ = self.set(self.now - self.step * 10_i32);
            return true;
        }
        if key == "Home" {
            let _ = self.set(self.min);
            return true;
        }
        if key == "End" {
            let _ = self.set(self.max);
            return true;
        }
        false
}
#[inline]
fn focus_id(&self) -> String {
        String::new()
}

fn view(&self) -> VNode {
        { let _temp0 = format!("{}", self.now); { let _temp0 = format!("{}", self.now); { let _temp0 = format!("{}", self.max); { let _temp0 = format!("{}", self.min); VNode::div().add_class("wj-slider".to_string()).set_id(part_id(&self.base, "slider", 0_i32)).add_attr("role".to_string(), "slider".to_string()).add_attr("tabindex".to_string(), "0".to_string()).add_attr("aria-label".to_string(), self.label.clone()).add_attr("aria-orientation".to_string(), "horizontal".to_string()).add_attr("aria-valuemin".to_string(), _temp0) }.add_attr("aria-valuemax".to_string(), _temp0) }.add_attr("aria-valuenow".to_string(), _temp0) }.add_attr("data-wj-act".to_string(), "slider".to_string()).add_text(_temp0) }
}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct PageNav {
    pub base: String,
    pub label: String,
    pub page: i32,
    pub pages: i32,
    pub focus: String,
}

impl PageNav {
#[inline]
pub fn new(base: String, label: String, pages: i32) -> PageNav {
        PageNav { base, label, page: 1_i32, pages, focus: String::new() }
}
#[inline]
pub fn current(&self) -> i32 {
        self.page
}
#[inline]
pub fn go(&mut self, page: i32) {
        if page >= 1_i32 && page <= self.pages {
            self.page = page;
        }
        self.focus = part_id(&self.base, "page", self.page);
}
}

impl Interactive for PageNav {
#[inline]
fn on_event(&mut self, event: String, act: String, key: String, value: String) -> bool {
        let wanted = {
            if act == "prev" {
                self.page - 1_i32
            } else {
                if act == "next" {
                    self.page + 1_i32
                } else {
                    act_index(&act, "page")
                }
            }
        };
        if wanted < 0_i32 {
            return false;
        }
        if event == "click" || event == "keydown" && is_activation_key(&key) {
            let _ = self.go(wanted);
            return true;
        }
        false
}
#[inline]
fn focus_id(&self) -> String {
        self.focus.clone()
}

fn view(&self) -> VNode {
        let mut prev = VNode::button().add_class("wj-page".to_string()).set_id(part_id(&self.base, "prev", 0_i32)).add_attr("type".to_string(), "button".to_string()).add_attr("aria-label".to_string(), "Previous page".to_string()).add_attr("data-wj-act".to_string(), "prev".to_string()).add_text("Prev".to_string());
        if self.page <= 1_i32 {
            prev = prev.add_attr("disabled".to_string(), "true".to_string());
        }
        let mut next = VNode::button().add_class("wj-page".to_string()).set_id(part_id(&self.base, "next", 0_i32)).add_attr("type".to_string(), "button".to_string()).add_attr("aria-label".to_string(), "Next page".to_string()).add_attr("data-wj-act".to_string(), "next".to_string()).add_text("Next".to_string());
        if self.page >= self.pages {
            next = next.add_attr("disabled".to_string(), "true".to_string());
        }
        let mut nav = VNode::element("nav".to_string()).add_class("wj-pagination".to_string()).add_attr("aria-label".to_string(), self.label.clone()).child(prev);
        let mut p = 1_i32;
        while p <= self.pages {
            let mut btn = { let _temp0 = format!("{}", p); { let _temp0 = format!("page:{}", p); { let _temp0 = format!("Page {}", p); VNode::button().add_class("wj-page".to_string()).set_id(part_id(&self.base, "page", p)).add_attr("type".to_string(), "button".to_string()).add_attr("aria-label".to_string(), _temp0) }.add_attr("data-wj-act".to_string(), _temp0) }.add_text(_temp0) };
            if p == self.page {
                btn = btn.add_attr("aria-current".to_string(), "page".to_string());
            }
            nav = nav.child(btn);
            p += 1;
        }
        nav.child(next)
}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct TooltipHost {
    pub base: String,
    pub label: String,
    pub tip: String,
    pub shown: bool,
}

impl TooltipHost {
#[inline]
pub fn new(base: String, label: String, tip: String) -> TooltipHost {
        TooltipHost { base, label, tip, shown: false }
}
#[inline]
pub fn is_shown(&self) -> bool {
        self.shown
}
}

impl Interactive for TooltipHost {
#[inline]
fn on_event(&mut self, event: String, act: String, key: String, value: String) -> bool {
        if act != "tip" {
            return false;
        }
        if event == "focusin" {
            self.shown = true;
            return false;
        }
        if event == "focusout" {
            self.shown = false;
            return false;
        }
        if event == "keydown" && key == "Escape" && self.shown {
            self.shown = false;
            return true;
        }
        false
}
#[inline]
fn focus_id(&self) -> String {
        String::new()
}

fn view(&self) -> VNode {
        let mut tip = VNode::span().add_class("wj-tooltip".to_string()).set_id(part_id(&self.base, "tip", 0_i32)).add_attr("role".to_string(), "tooltip".to_string()).add_text(self.tip.clone());
        if !self.shown {
            tip = tip.add_attr("hidden".to_string(), "hidden".to_string());
        }
        VNode::span().add_class("wj-tooltip-host".to_string()).child(VNode::button().add_class("wj-button".to_string()).set_id(part_id(&self.base, "trigger", 0_i32)).add_attr("type".to_string(), "button".to_string()).add_attr("aria-describedby".to_string(), part_id(&self.base, "tip", 0_i32)).add_attr("data-wj-act".to_string(), "tip".to_string()).add_text(self.label.clone())).child(tip)
}
}

