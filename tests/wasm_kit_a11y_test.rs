//! Accessibility of the interactive kit components in a real browser: roles, accessible names, ARIA state, tab order,
//! key handling and focus management, one group of tests per component.
//!
//!   CHROMEDRIVER=/path/to/chromedriver CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
//!     cargo test --target wasm32-unknown-unknown --no-default-features --features web --test wasm_kit_a11y_test
//!
//! Browsers: Chrome via `CHROMEDRIVER`; Edge is the same engine, driven through `msedgedriver` (see docs/TESTING_GUIDE.md).
#![cfg(all(target_arch = "wasm32", feature = "web"))]

mod kit_dom;

use kit_dom::*;
use wasm_bindgen_test::*;
use windjammer_ui::components::generated::kit_choice::*;
use windjammer_ui::components::generated::kit_disclosure::*;
use windjammer_ui::components::generated::kit_overlay::*;

wasm_bindgen_test_configure!(run_in_browser);

fn s(v: &str) -> String {
    v.to_string()
}

// ---- Tabs -----------------------------------------------------------------------------------------------------

fn tabs(base: &str) -> TabStrip {
    TabStrip::new(s(base))
        .tab(s("One"), s("First panel"))
        .tab(s("Two"), s("Second panel"))
        .tab(s("Three"), s("Third panel"))
}

#[wasm_bindgen_test]
fn tabs_expose_tablist_tab_and_tabpanel_roles_wired_by_id() {
    let (_kit, _c) = mount_kit(tabs("tabs-roles"));
    assert_eq!(attr("tabs-roles-tab-0", "role").as_deref(), Some("tab"));
    assert_eq!(
        by_id("tabs-roles-tab-0")
            .parent_element()
            .unwrap()
            .get_attribute("role")
            .as_deref(),
        Some("tablist")
    );
    assert_eq!(
        attr("tabs-roles-panel-0", "role").as_deref(),
        Some("tabpanel")
    );
    // Each tab controls its panel and each panel is named by its tab.
    assert_eq!(
        attr("tabs-roles-tab-1", "aria-controls").as_deref(),
        Some("tabs-roles-panel-1")
    );
    assert_eq!(labelled_by_text("tabs-roles-panel-1"), "Two");
    assert_eq!(
        attr("tabs-roles-tab-0", "aria-selected").as_deref(),
        Some("true")
    );
    assert_eq!(
        attr("tabs-roles-tab-1", "aria-selected").as_deref(),
        Some("false")
    );
    assert!(attr("tabs-roles-panel-0", "hidden").is_none());
    assert!(attr("tabs-roles-panel-1", "hidden").is_some());
}

#[wasm_bindgen_test]
fn tabs_have_one_tab_stop_that_follows_the_selection() {
    let (_kit, c) = mount_kit(tabs("tabs-order"));
    assert_eq!(
        tab_order(&c),
        ["tabs-order-tab-0", "tabs-order-panel-0"],
        "one tab and the open panel"
    );
    focus("tabs-order-tab-0");
    assert!(
        press("tabs-order-tab-0", "ArrowRight"),
        "arrow keys are handled, so the page does not scroll"
    );
    assert_eq!(active_id(), "tabs-order-tab-1");
    assert_eq!(tab_order(&c), ["tabs-order-tab-1", "tabs-order-panel-1"]);
    assert_eq!(
        attr("tabs-order-tab-1", "aria-selected").as_deref(),
        Some("true")
    );
    assert_eq!(
        attr("tabs-order-tab-0", "aria-selected").as_deref(),
        Some("false")
    );
}

#[wasm_bindgen_test]
fn tabs_arrow_keys_wrap_and_home_end_jump() {
    let (kit, _c) = mount_kit(tabs("tabs-keys"));
    focus("tabs-keys-tab-0");
    press("tabs-keys-tab-0", "ArrowLeft");
    assert_eq!(
        active_id(),
        "tabs-keys-tab-2",
        "ArrowLeft from the first tab wraps to the last"
    );
    press("tabs-keys-tab-2", "ArrowRight");
    assert_eq!(active_id(), "tabs-keys-tab-0");
    press("tabs-keys-tab-0", "End");
    assert_eq!(active_id(), "tabs-keys-tab-2");
    press("tabs-keys-tab-2", "Home");
    assert_eq!(active_id(), "tabs-keys-tab-0");
    assert!(
        !press("tabs-keys-tab-0", "q"),
        "unrelated keys are left to the browser"
    );
    assert_eq!(kit.with_model(|m| m.selected_index()), 0);
}

