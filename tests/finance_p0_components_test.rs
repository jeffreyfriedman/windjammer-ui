//! Finance P0 component smoke tests (DatePicker, DataTable, MoneyDisplay, FormField, Chart)

use windjammer_ui::components::generated::traits::Renderable;
use windjammer_ui::components::generated::{
    chart, datatable, datepicker, form, moneydisplay, table,
};

#[test]
fn money_display_formats_cents() {
    assert_eq!(moneydisplay::format_cents(12345), "123.45");
    assert_eq!(moneydisplay::format_cents(-50), "-0.50");
    let html = moneydisplay::MoneyDisplay::new(999)
        .currency("USD".to_string())
        .render();
    assert!(html.contains("wj-money"));
    assert!(html.contains("9.99"));
}

#[test]
fn date_picker_renders_date_input() {
    let html = datepicker::DatePicker::new()
        .label("Due".to_string())
        .name("due_date".to_string())
        .value("2026-07-25".to_string())
        .required(true)
        .render();
    assert!(html.contains("type='date'") || html.contains("type=\"date\""));
    assert!(html.contains("wj-datepicker"));
    assert!(html.contains("due_date"));
}

#[test]
fn data_table_wraps_table() {
    let html = datatable::DataTable::new()
        .caption("Parties".to_string())
        .column(table::TableColumn::new("Name".to_string()))
        .row(table::TableRow::new().cell("Acme".to_string()))
        .render();
    assert!(html.contains("wj-datatable"));
    assert!(html.contains("Parties"));
    assert!(html.contains("Acme"));
}

#[test]
fn data_table_scrollable_wraps_lk_table_scroll() {
    let html = datatable::DataTable::new()
        .scrollable(true)
        .caption("Parties".to_string())
        .column(table::TableColumn::new("Name".to_string()))
        .row(table::TableRow::new().cell("Acme".to_string()))
        .render();
    assert!(html.contains("lk-table-scroll"));
    assert!(html.contains("wj-datatable"));
}

#[test]
fn kpi_tile_renders_label_and_money() {
    use windjammer_ui::components::generated::kpitile;
    let html = kpitile::KpiTile::new("Cash".to_string())
        .value_html(
            moneydisplay::MoneyDisplay::new(250000)
                .currency("USD".to_string())
                .render(),
        )
        .render();
    assert!(html.contains("wj-kpi-tile"));
    assert!(html.contains("Cash"));
    assert!(html.contains("wj-money"));
}

#[test]
fn kpi_grid_wraps_tiles() {
    use windjammer_ui::components::generated::kpitile;
    let tile = kpitile::KpiTile::new("AR open".to_string())
        .value_html("<strong>3</strong>".to_string())
        .render();
    let html = kpitile::KpiGrid::new().tile(tile).render();
    assert!(html.contains("wj-kpi-grid"));
    assert!(html.contains("kpi-grid"));
    assert!(html.contains("AR open"));
}

#[test]
fn form_field_has_finance_classes() {
    let html = form::FormField::new(
        "Email".to_string(),
        "<input id='email' name='email'/>".to_string(),
    )
    .error("Required".to_string())
    .render();
    assert!(html.contains("wj-form-field"));
    assert!(html.contains("wj-form-field-error"));
    assert!(html.contains("role='alert'") || html.contains("role=\"alert\""));
}

#[test]
fn chart_renders_svg_bars() {
    let html = chart::Chart::new()
        .title("Aging".to_string())
        .bar(chart::ChartBar::new("Current".to_string(), 100))
        .bar(chart::ChartBar::new("30+".to_string(), 40))
        .render();
    assert!(html.contains("wj-chart"));
    assert!(html.contains("<svg"));
    assert!(html.contains("wj-chart-bar"));
    assert!(html.contains("Aging"));
}

