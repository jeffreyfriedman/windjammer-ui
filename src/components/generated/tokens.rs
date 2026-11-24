#[allow(unused_imports)]
use super::*;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ThemeMode {
    Light,
    Dark,
    HighContrast,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct Token {
    pub name: String,
    pub light: String,
    pub dark: String,
    pub contrast: String,
}

impl Token {
#[inline]
pub fn new(name: String, light: String, dark: String, contrast: String) -> Token {
        Token { name, light, dark, contrast }
}
#[inline]
pub fn value(&self, mode: ThemeMode) -> String {
        match mode {
            ThemeMode::Light => self.light.clone(),
            ThemeMode::Dark => self.dark.clone(),
            ThemeMode::HighContrast => self.contrast.clone(),
        }
}
}

/// The value of the `data-wj-theme` attribute that selects `mode`.
#[inline]
pub fn mode_name(mode: ThemeMode) -> String {
    match mode {
        ThemeMode::Light => String::from("light"),
        ThemeMode::Dark => String::from("dark"),
        ThemeMode::HighContrast => String::from("high-contrast"),
    }
}

/// Every token, declared once. Colors are six-digit hex so contrast can be checked mechanically.

pub fn tokens() -> Vec<Token> {
    let mut t: Vec<Token> = Vec::new();
    t.push(Token::new("color-bg".to_string(), "#ffffff".to_string(), "#0f1419".to_string(), "#000000".to_string()));
    t.push(Token::new("color-surface".to_string(), "#f3f5f8".to_string(), "#1a2029".to_string(), "#000000".to_string()));
    t.push(Token::new("color-text".to_string(), "#1a1f27".to_string(), "#e8ecf1".to_string(), "#ffffff".to_string()));
    t.push(Token::new("color-text-muted".to_string(), "#4a5361".to_string(), "#a9b4c2".to_string(), "#ffffff".to_string()));
    t.push(Token::new("color-border".to_string(), "#6b7686".to_string(), "#74839a".to_string(), "#ffffff".to_string()));
    t.push(Token::new("color-primary".to_string(), "#1a56db".to_string(), "#6ea8fe".to_string(), "#ffff00".to_string()));
    t.push(Token::new("color-on-primary".to_string(), "#ffffff".to_string(), "#0b1220".to_string(), "#000000".to_string()));
    t.push(Token::new("color-focus-ring".to_string(), "#1a56db".to_string(), "#9ec5fe".to_string(), "#00ffff".to_string()));
    t.push(Token::new("color-danger".to_string(), "#b42318".to_string(), "#ff8a80".to_string(), "#ff6b6b".to_string()));
    t.push(Token::new("color-success".to_string(), "#067647".to_string(), "#5fd38d".to_string(), "#00ff7f".to_string()));
    t.push(Token::new("focus-width".to_string(), "2px".to_string(), "2px".to_string(), "3px".to_string()));
    t.push(Token::new("border-width".to_string(), "1px".to_string(), "1px".to_string(), "2px".to_string()));
    t.push(Token::new("radius".to_string(), "6px".to_string(), "6px".to_string(), "2px".to_string()));
    t.push(Token::new("space-1".to_string(), "4px".to_string(), "4px".to_string(), "4px".to_string()));
    t.push(Token::new("space-2".to_string(), "8px".to_string(), "8px".to_string(), "8px".to_string()));
    t.push(Token::new("space-3".to_string(), "16px".to_string(), "16px".to_string(), "16px".to_string()));
    t
}

#[inline]
fn declarations(mode: ThemeMode) -> String {
    let mut css = String::new();
    for tok in tokens() {
        { use std::fmt::Write as _; write!(css, "  --wj-{}: {};\n", tok.name, tok.value(mode)).expect("a formatting trait implementation returned an error"); };
    }
    css
}

/// All token declarations for every mode, plus the system-preference fallbacks.

pub fn theme_css() -> String {
    let light = declarations(ThemeMode::Light);
    let dark = declarations(ThemeMode::Dark);
    let contrast = declarations(ThemeMode::HighContrast);
    let mut css = format!(":root, [data-wj-theme=\"light\"] {{\n{}  color-scheme: light;\n}}\n", light);
    css = format!("{}[data-wj-theme=\"dark\"] {{\n{}  color-scheme: dark;\n}}\n", css, dark);
    css = format!("{}[data-wj-theme=\"high-contrast\"] {{\n{}  color-scheme: dark;\n}}\n", css, contrast);
    css = format!("{}@media (prefers-color-scheme: dark) {{\n:root:not([data-wj-theme]) {{\n{}  color-scheme: dark;\n}}\n}}\n", css, dark);
    css = format!("{}@media (prefers-contrast: more) {{\n:root:not([data-wj-theme]) {{\n{}  color-scheme: dark;\n}}\n}}\n", css, contrast);
    css
}

/// Kit component styles. Colors, widths and spacing come only from `var(--wj-...)` tokens.

pub fn kit_css() -> String {
    let mut css = String::new();
    { use std::fmt::Write as _; write!(css, "body {{ background: var(--wj-color-bg); color: var(--wj-color-text); }}\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, ".wj-button, .wj-tab, .wj-sort, .wj-page, .wj-menu-item, .wj-switch, .wj-accordion-head {{\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, "  background: var(--wj-color-surface); color: var(--wj-color-text);\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, "  border: var(--wj-border-width) solid var(--wj-color-border); border-radius: var(--wj-radius);\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, "  padding: var(--wj-space-2) var(--wj-space-3); cursor: pointer;\n}}\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, ".wj-button-primary {{ background: var(--wj-color-primary); color: var(--wj-color-on-primary); }}\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, ".wj-tab[aria-selected=\"true\"], .wj-page[aria-current=\"page\"], .wj-switch[aria-checked=\"true\"], [aria-pressed=\"true\"] {{\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, "  background: var(--wj-color-primary); color: var(--wj-color-on-primary);\n}}\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, ":focus-visible, [tabindex]:focus-visible {{\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, "  outline: var(--wj-focus-width) solid var(--wj-color-focus-ring); outline-offset: 2px;\n}}\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, "input, select, .wj-input {{\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, "  background: var(--wj-color-bg); color: var(--wj-color-text);\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, "  border: var(--wj-border-width) solid var(--wj-color-border); border-radius: var(--wj-radius);\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, "  padding: var(--wj-space-1) var(--wj-space-2);\n}}\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, "[aria-invalid=\"true\"] {{ border-color: var(--wj-color-danger); }}\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, ".wj-error {{ color: var(--wj-color-danger); margin: var(--wj-space-1) 0 0; }}\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, ".wj-hint {{ color: var(--wj-color-text-muted); margin: var(--wj-space-1) 0 0; }}\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, ".wj-form-status {{ color: var(--wj-color-success); }}\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, ".wj-field {{ margin-bottom: var(--wj-space-3); }}\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, ".wj-table {{ border-collapse: collapse; width: 100%; }}\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, ".wj-table th, .wj-table td {{ border: var(--wj-border-width) solid var(--wj-color-border); padding: var(--wj-space-2); text-align: left; }}\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, ".wj-modal-backdrop {{ background: var(--wj-color-bg); }}\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, ".wj-modal, .wj-menu, .wj-tooltip {{ background: var(--wj-color-surface); color: var(--wj-color-text);\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, "  border: var(--wj-border-width) solid var(--wj-color-border); border-radius: var(--wj-radius); padding: var(--wj-space-3); }}\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, ".wj-vlist {{ border: var(--wj-border-width) solid var(--wj-color-border); }}\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, ".wj-vlist-row[aria-selected=\"true\"] {{ background: var(--wj-color-primary); color: var(--wj-color-on-primary); }}\n").expect("a formatting trait implementation returned an error"); };
    { use std::fmt::Write as _; write!(css, "[hidden] {{ display: none !important; }}\n").expect("a formatting trait implementation returned an error"); };
    css
}

