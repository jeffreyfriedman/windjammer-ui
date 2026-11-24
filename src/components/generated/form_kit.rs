use super::a11y::*;
use super::vnode::VNode;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum FieldKind {
    Text,
    Email,
    Number,
    Date,
    Select,
    Checkbox,
    Radio,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct FormField {
    pub name: String,
    pub label: String,
    pub kind: FieldKind,
    pub value: String,
    pub initial: String,
    pub required: bool,
    pub min_len: i32,
    pub min: String,
    pub max: String,
    pub hint: String,
    pub options: Vec<String>,
    pub option_labels: Vec<String>,
    pub touched: bool,
}

impl FormField {
#[inline]
pub fn new(name: String, label: String, kind: FieldKind) -> FormField {
        FormField { name, label, kind, value: String::new(), initial: String::new(), required: false, min_len: 0_i32, min: String::new(), max: String::new(), hint: String::new(), options: Vec::new(), option_labels: Vec::new(), touched: false }
}
#[inline]
pub fn value(mut self, v: impl Into<String>) -> FormField {
        self.value = v.into();
        self.initial = self.value.clone();
        self
}
#[inline]
pub fn required(mut self, on: bool) -> FormField {
        self.required = on;
        self
}
#[inline]
pub fn min_len(mut self, n: i32) -> FormField {
        self.min_len = n;
        self
}
/// Lower bound: a number for Number fields, an ISO date (YYYY-MM-DD) for Date fields.
#[inline]
pub fn min(mut self, v: impl Into<String>) -> FormField {
        self.min = v.into();
        self
}
#[inline]
pub fn max(mut self, v: impl Into<String>) -> FormField {
        self.max = v.into();
        self
}
#[inline]
pub fn hint(mut self, text: impl Into<String>) -> FormField {
        self.hint = text.into();
        self
}
#[inline]
pub fn option(mut self, value: impl Into<String>, label: impl Into<String>) -> FormField {
        self.options.push(value.into());
        self.option_labels.push(label.into());
        self
}
#[inline]
pub fn is_empty(&self) -> bool {
        match self.kind {
            FieldKind::Checkbox => self.value != "true",
            _ => self.value.trim().is_empty(),
        }
}
/// The validation message for the current value, or an empty string when the value is acceptable.

pub fn error_message(&self) -> String {
        if self.is_empty() {
            if self.required {
                return format!("{} is required", self.label);
            }
            return String::new();
        }
        match self.kind {
            FieldKind::Text => {
                if self.value.len() < (self.min_len as usize) {
                    return format!("{} must be at least {} characters", self.label, self.min_len);
                }
                String::new()
            },
            FieldKind::Email => {
                let at = self.value.find("@");
                let ok = match at {
                    Some(i) => i > 0_usize && self.value.rfind(".").unwrap_or(0) > i + 1_usize && !self.value.ends_with("."),
                    None => false,
                };
                if ok {
                    return String::new();
                }
                format!("{} must be an email address", self.label)
            },
            FieldKind::Number => {
                let parsed = self.value.trim().parse::<f64>();
                match parsed {
                    Ok(n) => {
                        if !self.min.is_empty() && n < self.min.parse::<f64>().unwrap_or(n) {
                            return format!("{} must be at least {}", self.label, self.min);
                        }
                        if !self.max.is_empty() && n > self.max.parse::<f64>().unwrap_or(n) {
                            return format!("{} must be at most {}", self.label, self.max);
                        }
                        String::new()
                    },
                    Err(_) => format!("{} must be a number", self.label),
                }
            },
            FieldKind::Date => {
                let v = self.value.clone();
                let shaped = v.len() == 10_usize && v.chars().nth(4) == Some('-') && v.chars().nth(7) == Some('-');
                if !shaped {
                    return format!("{} must be a date (YYYY-MM-DD)", self.label);
                }
                if !self.min.is_empty() && v < self.min {
                    return format!("{} must be on or after {}", self.label, self.min);
                }
                if !self.max.is_empty() && v > self.max {
                    return format!("{} must be on or before {}", self.label, self.max);
                }
                String::new()
            },
            FieldKind::Select => {
                if self.options.iter().any(|m| *m == *self.value) {
                    return String::new();
                }
                format!("{} must be one of the listed options", self.label)
            },
            FieldKind::Radio => {
                if self.options.iter().any(|m| *m == *self.value) {
                    return String::new();
                }
                format!("{} must be one of the listed options", self.label)
            },
            FieldKind::Checkbox => String::new(),
        }
}
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct FormKit {
    pub base: String,
    pub title: String,
    pub fields: Vec<FormField>,
    pub attempted: bool,
    pub status: String,
    pub submitted: String,
    pub submit_count: i32,
    pub focus: String,
}

impl FormKit {
#[inline]
pub fn new(base: String, title: String) -> FormKit {
        FormKit { base, title, fields: Vec::new(), attempted: false, status: String::new(), submitted: String::new(), submit_count: 0_i32, focus: String::new() }
}
#[inline]
pub fn field(mut self, f: FormField) -> FormKit {
        self.fields.push(f);
        self
}
/// Number of fields that currently fail validation.
#[inline]
pub fn error_count(&self) -> i32 {
        let mut n = 0_i32;
        let mut i = 0_usize;
        while i < self.fields.len() {
            if !self.fields[i].error_message().is_empty() {
                n += 1;
            }
            i = i.wrapping_add(1);
        }
        n
}
/// "name=value" pairs (joined with `&`) of the last successful submit, empty before one.
#[inline]
pub fn submitted(&self) -> String {
        self.submitted.clone()
}
#[inline]
pub fn submit_count(&self) -> i32 {
        self.submit_count
}
#[inline]
pub fn status(&self) -> String {
        self.status.clone()
}

pub fn submit(&mut self) {
        self.attempted = true;
        let mut first_bad = -1_i32;
        let mut bad = 0_i64;
        let mut i = 0_usize;
        while i < self.fields.len() {
            if !self.fields[i].error_message().is_empty() {
                bad += 1;
                if first_bad < 0_i32 {
                    first_bad = i as i32;
                }
            }
            i = i.wrapping_add(1);
        }
        if bad > 0_i64 {
            self.status = {
                if bad == 1_i64 {
                    String::from("1 field needs attention")
                } else {
                    format!("{} fields need attention", bad)
                }
            };
            self.focus = part_id(&self.base, "f", first_bad);
            return;
        }
        let mut pairs = String::new();
        let mut j = 0_usize;
        while j < self.fields.len() {
            if j > 0_usize {
                { use std::fmt::Write as _; write!(pairs, "&").expect("a formatting trait implementation returned an error"); };
            }
            { use std::fmt::Write as _; write!(pairs, "{}={}", self.fields[j].name, self.fields[j].value).expect("a formatting trait implementation returned an error"); };
            j = j.wrapping_add(1);
        }
        self.submitted = pairs;
        self.submit_count += 1;
        self.status = String::from("Submitted");
        self.focus = String::new();
}
#[inline]
pub fn reset(&mut self) {
        let mut i = 0_usize;
        while i < self.fields.len() {
            self.fields[i].value = self.fields[i].initial.clone();
            self.fields[i].touched = false;
            i = i.wrapping_add(1);
        }
        self.attempted = false;
        self.status = String::from("Form reset");
        self.focus = part_id(&self.base, "f", 0_i32);
}
}

impl Interactive for FormKit {

fn on_event(&mut self, event: String, act: String, key: String, value: String) -> bool {
        if act == "submit" && event == "click" {
            let _ = self.submit();
            return true;
        }
        if act == "reset" && event == "click" {
            let _ = self.reset();
            return true;
        }
        let index = act_index(&act, "field");
        if index < 0_i32 {
            return false;
        }
        let at = index as usize;
        if event == "input" || event == "change" {
            self.fields[at].value = value;
            return false;
        }
        if event == "focusout" {
            self.fields[at].touched = true;
            return false;
        }
        if event == "keydown" && key == "Enter" {
            let free_text = match &self.fields[at].kind {
                FieldKind::Text => true,
                FieldKind::Email => true,
                FieldKind::Number => true,
                FieldKind::Date => true,
                _ => false,
            };
            if free_text {
                let _ = self.submit();
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
        let mut form = VNode::element("form".to_string()).add_class("wj-form".to_string()).add_attr("novalidate".to_string(), "novalidate".to_string()).add_attr("aria-label".to_string(), self.title.clone());
        let mut i = 0_i32;
        while i < (self.fields.len() as i32) {
            form = form.child(self.field_view(i));
            i = i.wrapping_add(1);
        }
        let actions = VNode::div().add_class("wj-form-actions".to_string()).child(VNode::button().add_class("wj-button wj-button-primary".to_string()).set_id(part_id(&self.base, "submit", 0_i32)).add_attr("type".to_string(), "submit".to_string()).add_attr("data-wj-act".to_string(), "submit".to_string()).add_text("Submit".to_string())).child(VNode::button().add_class("wj-button".to_string()).set_id(part_id(&self.base, "reset", 0_i32)).add_attr("type".to_string(), "reset".to_string()).add_attr("data-wj-act".to_string(), "reset".to_string()).add_text("Reset".to_string()));
        let status = VNode::div().add_class("wj-form-status".to_string()).set_id(part_id(&self.base, "status", 0_i32)).add_attr("role".to_string(), "status".to_string()).add_attr("aria-live".to_string(), "polite".to_string()).add_text(self.status.clone());
        form.child(actions).child(status)
}
}

impl FormKit {

pub fn field_view(&self, i: i32) -> VNode {
        let f = &self.fields[i as usize];
        let id = part_id(&self.base, "f", i);
        let err_id = part_id(&self.base, "err", i);
        let hint_id = part_id(&self.base, "hint", i);
        let message = f.error_message();
        let shown = !message.is_empty() && (f.touched || self.attempted);
        let act = format!("field:{}", i);
        let mut described = String::new();
        if !f.hint.is_empty() {
            described = hint_id;
        }
        if shown {
            described = {
                if !described.is_empty() {
                    format!("{} {}", described, err_id)
                } else {
                    err_id
                }
            };
        }
        let label_text = {
            if f.required {
                format!("{} *", f.label)
            } else {
                f.label.clone()
            }
        };
        let mut wrap = VNode::div().add_class("wj-field".to_string());
        let is_group = match f.kind {
            FieldKind::Radio => true,
            _ => false,
        };
        if is_group {
            let mut group = VNode::element("fieldset".to_string()).add_class("wj-fieldset".to_string()).set_id(id.clone()).add_attr("aria-required".to_string(), aria_bool(f.required)).add_attr("aria-invalid".to_string(), aria_bool(shown)).child(VNode::element("legend".to_string()).add_text(label_text.to_string()));
            if !described.is_empty() {
                group = group.add_attr("aria-describedby".to_string(), described.clone());
            }
            let mut j = 0_usize;
            while j < f.options.len() {
                let opt_id = format!("{}-opt-{}", id, j);
                let mut radio = VNode::input().set_id(opt_id.to_string()).set_type("radio".to_string()).add_attr("name".to_string(), f.name.clone()).set_value(f.options[j].clone()).add_attr("data-wj-act".to_string(), act.to_string());
                if f.value == f.options[j] {
                    radio = radio.add_attr("checked".to_string(), "true".to_string());
                }
                group = { group.child({ let _temp0 = format!("{}-opt-{}", id, j); VNode::element("label".to_string()).add_attr("for".to_string(), _temp0) }.child(radio).child(VNode::span().add_text(f.option_labels[j].clone()))) };
                j = j.wrapping_add(1);
            }
            wrap = wrap.child(group);
        } else {
            let label = VNode::element("label".to_string()).add_attr("for".to_string(), id.clone()).add_text(label_text.to_string());
            let mut control = match f.kind {
                FieldKind::Select => {
                    let mut sel = VNode::element("select".to_string()).set_value(f.value.clone());
                    sel = sel.child(VNode::element("option".to_string()).set_value("".to_string()).add_text("Select...".to_string()));
                    let mut j = 0_usize;
                    while j < f.options.len() {
                        let mut opt = VNode::element("option".to_string()).set_value(f.options[j].clone()).add_text(f.option_labels[j].clone());
                        if f.value == f.options[j] {
                            opt = opt.add_attr("selected".to_string(), "true".to_string());
                        }
                        sel = sel.child(opt);
                        j = j.wrapping_add(1);
                    }
                    sel
                },
                FieldKind::Checkbox => {
                    let mut cb = VNode::input().set_type("checkbox".to_string());
                    if f.value == "true" {
                        cb = cb.add_attr("checked".to_string(), "true".to_string());
                    }
                    cb
                },
                FieldKind::Number => VNode::input().set_type("number".to_string()).set_value(f.value.clone()),
                FieldKind::Date => VNode::input().set_type("date".to_string()).set_value(f.value.clone()),
                FieldKind::Email => VNode::input().set_type("email".to_string()).set_value(f.value.clone()),
                _ => VNode::input().set_type("text".to_string()).set_value(f.value.clone()),
            };
            control = control.set_id(id).add_attr("name".to_string(), f.name.clone()).add_attr("aria-required".to_string(), aria_bool(f.required)).add_attr("aria-invalid".to_string(), aria_bool(shown)).add_attr("data-wj-act".to_string(), act.to_string());
            if !described.is_empty() {
                control = control.add_attr("aria-describedby".to_string(), described);
            }
            wrap = wrap.child(label).child(control);
        }
        if !f.hint.is_empty() {
            wrap = wrap.child(VNode::element("p".to_string()).set_id(part_id(&self.base, "hint", i)).add_class("wj-hint".to_string()).add_text(f.hint.clone()));
        }
        if shown {
            wrap = wrap.child(VNode::element("p".to_string()).set_id(part_id(&self.base, "err", i)).add_class("wj-error".to_string()).add_text(message));
        }
        wrap
}
}