#[wasm_bindgen_test]
fn tabs_click_selects_and_focuses() {
    let (kit, _c) = mount_kit(tabs("tabs-click"));
    click("tabs-click-tab-2");
    assert_eq!(kit.with_model(|m| m.selected_index()), 2);
    assert_eq!(active_id(), "tabs-click-tab-2");
    assert!(attr("tabs-click-panel-2", "hidden").is_none());
    assert!(attr("tabs-click-panel-0", "hidden").is_some());
}

// ---- Accordion ------------------------------------------------------------------------------------------------

fn accordion(base: &str) -> AccordionStack {
    AccordionStack::new(s(base))
        .item(s("Shipping"), s("Ships in two days."))
        .item(s("Returns"), s("Return within 30 days."))
        .item(s("Support"), s("Chat 24/7."))
}

#[wasm_bindgen_test]
fn accordion_headers_expose_expanded_state_and_control_their_region() {
    let (_kit, _c) = mount_kit(accordion("acc-aria"));
    assert_eq!(
        attr("acc-aria-head-0", "aria-expanded").as_deref(),
        Some("false")
    );
    assert_eq!(
        attr("acc-aria-head-0", "aria-controls").as_deref(),
        Some("acc-aria-body-0")
    );
    assert_eq!(attr("acc-aria-body-0", "role").as_deref(), Some("region"));
    assert_eq!(labelled_by_text("acc-aria-body-0"), "Shipping");
    assert!(attr("acc-aria-body-0", "hidden").is_some());
    click("acc-aria-head-0");
    assert_eq!(
        attr("acc-aria-head-0", "aria-expanded").as_deref(),
        Some("true")
    );
    assert!(attr("acc-aria-body-0", "hidden").is_none());
}

#[wasm_bindgen_test]
fn accordion_enter_and_space_toggle_and_arrows_move_between_headers() {
    let (kit, c) = mount_kit(accordion("acc-keys"));
    focus("acc-keys-head-0");
    assert!(
        press("acc-keys-head-0", "Enter"),
        "Enter is handled (no duplicate click)"
    );
    assert!(kit.with_model(|m| m.is_open(0)));
    press("acc-keys-head-0", " ");
    assert!(!kit.with_model(|m| m.is_open(0)), "Space toggles back");
    press("acc-keys-head-0", "ArrowDown");
    assert_eq!(active_id(), "acc-keys-head-1");
    press("acc-keys-head-1", "End");
    assert_eq!(active_id(), "acc-keys-head-2");
    press("acc-keys-head-2", "ArrowDown");
    assert_eq!(active_id(), "acc-keys-head-0", "arrow navigation wraps");
    // Every header is a tab stop; closed bodies are not.
    assert_eq!(
        tab_order(&c),
        ["acc-keys-head-0", "acc-keys-head-1", "acc-keys-head-2"]
    );
}

#[wasm_bindgen_test]
fn exclusive_accordion_keeps_one_section_open() {
    let (kit, _c) = mount_kit(accordion("acc-one").exclusive(true));
    click("acc-one-head-0");
    click("acc-one-head-2");
    assert!(!kit.with_model(|m| m.is_open(0)));
    assert!(kit.with_model(|m| m.is_open(2)));
    assert_eq!(
        attr("acc-one-head-0", "aria-expanded").as_deref(),
        Some("false")
    );
}

// ---- Modal dialog ---------------------------------------------------------------------------------------------

fn dialog(base: &str) -> ModalDialog {
    ModalDialog::new(s(base), s("Delete project"))
        .trigger(s("Delete..."))
        .body(s("This cannot be undone."))
}

