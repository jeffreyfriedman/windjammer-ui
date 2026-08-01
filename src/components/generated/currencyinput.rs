#![allow(clippy::all)]
#![allow(noop_method_call)]
//! Regenerated — Windjammer is source of truth.
//! Note: avoid `use super::*` (ambiguous glob imports under wasm deny).
use super::moneydisplay::format_cents;
use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct CurrencyInput {
    pub name: String,
    pub input_id: String,
    pub label: String,
    pub currency: String,
    pub value_cents: i64,
    pub required: bool,
    pub extra_attrs: String,
}

impl CurrencyInput {
    #[inline]
    pub fn new() -> CurrencyInput {
        CurrencyInput {
            name: "amount".to_string(),
            input_id: "".to_string(),
            label: "".to_string(),
            currency: "USD".to_string(),
            value_cents: 0_i64,
            required: false,
            extra_attrs: "".to_string(),
        }
    }
    #[inline]
    pub fn name(mut self, name: String) -> CurrencyInput {
        self.name = name;
        self
    }
    #[inline]
    pub fn input_id(mut self, input_id: String) -> CurrencyInput {
        self.input_id = input_id;
        self
    }
    #[inline]
    pub fn label(mut self, label: String) -> CurrencyInput {
        self.label = label;
        self
    }
    #[inline]
    pub fn currency(mut self, currency: String) -> CurrencyInput {
        self.currency = currency;
        self
    }
    #[inline]
    pub fn value_cents(mut self, value_cents: i64) -> CurrencyInput {
        self.value_cents = value_cents;
        self
    }
    #[inline]
    pub fn required(mut self, required: bool) -> CurrencyInput {
        self.required = required;
        self
    }
    #[inline]
    pub fn extra_attrs(mut self, extra_attrs: String) -> CurrencyInput {
        self.extra_attrs = extra_attrs;
        self
    }
}

impl Renderable for CurrencyInput {
    #[inline]
    fn render(&self) -> String {
        let value = format_cents(self.value_cents);
        let id = {
            if self.input_id.is_empty() {
                self.name.clone()
            } else {
                self.input_id.clone()
            }
        };
        let req = {
            if self.required {
                String::from(" required")
            } else {
                String::new()
            }
        };
        let extra = {
            if self.extra_attrs.is_empty() {
                "".to_string()
            } else {
                " ".to_string() + &self.extra_attrs.clone()
            }
        };
        let label_html: String = {
            if self.label.is_empty() {
                "".to_string()
            } else {
                format!(
                    "<label class='wj-currency-input-label' for='{}'>{}</label>",
                    id,
                    self.label.clone()
                )
            }
        };
        format!("<div class='wj-currency-input' data-currency='{}'>{}<span class='wj-currency-input-code'>{}</span><input type='text' inputmode='decimal' class='wj-currency-input-field' id='{}' name='{}' value='{}'{}{} /></div>", self.currency.clone(), label_html, self.currency.clone(), id, self.name.clone(), value, extra, req)
    }
}
