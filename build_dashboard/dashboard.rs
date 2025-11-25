use windjammer_ui::prelude::*;
use windjammer_ui::components::*;
use windjammer_ui::simple_vnode::{VNode, VAttr};



pub fn start() {
    println!("📊 Starting Dashboard");
    let total_users = Signal::new(1247);
    let active_users = Signal::new(892);
    let revenue = Signal::new(45678);
    let growth = Signal::new(12);
    let total_display = total_users.clone();
    let active_display = active_users.clone();
    let revenue_display = revenue.clone();
    let growth_display = growth.clone();
    let ui = Container::new().max_width("1200px").child((Flex::new().direction(FlexDirection::Column).gap("24px").child((Flex::new().direction(FlexDirection::Row).gap("16px").child((Text::new("📊 Dashboard".to_string()).size(TextSize::XLarge).bold()).to_vnode())).to_vnode()).child((Flex::new().direction(FlexDirection::Row).gap("16px").child((Card::new().child((Flex::new().direction(FlexDirection::Column).gap("8px").child((Text::new("👥 Total Users".to_string()).size(TextSize::Small)).to_vnode()).child((Text::new(format!("{}", total_display.get())).size(TextSize::XLarge).bold()).to_vnode())).to_vnode())).to_vnode()).child((Card::new().child((Flex::new().direction(FlexDirection::Column).gap("8px").child((Text::new("✓ Active Users".to_string()).size(TextSize::Small)).to_vnode()).child((Text::new(format!("{}", active_display.get())).size(TextSize::XLarge).bold()).to_vnode())).to_vnode())).to_vnode()).child((Card::new().child((Flex::new().direction(FlexDirection::Column).gap("8px").child((Text::new("💰 Revenue".to_string()).size(TextSize::Small)).to_vnode()).child((Text::new(format!(format!("{}"), revenue_display.get())).size(TextSize::XLarge).bold()).to_vnode())).to_vnode())).to_vnode()).child((Card::new().child((Flex::new().direction(FlexDirection::Column).gap("8px").child((Text::new("📈 Growth".to_string()).size(TextSize::Small)).to_vnode()).child((Text::new(format!("{}%", growth_display.get())).size(TextSize::XLarge).bold()).to_vnode())).to_vnode())).to_vnode())).to_vnode()).child((Text::new("Welcome back! Here's your overview.".to_string()).size(TextSize::Medium)).to_vnode())).to_vnode());
    App::new("Dashboard".to_string(), ui.to_vnode()).run()
}

fn main() {
    start()
}

