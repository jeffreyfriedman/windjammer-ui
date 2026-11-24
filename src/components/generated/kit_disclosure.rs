use super::a11y::*;
use super::vnode::VNode;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct TabStrip {
    pub base: String,
    pub labels: Vec<String>,
    pub panels: Vec<String>,
    pub selected: i32,
    pub focus: String,
}

impl TabStrip {
#[inline]
pub fn new(base: String) -> TabStrip {
        TabStrip { base, labels: Vec::new(), panels: Vec::new(), selected: 0_i32, focus: String::new() }
}
#[inline]
pub fn tab(mut self, label: impl Into<String>, panel: impl Into<String>) -> TabStrip {
        self.labels.push(label.into());
        self.panels.push(panel.into());
        self
}
#[inline]
pub fn selected(mut self, index: i32) -> TabStrip {
        self.selected = index;
        self
}
#[inline]
pub fn selected_index(&self) -> i32 {
        self.selected
}
#[inline]
pub fn pick(&mut self, index: i32) {
        self.selected = index;
        self.focus = part_id(&self.base, "tab", index);
}
}

impl Interactive for TabStrip {
#[inline]
fn on_event(&mut self, event: String, act: String, key: String, value: String) -> bool {
        let count = self.labels.len() as i32;
        let index = act_index(&act, "tab");
        if index < 0_i32 {
            return false;
        }
        if event == "click" {
            let _ = self.pick(index);
            return true;
        }
        if event == "keydown" {
            let next = roving_next(&key, index, count, false);
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
        let mut list = VNode::div().add_class("wj-tablist".to_string()).add_attr("role".to_string(), "tablist".to_string()).add_attr("aria-orientation".to_string(), "horizontal".to_string());
        let mut panels = VNode::div().add_class("wj-tabpanels".to_string());
        let mut i = 0_i32;
        while i < (self.labels.len() as i32) {
            let on = i == self.selected;
            let tab_id = part_id(&self.base, "tab", i);
            let panel_id = part_id(&self.base, "panel", i);
            let mut tab = { let _temp0 = format!("tab:{}", i); VNode::button().add_class("wj-tab".to_string()).set_id(tab_id.clone()).add_attr("type".to_string(), "button".to_string()).add_attr("role".to_string(), "tab".to_string()).add_attr("aria-selected".to_string(), aria_bool(on)).add_attr("aria-controls".to_string(), panel_id.clone()).add_attr("tabindex".to_string(), roving_tabindex(on)).add_attr("data-wj-act".to_string(), _temp0) }.add_text(self.labels[i as usize].clone());
            list = list.child(tab);
            let mut panel = VNode::div().add_class("wj-tabpanel".to_string()).set_id(panel_id).add_attr("role".to_string(), "tabpanel".to_string()).add_attr("aria-labelledby".to_string(), tab_id).add_attr("tabindex".to_string(), "0".to_string()).add_text(self.panels[i as usize].clone());
            if !on {
                panel = panel.add_attr("hidden".to_string(), "hidden".to_string());
            }
            panels = panels.child(panel);
            i = i.wrapping_add(1);
        }
        VNode::div().add_class("wj-tabs".to_string()).child(list).child(panels)
}
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct AccordionStack {
    pub base: String,
    pub titles: Vec<String>,
    pub bodies: Vec<String>,
    pub open: Vec<bool>,
    pub exclusive: bool,
    pub focus: String,
}

impl AccordionStack {
#[inline]
pub fn new(base: String) -> AccordionStack {
        AccordionStack { base, titles: Vec::new(), bodies: Vec::new(), open: Vec::new(), exclusive: false, focus: String::new() }
}
#[inline]
pub fn item(mut self, title: impl Into<String>, body: impl Into<String>) -> AccordionStack {
        self.titles.push(title.into());
        self.bodies.push(body.into());
        self.open.push(false);
        self
}
/// At most one section open at a time.
#[inline]
pub fn exclusive(mut self, on: bool) -> AccordionStack {
        self.exclusive = on;
        self
}
#[inline]
pub fn is_open(&self, index: i32) -> bool {
        self.open[index as usize]
}
#[inline]
pub fn toggle(&mut self, index: i32) {
        let was = self.open[index as usize];
        if self.exclusive {
            let mut j = 0_usize;
            while j < self.open.len() {
                self.open[j] = false;
                j = j.wrapping_add(1);
            }
        }
        self.open[index as usize] = !was;
        self.focus = part_id(&self.base, "head", index);
}
}

impl Interactive for AccordionStack {

fn on_event(&mut self, event: String, act: String, key: String, value: String) -> bool {
        let count = self.titles.len() as i32;
        let index = act_index(&act, "head");
        if index < 0_i32 {
            return false;
        }
        if event == "click" {
            let _ = self.toggle(index);
            return true;
        }
        if event == "keydown" {
            if is_activation_key(&key) {
                let _ = self.toggle(index);
                return true;
            }
            let next = roving_next(&key, index, count, true);
            if next >= 0_i32 {
                self.focus = part_id(&self.base, "head", next);
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
        let mut root = VNode::div().add_class("wj-accordion".to_string());
        let mut i = 0_i32;
        while i < (self.titles.len() as i32) {
            let on = self.open[i as usize];
            let head_id = part_id(&self.base, "head", i);
            let body_id = part_id(&self.base, "body", i);
            let head = { let _temp0 = format!("head:{}", i); VNode::button().add_class("wj-accordion-head".to_string()).set_id(head_id.clone()).add_attr("type".to_string(), "button".to_string()).add_attr("aria-expanded".to_string(), aria_bool(on)).add_attr("aria-controls".to_string(), body_id.clone()).add_attr("data-wj-act".to_string(), _temp0) }.add_text(self.titles[i as usize].clone());
            let mut body = VNode::div().add_class("wj-accordion-body".to_string()).set_id(body_id).add_attr("role".to_string(), "region".to_string()).add_attr("aria-labelledby".to_string(), head_id).add_text(self.bodies[i as usize].clone());
            if !on {
                body = body.add_attr("hidden".to_string(), "hidden".to_string());
            }
            root = root.child(head).child(body);
            i = i.wrapping_add(1);
        }
        root
}
}

