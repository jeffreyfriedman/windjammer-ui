//! ReconcileActions — bank recon strip / queue / finish / export runtime.
//! Hand-maintained. Always SKIP_WJ_REGEN=1.
//! `.wj` source: `components_wj/reconcileactions.wj` (structural marker).
//! Product markup: finance-screens `panels.wj` (`data-wj-reconcile-*`).

/// Framework runtime: apply / clear-all / finish / handoff / CSV / PDF / queue select.
pub fn reconcile_actions_runtime_js() -> &'static str {
    r##"
(function () {
  if (window.__wjReconcileFinishBound) return;
  window.__wjReconcileFinishBound = true;
  function reloadReconSurfaces() {
    function reload(el) {
      if (!el) return;
      if (typeof window.wjAuthFetch === 'function') {
        try { window.wjAuthFetch(el); } catch (e) {}
      } else {
        try { el.click(); } catch (e) {}
      }
    }
    reload(document.getElementById('loadBankRecon')
      || document.querySelector('[data-wj-auth-fetch][data-wj-render-kind="bank-reconciliation"]'));
    reload(document.getElementById('loadBankReconHistory')
      || document.querySelector('[data-wj-auth-fetch][data-wj-render-kind="bank-reconciliation-history"]'));
    reload(document.getElementById('loadBankReconReport')
      || document.querySelector('[data-wj-auth-fetch][data-wj-render-kind="bank-reconciliation-report"]'));
    reload(document.getElementById('loadBankReconQueue')
      || document.querySelector('[data-wj-auth-fetch][data-wj-render-kind="bank-reconciliation-queue"]'));
    reload(document.getElementById('loadAccountRail')
      || document.querySelector('[data-wj-auth-fetch][data-wj-render-kind="account-rail"]'));
    reload(document.getElementById('loadCheckbook')
      || document.querySelector('[data-wj-auth-fetch][data-wj-render-kind="checkbook"]'));
    reload(document.getElementById('loadBank')
      || document.querySelector('[data-wj-auth-fetch][data-wj-render-kind="bank"]'));
  }
  function selectReconAccount(code) {
    if (!code) return;
    var railBtn = document.querySelector('[data-wj-account-rail-item][data-code="' + code + '"]');
    if (railBtn) {
      try { railBtn.click(); } catch (e) {}
    }
  }
  function applyScopeDenied(res, data) {
    if (typeof window.wjHandleScopeDeniedHint === 'function') {
      window.wjHandleScopeDeniedHint(res.status, data || {});
    } else {
      var show = res.status === 403 && String(((data && (data.error || data.message)) || '')).toLowerCase().indexOf('grant scope') >= 0;
      document.querySelectorAll('[data-wj-scope-denied]').forEach(function (el) {
        el.hidden = !show;
      });
    }
  }
  function downloadReconExport(res, filename) {
    if (!res.ok) {
      return res.json().then(function (data) {
        applyScopeDenied(res, data);
      }).catch(function () {
        applyScopeDenied(res, {});
      });
    }
    applyScopeDenied(res, {});
    return res.blob().then(function (blob) {
      var url = URL.createObjectURL(blob);
      var a = document.createElement('a');
      a.href = url;
      a.download = filename;
      document.body.appendChild(a);
      a.click();
      a.remove();
      URL.revokeObjectURL(url);
    });
  }
  document.addEventListener('click', function (ev) {
    var t = ev.target;
    if (!t || !t.closest) return;
    var api = window.LEDGERKIT_API || '';
    var token = localStorage.getItem('ledgerkit_token') || '';

    var queuePick = t.closest('[data-wj-reconcile-queue-select],[data-wj-reconcile-start-next]');
    if (queuePick) {
      ev.preventDefault();
      selectReconAccount(queuePick.getAttribute('data-account') || '');
      return;
    }

    var applyBtn = t.closest('[data-wj-reconcile-apply]');
    if (applyBtn) {
      ev.preventDefault();
      if (!token) return;
      var account = applyBtn.getAttribute('data-account') || '1000';
      var strip = applyBtn.closest('.wj-reconcile-strip') || document;
      var input = strip.querySelector('[data-wj-reconcile-statement][data-account="' + account + '"]')
        || strip.querySelector('[data-wj-reconcile-statement]');
      if (!input) return;
      var dollars = parseFloat(input.value);
      if (isNaN(dollars)) return;
      var cents = Math.round(dollars * 100);
      fetch(api + '/api/v1/bank-reconciliation?account=' + encodeURIComponent(account), {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json', Authorization: 'Bearer ' + token },
        body: JSON.stringify({ statement_ending_balance_cents: cents })
      }).then(function (res) {
        if (!res.ok) return;
        reloadReconSurfaces();
      }).catch(function () {});
      return;
    }

    var clearAllBtn = t.closest('[data-wj-reconcile-clear-all]');
    if (clearAllBtn) {
      if (clearAllBtn.disabled) return;
      ev.preventDefault();
      if (!token) return;
      var clearAccount = clearAllBtn.getAttribute('data-account') || '1000';
      fetch(api + '/api/v1/bank-reconciliation/clear-all?account=' + encodeURIComponent(clearAccount), {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', Authorization: 'Bearer ' + token },
        body: '{}'
      }).then(function (res) {
        if (!res.ok) return;
        reloadReconSurfaces();
      }).catch(function () {});
      return;
    }

    var btn = t.closest('[data-wj-reconcile-finish]');
    if (btn && !btn.disabled) {
      ev.preventDefault();
      if (!token) return;
      var finishAccount = btn.getAttribute('data-account') || '1000';
      var asOf = new Date().toISOString().slice(0, 10);
      fetch(api + '/api/v1/bank-reconciliation/finish?account=' + encodeURIComponent(finishAccount), {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', Authorization: 'Bearer ' + token },
        body: JSON.stringify({ as_of: asOf })
      }).then(function (res) {
        if (!res.ok) return;
        return res.json().then(function (summary) {
          reloadReconSurfaces();
          var mount = document.getElementById('reconHandoffMount');
          if (mount && window.__wjRenderReadModel) {
            try {
              mount.innerHTML = window.__wjRenderReadModel(
                'bank-reconciliation-handoff',
                JSON.stringify(summary || {})
              );
            } catch (e) {}
          }
        });
      }).catch(function () {});
      return;
    }

    var handoffMem = t.closest('[data-wj-reconcile-handoff-memorized]');
    if (handoffMem) {
      ev.preventDefault();
      var title = handoffMem.getAttribute('data-title') || '';
      var body = handoffMem.getAttribute('data-body') || '';
      var tab = document.querySelector('[data-wj-bw-tab="memorized"]');
      if (tab) { try { tab.click(); } catch (e) {} }
      if (typeof window.wjMemorizedPrefill === 'function') {
        try { window.wjMemorizedPrefill(title, body); } catch (e) {}
      } else {
        var titleEl = document.getElementById('memTitle');
        var bodyEl = document.getElementById('memBody');
        var details = document.querySelector('[data-wj-memorized-list] details');
        if (details) details.open = true;
        if (titleEl) titleEl.value = title;
        if (bodyEl) bodyEl.value = body;
      }
      return;
    }

    var handoffDismiss = t.closest('[data-wj-reconcile-handoff-dismiss]');
    if (handoffDismiss) {
      ev.preventDefault();
      var handoff = handoffDismiss.closest('[data-wj-reconcile-handoff]');
      if (handoff) handoff.remove();
      var handoffMount = document.getElementById('reconHandoffMount');
      if (handoffMount) handoffMount.innerHTML = '';
      return;
    }

    var csvBtn = t.closest('[data-wj-reconcile-csv]');
    if (csvBtn) {
      ev.preventDefault();
      if (!token) return;
      var csvAccount = csvBtn.getAttribute('data-account') || '1000';
      fetch(api + '/api/v1/bank-reconciliation/report?account=' + encodeURIComponent(csvAccount) + '&format=csv', {
        method: 'GET',
        headers: { Authorization: 'Bearer ' + token }
      }).then(function (res) {
        return downloadReconExport(res, 'recon-' + csvAccount + '.csv');
      }).catch(function () {});
      return;
    }

    var pdfBtn = t.closest('[data-wj-reconcile-pdf]');
    if (pdfBtn) {
      ev.preventDefault();
      if (!token) return;
      var pdfAccount = pdfBtn.getAttribute('data-account') || '1000';
      fetch(api + '/api/v1/bank-reconciliation/report?account=' + encodeURIComponent(pdfAccount) + '&format=pdf', {
        method: 'GET',
        headers: { Authorization: 'Bearer ' + token }
      }).then(function (res) {
        return downloadReconExport(res, 'recon-' + pdfAccount + '.pdf');
      }).catch(function () {});
    }
  });
})();
"##
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reconcile_actions_runtime_covers_strip_and_exports() {
        let js = reconcile_actions_runtime_js();
        assert!(js.contains("__wjReconcileFinishBound"));
        assert!(js.contains("data-wj-reconcile-finish"));
        assert!(js.contains("data-wj-reconcile-apply"));
        assert!(js.contains("data-wj-reconcile-clear-all"));
        assert!(js.contains("data-wj-reconcile-csv"));
        assert!(js.contains("data-wj-reconcile-pdf"));
        assert!(js.contains("data-wj-reconcile-handoff-memorized"));
        assert!(js.contains("bank-reconciliation/finish"));
    }

    #[test]
    fn reconcile_export_runtime_toggles_scope_denied_on_403() {
        let js = reconcile_actions_runtime_js();
        assert!(
            js.contains("data-wj-scope-denied"),
            "CSV/PDF 403 must toggle grant-scope hint slot: {js}"
        );
        assert!(
            js.contains("wjHandleScopeDeniedHint") || js.contains("grant scope"),
            "recon export must classify grant-scope 403: {js}"
        );
        assert!(js.contains("format=csv"), "csv download: {js}");
        assert!(js.contains("format=pdf"), "pdf download: {js}");
    }
}
