//! ReadModelBridge — window.lkRender → WASM/WJ read-model renderers.
//! Hand-maintained. Always SKIP_WJ_REGEN=1.
//! `.wj` source: `components_wj/readmodelbridge.wj` (structural marker).
//! AuthFetch and product hosts call `window.lkRender[kind](data)`.

/// Framework runtime: thin lkRender stub; table HTML comes from WASM/WJ.
pub fn read_model_bridge_runtime_js() -> &'static str {
    r##"
/** F4: product table builders removed. Prefer make web-wasm (install_lk_render). */
window.lkRender = window.lkRender || {
  _missing(kind) {
    return '<p class="err">Renderer unavailable (' + kind + '). Use WASM boot: make web-wasm</p>';
  },
  parties(d) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('parties', JSON.stringify(d||{}))) || this._missing('parties'); },
  invoices(d) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('invoices', JSON.stringify(d||{}))) || this._missing('invoices'); },
  bills(d) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('bills', JSON.stringify(d||{}))) || this._missing('bills'); },
  bank(d) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('bank', JSON.stringify(d||{}))) || this._missing('bank'); },
  'bank-reconciliation'(d) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('bank-reconciliation', JSON.stringify(d||{}))) || this._missing('bank-reconciliation'); },
  'bank-reconciliation-history'(d) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('bank-reconciliation-history', JSON.stringify(d||{}))) || this._missing('bank-reconciliation-history'); },
  'bank-reconciliation-queue'(d) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('bank-reconciliation-queue', JSON.stringify(d||{}))) || this._missing('bank-reconciliation-queue'); },
  'bank-reconciliation-handoff'(d) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('bank-reconciliation-handoff', JSON.stringify(d||{}))) || this._missing('bank-reconciliation-handoff'); },
  'bank-reconciliation-report'(d) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('bank-reconciliation-report', JSON.stringify(d||{}))) || this._missing('bank-reconciliation-report'); },
  trialBalance(d) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('trialBalance', JSON.stringify(d||{}))) || this._missing('trialBalance'); },
  balanceSheet(d) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('balanceSheet', JSON.stringify(d||{}))) || this._missing('balanceSheet'); },
  incomeStatement(d) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('incomeStatement', JSON.stringify(d||{}))) || this._missing('incomeStatement'); },
  'compliance-dashboard'(d) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('compliance-dashboard', JSON.stringify(d||{}))) || this._missing('compliance-dashboard'); },
  'pending-approvals'(d) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('pending-approvals', JSON.stringify(d||{}))) || this._missing('pending-approvals'); },
  checkbook(data) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('checkbook', JSON.stringify(data||{}))) || this._missing('checkbook'); },
  accounts(d) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('accounts', JSON.stringify(d||{}))) || this._missing('accounts'); },
  'account-rail'(d) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('account-rail', JSON.stringify(d||{}))) || this._missing('account-rail'); },
  'journal-options'(d) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('journal-options', JSON.stringify(d||{}))) || this._missing('journal-options'); },
  'write-check-tags'(d) {
    if (window.wjWriteCheckApplyTags && d && d.tags) {
      try { window.wjWriteCheckApplyTags(d.tags); } catch (e) {}
    }
    return (window.__wjRenderReadModel && window.__wjRenderReadModel('write-check-tags', JSON.stringify(d||{}))) || '';
  },
  aging(d) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('aging', JSON.stringify(d||{}))) || this._missing('aging'); },
  migrations(d) { return (window.__wjRenderReadModel && window.__wjRenderReadModel('migrations', JSON.stringify(d||{}))) || this._missing('migrations'); },
  homeKpis(b,i,bi,t) {
    if (window.__wjRenderHomeKpis) return window.__wjRenderHomeKpis(JSON.stringify(b||{}), JSON.stringify(i||{}), JSON.stringify(bi||{}), JSON.stringify(t||{}));
    return this._missing('homeKpis');
  },
  tasksFromKpis(b,i,bi,t) {
    if (window.__wjRenderTasks) return window.__wjRenderTasks(JSON.stringify(b||{}), JSON.stringify(i||{}), JSON.stringify(bi||{}), JSON.stringify(t||{}));
    return this._missing('tasks');
  }
};
"##
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_model_bridge_covers_authfetch_kinds() {
        let js = read_model_bridge_runtime_js();
        assert!(js.contains("window.lkRender"));
        assert!(js.contains("__wjRenderReadModel"));
        assert!(js.contains("account-rail"));
        assert!(js.contains("journal-options"));
        assert!(js.contains("bank-reconciliation"));
        assert!(js.contains("write-check-tags"));
        assert!(js.contains("homeKpis"));
        assert!(js.contains("tasksFromKpis"));
        assert!(!js.contains("function lkMoney"));
        assert!(!js.contains("function lkTable"));
    }
}