#[wasm_bindgen_test]
fn dialog_is_labelled_and_described_and_moves_focus_in() {
    let (kit, _c) = mount_kit(dialog("dlg-aria"));
    assert!(
        !exists("dlg-aria-title-0"),
        "closed dialogs are not in the DOM"
    );
    assert_eq!(
        attr("dlg-aria-trigger-0", "aria-haspopup").as_deref(),
        Some("dialog")
    );
    assert_eq!(
        attr("dlg-aria-trigger-0", "aria-expanded").as_deref(),
        Some("false")
    );
    click("dlg-aria-trigger-0");
    assert!(kit.with_model(|m| m.is_open()));
    let node = by_id("dlg-aria-title-0").parent_element().unwrap();
    assert_eq!(node.get_attribute("role").as_deref(), Some("dialog"));
    assert_eq!(node.get_attribute("aria-modal").as_deref(), Some("true"));
    assert_eq!(
        node.get_attribute("aria-labelledby").as_deref(),
        Some("dlg-aria-title-0")
    );
    assert_eq!(text_of("dlg-aria-title-0"), "Delete project");
    assert_eq!(
        node.get_attribute("aria-describedby").as_deref(),
        Some("dlg-aria-body-0")
    );
    assert_eq!(text_of("dlg-aria-body-0"), "This cannot be undone.");
    assert_eq!(
        active_id(),
        "dlg-aria-btn-0",
        "focus moves to the first control in the dialog"
    );
    assert_eq!(
        attr("dlg-aria-trigger-0", "aria-expanded").as_deref(),
        Some("true")
    );
}

#[wasm_bindgen_test]
fn dialog_traps_tab_and_shift_tab() {
    let (_kit, c) = mount_kit(dialog("dlg-trap"));
    click("dlg-trap-trigger-0");
    assert_eq!(active_id(), "dlg-trap-btn-0");
    assert!(press("dlg-trap-btn-0", "Tab"));
    assert_eq!(active_id(), "dlg-trap-btn-1");
    assert!(press("dlg-trap-btn-1", "Tab"));
    assert_eq!(
        active_id(),
        "dlg-trap-btn-0",
        "Tab from the last control wraps to the first"
    );
    press("dlg-trap-btn-0", "Shift+Tab");
    assert_eq!(
        active_id(),
        "dlg-trap-btn-1",
        "Shift+Tab from the first control wraps to the last"
    );
    assert_eq!(
        tab_order(&c).len(),
        3,
        "trigger plus the dialog's two buttons"
    );
}

#[wasm_bindgen_test]
fn dialog_escape_and_buttons_close_and_return_focus_to_the_trigger() {
    let (kit, _c) = mount_kit(dialog("dlg-close"));
    click("dlg-close-trigger-0");
    assert!(press("dlg-close-btn-0", "Escape"));
    assert!(!kit.with_model(|m| m.is_open()));
    assert!(!exists("dlg-close-btn-0"));
    assert_eq!(
        active_id(),
        "dlg-close-trigger-0",
        "focus returns to the opener"
    );
    assert!(!kit.with_model(|m| m.was_confirmed()));

    click("dlg-close-trigger-0");
    click("dlg-close-btn-1");
    assert!(kit.with_model(|m| m.was_confirmed()));
    assert_eq!(active_id(), "dlg-close-trigger-0");
}

// ---- Menu button ----------------------------------------------------------------------------------------------

fn menu(base: &str) -> MenuButton {
    MenuButton::new(s(base), s("Actions"))
        .item(s("Rename"))
        .item(s("Duplicate"))
        .item(s("Delete"))
}

#[wasm_bindgen_test]
fn menu_button_exposes_menu_roles_and_expanded_state() {
    let (_kit, _c) = mount_kit(menu("menu-aria"));
    assert_eq!(
        attr("menu-aria-trigger-0", "aria-haspopup").as_deref(),
        Some("menu")
    );
    assert_eq!(
        attr("menu-aria-trigger-0", "aria-expanded").as_deref(),
        Some("false")
    );
    assert_eq!(
        attr("menu-aria-trigger-0", "aria-controls").as_deref(),
        Some("menu-aria-menu-0")
    );
    assert_eq!(attr("menu-aria-menu-0", "role").as_deref(), Some("menu"));
    assert_eq!(labelled_by_text("menu-aria-menu-0"), "Actions");
    assert_eq!(
        attr("menu-aria-item-1", "role").as_deref(),
        Some("menuitem")
    );
    assert!(attr("menu-aria-menu-0", "hidden").is_some());
}

