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