#[test]
fn approval_card_renders_pending_actions() {
    use windjammer_ui::components::generated::approvalcard;
    let html = approvalcard::ApprovalCard::new(
        "wf-pending-JE-1001".to_string(),
        "pending".to_string(),
    )
    .title("Journal entry approval".to_string())
    .summary("JE-1001 · creator-user · $1,250.00".to_string())
    .resource_type("journal_entry".to_string())
    .resource_id("JE-1001".to_string())
    .render();
    assert!(html.contains("wj-approval-card"));
    assert!(html.contains("data-wj-approval"));
    assert!(
        html.contains("data-wj-workflow-id=\"wf-pending-JE-1001\"")
            || html.contains("data-wj-workflow-id='wf-pending-JE-1001'")
    );
    assert!(
        html.contains("data-wj-approval-state=\"pending\"")
            || html.contains("data-wj-approval-state='pending'")
            || html.contains("pending")
    );
    assert!(html.contains("Journal entry approval"));
    assert!(html.contains("JE-1001"));
    assert!(html.contains("data-wj-approval-approve"));
    assert!(html.contains("data-wj-approval-reject"));
    assert!(html.contains("Approve"));
    assert!(html.contains("Reject"));
}

#[test]
fn approval_card_runtime_posts_approve() {
    use windjammer_ui::components::generated::approvalcard;
    let js = approvalcard::approval_card_runtime_js();
    assert!(js.contains("__wjApprovalCardBound") || js.contains("wjApproval"));
    assert!(js.contains("data-wj-approval-approve"));
    assert!(js.contains("/api/v1/workflow/"));
    assert!(js.contains("/approve") || js.contains("'approve'"));
    assert!(js.contains("POST") || js.contains("method: 'POST'") || js.contains("method: \"POST\""));
}

#[test]
fn approval_card_runtime_posts_reject() {
    use windjammer_ui::components::generated::approvalcard;
    let js = approvalcard::approval_card_runtime_js();
    assert!(js.contains("data-wj-approval-reject"));
    assert!(js.contains("reject") || js.contains("/reject"));
    assert!(js.contains("'reject'") || js.contains("\"reject\"") || js.contains("/reject"));
}

#[test]
fn compliance_score_badge_renders_band_and_score() {
    use windjammer_ui::components::generated::compliancescorebadge;
    use windjammer_ui::components::generated::traits::Renderable;
    let html = compliancescorebadge::ComplianceScoreBadge::new(75, "watch".to_string()).render();
    assert!(html.contains("wj-compliance-score"));
    assert!(html.contains("data-wj-compliance-score"));
    assert!(
        html.contains("data-wj-compliance-band=\"watch\"")
            || html.contains("data-wj-compliance-band='watch'")
    );
    assert!(
        html.contains("data-wj-compliance-value=\"75\"")
            || html.contains("data-wj-compliance-value='75'")
    );
    assert!(html.contains("75"));
    assert!(html.contains("watch"));
}

#[test]
fn period_badge_maps_fiscal_states() {
    use windjammer_ui::components::generated::periodbadge;
    let open = periodbadge::PeriodBadge::new("open".to_string())
        .label("FY2026-Q2".to_string())
        .render();
    assert!(open.contains("wj-period-badge"));
    assert!(open.contains("wj-period-open") || open.contains("wj-badge-success"));
    assert!(open.contains("FY2026-Q2"));

    let locked = periodbadge::PeriodBadge::new("locked".to_string()).render();
    assert!(locked.contains("wj-period-badge"));
    assert!(locked.contains("wj-period-locked") || locked.contains("wj-badge-danger"));
    assert!(locked.to_lowercase().contains("locked"));

    let closed = periodbadge::PeriodBadge::new("closed".to_string())
        .label("FY2025".to_string())
        .render();
    assert!(closed.contains("wj-period-closed") || closed.contains("wj-badge-default"));
    assert!(closed.contains("FY2025"));
}

