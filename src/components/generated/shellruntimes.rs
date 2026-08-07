//! ShellRuntimes — packs LedgerKit-oriented WJ-UI host runtimes (P3.50).
//! Hand-maintained. Always SKIP_WJ_REGEN=1.
//! `.wj` source: `components_wj/shellruntimes.wj` (structural marker).
//! Product host wires this once instead of concatenating individual runtimes.

use super::{
    accountrail, approvalcard, authfetch, bankmatch, businessworkspace, checkbookregister,
    commandpalette, jsonpost, loginform, memorizedlist, reconcileactions, shellchrome,
    writecheckform,
};

/// Concatenated host JS for shell chrome + finance interaction runtimes.
pub fn shell_product_runtimes_js() -> String {
    [
        shellchrome::shell_chrome_runtime_js(),
        commandpalette::command_palette_runtime_js(),
        authfetch::auth_fetch_runtime_js(),
        memorizedlist::memorized_list_runtime_js(),
        businessworkspace::business_workspace_runtime_js(),
        accountrail::account_rail_runtime_js(),
        writecheckform::write_check_form_runtime_js(),
        bankmatch::bank_match_runtime_js(),
        checkbookregister::checkbook_register_runtime_js(),
        reconcileactions::reconcile_actions_runtime_js(),
        approvalcard::approval_card_runtime_js(),
        jsonpost::json_post_runtime_js(),
        loginform::login_form_runtime_js(),
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_product_runtimes_js_covers_palette_auth_and_recon() {
        let js = shell_product_runtimes_js();
        assert!(js.contains("function lkPersona(") || js.contains("lkApplyShellNav"));
        assert!(js.contains("wjCmdOpen") || js.contains("function wjCmdOpen"));
        assert!(js.contains("wjAuthFetch") || js.contains("lkSetSyncStatus"));
        assert!(js.contains("__wjBankMatchBound") || js.contains("data-wj-bank-match"));
        assert!(js.contains("__wjJsonPostBound"));
        assert!(js.contains("__wjLoginFormBound"));
    }
}
