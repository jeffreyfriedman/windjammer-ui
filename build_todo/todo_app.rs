


use windjammer_ui::prelude::*;
use windjammer_ui::components::*;
use windjammer_ui::simple_vnode::{VNode, VAttr};


use std::collections::*;


#[derive(Clone)]
struct TodoItem {
    id: i64,
    text: String,
    completed: bool,
}

pub fn start() {
    println!("📝 Starting Todo App");
    let todos = Signal::new(vec![TodoItem { id: 1, text: "Learn Windjammer".to_string(), completed: false }, TodoItem { id: 2, text: "Build UI framework".to_string(), completed: true }, TodoItem { id: 3, text: "Ship v0.1.0".to_string(), completed: false }]);
    let next_id = Signal::new(4);
    let input_text = Signal::new("".to_string());
    let filter = Signal::new("all".to_string());
    let todos_add = todos.clone();
    let todos_toggle = todos.clone();
    let todos_delete = todos.clone();
    let todos_display = todos.clone();
    let next_id_add = next_id.clone();
    let input_add = input_text.clone();
    let input_display = input_text.clone();
    let filter_display = filter.clone();
    let ui = Container::new().max_width("600px").child((Flex::new().direction(FlexDirection::Column).gap("16px").child((Text::new("📝 Todo App".to_string()).size(TextSize::XLarge).bold()).to_vnode()).child((Flex::new().direction(FlexDirection::Row).gap("8px").child((Input::new().placeholder("What needs to be done?").value(input_display.get())).to_vnode()).child((Button::new("Add".to_string()).variant(ButtonVariant::Primary).on_click(move || {
        let text = input_add.get();
        if !text.is_empty() {
            let mut current_todos = todos_add.get();
            let id = next_id_add.get();
            current_todos.push(TodoItem { id, text: text.clone(), completed: false });
            todos_add.set(current_todos);
            next_id_add.set(id + 1);
            input_add.set("".to_string());
            println!("Added todo: {}", text)
        }
    })).to_vnode())).to_vnode()).child((Text::new(format!("{} items", todos_display.get().len())).size(TextSize::Small)).to_vnode())).to_vnode());
    App::new("Todo App".to_string(), ui.to_vnode()).run()
}

fn main() {
    start()
}

