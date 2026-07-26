//! WriteCheckForm — Business write-check fields + JE sync (ADR-002 / R1.3 / R2.2).
//! Hand-maintained. Always SKIP_WJ_REGEN=1.

use super::traits::Renderable;

#[derive(Clone, Debug)]
pub struct WriteCheckForm {
    pub sample_body: String,
    pub bank_code: String,
    pub expense_code: String,
}

impl WriteCheckForm {
    pub fn new() -> Self {
        Self {
            sample_body: "{}".to_string(),
            bank_code: "1000".to_string(),
            expense_code: "5000".to_string(),
        }
    }

    pub fn sample_body(mut self, body: impl Into<String>) -> Self {
        self.sample_body = body.into();
        self
    }

    pub fn bank_code(mut self, code: impl Into<String>) -> Self {
        self.bank_code = code.into();
        self
    }

    pub fn expense_code(mut self, code: impl Into<String>) -> Self {
        self.expense_code = code.into();
        self
    }
}

impl Default for WriteCheckForm {
    fn default() -> Self {
        Self::new()
    }
}

fn expense_option(code: &str, label: &str, selected: &str) -> String {
    let sel = if code == selected { " selected" } else { "" };
    format!(
        r#"<option value="{code}"{sel}>{code} · {label}</option>"#,
        code = code,
        label = label,
        sel = sel
    )
}

impl Renderable for WriteCheckForm {
    fn render(&self) -> String {
        let expense = self.expense_code.as_str();
        let options = [
            expense_option("5000", "Payroll Expense", expense),
            expense_option("5100", "Office Supplies", expense),
            expense_option("5200", "Equipment Expense", expense),
            expense_option("5300", "Software & SaaS", expense),
            expense_option("5400", "Bank Fees", expense),
        ]
        .join("");
        format!(
            r##"<div class="wj-write-check-form" data-wj-write-check data-wj-bank-code="{bank}" data-wj-expense-code="{expense}">
<p class="hub-kicker">Write check</p>
<label for="checkPayee">Payee</label>
<input id="checkPayee" name="payee" type="text" placeholder="Vendor or payee" data-wj-write-check-payee/>
<label for="checkAmount">Amount</label>
<input id="checkAmount" name="amount" type="text" inputmode="decimal" placeholder="0.00" data-wj-write-check-amount/>
<label for="checkNum">Check number</label>
<input id="checkNum" name="check_number" type="text" value="1001" data-wj-write-check-num/>
<label for="checkExpense">Expense account</label>
<select id="checkExpense" name="expense_account" data-wj-write-check-expense>{options}</select>
<label for="checkMemo">Memo</label>
<input id="checkMemo" name="memo" type="text" placeholder="Optional memo" data-wj-write-check-memo/>
<details class="bw-advanced"><summary>Journal body (advanced)</summary>
<label for="checkJeBody">Journal body (editable)</label>
<textarea id="checkJeBody" rows="6" data-wj-write-check-body>{sample}</textarea>
</details>
<div class="row"><button type="button" class="btn-secondary" id="postCheck" data-wj-write-check-post>Post check</button></div>
<p id="out" class="lk-status" role="status" hidden></p>
</div>"##,
            bank = self.bank_code,
            expense = self.expense_code,
            options = options,
            sample = self.sample_body,
        )
    }
}

/// Sync visible fields → journal JSON; post; refresh register.
pub fn write_check_form_runtime_js() -> &'static str {
    r##"
