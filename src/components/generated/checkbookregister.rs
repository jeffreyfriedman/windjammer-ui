//! CheckbookRegister — spent/received register shell (LedgerKit R0 / ADR-002).
//! Hand-maintained until Windjammer compose codegen is green.
//! Source: `src/components_wj/checkbookregister.wj`. Always SKIP_WJ_REGEN=1.

use super::traits::Renderable;

#[derive(Clone, Debug)]
pub struct CheckbookRegister {
    pub account_label: String,
    pub empty_message: String,
    pub mount_id: String,
}

impl CheckbookRegister {
    pub fn new() -> Self {
        Self {
            account_label: "Operating cash".to_string(),
            empty_message: "No register lines yet — write a check or import bank lines."
                .to_string(),
            mount_id: "registerMount".to_string(),
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
}

impl Default for CheckbookRegister {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for CheckbookRegister {
    fn render(&self) -> String {
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
        <tr class="lk-empty-row"><td colspan="6" class="muted">{empty}</td></tr>
      </tbody>
    </table>
  </div>
</div>"##,
            account = self.account_label,
            mount = self.mount_id,
            empty = self.empty_message
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkbook_register_has_spent_received_columns() {
        let html = CheckbookRegister::new()
            .account_label("1000 · Cash")
            .render();
        assert!(html.contains("wj-checkbook-register"));
        assert!(html.contains("Spent"));
        assert!(html.contains("Received"));
        assert!(html.contains("1000 · Cash"));
    }
}
