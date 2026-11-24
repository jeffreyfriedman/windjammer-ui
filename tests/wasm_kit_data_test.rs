//! Forms, data display and theming of the kit in a real browser: validation state bound to inputs, form submit and
//! reset, table sorting, filtering and paging, list virtualization, and the light/dark/high-contrast token set.
//!
//!   CHROMEDRIVER=/path/to/chromedriver CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
//!     cargo test --target wasm32-unknown-unknown --no-default-features --features web --test wasm_kit_data_test
#![cfg(all(target_arch = "wasm32", feature = "web"))]

mod kit_dom;

use kit_dom::*;
use wasm_bindgen_test::*;
use windjammer_ui::components::generated::form_kit::*;
use windjammer_ui::components::generated::grid_kit::*;
use windjammer_ui::components::generated::kit_choice::SwitchControl;
use windjammer_ui::components::generated::tokens;

wasm_bindgen_test_configure!(run_in_browser);

fn s(v: &str) -> String {
    v.to_string()
}

// ---- Form kit -------------------------------------------------------------------------------------------------

fn signup(base: &str) -> FormKit {
    FormKit::new(s(base), s("Sign up"))
        .field(
            FormField::new(s("name"), s("Name"), FieldKind::Text)
                .required(true)
                .min_len(2)
                .hint(s("As on your passport")),
        )
        .field(FormField::new(s("email"), s("Email"), FieldKind::Email).required(true))
        .field(
            FormField::new(s("age"), s("Age"), FieldKind::Number)
                .min(s("18"))
                .max(s("120")),
        )
        .field(FormField::new(s("born"), s("Birthday"), FieldKind::Date).max(s("2010-01-01")))
        .field(
            FormField::new(s("country"), s("Country"), FieldKind::Select)
                .required(true)
                .option(s("us"), s("United States"))
                .option(s("fr"), s("France")),
        )
        .field(
            FormField::new(s("plan"), s("Plan"), FieldKind::Radio)
                .required(true)
                .option(s("free"), s("Free"))
                .option(s("pro"), s("Pro")),
        )
        .field(
            FormField::new(s("terms"), s("I accept the terms"), FieldKind::Checkbox).required(true),
        )
}

fn fill_valid(base: &str) {
    type_into(&format!("{base}-f-0"), "Ada");
    type_into(&format!("{base}-f-1"), "ada@example.org");
    type_into(&format!("{base}-f-2"), "36");
    type_into(&format!("{base}-f-3"), "1990-12-10");
    choose(&format!("{base}-f-4"), "fr");
    choose(&format!("{base}-f-5-opt-1"), "pro");
    click(&format!("{base}-f-6"));
    fire(&format!("{base}-f-6"), "change");
}

#[wasm_bindgen_test]
fn form_fields_have_labels_types_and_required_state() {
    let (_kit, c) = mount_kit(signup("fa"));
    let form = c.query_selector("form").unwrap().unwrap();
    assert_eq!(form.get_attribute("aria-label").as_deref(), Some("Sign up"));
    for (i, ty) in [(0, "text"), (1, "email"), (2, "number"), (3, "date")] {
        let id = format!("fa-f-{i}");
        assert_eq!(attr(&id, "type").as_deref(), Some(ty));
        let label = c
            .query_selector(&format!("label[for='{id}']"))
            .unwrap()
            .expect("a label is bound to the input");
        assert!(!label.text_content().unwrap().is_empty());
    }
    assert_eq!(by_id("fa-f-4").tag_name(), "SELECT");
    assert_eq!(attr("fa-f-0", "aria-required").as_deref(), Some("true"));
    assert_eq!(attr("fa-f-2", "aria-required").as_deref(), Some("false"));
    assert_eq!(by_id("fa-f-5").tag_name(), "FIELDSET");
    assert_eq!(
        by_id("fa-f-5").first_element_child().unwrap().tag_name(),
        "LEGEND"
    );
    assert_eq!(
        attr("fa-f-0", "aria-invalid").as_deref(),
        Some("false"),
        "no error is shown before the user interacts"
    );
    assert_eq!(text_of("fa-hint-0"), "As on your passport");
    assert_eq!(described_by_text("fa-f-0"), "As on your passport");
}

