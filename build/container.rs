struct Container {
    children: Vec<String>,
    max_width: String,
    max_height: String,
    padding: String,
    background_color: String,
}

impl Container {
#[inline]
fn new() -> Container {
        Container { children: Vec::new(), max_width: "".to_string(), max_height: "".to_string(), padding: "16px".to_string(), background_color: "".to_string() }
}
#[inline]
fn child(mut self, child: String) -> Container {
        self.children.push(child);
        self
}
#[inline]
fn children(mut self, children: Vec<String>) -> Container {
        self.children = children;
        self
}
#[inline]
fn max_width(mut self, width: String) -> Container {
        self.max_width = width;
        self
}
#[inline]
fn max_height(mut self, height: String) -> Container {
        self.max_height = height;
        self
}
#[inline]
fn padding(mut self, padding: String) -> Container {
        self.padding = padding;
        self
}
#[inline]
fn background_color(mut self, color: String) -> Container {
        self.background_color = color;
        self
}
fn render(&self) -> String {
        let mut style = "margin: 0 auto; ".to_string();
        if self.max_width != "" {
            style = format!("{}{}{}{}", style, "max-width: ", self.max_width, "; ");
        }
        if self.max_height != "" {
            style = format!("{}{}{}{}", style, "max-height: ", self.max_height, "; ");
        }
        if self.padding != "" {
            style = format!("{}{}{}{}", style, "padding: ", self.padding, "; ");
        }
        if self.background_color != "" {
            style = format!("{}{}{}{}", style, "background-color: ", self.background_color, "; ");
        }
        let children_html = self.children.join("
  ");
        format!("<div class='wj-container' style='{}'>
  {}
</div>", style, children_html)
}
}

fn main() {
    let container1 = Container::new().max_width("800px".to_string()).padding("24px".to_string()).child(("<p>First child</p>".to_string()).to_vnode()).child(("<p>Second child</p>".to_string()).to_vnode());
    let container2 = Container::new().max_width("1200px".to_string()).max_height("600px".to_string()).background_color("#f0f0f0".to_string()).children(vec!["<h1>Title</h1>".to_string(), "<p>Content</p>".to_string()]);
    println!("Container 1:
{}", container1.render());
    println!("
Container 2:
{}", container2.render())
}

