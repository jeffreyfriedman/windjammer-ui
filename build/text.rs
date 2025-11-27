enum TextSize {
    Small,
    Medium,
    Large,
    XLarge,
}

enum TextWeight {
    Normal,
    Bold,
}

struct Text {
    content: String,
    size: TextSize,
    weight: TextWeight,
}

impl Text {
#[inline]
fn new(content: String) -> Text {
        Text { content, size: TextSize::Medium, weight: TextWeight::Normal }
}
#[inline]
fn size(mut self, size: TextSize) -> Text {
        self.size = size;
        self
}
#[inline]
fn bold(mut self) -> Text {
        self.weight = TextWeight::Bold;
        self
}
#[inline]
fn render(mut self) -> String {
        let size_class = match self.size {
            TextSize::Small => "sm",
            TextSize::Medium => "md",
            TextSize::Large => "lg",
            TextSize::XLarge => "xl",
        };
        let weight_class = match self.weight {
            TextWeight::Normal => "normal",
            TextWeight::Bold => "bold",
        };
        format!("<span class='wj-text {} {}'>{}</span>", size_class, weight_class, self.content)
}
}

fn main() {
    let text = Text::new("Hello, Windjammer!".to_string()).size(TextSize::Large).bold();
    let html = text.render();
    println!("{}", html)
}