#[wasm_bindgen_test]
fn error_appears_on_blur_bound_to_the_input_and_clears_when_fixed() {
    let (_kit, c) = mount_kit(signup("fb"));
    fire("fb-f-0", "focusout");
    assert_eq!(attr("fb-f-0", "aria-invalid").as_deref(), Some("true"));
    assert_eq!(text_of("fb-err-0"), "Name is required");
    assert_eq!(
        attr("fb-f-0", "aria-describedby").as_deref(),
        Some("fb-hint-0 fb-err-0"),
        "hint and error are both announced"
    );
    assert!(
        c.query_selector("#fb-err-1").unwrap().is_none(),
        "untouched fields show nothing"
    );

    type_into("fb-f-0", "A");
    assert_eq!(
        text_of("fb-err-0"),
        "Name must be at least 2 characters",
        "the message tracks the value as the user types"
    );
    type_into("fb-f-0", "Al");
    assert!(!exists("fb-err-0"));
    assert_eq!(attr("fb-f-0", "aria-invalid").as_deref(), Some("false"));
    assert_eq!(
        attr("fb-f-0", "aria-describedby").as_deref(),
        Some("fb-hint-0")
    );
}

#[wasm_bindgen_test]
fn each_field_kind_validates_its_own_way() {
    let (_kit, _c) = mount_kit(signup("fc"));
    type_into("fc-f-1", "not-an-email");
    fire("fc-f-1", "focusout");
    assert_eq!(text_of("fc-err-1"), "Email must be an email address");
    type_into("fc-f-2", "7");
    fire("fc-f-2", "focusout");
    assert_eq!(text_of("fc-err-2"), "Age must be at least 18");
    type_into("fc-f-2", "300");
    assert_eq!(text_of("fc-err-2"), "Age must be at most 120");
    type_into("fc-f-3", "2020-05-05");
    fire("fc-f-3", "focusout");
    assert_eq!(
        text_of("fc-err-3"),
        "Birthday must be on or before 2010-01-01"
    );
    fire("fc-f-4", "focusout");
    assert_eq!(text_of("fc-err-4"), "Country is required");
    choose("fc-f-4", "us");
    assert!(!exists("fc-err-4"));
    assert_eq!(
        js_sys::Reflect::get(&by_id("fc-f-4"), &"value".into())
            .unwrap()
            .as_string()
            .as_deref(),
        Some("us")
    );
}

#[wasm_bindgen_test]
fn failed_submit_announces_the_count_and_focuses_the_first_invalid_field() {
    let (kit, _c) = mount_kit(signup("fd"));
    click("fd-submit-0");
    assert_eq!(kit.with_model(|m| m.submit_count()), 0);
    assert_eq!(text_of("fd-status-0"), "5 fields need attention");
    assert_eq!(attr("fd-status-0", "role").as_deref(), Some("status"));
    assert_eq!(
        active_id(),
        "fd-f-0",
        "focus lands on the first invalid input"
    );
    assert_eq!(text_of("fd-err-1"), "Email is required");
    assert_eq!(
        attr("fd-f-5", "aria-invalid").as_deref(),
        Some("true"),
        "radio group is marked invalid too"
    );
    assert_eq!(text_of("fd-err-6"), "I accept the terms is required");
}

