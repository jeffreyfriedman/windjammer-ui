#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

pub struct Spacer {
    width: String,
    height: String,
    flex: bool,
}

impl Spacer {
    #[inline]
    pub fn new() -> Spacer {
        Spacer {
            width: "".to_string(),
            height: "".to_string(),
            flex: false,
        }
    }
    #[inline]
    pub fn horizontal(width: String) -> Spacer {
        Spacer {
            width,
            height: "".to_string(),
            flex: false,
        }
    }
    #[inline]
    pub fn vertical(height: String) -> Spacer {
        Spacer {
            width: "".to_string(),
            height,
            flex: false,
        }
    }
    #[inline]
    pub fn flexible() -> Spacer {
        Spacer {
            width: "".to_string(),
            height: "".to_string(),
            flex: true,
        }
    }
    #[inline]
    pub fn width(mut self, width: String) -> Spacer {
        self.width = width;
        self
    }
    #[inline]
    pub fn height(mut self, height: String) -> Spacer {
        self.height = height;
        self
    }
    #[inline]
    pub fn xxs() -> Spacer {
        Spacer::vertical("4px".to_string())
    }
    #[inline]
    pub fn xs() -> Spacer {
        Spacer::vertical("8px".to_string())
    }
    #[inline]
    pub fn sm() -> Spacer {
        Spacer::vertical("12px".to_string())
    }
    #[inline]
    pub fn md() -> Spacer {
        Spacer::vertical("16px".to_string())
    }
    #[inline]
    pub fn lg() -> Spacer {
        Spacer::vertical("24px".to_string())
    }
    #[inline]
    pub fn xl() -> Spacer {
        Spacer::vertical("32px".to_string())
    }
    #[inline]
    pub fn xxl() -> Spacer {
        Spacer::vertical("48px".to_string())
    }
}

impl Renderable for Spacer {
    fn render(self) -> String {
        let mut style = "".to_string();
        if self.flex {
            style = "flex: 1; ".to_string();
        }
        if self.width != "" {
            style = format!("{}width: {}; ", style, self.width);
        }
        if self.height != "" {
            style = format!("{}height: {}; ", style, self.height);
        }
        format!("<div class='wj-spacer' style='{}'></div>", style)
    }
}
