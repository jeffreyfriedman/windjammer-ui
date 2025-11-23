pub struct Input {
    value: String,
    placeholder: String,
}

impl Input {
    #[inline]
    pub fn new() -> Input {
        Input {
            value: "".to_string(),
            placeholder: "".to_string(),
        }
    }
    #[inline]
    pub fn value(mut self, value: String) -> Input {
        self.value = value;
        self
    }
    #[inline]
    pub fn placeholder(mut self, placeholder: String) -> Input {
        self.placeholder = placeholder;
        self
    }
    #[inline]
    pub fn render(mut self) -> String {
        format!(
            "<input class='wj-input' value='{}' placeholder='{}'/>",
            self.value, self.placeholder
        )
    }
}
