#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::moneydisplay::format_cents;
use super::traits::Renderable;

/// Mirrors `components_wj/currencyinput.wj`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct CurrencyInput {
    pub name: String,
    pub label: String,
    pub currency: String,
    pub value_cents: i64,
    pub required: bool,
}

impl CurrencyInput {
    #[inline]
    pub fn new() -> CurrencyInput {
        CurrencyInput {
            name: String::from("amount"),
            label: String::new(),
            currency: String::from("USD"),
            value_cents: 0,
            required: false,
        }
    }

    #[inline]
    pub fn name(mut self, name: impl Into<String>) -> CurrencyInput {
        self.name = name.into();
        self
    }

    #[inline]
    pub fn label(mut self, label: impl Into<String>) -> CurrencyInput {
        self.label = label.into();
        self
    }

    #[inline]
    pub fn currency(mut self, currency: impl Into<String>) -> CurrencyInput {
        self.currency = currency.into();
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
}

impl Renderable for CurrencyInput {
    #[inline]
    fn render(&self) -> String {
        let value = format_cents(self.value_cents);
        let req = if self.required { " required" } else { "" };
        let label_html = if self.label.is_empty() {
            String::new()
        } else {
            format!(
                "<label class='wj-currency-input-label' for='{}'>{}</label>",
                self.name, self.label
            )
        };
        format!(
            "<div class='wj-currency-input' data-currency='{}'>{}<span class='wj-currency-input-code'>{}</span><input type='text' inputmode='decimal' class='wj-currency-input-field' id='{}' name='{}' value='{}'{} /></div>",
            self.currency,
            label_html,
            self.currency,
            self.name,
            self.name,
            value,
            req
        )
    }
}
