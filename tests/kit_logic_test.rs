//! Host-side checks of the kit's logic and declared tokens (no browser): validation messages, data grid ordering,
//! virtual list windows, and the token set's coverage and WCAG contrast.

use windjammer_ui::components::generated::a11y::*;
use windjammer_ui::components::generated::form_kit::*;
use windjammer_ui::components::generated::grid_kit::*;
use windjammer_ui::components::generated::tokens::*;

fn s(v: &str) -> String {
    v.to_string()
}

// ---- keyboard rules -------------------------------------------------------------------------------------------

#[test]
fn roving_next_wraps_and_ignores_the_other_axis() {
    assert_eq!(roving_next("ArrowRight", 2, 3, false), 0);
    assert_eq!(roving_next("ArrowLeft", 0, 3, false), 2);
    assert_eq!(
        roving_next("ArrowDown", 0, 3, false),
        -1,
        "vertical keys do nothing in a horizontal group"
    );
    assert_eq!(roving_next("ArrowDown", 0, 3, true), 1);
    assert_eq!(roving_next("Home", 2, 3, true), 0);
    assert_eq!(roving_next("End", 0, 3, true), 2);
    assert_eq!(roving_next("ArrowRight", 0, 0, false), -1, "empty group");
}

#[test]
fn act_index_parses_only_its_own_prefix() {
    assert_eq!(act_index("tab:3", "tab"), 3);
    assert_eq!(act_index("tab:3", "head"), -1);
    assert_eq!(act_index("tab:x", "tab"), -1);
    assert_eq!(act_index("tab", "tab"), -1);
}

// ---- validation -----------------------------------------------------------------------------------------------

fn msg(kind: FieldKind, value: &str) -> String {
    FormField::new(s("f"), s("Field"), kind)
        .required(true)
        .min_len(3)
        .min(s("5"))
        .max(s("10"))
        .option(s("a"), s("A"))
        .value(s(value))
        .error_message()
}

#[test]
fn validation_messages_per_kind() {
    assert_eq!(msg(FieldKind::Text, ""), "Field is required");
    assert_eq!(
        msg(FieldKind::Text, "   "),
        "Field is required",
        "whitespace is empty"
    );
    assert_eq!(
        msg(FieldKind::Text, "ab"),
        "Field must be at least 3 characters"
    );
    assert_eq!(msg(FieldKind::Text, "abc"), "");
    assert_eq!(
        msg(FieldKind::Email, "a@b"),
        "Field must be an email address"
    );
    assert_eq!(msg(FieldKind::Email, "a@b.co"), "");
    assert_eq!(
        msg(FieldKind::Email, "@b.co"),
        "Field must be an email address"
    );
    assert_eq!(msg(FieldKind::Number, "x"), "Field must be a number");
    assert_eq!(msg(FieldKind::Number, "4"), "Field must be at least 5");
    assert_eq!(msg(FieldKind::Number, "11"), "Field must be at most 10");
    assert_eq!(msg(FieldKind::Number, "7.5"), "");
    assert_eq!(
        msg(FieldKind::Select, "zzz"),
        "Field must be one of the listed options"
    );
    assert_eq!(msg(FieldKind::Select, "a"), "");
    assert_eq!(msg(FieldKind::Checkbox, "false"), "Field is required");
    assert_eq!(msg(FieldKind::Checkbox, "true"), "");
}

#[test]
fn optional_empty_fields_are_valid_and_dates_compare_as_iso_strings() {
    let optional = FormField::new(s("n"), s("Age"), FieldKind::Number).min(s("18"));
    assert_eq!(optional.error_message(), "");
    let born = |v: &str| {
        FormField::new(s("d"), s("Born"), FieldKind::Date)
            .min(s("2000-01-01"))
            .max(s("2030-12-31"))
            .value(s(v))
            .error_message()
    };
    assert_eq!(born("2010-06-01"), "");
    assert_eq!(born("1999-12-31"), "Born must be on or after 2000-01-01");
    assert_eq!(born("2031-01-01"), "Born must be on or before 2030-12-31");
    assert_eq!(born("06/01/2010"), "Born must be a date (YYYY-MM-DD)");
}

