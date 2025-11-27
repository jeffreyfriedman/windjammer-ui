use windjammer_ui::*::*;


#[inline]
pub fn start() {
    println!("🎉 Starting Windjammer Counter");
    let count = Signal::new(0);
    let count_inc = count.clone();
    let count_display = count.clone();
    let ui = Container::new().max_width("400px").child(Flex::new().direction(FlexDirection::Column).gap("20px").child(Text::new("🎉 Windjammer Counter".to_string()).size(TextSize::Large).bold()).child(Text::new(format!("Count: {}", count_display.get())).size(TextSize::XLarge).bold()).child(Button::new("Increment".to_string()).variant(ButtonVariant::Primary).on_click(move || {
        let current = count_inc.get();
        count_inc.set(current + 1);
        println!("Count: {}", current + 1)
    })));
    App::new("Counter Example".to_string(), ui.to_vnode()).run()
}

fn main() {
    start()
}

