//! TDD: packed LedgerKit-oriented WJ-UI host runtimes (P3.50).
//! Hand-maintained module: `src/components/generated/shellruntimes.rs`.

use windjammer_ui::components::generated::shellruntimes::shell_product_runtimes_js;

#[test]
fn shell_product_runtimes_js_includes_core_hosts() {
    let js = shell_product_runtimes_js();
    assert!(js.contains("lkApplyShellNav") || js.contains("function lkPersona"), "shell chrome: {js}");
    assert!(js.contains("wjCmdOpen") || js.contains("data-wj-cmd"), "command palette: {js}");
    assert!(js.contains("wjAuthFetch") || js.contains("lkSetSyncStatus"), "auth fetch: {js}");
    assert!(
        js.contains("__wjBankMatchBound") || js.contains("data-wj-bank-match"),
        "bank match: {js}"
    );
    assert!(
        js.contains("__wjReconcile") || js.contains("data-wj-reconcile"),
        "reconcile actions: {js}"
    );
    assert!(js.contains("__wjJsonPostBound"), "json post: {js}");
    assert!(js.contains("__wjLoginFormBound"), "login form: {js}");
}
