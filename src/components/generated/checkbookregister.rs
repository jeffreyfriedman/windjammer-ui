//! CheckbookRegister — spent/received register (LedgerKit R0+ / ADR-002 Business mode).
//! Hand-maintained. Always SKIP_WJ_REGEN=1.

use super::traits::Renderable;

#[derive(Clone, Debug)]
pub struct CheckbookRow {
    pub date: String,
    pub num: String,
    pub payee: String,
    pub spent_html: String,
    pub received_html: String,
    pub balance_html: String,
}

impl CheckbookRow {
    pub fn new(
        date: impl Into<String>,
        num: impl Into<String>,
        payee: impl Into<String>,
        spent_html: impl Into<String>,
        received_html: impl Into<String>,
        balance_html: impl Into<String>,
    ) -> Self {
        Self {
            date: date.into(),
            num: num.into(),
            payee: payee.into(),
            spent_html: spent_html.into(),
            received_html: received_html.into(),
            balance_html: balance_html.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CheckbookRegister {
    pub account_label: String,
    pub empty_message: String,
    pub mount_id: String,
    pub rows: Vec<CheckbookRow>,
}

impl CheckbookRegister {
    pub fn new() -> Self {
        Self {
            account_label: "Operating cash".to_string(),
            empty_message: "No register lines yet — write a check or import bank lines.".to_string(),
            mount_id: "registerMount".to_string(),
            rows: Vec::new(),
        }
    }

    pub fn account_label(mut self, label: impl Into<String>) -> Self {
        self.account_label = label.into();
        self
    }

    pub fn empty_message(mut self, message: impl Into<String>) -> Self {
        self.empty_message = message.into();
        self
    }

    pub fn mount_id(mut self, id: impl Into<String>) -> Self {
        self.mount_id = id.into();
        self
    }

    pub fn row(mut self, row: CheckbookRow) -> Self {
        self.rows.push(row);
        self
    }
}

impl Default for CheckbookRegister {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for CheckbookRegister {
    fn render(&self) -> String {
        let body = if self.rows.is_empty() {
            format!(
                r#"<tr class="lk-empty-row"><td colspan="6" class="muted">{}</td></tr>"#,
                self.empty_message
            )
        } else {
            self.rows
                .iter()
                .map(|r| {
                    format!(
                        r#"<tr class="wj-checkbook-row"><td>{}</td><td>{}</td><td>{}</td><td class="lk-num">{}</td><td class="lk-num">{}</td><td class="lk-num">{}</td></tr>"#,
                        r.date, r.num, r.payee, r.spent_html, r.received_html, r.balance_html
                    )
                })
                .collect::<Vec<_>>()
                .join("")
        };
        format!(
            r##"<div class="wj-checkbook-register" data-wj-checkbook-register>
  <div class="checkbook-head">
    <p class="hub-kicker">Checkbook</p>
    <h3 class="checkbook-account">{account}</h3>
  </div>
  <div class="lk-table-scroll">
    <table class="lk-table wj-checkbook-table" aria-label="Checkbook register">
      <thead>
        <tr>
          <th>Date</th>
          <th>Num</th>
          <th>Payee / memo</th>
          <th class="lk-num">Spent</th>
          <th class="lk-num">Received</th>
          <th class="lk-num">Balance</th>
        </tr>
      </thead>
      <tbody id="{mount}">
        {body}
      </tbody>
    </table>
  </div>
</div>"##,
            account = self.account_label,
            mount = self.mount_id,
            body = body
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkbook_register_has_spent_received_columns() {
        let html = CheckbookRegister::new().account_label("1000 · Cash").render();
        assert!(html.contains("wj-checkbook-register"));
        assert!(html.contains("Spent"));
        assert!(html.contains("Received"));
    }

    #[test]
    fn checkbook_register_renders_rows() {
        let html = CheckbookRegister::new()
            .row(CheckbookRow::new("2026-07-01", "", "FEE", "$45.00", "", ""))
            .render();
        assert!(html.contains("wj-checkbook-row"));
        assert!(html.contains("FEE"));
        assert!(!html.contains("lk-empty-row"));
    }
}
