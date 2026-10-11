//! Generic data-display component smoke tests (DatePicker, DataTable, KpiTile, FormField, Chart, PanelHead)

use windjammer_ui::components::generated::traits::Renderable;
use windjammer_ui::components::generated::{chart, datatable, datepicker, form, table};

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
        .caption("Records".to_string())
        .column(table::TableColumn::new("Name".to_string()))
        .row(table::TableRow::new().cell("Alpha".to_string()))
        .render();
    assert!(html.contains("wj-datatable"));
    assert!(html.contains("Records"));
    assert!(html.contains("Alpha"));
}

#[test]
fn data_table_scrollable_wraps_lk_table_scroll() {
    let html = datatable::DataTable::new()
        .scrollable(true)
        .caption("Records".to_string())
        .column(table::TableColumn::new("Name".to_string()))
        .row(table::TableRow::new().cell("Alpha".to_string()))
        .render();
    assert!(html.contains("lk-table-scroll"));
    assert!(html.contains("wj-datatable"));
}

#[test]
fn kpi_tile_renders_label_and_value() {
    use windjammer_ui::components::generated::kpitile;
    let html = kpitile::KpiTile::new("Throughput".to_string())
        .value_html("<strong>2500</strong>".to_string())
        .render();
    assert!(html.contains("wj-kpi-tile"));
    assert!(html.contains("Throughput"));
    assert!(html.contains("2500"));
}

#[test]
fn kpi_grid_wraps_tiles() {
    use windjammer_ui::components::generated::kpitile;
    let tile = kpitile::KpiTile::new("Open items".to_string())
        .value_html("<strong>3</strong>".to_string())
        .render();
    let html = kpitile::KpiGrid::new().tile(tile).render();
    assert!(html.contains("wj-kpi-grid"));
    assert!(html.contains("kpi-grid"));
    assert!(html.contains("Open items"));
}

#[test]
fn form_field_has_error_classes() {
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
        .title("Distribution".to_string())
        .bar(chart::ChartBar::new("Current".to_string(), 100))
        .bar(chart::ChartBar::new("30+".to_string(), 40))
        .render();
    assert!(html.contains("wj-chart"));
    assert!(html.contains("<svg"));
    assert!(html.contains("wj-chart-bar"));
    assert!(html.contains("Distribution"));
}

#[test]
fn panel_head_renders_kicker_title_and_actions() {
    use windjammer_ui::components::generated::panelhead;
    let html = panelhead::PanelHead::new("Overview", "Status overview")
        .actions("<button type=\"button\" class=\"btn-secondary\">Refresh</button>")
        .render();
    assert!(html.contains("panel-head"));
    assert!(html.contains("hub-kicker\">Overview<"));
    assert!(html.contains("<h2>Status overview</h2>"));
    assert!(html.contains("btn-secondary"));
    let with_lede = panelhead::PanelHead::new("Ongoing", "Summary")
        .lede("Always-on summary.")
        .actions_row("<button>Refresh</button>")
        .render();
    assert!(with_lede.contains("class=\"lede\""));
    assert!(with_lede.contains("class=\"row\""));
    let inline = panelhead::PanelHead::new("Planning", "Plans")
        .inline()
        .lede("Plan vs actual.")
        .render();
    assert!(!inline.contains("panel-head"), "inline must omit panel-head wrapper: {inline}");
    assert!(inline.contains("hub-kicker\">Planning<"));
    assert!(inline.contains("<h2>Plans</h2>"));
    assert!(inline.contains("class=\"muted\""));
    let kicker_only = panelhead::PanelHead::new("Saved", "")
        .inline()
        .render();
    assert!(kicker_only.contains("hub-kicker\">Saved<"));
    assert!(!kicker_only.contains("<h2>"), "kicker-only inline must omit h2: {kicker_only}");
}

#[test]
fn panel_section_head_renders_h3_and_actions() {
    use windjammer_ui::components::generated::panelsectionhead;
    let html = panelsectionhead::PanelSectionHead::new("Semantic schema")
        .actions("<button type=\"button\" class=\"btn-secondary\">Refresh</button>")
        .render();
    assert!(html.contains("panel-head"));
    assert!(html.contains("<h3>Semantic schema</h3>"));
    assert!(html.contains("btn-secondary"));
}