#[wasm_bindgen_test]
fn valid_submit_collects_values_and_reset_restores_the_initial_state() {
    let (kit, _c) = mount_kit(signup("fe"));
    fill_valid("fe");
    assert_eq!(kit.with_model(|m| m.error_count()), 0);
    click("fe-submit-0");
    assert_eq!(kit.with_model(|m| m.submit_count()), 1);
    assert_eq!(text_of("fe-status-0"), "Submitted");
    let sent = kit.with_model(|m| m.submitted());
    assert_eq!(
        sent,
        "name=Ada&email=ada@example.org&age=36&born=1990-12-10&country=fr&plan=pro&terms=true"
    );

    click("fe-reset-0");
    assert_eq!(text_of("fe-status-0"), "Form reset");
    assert_eq!(
        js_sys::Reflect::get(&by_id("fe-f-0"), &"value".into())
            .unwrap()
            .as_string()
            .as_deref(),
        Some("")
    );
    assert_eq!(kit.with_model(|m| m.error_count()), 5);
    assert_eq!(
        attr("fe-f-0", "aria-invalid").as_deref(),
        Some("false"),
        "reset hides errors until the user interacts again"
    );
    assert_eq!(active_id(), "fe-f-0");
}

#[wasm_bindgen_test]
fn enter_in_a_text_field_submits_the_form() {
    let (kit, _c) = mount_kit(signup("ff"));
    fill_valid("ff");
    focus("ff-f-0");
    assert!(
        press("ff-f-0", "Enter"),
        "implicit submission is handled and the page does not reload"
    );
    assert_eq!(kit.with_model(|m| m.submit_count()), 1);
}

#[wasm_bindgen_test]
fn typed_text_and_focus_survive_validation_rerenders() {
    let (_kit, _c) = mount_kit(signup("fg"));
    focus("fg-f-0");
    type_into("fg-f-0", "Gra");
    fire("fg-f-0", "focusout");
    focus("fg-f-0");
    type_into("fg-f-0", "Grace");
    assert_eq!(active_id(), "fg-f-0");
    assert_eq!(
        js_sys::Reflect::get(&by_id("fg-f-0"), &"value".into())
            .unwrap()
            .as_string()
            .as_deref(),
        Some("Grace")
    );
}

// ---- Data grid ------------------------------------------------------------------------------------------------

fn people(base: &str) -> DataGrid {
    let mut g = DataGrid::new(s(base), s("Team"))
        .column(GridColumn::text(s("Name")))
        .column(GridColumn::number(s("Age")))
        .page_size(3);
    for (n, a) in [
        ("Mia", "31"),
        ("Zed", "9"),
        ("Ana", "100"),
        ("Bob", "45"),
        ("Cy", "27"),
        ("Dee", "52forty"),
    ] {
        g = g.row(vec![s(n), s(a)]);
    }
    g
}

fn column_texts(c: &web_sys::Element, col: usize) -> Vec<String> {
    let rows = c.query_selector_all("tbody tr").unwrap();
    (0..rows.length())
        .map(|i| {
            let row: web_sys::Element =
                wasm_bindgen::JsCast::dyn_into(rows.item(i).unwrap()).unwrap();
            row.children()
                .item(col as u32)
                .unwrap()
                .text_content()
                .unwrap()
        })
        .collect()
}

#[wasm_bindgen_test]
fn grid_is_a_native_table_with_caption_headers_and_sort_state() {
    let (_kit, c) = mount_kit(people("g1"));
    assert_eq!(by_id("g1-table-0").tag_name(), "TABLE");
    assert_eq!(
        c.query_selector("caption")
            .unwrap()
            .unwrap()
            .text_content()
            .unwrap(),
        "Team"
    );
    assert_eq!(attr("g1-table-0", "aria-rowcount").as_deref(), Some("6"));
    let th = by_id("g1-sort-0").parent_element().unwrap();
    assert_eq!(th.tag_name(), "TH");
    assert_eq!(th.get_attribute("scope").as_deref(), Some("col"));
    assert_eq!(th.get_attribute("aria-sort").as_deref(), Some("none"));
    assert_eq!(
        attr("g1-filter-0", "aria-label").as_deref(),
        Some("Filter rows")
    );
    assert_eq!(text_of("g1-status-0"), "Showing 1-3 of 6");
    assert_eq!(column_texts(&c, 0), ["Mia", "Zed", "Ana"]);
}