(function () {
  if (window.__wjWriteCheckBound) return;
  window.__wjWriteCheckBound = true;

  function esc(s) {
    return String(s || '').replace(/\\/g, '\\\\').replace(/"/g, '\\"');
  }
  function centsFromAmount(raw) {
    var t = String(raw || '').replace(/[^0-9.]/g, '');
    if (!t) return 0;
    var n = Math.round(parseFloat(t) * 100);
    return isFinite(n) ? n : 0;
  }
  function today() {
    var d = new Date();
    var m = (d.getMonth() + 1);
    var day = d.getDate();
    return d.getFullYear() + '-' + (m < 10 ? '0' : '') + m + '-' + (day < 10 ? '0' : '') + day;
  }
  function activeBankCode(root) {
    var rail = document.querySelector('[data-wj-account-rail-item].is-active');
    if (rail && rail.getAttribute('data-code')) return rail.getAttribute('data-code');
    return (root && root.getAttribute('data-wj-bank-code')) || '1000';
  }
  function expenseCode(root) {
    var sel = root.querySelector('[data-wj-write-check-expense]');
    if (sel && sel.value) {
      root.setAttribute('data-wj-expense-code', sel.value);
      return sel.value;
    }
    return (root.getAttribute('data-wj-expense-code')) || '5000';
  }
  function buildBody(root) {
    var payee = (root.querySelector('[data-wj-write-check-payee]') || {}).value || '';
    var amount = (root.querySelector('[data-wj-write-check-amount]') || {}).value || '';
    var num = (root.querySelector('[data-wj-write-check-num]') || {}).value || '1001';
    var memo = (root.querySelector('[data-wj-write-check-memo]') || {}).value || '';
    var bank = activeBankCode(root);
    var expense = expenseCode(root);
    var cents = centsFromAmount(amount);
    if (!cents) throw new Error('Enter an amount greater than zero.');
    if (!payee.trim()) throw new Error('Enter a payee.');
    var desc = payee.trim();
    var chk = 'CHK ' + String(num).trim();
    return '{"reference":"CHK-' + esc(String(num).trim()) + '","transaction_date":"' + today()
      + '","memo":"' + esc(memo.trim() || desc) + '","lines":['
      + '{"account_code":"' + esc(expense) + '","amount_cents":' + cents + ',"description":"' + esc(desc) + '"},'
      + '{"account_code":"' + esc(bank) + '","amount_cents":' + (-cents) + ',"description":"' + esc(chk) + '"}'
      + ']}';
  }
  function syncBody(root) {
    var ta = root.querySelector('[data-wj-write-check-body]');
    if (!ta) return null;
    var body = buildBody(root);
    ta.value = body;
    return body;
  }
  function bumpCheckNum(root) {
    var el = root.querySelector('[data-wj-write-check-num]');
    if (!el) return;
    var n = parseInt(String(el.value || '1001').replace(/[^0-9]/g, ''), 10);
    if (!isFinite(n)) n = 1001;
    el.value = String(n + 1);
  }

  document.addEventListener('click', function (ev) {
    var t = ev.target;
    if (!t || !t.closest) return;
    var btn = t.closest('[data-wj-write-check-post]');
    if (!btn) return;
    var root = btn.closest('[data-wj-write-check]');
    if (!root) return;
    ev.preventDefault();
    var out = root.querySelector('#out') || document.getElementById('out');
    var tokenKey = 'ledgerkit_token';
    try {
      var body = syncBody(root);
      var tkn = localStorage.getItem(tokenKey) || '';
      if (!out) return;
      if (!tkn) { out.hidden = false; out.classList.add('is-error'); out.textContent = 'Sign in first.'; return; }
      out.hidden = false; out.classList.remove('is-error'); out.textContent = 'Posting check…';
      var api = window.LEDGERKIT_API || '';
      fetch(api + '/api/v1/journal-entries', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', Authorization: 'Bearer ' + tkn },
        body: body
      }).then(function (res) {
        if (!res.ok) { out.classList.add('is-error'); out.textContent = 'Could not post (' + res.status + ')'; return; }
        out.textContent = 'Check posted — register refreshing…';
        bumpCheckNum(root);
        var load = document.getElementById('loadCheckbook')
          || document.querySelector('[data-wj-render-kind="checkbook"]');
        if (load) { try { load.click(); } catch (e) {} }
      }).catch(function (err) {
        out.classList.add('is-error'); out.textContent = String(err);
      });
    } catch (err) {
      if (out) { out.hidden = false; out.classList.add('is-error'); out.textContent = String(err.message || err); }
    }
  });

  document.addEventListener('input', function (ev) {
    var t = ev.target;
    if (!t || !t.closest) return;
    var root = t.closest('[data-wj-write-check]');
    if (!root) return;
    if (!t.matches('[data-wj-write-check-payee],[data-wj-write-check-amount],[data-wj-write-check-num],[data-wj-write-check-memo],[data-wj-write-check-expense]')) return;
    try { syncBody(root); } catch (e) {}
  });
  document.addEventListener('change', function (ev) {
    var t = ev.target;
    if (!t || !t.closest) return;
    var root = t.closest('[data-wj-write-check]');
    if (!root) return;
    if (!t.matches('[data-wj-write-check-expense]')) return;
    try { syncBody(root); } catch (e) {}
  });
})();
"##
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_fields_and_hooks() {
        let html = WriteCheckForm::new()
            .sample_body(r#"{"reference":"CHK-1001"}"#)
            .expense_code("5100")
            .render();
        assert!(html.contains("wj-write-check-form"));
        assert!(html.contains("data-wj-write-check-payee"));
        assert!(html.contains("data-wj-write-check-post"));
        assert!(html.contains("data-wj-write-check-expense"));
        assert!(html.contains("checkExpense"));
        assert!(html.contains("value=\"5100\" selected") || html.contains("5100 · Office"));
        assert!(html.contains("checkJeBody"));
    }
}