#[test]
fn form_submit_and_reset_without_a_browser() {
    let mut form = FormKit::new(s("x"), s("T"))
        .field(
            FormField::new(s("a"), s("A"), FieldKind::Text)
                .required(true)
                .value(s("keep")),
        )
        .field(FormField::new(s("b"), s("B"), FieldKind::Number).required(true));
    assert_eq!(form.error_count(), 1);
    assert!(form.on_event(s("click"), s("submit"), s(""), s("")));
    assert_eq!(form.status(), "1 field needs attention");
    assert_eq!(form.focus_id(), "x-f-1");
    form.on_event(s("input"), s("field:1"), s(""), s("3"));
    form.on_event(s("input"), s("field:0"), s(""), s("changed"));
    form.on_event(s("click"), s("submit"), s(""), s(""));
    assert_eq!(form.submitted(), "a=changed&b=3");
    form.on_event(s("click"), s("reset"), s(""), s(""));
    assert_eq!(
        form.error_count(),
        1,
        "back to the initial values: a is `keep`, b is empty"
    );
    assert_eq!(
        form.submitted(),
        "a=changed&b=3",
        "the last successful submission is kept for the host"
    );
}

// ---- data grid ------------------------------------------------------------------------------------------------

fn grid() -> DataGrid {
    let mut g = DataGrid::new(s("g"), s("c"))
        .column(GridColumn::text(s("N")))
        .column(GridColumn::number(s("V")))
        .page_size(2);
    for (n, v) in [("b", "10"), ("a", "9"), ("c", "9"), ("d", "100")] {
        g = g.row(vec![s(n), s(v)]);
    }
    g
}

#[test]
fn compare_cells_orders_numbers_numerically_and_text_case_insensitively() {
    assert_eq!(compare_cells("9", "100", true), -1);
    assert_eq!(
        compare_cells("9", "100", false),
        1,
        "as text, 9 sorts after 100"
    );
    assert_eq!(compare_cells("Abc", "abd", false), -1);
    assert_eq!(compare_cells("x", "X", false), 0);
}

#[test]
fn sort_is_stable_and_descending_reverses_it() {
    let mut g = grid();
    g.on_event(s("click"), s("sort:1"), s(""), s(""));
    assert_eq!(
        g.matching_rows(),
        [1, 2, 0, 3],
        "equal values (a, c) keep their original order"
    );
    g.on_event(s("click"), s("sort:1"), s(""), s(""));
    assert_eq!(g.matching_rows(), [3, 0, 1, 2]);
    g.on_event(s("click"), s("sort:1"), s(""), s(""));
    assert_eq!(
        g.matching_rows(),
        [0, 1, 2, 3],
        "third click clears the sort"
    );
}

#[test]
fn filter_and_paging_compose() {
    let mut g = grid();
    assert_eq!(g.page_count(), 2);
    assert_eq!(g.visible_rows(), [0, 1]);
    g.on_event(s("click"), s("next"), s(""), s(""));
    assert_eq!(g.visible_rows(), [2, 3]);
    g.on_event(s("click"), s("next"), s(""), s(""));
    assert_eq!(g.current_page(), 2, "cannot page past the end");
    g.on_event(s("input"), s("filter"), s(""), s("9"));
    assert_eq!(g.current_page(), 1, "filtering returns to the first page");
    assert_eq!(g.matching_rows(), [1, 2]);
    assert_eq!(g.page_count(), 1);
    g.on_event(s("input"), s("filter"), s(""), s("nothing"));
    assert_eq!(g.page_count(), 1);
    assert!(g.visible_rows().is_empty());
}

// ---- virtual list ---------------------------------------------------------------------------------------------

#[test]
fn virtual_window_tracks_scroll_and_clamps_to_the_list() {
    let mut v = VirtualList::new(s("v"), s("l"), 1000, 10, 100);
    assert_eq!((v.first_rendered(), v.last_rendered()), (0, 13));
    v.on_event(s("scroll"), s("vlist"), s(""), s("5000"));
    assert_eq!((v.first_rendered(), v.last_rendered()), (497, 513));
    v.on_event(s("scroll"), s("vlist"), s(""), s("999999"));
    assert_eq!(v.last_rendered(), 999, "never past the last row");
    let tiny = VirtualList::new(s("t"), s("l"), 3, 10, 100);
    assert_eq!((tiny.first_rendered(), tiny.last_rendered()), (0, 2));
}