#[wasm_bindgen_test]
fn grid_sorts_by_click_and_keyboard_text_then_numeric_and_updates_aria_sort() {
    let (_kit, c) = mount_kit(people("g2").page_size(10));
    click("g2-sort-0");
    assert_eq!(
        column_texts(&c, 0),
        ["Ana", "Bob", "Cy", "Dee", "Mia", "Zed"]
    );
    assert_eq!(
        by_id("g2-sort-0")
            .parent_element()
            .unwrap()
            .get_attribute("aria-sort")
            .as_deref(),
        Some("ascending")
    );
    assert!(press("g2-sort-0", "Enter"));
    assert_eq!(
        column_texts(&c, 0),
        ["Zed", "Mia", "Dee", "Cy", "Bob", "Ana"]
    );
    assert_eq!(
        by_id("g2-sort-0")
            .parent_element()
            .unwrap()
            .get_attribute("aria-sort")
            .as_deref(),
        Some("descending")
    );
    press("g2-sort-0", " ");
    assert_eq!(
        column_texts(&c, 0),
        ["Mia", "Zed", "Ana", "Bob", "Cy", "Dee"],
        "third activation returns to the original order"
    );
    assert_eq!(
        by_id("g2-sort-0")
            .parent_element()
            .unwrap()
            .get_attribute("aria-sort")
            .as_deref(),
        Some("none")
    );
    click("g2-sort-1");
    assert_eq!(
        column_texts(&c, 1),
        ["52forty", "9", "27", "31", "45", "100"],
        "numbers sort numerically, 9 before 100"
    );
    assert_eq!(active_id(), "g2-sort-1", "focus stays on the header button");
    assert_eq!(
        by_id("g2-sort-0")
            .parent_element()
            .unwrap()
            .get_attribute("aria-sort")
            .as_deref(),
        Some("none")
    );
}

#[wasm_bindgen_test]
fn grid_filters_as_you_type_and_reports_the_match_count() {
    let (_kit, c) = mount_kit(people("g3"));
    focus("g3-filter-0");
    type_into("g3-filter-0", "A");
    assert_eq!(
        column_texts(&c, 0),
        ["Mia", "Ana"],
        "case-insensitive match on any cell"
    );
    assert_eq!(text_of("g3-status-0"), "Showing 1-2 of 2");
    assert_eq!(attr("g3-table-0", "aria-rowcount").as_deref(), Some("2"));
    assert_eq!(
        active_id(),
        "g3-filter-0",
        "typing keeps focus in the filter"
    );
    type_into("g3-filter-0", "zzz");
    assert_eq!(text_of("g3-status-0"), "No rows match");
    type_into("g3-filter-0", "");
    assert_eq!(column_texts(&c, 0).len(), 3);
}

#[wasm_bindgen_test]
fn grid_pages_forward_and_back_and_disables_the_ends() {
    let (kit, c) = mount_kit(people("g4"));
    assert!(attr("g4-prev-0", "disabled").is_some());
    assert!(attr("g4-next-0", "disabled").is_none());
    click("g4-next-0");
    assert_eq!(column_texts(&c, 0), ["Bob", "Cy", "Dee"]);
    assert_eq!(text_of("g4-status-0"), "Showing 4-6 of 6");
    assert!(attr("g4-next-0", "disabled").is_some());
    assert_eq!(kit.with_model(|m| m.current_page()), 2);
    click("g4-prev-0");
    assert_eq!(column_texts(&c, 0), ["Mia", "Zed", "Ana"]);
    // Sorting returns to the first page.
    click("g4-next-0");
    click("g4-sort-0");
    assert_eq!(kit.with_model(|m| m.current_page()), 1);
    assert_eq!(column_texts(&c, 0), ["Ana", "Bob", "Cy"]);
    assert_eq!(
        by_id("g4-prev-0")
            .parent_element()
            .unwrap()
            .get_attribute("aria-label")
            .as_deref(),
        Some("Table pages")
    );
}