#[wasm_bindgen_test]
fn menu_arrow_keys_open_navigate_and_choose() {
    let (kit, c) = mount_kit(menu("menu-keys"));
    assert_eq!(
        tab_order(&c),
        ["menu-keys-trigger-0"],
        "closed menu: only the button is a tab stop"
    );
    focus("menu-keys-trigger-0");
    assert!(press("menu-keys-trigger-0", "ArrowDown"));
    assert!(kit.with_model(|m| m.is_open()));
    assert_eq!(active_id(), "menu-keys-item-0");
    assert_eq!(
        attr("menu-keys-trigger-0", "aria-expanded").as_deref(),
        Some("true")
    );
    press("menu-keys-item-0", "ArrowDown");
    assert_eq!(active_id(), "menu-keys-item-1");
    press("menu-keys-item-1", "End");
    assert_eq!(active_id(), "menu-keys-item-2");
    press("menu-keys-item-2", "ArrowDown");
    assert_eq!(active_id(), "menu-keys-item-0", "wraps");
    press("menu-keys-item-0", "ArrowUp");
    assert_eq!(active_id(), "menu-keys-item-2");
    assert!(press("menu-keys-item-2", "Enter"));
    assert_eq!(kit.with_model(|m| m.chosen()), "Delete");
    assert!(!kit.with_model(|m| m.is_open()));
    assert_eq!(
        active_id(),
        "menu-keys-trigger-0",
        "choosing returns focus to the button"
    );
}

#[wasm_bindgen_test]
fn menu_escape_closes_and_arrow_up_opens_on_the_last_item() {
    let (kit, _c) = mount_kit(menu("menu-esc"));
    focus("menu-esc-trigger-0");
    press("menu-esc-trigger-0", "ArrowUp");
    assert_eq!(active_id(), "menu-esc-item-2");
    assert!(press("menu-esc-item-2", "Escape"));
    assert!(!kit.with_model(|m| m.is_open()));
    assert_eq!(active_id(), "menu-esc-trigger-0");
    assert_eq!(kit.with_model(|m| m.chosen()), "");
}

// ---- Checkbox, switch, toggle button -------------------------------------------------------------------------

#[wasm_bindgen_test]
fn checkbox_is_a_labelled_native_checkbox_that_reports_changes() {
    let (kit, c) = mount_kit(CheckboxField::new(s("cb"), s("Subscribe to updates")));
    let input = by_id("cb-input-0");
    assert_eq!(
        input.get_attribute("type").as_deref(),
        Some("checkbox"),
        "native checkbox role and Space/Tab behaviour"
    );
    let label = input.parent_element().unwrap();
    assert_eq!(label.tag_name(), "LABEL");
    assert_eq!(label.get_attribute("for").as_deref(), Some("cb-input-0"));
    assert_eq!(
        label.text_content().unwrap(),
        "Subscribe to updates",
        "the label is the accessible name"
    );
    assert_eq!(tab_order(&c), ["cb-input-0"]);
    click("cb-input-0"); // the browser toggles the box and fires click then input/change
    fire("cb-input-0", "change");
    assert!(kit.with_model(|m| m.is_checked()));
    assert!(
        by_id("cb-input-0").has_attribute("checked")
            || js_sys::Reflect::get(&by_id("cb-input-0"), &"checked".into())
                .unwrap()
                .as_bool()
                .unwrap()
    );
}

#[wasm_bindgen_test]
fn switch_has_role_switch_and_responds_to_space_enter_and_click() {
    let (kit, c) = mount_kit(SwitchControl::new(s("sw"), s("Wi-Fi")));
    assert_eq!(attr("sw-switch-0", "role").as_deref(), Some("switch"));
    assert_eq!(
        attr("sw-switch-0", "aria-checked").as_deref(),
        Some("false")
    );
    assert_eq!(text_of("sw-switch-0"), "Wi-Fi", "name from content");
    assert_eq!(tab_order(&c), ["sw-switch-0"]);
    assert!(press("sw-switch-0", " "));
    assert_eq!(attr("sw-switch-0", "aria-checked").as_deref(), Some("true"));
    press("sw-switch-0", "Enter");
    assert_eq!(
        attr("sw-switch-0", "aria-checked").as_deref(),
        Some("false")
    );
    click("sw-switch-0");
    assert!(kit.with_model(|m| m.is_on()));
    assert!(!press("sw-switch-0", "a"));
}

