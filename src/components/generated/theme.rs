#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
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
    #[inline]
    pub fn dark() -> ColorPalette {
        ColorPalette {
            bg_primary: "#0a0a1a".to_string(),
            bg_secondary: "#16213e".to_string(),
            bg_tertiary: "#1a2744".to_string(),
            bg_elevated: "#1e2d4d".to_string(),
            text_primary: "#e0e0e0".to_string(),
            text_secondary: "#a0a0a0".to_string(),
            text_muted: "#666666".to_string(),
            accent_primary: "#e94560".to_string(),
            accent_secondary: "#0f3460".to_string(),
            accent_success: "#4ade80".to_string(),
            accent_warning: "#facc15".to_string(),
            accent_error: "#ef4444".to_string(),
            border_subtle: "rgba(255,255,255,0.05)".to_string(),
            border_default: "rgba(255,255,255,0.1)".to_string(),
            axis_x: "#e94560".to_string(),
            axis_y: "#4ade80".to_string(),
            axis_z: "#60a5fa".to_string(),
        }
    }
    #[inline]
    pub fn light() -> ColorPalette {
        ColorPalette {
            bg_primary: "#f5f5f5".to_string(),
            bg_secondary: "#ffffff".to_string(),
            bg_tertiary: "#fafafa".to_string(),
            bg_elevated: "#ffffff".to_string(),
            text_primary: "#1a1a2e".to_string(),
            text_secondary: "#666666".to_string(),
            text_muted: "#999999".to_string(),
            accent_primary: "#e94560".to_string(),
            accent_secondary: "#3b82f6".to_string(),
            accent_success: "#22c55e".to_string(),
            accent_warning: "#eab308".to_string(),
            accent_error: "#dc2626".to_string(),
            border_subtle: "rgba(0,0,0,0.05)".to_string(),
            border_default: "rgba(0,0,0,0.1)".to_string(),
            axis_x: "#dc2626".to_string(),
            axis_y: "#16a34a".to_string(),
            axis_z: "#2563eb".to_string(),
        }
    }
    #[inline]
    pub fn high_contrast() -> ColorPalette {
        ColorPalette {
            bg_primary: "#000000".to_string(),
            bg_secondary: "#1a1a1a".to_string(),
            bg_tertiary: "#2a2a2a".to_string(),
            bg_elevated: "#333333".to_string(),
            text_primary: "#ffffff".to_string(),
            text_secondary: "#cccccc".to_string(),
            text_muted: "#888888".to_string(),
            accent_primary: "#ffcc00".to_string(),
            accent_secondary: "#00ffcc".to_string(),
            accent_success: "#00ff00".to_string(),
            accent_warning: "#ffff00".to_string(),
            accent_error: "#ff0000".to_string(),
            border_subtle: "rgba(255,255,255,0.2)".to_string(),
            border_default: "rgba(255,255,255,0.4)".to_string(),
            axis_x: "#ff6666".to_string(),
            axis_y: "#66ff66".to_string(),
            axis_z: "#6666ff".to_string(),
        }
    }
    #[inline]
    pub fn monokai() -> ColorPalette {
        ColorPalette {
            bg_primary: "#272822".to_string(),
            bg_secondary: "#3e3d32".to_string(),
            bg_tertiary: "#49483e".to_string(),
            bg_elevated: "#75715e".to_string(),
            text_primary: "#f8f8f2".to_string(),
            text_secondary: "#a6a997".to_string(),
            text_muted: "#75715e".to_string(),
            accent_primary: "#f92672".to_string(),
            accent_secondary: "#66d9ef".to_string(),
            accent_success: "#a6e22e".to_string(),
            accent_warning: "#fd971f".to_string(),
            accent_error: "#f92672".to_string(),
            border_subtle: "rgba(255,255,255,0.05)".to_string(),
            border_default: "rgba(255,255,255,0.1)".to_string(),
            axis_x: "#f92672".to_string(),
            axis_y: "#a6e22e".to_string(),
            axis_z: "#66d9ef".to_string(),
        }
    }
    #[inline]
    pub fn nord() -> ColorPalette {
        ColorPalette {
            bg_primary: "#2e3440".to_string(),
            bg_secondary: "#3b4252".to_string(),
            bg_tertiary: "#434c5e".to_string(),
            bg_elevated: "#4c566a".to_string(),
            text_primary: "#eceff4".to_string(),
            text_secondary: "#d8dee9".to_string(),
            text_muted: "#81a1c1".to_string(),
            accent_primary: "#88c0d0".to_string(),
            accent_secondary: "#81a1c1".to_string(),
            accent_success: "#a3be8c".to_string(),
            accent_warning: "#ebcb8b".to_string(),
            accent_error: "#bf616a".to_string(),
            border_subtle: "rgba(255,255,255,0.05)".to_string(),
            border_default: "rgba(255,255,255,0.1)".to_string(),
            axis_x: "#bf616a".to_string(),
            axis_y: "#a3be8c".to_string(),
            axis_z: "#5e81ac".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
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
        EditorTheme {
            name: "Windjammer Dark".to_string(),
            palette: ColorPalette::dark(),
            font_family: "'Inter', 'SF Pro', -apple-system, sans-serif".to_string(),
            font_size_base: 13,
            border_radius: 6,
            spacing_unit: 8,
            animation_duration: 150,
        }
    }
    #[inline]
    pub fn light() -> EditorTheme {
        EditorTheme {
            name: "Windjammer Light".to_string(),
            palette: ColorPalette::light(),
            font_family: "'Inter', 'SF Pro', -apple-system, sans-serif".to_string(),
            font_size_base: 13,
            border_radius: 6,
            spacing_unit: 8,
            animation_duration: 150,
        }
    }
    #[inline]
    pub fn monokai() -> EditorTheme {
        EditorTheme {
            name: "Monokai Pro".to_string(),
            palette: ColorPalette::monokai(),
            font_family: "'Fira Code', 'JetBrains Mono', monospace".to_string(),
            font_size_base: 13,
            border_radius: 4,
            spacing_unit: 8,
            animation_duration: 100,
        }
    }
    #[inline]
    pub fn nord() -> EditorTheme {
        EditorTheme {
            name: "Nord".to_string(),
            palette: ColorPalette::nord(),
            font_family: "'Source Sans Pro', 'Nunito', sans-serif".to_string(),
            font_size_base: 14,
            border_radius: 8,
            spacing_unit: 10,
            animation_duration: 200,
        }
    }
}

