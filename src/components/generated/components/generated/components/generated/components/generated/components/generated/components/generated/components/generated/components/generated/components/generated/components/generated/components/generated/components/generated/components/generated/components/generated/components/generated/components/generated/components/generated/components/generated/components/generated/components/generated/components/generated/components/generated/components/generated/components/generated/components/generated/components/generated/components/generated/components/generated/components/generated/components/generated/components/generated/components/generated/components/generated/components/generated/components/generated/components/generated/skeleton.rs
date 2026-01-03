#[derive(Debug, Clone, PartialEq)]
pub struct Skeleton {
    pub variant: SkeletonVariant,
    pub width: String,
    pub height: String,
    pub class: String,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum SkeletonVariant {
    Text,
    Circle,
    Rectangle,
}

impl Skeleton {
#[inline]
pub fn new() -> Skeleton {
        Skeleton { variant: SkeletonVariant::Text, width: "100%".to_string(), height: "20px".to_string(), class: String::new() }
}
#[inline]
pub fn variant(mut self, variant: SkeletonVariant) -> Skeleton {
        self.variant = variant;
        self
}
#[inline]
pub fn width(mut self, width: String) -> Skeleton {
        self.width = width;
        self
}
#[inline]
pub fn height(mut self, height: String) -> Skeleton {
        self.height = height;
        self
}
#[inline]
pub fn class(mut self, class: String) -> Skeleton {
        self.class = class;
        self
}
#[inline]
pub fn render(&self) -> String {
        let border_radius = match self.variant {
            SkeletonVariant::Text => "4px".to_string(),
            SkeletonVariant::Circle => "50%".to_string(),
            SkeletonVariant::Rectangle => "8px".to_string(),
        };
        let mut html = String::new();
        html.push_str("<div class=\"wj-skeleton ");
        html.push_str(&self.class.as_str());
        html.push_str("\" style=\"width: ");
        html.push_str(&self.width.as_str());
        html.push_str("; height: ");
        html.push_str(&self.height.as_str());
        html.push_str("; border-radius: ");
        html.push_str(&border_radius);
        html.push_str("; background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%); background-size: 200% 100%; animation: skeleton-loading 1.5s ease-in-out infinite;\"></div>");
        html.push_str("<style>@keyframes skeleton-loading { 0% { background-position: 200% 0; } 100% { background-position: -200% 0; } }</style>");
        html
}
}