// ---- tokens ---------------------------------------------------------------------------------------------------

fn lum(hex: &str) -> f64 {
    let c = |i: usize| {
        let v = u8::from_str_radix(&hex[i..i + 2], 16).unwrap() as f64 / 255.0;
        if v <= 0.03928 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * c(1) + 0.7152 * c(3) + 0.0722 * c(5)
}

fn contrast(a: &str, b: &str) -> f64 {
    let (x, y) = (lum(a), lum(b));
    (x.max(y) + 0.05) / (x.min(y) + 0.05)
}

fn tok(name: &str, mode: ThemeMode) -> String {
    tokens()
        .into_iter()
        .find(|t| t.name == name)
        .unwrap_or_else(|| panic!("token {name}"))
        .value(mode)
}

#[test]
fn text_and_status_colors_meet_wcag_in_every_mode() {
    // (mode, text floor). AA is 4.5:1; the high-contrast mode holds itself to AAA, 7:1.
    for (label, floor) in [("light", 4.5), ("dark", 4.5), ("high-contrast", 7.0)] {
        let m = || match label {
            "light" => ThemeMode::Light,
            "dark" => ThemeMode::Dark,
            _ => ThemeMode::HighContrast,
        };
        let bg = tok("color-bg", m());
        let surface = tok("color-surface", m());
        for name in [
            "color-text",
            "color-text-muted",
            "color-danger",
            "color-success",
            "color-primary",
        ] {
            for ground in [&bg, &surface] {
                let c = contrast(&tok(name, m()), ground);
                assert!(
                    c >= floor,
                    "{label}: {name} on {ground} is {c:.2}:1, needs {floor}"
                );
            }
        }
        let on = contrast(&tok("color-on-primary", m()), &tok("color-primary", m()));
        assert!(on >= floor, "{label}: on-primary over primary is {on:.2}:1");
        // Non-text UI (borders, focus ring) needs 3:1 against the page.
        for name in ["color-border", "color-focus-ring"] {
            let c = contrast(&tok(name, m()), &bg);
            assert!(c >= 3.0, "{label}: {name} is {c:.2}:1 against the page");
        }
    }
}

#[test]
fn every_token_is_declared_for_every_mode_and_the_css_uses_only_declared_tokens() {
    let css = theme_css();
    let names: Vec<String> = tokens().into_iter().map(|t| t.name).collect();
    for n in &names {
        // light + dark + high-contrast blocks, plus the two system-preference fallbacks.
        assert_eq!(
            css.matches(&format!("--wj-{n}:")).count(),
            5,
            "token {n} must be declared in every mode"
        );
    }
    for mode in ["light", "dark", "high-contrast"] {
        assert!(css.contains(&format!("[data-wj-theme=\"{mode}\"]")));
    }
    assert!(css.contains("prefers-color-scheme: dark") && css.contains("prefers-contrast: more"));
    let kit = kit_css();
    let mut rest = kit.as_str();
    let mut used = 0;
    while let Some(i) = rest.find("var(--wj-") {
        let tail = &rest[i + "var(--wj-".len()..];
        let end = tail.find(')').unwrap();
        assert!(
            names.iter().any(|n| n == &tail[..end]),
            "kit_css reads undeclared token `{}`",
            &tail[..end]
        );
        used += 1;
        rest = &tail[end..];
    }
    assert!(used > 20, "the kit styles are written in tokens");
    assert!(
        !kit.contains('#'),
        "component styles contain no literal colors; they read tokens"
    );
}

#[test]
fn token_values_differ_between_modes_where_they_should() {
    assert_ne!(
        tok("color-bg", ThemeMode::Light),
        tok("color-bg", ThemeMode::Dark)
    );
    assert_ne!(
        tok("color-text", ThemeMode::Light),
        tok("color-text", ThemeMode::Dark)
    );
    assert_eq!(tok("border-width", ThemeMode::HighContrast), "2px");
    assert_eq!(mode_name(ThemeMode::HighContrast), "high-contrast");
}