impl Renderable for EditorTheme {
    #[inline]
    fn render(self) -> String {
        let css = format!(
            "{}{}{}{}{}{}{}{}{}{}{}",
            ":root { 
            --bg-primary: ",
            self.palette.bg_primary.as_str(),
            ";
            --bg-secondary: ",
            self.palette.bg_secondary.as_str(),
            ";
            --accent-primary: ",
            self.palette.accent_primary.as_str(),
            ";
            --text-primary: ",
            self.palette.text_primary.as_str(),
            ";
        }
        body {
            font-family: ",
            self.font_family.as_str(),
            ";
            color: var(--text-primary);
            background: var(--bg-primary);
        }"
        );
        css
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ThemeSwitcher {
    pub current_theme: String,
    pub themes: Vec<String>,
    pub on_change: String,
}

impl ThemeSwitcher {
    #[inline]
    pub fn new() -> ThemeSwitcher {
        let mut themes = Vec::new();
        themes.push("dark".to_string());
        themes.push("light".to_string());
        themes.push("monokai".to_string());
        themes.push("nord".to_string());
        themes.push("high-contrast".to_string());
        ThemeSwitcher {
            current_theme: "dark".to_string(),
            themes,
            on_change: "".to_string(),
        }
    }
    #[inline]
    pub fn on_change(mut self, handler: String) -> ThemeSwitcher {
        self.on_change = handler;
        self
    }
}

impl Renderable for ThemeSwitcher {
    #[inline]
    fn render(self) -> String {
        let mut options = "".to_string();
        for t in &self.themes {
            let selected = {
                if t.as_str() == self.current_theme.as_str() {
                    "selected".to_string()
                } else {
                    "".to_string()
                }
            };
            options += format!("<option value='{}' {}>{}</option>", t, selected, t).as_str();
        }
        format!(
            "
            <div class='theme-switcher'>
                <label>🎨 Theme</label>
                <select onchange='{}(this.value)'>
                    {}
                </select>
            </div>
        ",
            self.on_change, options
        )
    }
}
