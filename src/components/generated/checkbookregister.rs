//! CheckbookRegister — Amount + Balance register (LedgerKit ADR-002 / R2.6).
//! Hand-maintained. Always SKIP_WJ_REGEN=1.

use super::traits::Renderable;

#[derive(Clone, Debug)]
pub struct CheckbookRow {
    pub date: String,
    pub num: String,
    pub payee: String,
    pub amount_html: String,
    pub balance_html: String,
    pub line_id: String,
    pub unmatched: bool,
}

impl CheckbookRow {
    pub fn new(
        date: impl Into<String>,
        num: impl Into<String>,
        payee: impl Into<String>,
        amount_html: impl Into<String>,
        balance_html: impl Into<String>,
    ) -> Self {
        Self {
            date: date.into(),
            num: num.into(),
            payee: payee.into(),
            amount_html: amount_html.into(),
            balance_html: balance_html.into(),
            line_id: String::new(),
            unmatched: false,
        }
    }

    pub fn line_id(mut self, id: impl Into<String>) -> Self {
        self.line_id = id.into();
        self
    }

    pub fn unmatched(mut self, unmatched: bool) -> Self {
        self.unmatched = unmatched;
        self
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
                r#"<tr class="lk-empty-row"><td colspan="5" class="muted">{}</td></tr>"#,
                self.empty_message
            )
        } else {
            self.rows
                .iter()
                .map(|r| {
                    let payee = if r.unmatched && !r.line_id.is_empty() {
                        format!(
                            r#"{} <button type="button" class="btn-link" data-wj-register-match data-line-id="{id}">Match</button>"#,
                            r.payee,
                            id = r.line_id
                        )
                    } else {
                        r.payee.clone()
                    };
                    format!(
                        r#"<tr class="wj-checkbook-row" data-line-id="{line}"><td>{date}</td><td>{num}</td><td>{payee}</td><td class="lk-num">{amount}</td><td class="lk-num">{bal}</td></tr>"#,
                        line = r.line_id,
                        date = r.date,
                        num = r.num,
                        payee = payee,
                        amount = r.amount_html,
                        bal = r.balance_html
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
          <th class="lk-num">Amount</th>
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

/// Register → bank Match deep-link (R2.5). Scrolls/focuses the bank Match cell for the line.
pub fn checkbook_register_runtime_js() -> &'static str {
    r##"
(function () {
  if (window.__wjCheckbookMatchLinkBound) return;
  window.__wjCheckbookMatchLinkBound = true;

  document.addEventListener('click', function (ev) {
    var t = ev.target;
    if (!t || !t.closest) return;
    var btn = t.closest('[data-wj-register-match]');
    if (!btn) return;
    ev.preventDefault();
    var lineId = btn.getAttribute('data-line-id') || '';
    if (!lineId) return;
    var bank = document.querySelector('.wj-bw-bank');
    if (bank && bank.scrollIntoView) {
      try { bank.scrollIntoView({ behavior: 'smooth', block: 'nearest' }); } catch (e) { bank.scrollIntoView(); }
    }
    var matchBtn = null;
    document.querySelectorAll('[data-wj-bank-match][data-line-id]').forEach(function (el) {
      if (!matchBtn && el.getAttribute('data-line-id') === lineId) matchBtn = el;
    });
    var cell = matchBtn ? (matchBtn.closest('[data-wj-bank-match-cell]') || matchBtn.parentElement) : null;
    document.querySelectorAll('.wj-bank-match-action.is-focus').forEach(function (el) {
      el.classList.remove('is-focus');
    });
    if (cell) {
      cell.classList.add('is-focus');
      var sel = cell.querySelector('[data-wj-bank-match-je]');
      if (sel && sel.focus) { try { sel.focus(); } catch (e) {} }
      if (matchBtn && matchBtn.focus) { try { matchBtn.focus(); } catch (e) {} }
    }
  });
})();
"##
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkbook_register_has_amount_balance_columns() {
        let html = CheckbookRegister::new().account_label("1000 · Cash").render();
        assert!(html.contains("wj-checkbook-register"));
        assert!(html.contains("Amount"));
        assert!(html.contains("Balance"));
        assert!(!html.contains(">Spent<"));
        assert!(!html.contains(">Received<"));
    }

    #[test]
    fn checkbook_register_renders_rows() {
        let html = CheckbookRegister::new()
            .row(CheckbookRow::new("2026-07-01", "", "FEE", "-45.00", "100.00"))
            .render();
        assert!(html.contains("wj-checkbook-row"));
        assert!(html.contains("FEE"));
        assert!(!html.contains("lk-empty-row"));
    }

    #[test]
    fn unmatched_row_emits_register_match_link() {
        let html = CheckbookRegister::new()
            .row(
                CheckbookRow::new("2026-07-01", "", "OFFICE DEPOT", "-45.00", "100.00")
                    .line_id("bank~1000~demo-fit-01")
                    .unmatched(true),
            )
            .render();
        assert!(html.contains("data-wj-register-match"));
        assert!(html.contains("data-line-id=\"bank~1000~demo-fit-01\""));
        assert!(html.contains("Match"));
    }

    #[test]
    fn runtime_focuses_bank_match_cell() {
        let js = checkbook_register_runtime_js();
        assert!(js.contains("data-wj-register-match"));
        assert!(js.contains("data-wj-bank-match"));
        assert!(js.contains("scrollIntoView") || js.contains("is-focus"));
    }
}
