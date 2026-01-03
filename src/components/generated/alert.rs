#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum AlertVariant {
    Error,
    Warning,
    Info,
    Success,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alert {
    pub message: String,
    pub variant: AlertVariant,
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
            AlertVariant::Error => "wj-alert-error".to_string(),
            AlertVariant::Warning => "wj-alert-warning".to_string(),
            AlertVariant::Info => "wj-alert-info".to_string(),
            AlertVariant::Success => "wj-alert-success".to_string(),
        };
        let icon = match self.variant {
            AlertVariant::Error => "❌".to_string(),
            AlertVariant::Warning => "⚠️".to_string(),
            AlertVariant::Info => "ℹ️".to_string(),
            AlertVariant::Success => "✅".to_string(),
        };
        format!(
            "<div class='wj-alert {}'>{} {}</div>",
            variant_class, icon, self.message
        )
    }
}