#[wasm_bindgen_test]
fn disabled_switch_is_not_focusable_and_ignores_input() {
    let (kit, c) = mount_kit(SwitchControl::new(s("sw-off"), s("Locked")).disabled(true));
    assert_eq!(
        attr("sw-off-switch-0", "aria-disabled").as_deref(),
        Some("true")
    );
    assert!(tab_order(&c).is_empty());
    press("sw-off-switch-0", " ");
    assert!(!kit.with_model(|m| m.is_on()));
}

#[wasm_bindgen_test]
fn toggle_button_reports_pressed_state() {
    let (kit, _c) = mount_kit(ToggleButton::new(s("tb"), s("Bold")));
    assert_eq!(
        attr("tb-toggle-0", "aria-pressed").as_deref(),
        Some("false")
    );
    assert!(press("tb-toggle-0", "Enter"));
    assert_eq!(attr("tb-toggle-0", "aria-pressed").as_deref(), Some("true"));
    click("tb-toggle-0");
    assert!(!kit.with_model(|m| m.is_pressed()));
}

// ---- Radio group ----------------------------------------------------------------------------------------------

fn radios(base: &str) -> RadioChoice {
    RadioChoice::new(s(base), s("Plan"))
        .option(s("free"), s("Free"))
        .option(s("pro"), s("Pro"))
        .option(s("team"), s("Team"))
}

#[wasm_bindgen_test]
fn radio_group_has_roles_a_name_and_a_single_tab_stop() {
    let (_kit, c) = mount_kit(radios("rg-aria"));
    let group = by_id("rg-aria-radio-0").parent_element().unwrap();
    assert_eq!(group.get_attribute("role").as_deref(), Some("radiogroup"));
    assert_eq!(
        group.get_attribute("aria-labelledby").as_deref(),
        Some("rg-aria-label-0")
    );
    assert_eq!(text_of("rg-aria-label-0"), "Plan");
    assert_eq!(attr("rg-aria-radio-1", "role").as_deref(), Some("radio"));
    assert_eq!(
        attr("rg-aria-radio-1", "aria-checked").as_deref(),
        Some("false")
    );
    assert_eq!(
        tab_order(&c),
        ["rg-aria-radio-0"],
        "with no selection the first radio is the entry point"
    );
}

#[wasm_bindgen_test]
fn radio_arrows_move_and_select_and_the_tab_stop_follows() {
    let (kit, c) = mount_kit(radios("rg-keys"));
    focus("rg-keys-radio-0");
    assert!(press("rg-keys-radio-0", "ArrowDown"));
    assert_eq!(active_id(), "rg-keys-radio-1");
    assert_eq!(
        attr("rg-keys-radio-1", "aria-checked").as_deref(),
        Some("true")
    );
    assert_eq!(
        tab_order(&c),
        ["rg-keys-radio-1"],
        "the checked radio is the only tab stop"
    );
    press("rg-keys-radio-1", "ArrowRight");
    assert_eq!(active_id(), "rg-keys-radio-2");
    press("rg-keys-radio-2", "ArrowRight");
    assert_eq!(active_id(), "rg-keys-radio-0", "wraps");
    press("rg-keys-radio-0", "ArrowUp");
    assert_eq!(kit.with_model(|m| m.value()), "team");
    press("rg-keys-radio-2", "Home");
    assert_eq!(kit.with_model(|m| m.value()), "free");
}

#[wasm_bindgen_test]
fn radio_space_selects_the_focused_radio() {
    let (kit, _c) = mount_kit(radios("rg-space"));
    click("rg-space-radio-2");
    assert_eq!(kit.with_model(|m| m.value()), "team");
    assert_eq!(
        attr("rg-space-radio-2", "aria-checked").as_deref(),
        Some("true")
    );
    assert_eq!(
        attr("rg-space-radio-0", "aria-checked").as_deref(),
        Some("false")
    );
}

// ---- Slider ---------------------------------------------------------------------------------------------------

