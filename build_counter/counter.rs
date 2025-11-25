use windjammer_ui::prelude::*;
use windjammer_ui::components::*;
use windjammer_ui::simple_vnode::{VNode, VAttr};



#[inline]
pub fn start() {
    println!("🎉 Starting Windjammer Counter");
    App::new_reactive("Counter Example".to_string(), move || {
        let count = Signal::new(0);
        let count_inc = count.clone();
        let count_display = count.clone();
        let ui = Container::new().max_width("400px").child((Flex::new().direction(FlexDirection::Column).gap("20px").child((Text::new("🎉 Windjammer Counter".to_string()).size(TextSize::Large).bold()).to_vnode()).child((Text::new(format!("Count: {}", count_display.get())).size(TextSize::XLarge).bold()).to_vnode()).child((Button::new("Increment".to_string()).variant(ButtonVariant::Primary).on_click(move || {
        let current = count_inc.get();
        count_inc.set(current + 1);
        println!("Count: {}", current + 1)
        })).to_vnode())).to_vnode());
        ui.to_vnode()
    }).run()
}

fn main() {
    start()
}

