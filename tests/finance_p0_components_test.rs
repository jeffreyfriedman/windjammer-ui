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
