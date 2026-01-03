#[derive(Debug, Clone, PartialEq)]
pub struct Avatar {
    pub src: String,
    pub alt: String,
    pub size: AvatarSize,
    pub shape: AvatarShape,
    pub fallback: String,
    pub class: String,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum AvatarSize {
    Small,
    Medium,
    Large,
    XLarge,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum AvatarShape {
    Circle,
    Square,
    Rounded,
}

impl Avatar {
#[inline]
pub fn new(src: String) -> Avatar {
        Avatar { src, alt: "Avatar".to_string(), size: AvatarSize::Medium, shape: AvatarShape::Circle, fallback: String::new(), class: String::new() }
}
#[inline]
pub fn alt(mut self, alt: String) -> Avatar {
        self.alt = alt;
        self
}
#[inline]
pub fn size(mut self, size: AvatarSize) -> Avatar {
        self.size = size;
        self
}
#[inline]
pub fn shape(mut self, shape: AvatarShape) -> Avatar {
        self.shape = shape;
        self
}
#[inline]
pub fn fallback(mut self, fallback: String) -> Avatar {
        self.fallback = fallback;
        self
}
#[inline]
pub fn class(mut self, class: String) -> Avatar {
        self.class = class;
        self
}
#[inline]
pub fn render(&self) -> String {
        let size_px = match self.size {
            AvatarSize::Small => "32px".to_string(),
            AvatarSize::Medium => "48px".to_string(),
            AvatarSize::Large => "64px".to_string(),
            AvatarSize::XLarge => "96px".to_string(),
        };
        let border_radius = match self.shape {
            AvatarShape::Circle => "50%".to_string(),
            AvatarShape::Square => "0".to_string(),
            AvatarShape::Rounded => "8px".to_string(),
        };
        let mut html = String::new();
        if self.src.is_empty() && !self.fallback.is_empty() {
            html.push_str("<div class=\"wj-avatar wj-avatar-fallback ");
            html.push_str(&self.class.as_str());
            html.push_str("\" style=\"width: ");
            html.push_str(&size_px);
            html.push_str("; height: ");
            html.push_str(&size_px);
            html.push_str("; border-radius: ");
            html.push_str(&border_radius);
            html.push_str("; background-color: #3b82f6; color: white; display: flex; align-items: center; justify-content: center; font-weight: 600; font-size: ");
            let font_size = match self.size {
                AvatarSize::Small => "12px".to_string(),
                AvatarSize::Medium => "16px".to_string(),
                AvatarSize::Large => "20px".to_string(),
                AvatarSize::XLarge => "28px".to_string(),
            };
            html.push_str(&font_size);
            html.push_str(";\">");
            html.push_str(&self.fallback.as_str());
            html.push_str("</div>")
        } else {
            html.push_str("<img class=\"wj-avatar ");
            html.push_str(&self.class.as_str());
            html.push_str("\" src=\"");
            html.push_str(&self.src.as_str());
            html.push_str("\" alt=\"");
            html.push_str(&self.alt.as_str());
            html.push_str("\" style=\"width: ");
            html.push_str(&size_px);
            html.push_str("; height: ");
            html.push_str(&size_px);
            html.push_str("; border-radius: ");
            html.push_str(&border_radius);
            html.push_str("; object-fit: cover;\">")
        }
        html
}
}

