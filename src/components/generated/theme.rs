#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct ColorPalette {
    pub bg_primary: String,
    pub bg_secondary: String,
    pub bg_tertiary: String,
    pub bg_elevated: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub text_muted: String,
    pub accent_primary: String,
    pub accent_secondary: String,
    pub accent_success: String,
    pub accent_warning: String,
    pub accent_error: String,
    pub border_subtle: String,
    pub border_default: String,
    pub axis_x: String,
    pub axis_y: String,
    pub axis_z: String,
}

impl ColorPalette {

pub fn dark() -> ColorPalette {
        ColorPalette { bg_primary: String::from("#0a0a1a"), bg_secondary: String::from("#16213e"), bg_tertiary: String::from("#1a2744"), bg_elevated: String::from("#1e2d4d"), text_primary: String::from("#e0e0e0"), text_secondary: String::from("#a0a0a0"), text_muted: String::from("#666666"), accent_primary: String::from("#e94560"), accent_secondary: String::from("#0f3460"), accent_success: String::from("#4ade80"), accent_warning: String::from("#facc15"), accent_error: String::from("#ef4444"), border_subtle: String::from("rgba(255,255,255,0.05)"), border_default: String::from("rgba(255,255,255,0.1)"), axis_x: String::from("#e94560"), axis_y: String::from("#4ade80"), axis_z: String::from("#60a5fa") }
}

pub fn light() -> ColorPalette {
        ColorPalette { bg_primary: String::from("#f5f5f5"), bg_secondary: String::from("#ffffff"), bg_tertiary: String::from("#fafafa"), bg_elevated: String::from("#ffffff"), text_primary: String::from("#1a1a2e"), text_secondary: String::from("#666666"), text_muted: String::from("#999999"), accent_primary: String::from("#e94560"), accent_secondary: String::from("#3b82f6"), accent_success: String::from("#22c55e"), accent_warning: String::from("#eab308"), accent_error: String::from("#dc2626"), border_subtle: String::from("rgba(0,0,0,0.05)"), border_default: String::from("rgba(0,0,0,0.1)"), axis_x: String::from("#dc2626"), axis_y: String::from("#16a34a"), axis_z: String::from("#2563eb") }
}

pub fn fintech_light() -> ColorPalette {
        ColorPalette { bg_primary: String::from("#e6ecf1"), bg_secondary: String::from("#fafcfd"), bg_tertiary: String::from("#d4e4dc"), bg_elevated: String::from("#ffffff"), text_primary: String::from("#14222e"), text_secondary: String::from("#5a6b7a"), text_muted: String::from("#5a6b7a"), accent_primary: String::from("#1a6b57"), accent_secondary: String::from("#0e4a3c"), accent_success: String::from("#1f7a4d"), accent_warning: String::from("#b47a12"), accent_error: String::from("#b42a22"), border_subtle: String::from("rgba(20,34,46,0.06)"), border_default: String::from("#c5d0da"), axis_x: String::from("#b42a22"), axis_y: String::from("#1a6b57"), axis_z: String::from("#3b6ea5") }
}

pub fn high_contrast() -> ColorPalette {
        ColorPalette { bg_primary: String::from("#000000"), bg_secondary: String::from("#1a1a1a"), bg_tertiary: String::from("#2a2a2a"), bg_elevated: String::from("#333333"), text_primary: String::from("#ffffff"), text_secondary: String::from("#cccccc"), text_muted: String::from("#888888"), accent_primary: String::from("#ffcc00"), accent_secondary: String::from("#00ffcc"), accent_success: String::from("#00ff00"), accent_warning: String::from("#ffff00"), accent_error: String::from("#ff0000"), border_subtle: String::from("rgba(255,255,255,0.2)"), border_default: String::from("rgba(255,255,255,0.4)"), axis_x: String::from("#ff6666"), axis_y: String::from("#66ff66"), axis_z: String::from("#6666ff") }
}

pub fn monokai() -> ColorPalette {
        ColorPalette { bg_primary: String::from("#272822"), bg_secondary: String::from("#3e3d32"), bg_tertiary: String::from("#49483e"), bg_elevated: String::from("#75715e"), text_primary: String::from("#f8f8f2"), text_secondary: String::from("#a6a997"), text_muted: String::from("#75715e"), accent_primary: String::from("#f92672"), accent_secondary: String::from("#66d9ef"), accent_success: String::from("#a6e22e"), accent_warning: String::from("#fd971f"), accent_error: String::from("#f92672"), border_subtle: String::from("rgba(255,255,255,0.05)"), border_default: String::from("rgba(255,255,255,0.1)"), axis_x: String::from("#f92672"), axis_y: String::from("#a6e22e"), axis_z: String::from("#66d9ef") }
}

pub fn nord() -> ColorPalette {
        ColorPalette { bg_primary: String::from("#2e3440"), bg_secondary: String::from("#3b4252"), bg_tertiary: String::from("#434c5e"), bg_elevated: String::from("#4c566a"), text_primary: String::from("#eceff4"), text_secondary: String::from("#d8dee9"), text_muted: String::from("#81a1c1"), accent_primary: String::from("#88c0d0"), accent_secondary: String::from("#81a1c1"), accent_success: String::from("#a3be8c"), accent_warning: String::from("#ebcb8b"), accent_error: String::from("#bf616a"), border_subtle: String::from("rgba(255,255,255,0.05)"), border_default: String::from("rgba(255,255,255,0.1)"), axis_x: String::from("#bf616a"), axis_y: String::from("#a3be8c"), axis_z: String::from("#5e81ac") }
}
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct EditorTheme {
    pub name: String,
    pub palette: ColorPalette,
    pub font_family: String,
    pub font_size_base: i32,
    pub border_radius: i32,
    pub spacing_unit: i32,
    pub animation_duration: i32,
}

impl EditorTheme {
#[inline]
pub fn default() -> EditorTheme {
        EditorTheme { name: String::from("Windjammer Dark"), palette: ColorPalette::dark(), font_family: String::from("'Inter', 'SF Pro', -apple-system, sans-serif"), font_size_base: 13_i32, border_radius: 6_i32, spacing_unit: 8_i32, animation_duration: 150_i32 }
}
#[inline]
pub fn light() -> EditorTheme {
        EditorTheme { name: String::from("Windjammer Light"), palette: ColorPalette::light(), font_family: String::from("'Inter', 'SF Pro', -apple-system, sans-serif"), font_size_base: 13_i32, border_radius: 6_i32, spacing_unit: 8_i32, animation_duration: 150_i32 }
}
#[inline]
pub fn fintech_light() -> EditorTheme {
        EditorTheme { name: String::from("Calm Light"), palette: ColorPalette::fintech_light(), font_family: String::from("'Sora', 'IBM Plex Sans', sans-serif"), font_size_base: 14_i32, border_radius: 10_i32, spacing_unit: 8_i32, animation_duration: 160_i32 }
}
#[inline]
pub fn monokai() -> EditorTheme {
        EditorTheme { name: String::from("Monokai Pro"), palette: ColorPalette::monokai(), font_family: String::from("'Fira Code', 'JetBrains Mono', monospace"), font_size_base: 13_i32, border_radius: 4_i32, spacing_unit: 8_i32, animation_duration: 100_i32 }
}
#[inline]
pub fn nord() -> EditorTheme {
        EditorTheme { name: String::from("Nord"), palette: ColorPalette::nord(), font_family: String::from("'Source Sans Pro', 'Nunito', sans-serif"), font_size_base: 14_i32, border_radius: 8_i32, spacing_unit: 10_i32, animation_duration: 200_i32 }
}
}

impl Renderable for EditorTheme {
#[inline]
fn render(&self) -> String {
        let css = format!("{}{}{}{}{}{}{}{}{}{}{}", ":root { \n            --bg-primary: ", self.palette.bg_primary, ";\n            --bg-secondary: ", self.palette.bg_secondary, ";\n            --accent-primary: ", self.palette.accent_primary, ";\n            --text-primary: ", self.palette.text_primary, ";\n        }\n        body {\n            font-family: ", self.font_family, ";\n            color: var(--text-primary);\n            background: var(--bg-primary);\n        }");
        css
}
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct ThemeSwitcher {
    pub current_theme: String,
    pub themes: Vec<String>,
    pub on_change: String,
}

impl ThemeSwitcher {
#[inline]
pub fn new() -> ThemeSwitcher {
        let mut themes: Vec<String> = Vec::new();
        themes.push(String::from("dark"));
        themes.push(String::from("light"));
        themes.push(String::from("monokai"));
        themes.push(String::from("nord"));
        themes.push(String::from("high-contrast"));
        ThemeSwitcher { current_theme: String::from("dark"), themes, on_change: String::new() }
}
#[inline]
pub fn on_change(mut self, handler: impl Into<String>) -> ThemeSwitcher {
        self.on_change = handler.into();
        self
}
}

impl Renderable for ThemeSwitcher {

fn render(&self) -> String {
        let mut options = String::new();
        for t in &self.themes {
            let selected = {
                if *t == self.current_theme {
                    String::from("selected")
                } else {
                    String::new()
                }
            };
            options = options + &format!("<option value='{}' {}>{}</option>", t, selected, t);
        }
        format!("\n            <div class='theme-switcher'>\n                <label>🎨 Theme</label>\n                <select onchange='{}(this.value)'>\n                    {}\n                </select>\n            </div>\n        ", self.on_change, options)
}
}

