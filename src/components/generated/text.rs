pub enum TextSize {
    Small,
    Medium,
    Large,
    XLarge,
}

pub enum TextWeight {
    Normal,
    Bold,
}

pub struct Text {
    content: String,
    size: TextSize,
    weight: TextWeight,
}

impl Text {
    #[inline]
    pub fn new(content: String) -> Text {
        Text {
            content,
            size: TextSize::Medium,
            weight: TextWeight::Normal,
        }
    }
    #[inline]
    pub fn size(mut self, size: TextSize) -> Text {
        self.size = size;
        self
    }
    #[inline]
    pub fn bold(mut self) -> Text {
        self.weight = TextWeight::Bold;
        self
    }
    #[inline]
    pub fn render(mut self) -> String {
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
        format!(
            "<span class='wj-text {} {}'>{}</span>",
            size_class, weight_class, self.content
        )
    }
}