#[wasm_bindgen_test]
fn slider_exposes_value_range_and_name_and_handles_its_keys() {
    let (kit, c) = mount_kit(
        SliderControl::new(s("sl"), s("Volume"), 0, 100)
            .step(5)
            .value(50),
    );
    assert_eq!(attr("sl-slider-0", "role").as_deref(), Some("slider"));
    assert_eq!(attr("sl-slider-0", "aria-label").as_deref(), Some("Volume"));
    assert_eq!(attr("sl-slider-0", "aria-valuemin").as_deref(), Some("0"));
    assert_eq!(attr("sl-slider-0", "aria-valuemax").as_deref(), Some("100"));
    assert_eq!(attr("sl-slider-0", "aria-valuenow").as_deref(), Some("50"));
    assert_eq!(tab_order(&c), ["sl-slider-0"]);
    focus("sl-slider-0");
    assert!(press("sl-slider-0", "ArrowRight"));
    assert_eq!(attr("sl-slider-0", "aria-valuenow").as_deref(), Some("55"));
    press("sl-slider-0", "ArrowDown");
    assert_eq!(attr("sl-slider-0", "aria-valuenow").as_deref(), Some("50"));
    press("sl-slider-0", "PageUp");
    assert_eq!(
        attr("sl-slider-0", "aria-valuenow").as_deref(),
        Some("100"),
        "PageUp moves ten steps and clamps at max"
    );
    press("sl-slider-0", "Home");
    assert_eq!(kit.with_model(|m| m.current()), 0);
    press("sl-slider-0", "ArrowLeft");
    assert_eq!(kit.with_model(|m| m.current()), 0, "clamped at min");
    press("sl-slider-0", "End");
    assert_eq!(attr("sl-slider-0", "aria-valuenow").as_deref(), Some("100"));
    assert_eq!(
        active_id(),
        "sl-slider-0",
        "focus stays on the slider across re-renders"
    );
}

// ---- Pagination -----------------------------------------------------------------------------------------------

#[wasm_bindgen_test]
fn pagination_is_a_labelled_nav_with_current_page_and_disabled_ends() {
    let (kit, c) = mount_kit(PageNav::new(s("pg"), s("Results pages"), 3));
    let nav = by_id("pg-prev-0").parent_element().unwrap();
    assert_eq!(nav.tag_name(), "NAV");
    assert_eq!(
        nav.get_attribute("aria-label").as_deref(),
        Some("Results pages")
    );
    assert_eq!(attr("pg-page-1", "aria-current").as_deref(), Some("page"));
    assert!(attr("pg-page-2", "aria-current").is_none());
    assert!(attr("pg-prev-0", "disabled").is_some());
    assert_eq!(attr("pg-page-2", "aria-label").as_deref(), Some("Page 2"));
    assert_eq!(
        tab_order(&c),
        ["pg-page-1", "pg-page-2", "pg-page-3", "pg-next-0"],
        "a disabled Prev is skipped"
    );
    assert!(press("pg-next-0", "Enter"));
    assert_eq!(kit.with_model(|m| m.current()), 2);
    assert_eq!(attr("pg-page-2", "aria-current").as_deref(), Some("page"));
    assert!(attr("pg-prev-0", "disabled").is_none());
    click("pg-page-3");
    assert!(attr("pg-next-0", "disabled").is_some());
    assert_eq!(active_id(), "pg-page-3");
}

// ---- Tooltip --------------------------------------------------------------------------------------------------

#[wasm_bindgen_test]
fn tooltip_is_described_by_its_trigger_and_shows_on_focus_and_hides_on_escape() {
    let (kit, _c) = mount_kit(TooltipHost::new(
        s("tt"),
        s("Save"),
        s("Saves your work (Ctrl+S)"),
    ));
    assert_eq!(
        attr("tt-trigger-0", "aria-describedby").as_deref(),
        Some("tt-tip-0")
    );
    assert_eq!(attr("tt-tip-0", "role").as_deref(), Some("tooltip"));
    assert!(attr("tt-tip-0", "hidden").is_some());
    assert_eq!(
        described_by_text("tt-trigger-0"),
        "Saves your work (Ctrl+S)"
    );
    fire("tt-trigger-0", "focusin");
    assert!(kit.with_model(|m| m.is_shown()));
    assert!(attr("tt-tip-0", "hidden").is_none());
    assert!(
        press("tt-trigger-0", "Escape"),
        "Escape dismisses without moving focus"
    );
    assert!(attr("tt-tip-0", "hidden").is_some());
    fire("tt-trigger-0", "focusin");
    fire("tt-trigger-0", "focusout");
    assert!(!kit.with_model(|m| m.is_shown()));
}
