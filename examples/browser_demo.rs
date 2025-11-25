/// Interactive browser demo showcasing all Windjammer UI components
/// Run with: cargo build --example browser_demo --target wasm32-unknown-unknown
/// Then serve with: python3 -m http.server 8080 in the examples/ directory
use windjammer_ui::components::generated::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

fn generate_demo_page() -> String {
    // Header
    let header = Container::new()
        .max_width("1200px".to_string())
        .padding("32px".to_string())
        .background_color("#f8f9fa".to_string())
        .child("<h1 style='margin:0; color:#2c3e50;'>🎨 Windjammer UI Component Gallery</h1>".to_string())
        .child("<p style='margin:8px 0 0 0; color:#7f8c8d;'>All components written in pure Windjammer and transpiled to Rust!</p>".to_string())
        .render();

    // Text examples
    let text_section = Container::new()
        .max_width("1200px".to_string())
        .padding("24px".to_string())
        .child("<h2>Text Components</h2>".to_string())
        .child(
            Text::new("Small text example".to_string())
                .size(TextSize::Small)
                .render(),
        )
        .child(
            Text::new("Medium text example".to_string())
                .size(TextSize::Medium)
                .render(),
        )
        .child(
            Text::new("Large text example".to_string())
                .size(TextSize::Large)
                .render(),
        )
        .child(
            Text::new("Bold large text".to_string())
                .size(TextSize::XLarge)
                .bold()
                .render(),
        )
        .render();

    // Button examples
    let button_section = Container::new()
        .max_width("1200px".to_string())
        .padding("24px".to_string())
        .child("<h2>Button Components</h2>".to_string())
        .child(
            Flex::new()
                .direction(FlexDirection::Row)
                .gap("16px".to_string())
                .child(
                    Button::new("Primary".to_string())
                        .variant(ButtonVariant::Primary)
                        .render(),
                )
                .child(
                    Button::new("Secondary".to_string())
                        .variant(ButtonVariant::Secondary)
                        .render(),
                )
                .child(
                    Button::new("Success".to_string())
                        .variant(ButtonVariant::Success)
                        .render(),
                )
                .child(
                    Button::new("Danger".to_string())
                        .variant(ButtonVariant::Danger)
                        .render(),
                )
                .child(
                    Button::new("Ghost".to_string())
                        .variant(ButtonVariant::Ghost)
                        .render(),
                )
                .render(),
        )
        .child("<br/>".to_string())
        .child(
            Flex::new()
                .direction(FlexDirection::Row)
                .gap("16px".to_string())
                .child(
                    Button::new("Small".to_string())
                        .size(ButtonSize::Small)
                        .render(),
                )
                .child(
                    Button::new("Medium".to_string())
                        .size(ButtonSize::Medium)
                        .render(),
                )
                .child(
                    Button::new("Large".to_string())
                        .size(ButtonSize::Large)
                        .render(),
                )
                .child(Button::new("Disabled".to_string()).disabled(true).render())
                .render(),
        )
        .render();

    // Input & Form examples
    let form_section = Container::new()
        .max_width("1200px".to_string())
        .padding("24px".to_string())
        .child("<h2>Form Components</h2>".to_string())
        .child(
            Flex::new()
                .direction(FlexDirection::Column)
                .gap("16px".to_string())
                .child("<label>Username:</label>".to_string())
                .child(
                    Input::new()
                        .placeholder("Enter your username".to_string())
                        .render(),
                )
                .child("<label>Email:</label>".to_string())
                .child(
                    Input::new()
                        .placeholder("your@email.com".to_string())
                        .render(),
                )
                .child("<br/>".to_string())
                .child(
                    Checkbox::new("I agree to the terms".to_string())
                        .checked(false)
                        .render(),
                )
                .child(
                    Checkbox::new("Subscribe to newsletter".to_string())
                        .checked(true)
                        .render(),
                )
                .child(
                    Checkbox::new("Enable notifications".to_string())
                        .size(CheckboxSize::Large)
                        .render(),
                )
                .render(),
        )
        .render();

    // Slider examples
    let slider_section = Container::new()
        .max_width("1200px".to_string())
        .padding("24px".to_string())
        .child("<h2>Slider Components</h2>".to_string())
        .child(
            Slider::new()
                .label("Volume".to_string())
                .value(50.0)
                .render(),
        )
        .child("<br/>".to_string())
        .child(
            Slider::new()
                .label("Brightness".to_string())
                .min(0.0)
                .max(100.0)
                .value(75.0)
                .render(),
        )
        .child("<br/>".to_string())
        .child(
            Slider::new()
                .label("Precision".to_string())
                .min(0.0)
                .max(10.0)
                .step(0.1)
                .value(5.5)
                .render(),
        )
        .render();

    // Layout examples
    let layout_section = Container::new()
        .max_width("1200px".to_string())
        .padding("24px".to_string())
        .child("<h2>Layout Components</h2>".to_string())
        .child("<h3>Flex Row:</h3>".to_string())
        .child(
            Flex::new()
                .direction(FlexDirection::Row)
                .gap("16px".to_string())
                .padding("16px".to_string())
                .background_color("#e3f2fd".to_string())
                .child("<div style='padding:16px; background:#2196f3; color:white;'>Item 1</div>".to_string())
                .child("<div style='padding:16px; background:#2196f3; color:white;'>Item 2</div>".to_string())
                .child("<div style='padding:16px; background:#2196f3; color:white;'>Item 3</div>".to_string())
                .render()
        )
        .child("<br/>".to_string())
        .child("<h3>Flex Column:</h3>".to_string())
        .child(
            Flex::new()
                .direction(FlexDirection::Column)
                .gap("8px".to_string())
                .padding("16px".to_string())
                .background_color("#fff3e0".to_string())
                .child("<div style='padding:16px; background:#ff9800; color:white;'>Column Item 1</div>".to_string())
                .child("<div style='padding:16px; background:#ff9800; color:white;'>Column Item 2</div>".to_string())
                .child("<div style='padding:16px; background:#ff9800; color:white;'>Column Item 3</div>".to_string())
                .render()
        )
        .render();

    // Assemble page
    let full_page = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Windjammer UI Gallery</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #ffffff; }}
        
        /* Text styles */
        .wj-text {{ display: inline-block; margin: 8px 0; }}
        .wj-text.sm {{ font-size: 14px; }}
        .wj-text.md {{ font-size: 16px; }}
        .wj-text.lg {{ font-size: 20px; }}
        .wj-text.xl {{ font-size: 24px; }}
        .wj-text.bold {{ font-weight: bold; }}
        
        /* Button styles */
        .wj-button {{
            border: none; padding: 8px 16px; cursor: pointer; border-radius: 4px;
            font-size: 14px; font-weight: 500; transition: all 0.2s;
        }}
        .wj-button:hover {{ transform: translateY(-1px); box-shadow: 0 4px 8px rgba(0,0,0,0.1); }}
        .wj-button-primary {{ background: #007bff; color: white; }}
        .wj-button-secondary {{ background: #6c757d; color: white; }}
        .wj-button-success {{ background: #28a745; color: white; }}
        .wj-button-danger {{ background: #dc3545; color: white; }}
        .wj-button-ghost {{ background: transparent; color: #007bff; border: 1px solid #007bff; }}
        .wj-button-sm {{ padding: 4px 12px; font-size: 12px; }}
        .wj-button-md {{ padding: 8px 16px; font-size: 14px; }}
        .wj-button-lg {{ padding: 12px 24px; font-size: 16px; }}
        .wj-button[disabled] {{ opacity: 0.5; cursor: not-allowed; }}
        
        /* Input styles */
        .wj-input {{
            padding: 8px 12px; border: 1px solid #ced4da; border-radius: 4px;
            font-size: 14px; width: 100%; max-width: 400px;
        }}
        .wj-input:focus {{ outline: none; border-color: #007bff; box-shadow: 0 0 0 3px rgba(0,123,255,0.1); }}
        
        /* Checkbox styles */
        .wj-checkbox {{ display: flex; align-items: center; gap: 8px; cursor: pointer; margin: 8px 0; }}
        .wj-checkbox input {{ width: 16px; height: 16px; cursor: pointer; }}
        .wj-checkbox-lg input {{ width: 20px; height: 20px; }}
        .wj-checkbox-sm input {{ width: 14px; height: 14px; }}
        
        /* Slider styles */
        .wj-slider {{
            width: 100%; max-width: 400px; height: 6px;
            -webkit-appearance: none; appearance: none; background: #e0e0e0;
            border-radius: 3px; outline: none;
        }}
        .wj-slider::-webkit-slider-thumb {{
            -webkit-appearance: none; appearance: none;
            width: 18px; height: 18px; background: #007bff;
            border-radius: 50%; cursor: pointer;
        }}
        
        /* Container & Flex styles */
        .wj-container {{ display: block; }}
        .wj-flex {{ display: flex; }}
        
        h2 {{ margin: 24px 0 16px 0; color: #2c3e50; font-size: 24px; }}
        h3 {{ margin: 16px 0 8px 0; color: #34495e; font-size: 18px; }}
        label {{ font-weight: 500; color: #495057; }}
    </style>
</head>
<body>
    {}
    {}
    {}
    {}
    {}
    {}
    <footer style="text-align:center; padding:48px; color:#7f8c8d; font-size:14px;">
        <p>🚀 Built with <strong>Windjammer</strong> - 80% of Rust's power with 20% of the complexity!</p>
        <p style="margin-top:8px;">All components compiled from <code>.wj</code> source files.</p>
    </footer>
</body>
</html>"#,
        header, text_section, button_section, form_section, slider_section, layout_section
    );

    full_page
}

fn main() {
    let html = generate_demo_page();

    // Write to file for local viewing
    std::fs::write("examples/gallery.html", &html).expect("Failed to write gallery.html");

    println!("✅ Gallery generated: examples/gallery.html");
    println!(
        "📖 Open in browser: file://{}/examples/gallery.html",
        std::env::current_dir().unwrap().display()
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run() {
    let html = generate_demo_page();
    web_sys::window()
        .and_then(|win| win.document())
        .map(|doc| doc.body().map(|body| body.set_inner_html(&html)));
}