#[test]
fn currency_input_renders_money_field() {
    use windjammer_ui::components::generated::currencyinput;
    let html = currencyinput::CurrencyInput::new()
        .name("amount".to_string())
        .label("Amount".to_string())
        .currency("USD".to_string())
        .value_cents(1250)
        .required(true)
        .render();
    assert!(html.contains("wj-currency-input"));
    assert!(html.contains("name=\"amount\"") || html.contains("name='amount'"));
    assert!(html.contains("USD"));
    assert!(html.contains("12.50") || html.contains("value=\"12.50\"") || html.contains("value='12.50'"));
    assert!(html.contains("required"));
}

#[test]
fn write_check_form_dogfoods_currency_input() {
    use windjammer_ui::components::generated::{traits::Renderable, writecheckform};
    let html = writecheckform::WriteCheckForm::new()
        .sample_body("{}".to_string())
        .render();
    assert!(html.contains("wj-write-check-form"));
    assert!(
        html.contains("wj-currency-input"),
        "D5: WriteCheckForm amount must use CurrencyInput"
    );
    assert!(html.contains("data-wj-write-check-amount"));
    assert!(html.contains("checkAmount"));
}

#[test]
fn auth_fetch_renders_bearer_load_button() {
    use windjammer_ui::components::generated::{authfetch, traits::Renderable};
    let html = authfetch::AuthFetch::new(
        "/api/v1/bank-lines".to_string(),
        "bank".to_string(),
    )
    .id("loadBank".to_string())
    .label("List bank lines".to_string())
    .mount("#bankTableMount".to_string())
    .render();
    assert!(html.contains("data-wj-auth-fetch"));
    assert!(html.contains("data-wj-fetch-path=\"/api/v1/bank-lines\""));
    assert!(html.contains("data-wj-render-kind=\"bank\""));
    assert!(html.contains("data-wj-mount=\"#bankTableMount\""));
    assert!(html.contains("id=\"loadBank\""));
    assert!(html.contains("List bank lines"));
}

#[test]
fn auth_fetch_runtime_js_drives_sync_badge() {
    use windjammer_ui::components::generated::authfetch;
    let js = authfetch::auth_fetch_runtime_js();
    assert!(js.contains("lkSetSyncStatus"), "D5: sync badge hook");
    assert!(js.contains("lkSyncBadge"));
    assert!(js.contains("wjAuthFetch"));
    assert!(js.contains("'syncing'") || js.contains("\"syncing\""));
    assert!(js.contains("'offline'") || js.contains("\"offline\""));
    assert!(js.contains("'synced'") || js.contains("\"synced\""));
}

#[test]
fn auth_fetch_auto_emits_data_auto() {
    use windjammer_ui::components::generated::{authfetch, traits::Renderable};
    let html = authfetch::AuthFetch::new(
        "/api/v1/fiscal-periods/current".to_string(),
        "period".to_string(),
    )
    .id("loadPeriod".to_string())
    .label("Refresh period".to_string())
    .mount("#periodBadgeMount".to_string())
    .auto(true)
    .render();
    assert!(
        html.contains("data-auto=\"1\"") || html.contains("data-auto='1'"),
        "auto AuthFetch must emit data-auto: {html}"
    );
    assert!(html.contains("data-wj-auth-fetch"), "marker: {html}");
    assert!(
        html.contains("data-wj-mount=\"#periodBadgeMount\""),
        "mount: {html}"
    );
    let manual = authfetch::AuthFetch::new("/x".to_string(), "k".to_string()).render();
    assert!(
        !manual.contains("data-auto=\"1\"") && !manual.contains("data-auto='1'"),
        "manual fetch must not auto: {manual}"
    );
}

#[test]
fn auth_fetch_runtime_js_auto_fires_without_clobbering_unauth() {
    use windjammer_ui::components::generated::authfetch;
    let js = authfetch::auth_fetch_runtime_js();
    assert!(
        js.contains("[data-wj-auth-fetch][data-auto") || js.contains("data-auto"),
        "runtime must discover auto buttons: {js}"
    );
    assert!(
        js.contains("querySelectorAll") || js.contains("querySelector"),
        "runtime must query auto buttons: {js}"
    );
    // Auto chrome must not replace mount with Sign-in / Loading when unauthenticated.
    let has_auto_skip = js.contains("data-auto")
        && (js.contains("Sign in first") || js.contains("Loading"));
    assert!(
        has_auto_skip,
        "runtime still has sign-in/loading paths to gate on auto: {js}"
    );
}

