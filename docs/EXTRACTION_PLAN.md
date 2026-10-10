# Extraction plan: ledger-specific components

AGENTS.md says domain components belong in the consuming app, not here. Destination:
`financial-management/financial-management-platform/packages/finance-screens`
(it already depends on this crate via path). No code has been moved yet; this is the list.

## A. App-specific components (move to finance-screens)

Consumers are files in finance-screens `src/` or `tests/` that reference the component name.

| Component (`src/components_wj/`) | Consumers in finance-screens |
|---|---|
| checkbookregister, writecheckform | src/tables.wj, src/business_hosts.wj, tests/checkbook_html_test.rs, tests/business_hosts_html_test.rs |
| periodbadge | src/shell_chrome.wj, src/home.wj, tests/period_current_test.rs, tests/shell_chrome_html_test.rs |
| bankmatch | src/tables.wj, tests/bank_table_html_test.rs |
| approvalcard | src/panels.wj, src/control_panels.wj, tests/panels_html_test.rs, tests/control_panels_html_test.rs |
| compliancescorebadge | src/panels.wj, src/control_panels.wj |
| daystoclosemetric | src/home.wj |
| accountrail | src/tables.wj, src/business_hosts.wj, tests/account_rail_html_test.rs |
| statuschip (maps ledger statuses to Badge) | src/tables.wj, src/home.wj, tests/account_rail_html_test.rs |
| moneydisplay, currencyinput (cents formatting) | src/money.wj, tests/tables_html_test.rs (currencyinput: no consumer found) |
| kpitile ("finance KPI dogfood") | src/home.wj; could stay as a generic tile if renamed |
| shellnav, appshell, shellchrome, shellruntimes | src/shell_chrome.wj, tests/shell_chrome_html_test.rs, tests/wasm_boot_html_test.rs |
| syncbadge | src/shell_chrome.wj, tests/shell_chrome_html_test.rs |
| businessworkspace, memorizedlist | src/business_workspace.wj, src/business_hosts.wj |
| ledgertheme, reconcileactions, readmodelbridge | no direct consumer found by name; check `finance-ui` before deleting |
| loginform, authfetch, jsonpost (Bearer-token runtimes) | src/form_pages.wj, src/api.wj, src/tax_1099.wj, src/list_panels.wj, src/control_evidence.wj, src/agent_card.wj |
| hashrouter, shortcutchord | tests/spa_embed_maps_test.rs (hash routing is generic in principle; runtimes assume `LK_PAGES`/`LK_SHORTCUTS` globals, so treat as app code) |

Tests in this repo that go with them: `tests/finance_p0_components_test.rs`,
`tests/shell_product_runtimes_test.rs`, and ledger parts of `tests/generated_components_test.rs`.
No segregation-of-duties component exists by that name here; the SoD UI is in finance-screens.

## B. Hand-maintained "generated" files (17)

All under `src/components/generated/` and marked `Hand-maintained ... SKIP_WJ_REGEN=1`.
Regenerating them needs compiler work (framework JS helpers / runtime packing), so they
were left untouched:

accountrail.rs, authfetch.rs, bankmatch.rs, businessworkspace.rs, checkbookregister.rs,
hashrouter.rs, jsonpost.rs, ledgertheme.rs, lib.rs, loginform.rs, memorizedlist.rs, mod.rs,
readmodelbridge.rs, reconcileactions.rs, shellchrome.rs, shellruntimes.rs, shortcutchord.rs

Source `.wj` files also carrying the marker: checkbookregister.wj, writecheckform.wj.
Almost all of these are in table A, so extraction removes most of the divergence. `mod.rs`
and `lib.rs` need regeneration once the ledger modules are gone.

## C. Parked `.disabled` tests (not re-enabled)

tests/button_click_test.rs.disabled, crud_operations_test.rs.disabled,
dashboard_patterns_test.rs.disabled, form_validation_test.rs.disabled,
simple_button_test.rs.disabled, widget_integration_test.rs.disabled;
examples/serve_editor.rs.disabled, examples/simple_counter_wasm.rs.disabled.
They could not be validated without building. AGENTS.md forbids committed `.disabled`
tests, so each must be either rewritten against the current API or deleted.

## Steps

1. In finance-screens, copy each table-A `.wj` into a `components/` dir (and hand-maintained
   runtimes into its own crate), with the matching tests.
2. Switch finance-screens imports from `windjammer_ui::components::*` to local modules; run its tests.
3. Delete the copies here, together with `components/generated/{...}` entries and `mod.rs` lines.
4. Regenerate `mod.rs`/`lib.rs` with `wj`; confirm no `SKIP_WJ_REGEN` remains.
5. Resolve section C (rewrite or delete).
6. Add `WJ_PIN.md` (required by AGENTS.md; missing today) recording the known-good `wj` commit.