#[wasm_bindgen_test]
fn grid_tab_order_covers_filter_headers_and_pager() {
    let (_kit, c) = mount_kit(people("g5"));
    assert_eq!(
        tab_order(&c),
        ["g5-filter-0", "g5-sort-0", "g5-sort-1", "g5-next-0"],
        "disabled Prev is skipped"
    );
}

// ---- Virtual list ---------------------------------------------------------------------------------------------

fn rows_in_dom(c: &web_sys::Element) -> u32 {
    c.query_selector_all("[role='option']").unwrap().length()
}

#[wasm_bindgen_test]
fn ten_thousand_rows_render_only_the_visible_window() {
    let (kit, c) = mount_kit(VirtualList::new(s("v1"), s("Log lines"), 10_000, 20, 200));
    let list = by_id("v1-list-0");
    assert_eq!(list.get_attribute("role").as_deref(), Some("listbox"));
    assert_eq!(
        list.get_attribute("aria-label").as_deref(),
        Some("Log lines")
    );
    assert_eq!(list.get_attribute("tabindex").as_deref(), Some("0"));
    assert!(
        rows_in_dom(&c) <= 20,
        "a window of rows, not 10000 (saw {})",
        rows_in_dom(&c)
    );
    assert_eq!(attr("v1-row-0", "aria-setsize").as_deref(), Some("10000"));
    assert_eq!(attr("v1-row-0", "aria-posinset").as_deref(), Some("1"));
    assert_eq!(
        list.first_element_child()
            .unwrap()
            .get_attribute("style")
            .unwrap()
            .contains("height: 200000px"),
        true,
        "the spacer keeps the full scroll height"
    );
    assert!(!exists("v1-row-5000"));
    assert_eq!(kit.with_model(|m| m.last_rendered()), 13);
    assert_eq!(
        tab_order(&c),
        ["v1-list-0"],
        "rows are not tab stops; the list is one"
    );
}

#[wasm_bindgen_test]
fn scrolling_swaps_the_window_and_keeps_the_dom_small() {
    let (_kit, c) = mount_kit(VirtualList::new(s("v2"), s("Log lines"), 10_000, 20, 200));
    let list = by_id("v2-list-0");
    list.set_scroll_top(100_000); // row 5000
    fire("v2-list-0", "scroll");
    assert!(exists("v2-row-5000"));
    assert_eq!(text_of("v2-row-5000"), "Row 5001");
    assert!(!exists("v2-row-0"), "rows that scrolled away are removed");
    assert!(rows_in_dom(&c) <= 20);
    assert_eq!(
        list.scroll_top(),
        100_000,
        "re-rendering keeps the scroll offset"
    );
}

#[wasm_bindgen_test]
fn virtual_list_keys_move_the_active_row_and_scroll_it_into_view() {
    let (kit, c) = mount_kit(VirtualList::new(s("v3"), s("Log lines"), 10_000, 20, 200));
    focus("v3-list-0");
    assert_eq!(
        attr("v3-list-0", "aria-activedescendant").as_deref(),
        Some("v3-row-0")
    );
    assert_eq!(attr("v3-row-0", "aria-selected").as_deref(), Some("true"));
    assert!(press("v3-list-0", "ArrowDown"));
    assert_eq!(
        attr("v3-list-0", "aria-activedescendant").as_deref(),
        Some("v3-row-1")
    );
    press("v3-list-0", "PageDown");
    assert_eq!(kit.with_model(|m| m.active_index()), 11);
    press("v3-list-0", "End");
    assert_eq!(kit.with_model(|m| m.active_index()), 9_999);
    assert!(
        exists("v3-row-9999"),
        "End renders and scrolls to the last row"
    );
    assert_eq!(
        attr("v3-row-9999", "aria-selected").as_deref(),
        Some("true")
    );
    assert!(
        by_id("v3-list-0").scroll_top() > 190_000,
        "the browser element really scrolled"
    );
    assert!(rows_in_dom(&c) <= 20);
    press("v3-list-0", "Home");
    assert!(exists("v3-row-0"));
    assert_eq!(by_id("v3-list-0").scroll_top(), 0);
    assert_eq!(active_id(), "v3-list-0", "focus stays on the list");
}