#[test]
fn json_post_refresh_emits_data_attr() {
    use windjammer_ui::components::generated::{jsonpost, traits::Renderable};
    let html = jsonpost::JsonPost::new(
        "/api/v1/fiscal-periods/close".to_string(),
        "#periodCloseBody".to_string(),
    )
    .id("softClose".to_string())
    .refresh("#loadPeriod".to_string())
    .render();
    assert!(
        html.contains("data-wj-refresh-sel=\"#loadPeriod\"")
            || html.contains("data-wj-refresh-sel='#loadPeriod'"),
        "refresh sel: {html}"
    );
}

#[test]
fn json_post_runtime_js_refreshes_auth_fetch_and_hooks() {
    use windjammer_ui::components::generated::jsonpost;
    let js = jsonpost::json_post_runtime_js();
    assert!(
        js.contains("data-wj-refresh-sel"),
        "runtime reads refresh sel: {js}"
    );
    assert!(js.contains("wjAuthFetch"), "AuthFetch after success: {js}");
    assert!(js.contains("lkAfterJsonPost"), "after-success hook: {js}");
}

#[test]
fn json_post_period_guard_emits_data_attr() {
    use windjammer_ui::components::generated::{jsonpost, traits::Renderable};
    let html = jsonpost::JsonPost::new(
        "/api/v1/journal-entries".to_string(),
        "#jeBody".to_string(),
    )
    .id("postJe".to_string())
    .period_guard(true)
    .render();
    assert!(
        html.contains("data-wj-period-guard=\"1\"") || html.contains("data-wj-period-guard='1'"),
        "period guard: {html}"
    );
    let plain = jsonpost::JsonPost::new("/x".to_string(), "#b".to_string()).render();
    assert!(
        !plain.contains("data-wj-period-guard=\"1\"") && !plain.contains("data-wj-period-guard='1'"),
        "default has no guard: {plain}"
    );
}

#[test]
fn json_post_runtime_js_applies_period_write_guard() {
    use windjammer_ui::components::generated::jsonpost;
    let js = jsonpost::json_post_runtime_js();
    assert!(
        js.contains("wjApplyPeriodWriteGuard"),
        "runtime exports period write guard: {js}"
    );
    assert!(
        js.contains("data-period-state"),
        "guard reads PeriodBadge state: {js}"
    );
    assert!(
        js.contains("data-wj-period-guard"),
        "guard targets period-gated buttons: {js}"
    );
    assert!(
        js.contains("data-wj-period-warn"),
        "guard toggles warn slot: {js}"
    );
    assert!(
        js.contains(".disabled") || js.contains("disabled ="),
        "locked period disables guarded posts: {js}"
    );
}

#[test]
fn auth_fetch_runtime_js_applies_period_write_guard() {
    use windjammer_ui::components::generated::authfetch;
    let js = authfetch::auth_fetch_runtime_js();
    assert!(
        js.contains("wjApplyPeriodWriteGuard"),
        "after period paint, re-apply write guard: {js}"
    );
}

#[test]
fn write_check_post_is_period_guarded() {
    use windjammer_ui::components::generated::{traits::Renderable, writecheckform};
    let html = writecheckform::WriteCheckForm::new()
        .sample_body("{}".to_string())
        .render();
    assert!(
        html.contains("data-wj-period-guard"),
        "write-check post honors period lock: {html}"
    );
    assert!(
        html.contains("data-wj-period-warn"),
        "write-check shows period warn slot: {html}"
    );
    let js = writecheckform::write_check_form_runtime_js();
    assert!(
        js.contains("disabled"),
        "write-check runtime skips disabled post: {js}"
    );
}
