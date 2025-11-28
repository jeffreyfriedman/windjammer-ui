#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq)]
pub enum AlertVariant {
    Error,
    Warning,
    Info,
    Success,
}

pub struct Alert {
    message: String,
    variant: AlertVariant,
}

impl Alert {
    #[inline]
    pub fn error(message: String) -> Alert {
        Alert {
            message,
            variant: AlertVariant::Error,
        }
    }
    #[inline]
    pub fn warning(message: String) -> Alert {
        Alert {
            message,
            variant: AlertVariant::Warning,
        }
    }
    #[inline]
    pub fn info(message: String) -> Alert {
        Alert {
            message,
            variant: AlertVariant::Info,
        }
    }
    #[inline]
    pub fn success(message: String) -> Alert {
        Alert {
            message,
            variant: AlertVariant::Success,
        }
    }
}

impl Renderable for Alert {
    #[inline]
    fn render(self) -> String {
        let variant_class = match self.variant {
            AlertVariant::Error => "wj-alert-error",
            AlertVariant::Warning => "wj-alert-warning",
            AlertVariant::Info => "wj-alert-info",
            AlertVariant::Success => "wj-alert-success",
        };
        let icon = match self.variant {
            AlertVariant::Error => "❌",
            AlertVariant::Warning => "⚠️",
            AlertVariant::Info => "ℹ️",
            AlertVariant::Success => "✅",
        };
        format!(
            "<div class='wj-alert {}'>{} {}</div>",
            variant_class, icon, self.message
        )
    }
}
