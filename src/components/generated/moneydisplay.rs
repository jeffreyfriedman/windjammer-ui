#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct MoneyDisplay {
    pub amount_cents: i64,
    pub currency: String,
    pub show_currency: bool,
}

impl MoneyDisplay {
    #[inline]
    pub fn new(amount_cents: i64) -> MoneyDisplay {
        MoneyDisplay {
            amount_cents,
            currency: "USD".to_string(),
            show_currency: true,
        }
    }
    #[inline]
    pub fn currency(mut self, currency: String) -> MoneyDisplay {
        self.currency = currency;
        self
    }
    #[inline]
    pub fn show_currency(mut self, show: bool) -> MoneyDisplay {
        self.show_currency = show;
        self
    }
}

impl Renderable for MoneyDisplay {
    #[inline]
    fn render(&self) -> String {
        let formatted = format_cents(self.amount_cents);
        let class: String = {
            if (self.amount_cents as i64) < 0_i64 {
                String::from("wj-money wj-money-negative")
            } else {
                String::from("wj-money")
            }
        };
        if self.show_currency {
            format!(
                "<span class='{}' data-currency='{}'>{} {}</span>",
                class,
                self.currency.clone(),
                self.currency.clone(),
                formatted
            )
        } else {
            format!("<span class='{}'>{}</span>", class, formatted)
        }
    }
}

#[inline]
pub fn format_cents(amount_cents: i64) -> String {
    let negative = (amount_cents as i64) < 0_i64;
    let mut cents = amount_cents;
    if negative {
        cents = -cents;
    }
    let dollars = cents as i64 / 100_i64;
    let rem = cents as i64 % 100_i64;
    let mut rem_s = "".to_string();
    if (rem as i64) < 10_i64 {
        rem_s = format!("0{}", rem);
    } else {
        rem_s = format!("{}", rem);
    }
    let body = format!("{}.{}", dollars, rem_s);
    if negative {
        format!("-{}", body)
    } else {
        body
    }
}