// ---- Theming tokens -------------------------------------------------------------------------------------------

fn install_theme() -> web_sys::Element {
    let style = document().create_element("style").unwrap();
    style.set_text_content(Some(&format!(
        "{}\n{}",
        tokens::theme_css(),
        tokens::kit_css()
    )));
    document().body().unwrap().append_child(&style).unwrap();
    style
}

fn computed(id: &str, prop: &str) -> String {
    let cs = web_sys::window()
        .unwrap()
        .get_computed_style(&by_id(id))
        .unwrap()
        .unwrap();
    cs.get_property_value(prop).unwrap()
}

fn rgb(hex: &str) -> String {
    let n = |i: usize| u8::from_str_radix(&hex[i..i + 2], 16).unwrap();
    format!("rgb({}, {}, {})", n(1), n(3), n(5))
}

fn set_theme(name: Option<&str>) {
    let root = document().document_element().unwrap();
    match name {
        Some(n) => root.set_attribute("data-wj-theme", n).unwrap(),
        None => root.remove_attribute("data-wj-theme").unwrap(),
    }
}

#[wasm_bindgen_test]
fn components_read_the_same_token_set_in_light_dark_and_high_contrast() {
    let style = install_theme();
    let (_kit, _c) = mount_kit(SwitchControl::new(s("th"), s("Theme probe")).on(true));
    let value_of = |name: &str, mode: tokens::ThemeMode| {
        tokens::tokens()
            .into_iter()
            .find(|t| t.name == name)
            .unwrap()
            .value(mode)
    };
    set_theme(Some("light"));
    assert_eq!(
        computed("th-switch-0", "background-color"),
        rgb(&value_of("color-primary", tokens::ThemeMode::Light))
    );
    assert_eq!(
        computed("th-switch-0", "color"),
        rgb(&value_of("color-on-primary", tokens::ThemeMode::Light))
    );
    set_theme(Some("dark"));
    assert_eq!(
        computed("th-switch-0", "background-color"),
        rgb(&value_of("color-primary", tokens::ThemeMode::Dark))
    );
    assert_eq!(
        computed("th-switch-0", "color"),
        rgb(&value_of("color-on-primary", tokens::ThemeMode::Dark))
    );
    set_theme(Some("high-contrast"));
    assert_eq!(
        computed("th-switch-0", "background-color"),
        rgb(&value_of("color-primary", tokens::ThemeMode::HighContrast))
    );
    assert_eq!(
        computed("th-switch-0", "border-top-width"),
        "2px",
        "high contrast thickens borders through the border-width token"
    );
    set_theme(None);
    style.remove();
}

#[wasm_bindgen_test]
fn a_token_change_in_one_place_restyles_every_component() {
    let style = install_theme();
    let (_k1, _c1) = mount_kit(SwitchControl::new(s("tha"), s("A")));
    let (_k2, _c2) = mount_kit(SwitchControl::new(s("thb"), s("B")));
    set_theme(Some("dark"));
    let bg_a = computed("tha-switch-0", "background-color");
    assert_eq!(bg_a, computed("thb-switch-0", "background-color"));
    assert_eq!(bg_a, rgb("#1a2029"), "the dark surface token");
    document()
        .document_element()
        .unwrap()
        .set_attribute("style", "--wj-color-surface: #123456")
        .unwrap();
    assert_eq!(
        computed("tha-switch-0", "background-color"),
        "rgb(18, 52, 86)",
        "overriding one custom property re-themes the components"
    );
    assert_eq!(
        computed("thb-switch-0", "background-color"),
        "rgb(18, 52, 86)"
    );
    document()
        .document_element()
        .unwrap()
        .remove_attribute("style")
        .unwrap();
    set_theme(None);
    style.remove();
}
