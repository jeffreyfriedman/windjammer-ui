use windjammer_ui::*::*;


pub fn start() {
    println!("📬 Starting Contact Form");
    let name = Signal::new("".to_string());
    let email = Signal::new("".to_string());
    let message = Signal::new("".to_string());
    let name_error = Signal::new("".to_string());
    let email_error = Signal::new("".to_string());
    let message_error = Signal::new("".to_string());
    let is_submitting = Signal::new(false);
    let submit_success = Signal::new(false);
    let name_display = name.clone();
    let email_display = email.clone();
    let message_display = message.clone();
    let name_submit = name.clone();
    let email_submit = email.clone();
    let message_submit = message.clone();
    let name_error_display = name_error.clone();
    let email_error_display = email_error.clone();
    let message_error_display = message_error.clone();
    let is_submitting_display = is_submitting.clone();
    let submit_success_display = submit_success.clone();
    let ui = Container::new().max_width("600px").child(Flex::new().direction(FlexDirection::Column).gap("20px").child(Text::new("📬 Contact Us".to_string()).size(TextSize::XLarge).bold()).child(Text::new("We'd love to hear from you".to_string()).size(TextSize::Medium)).child(Flex::new().direction(FlexDirection::Column).gap("16px").child(Flex::new().direction(FlexDirection::Column).gap("4px").child(Text::new("Name *".to_string()).size(TextSize::Small)).child(Input::new().placeholder("Your name").value(name_display.get()))).child(Flex::new().direction(FlexDirection::Column).gap("4px").child(Text::new("Email *".to_string()).size(TextSize::Small)).child(Input::new().placeholder("your.email@example.com").value(email_display.get()))).child(Flex::new().direction(FlexDirection::Column).gap("4px").child(Text::new("Message *".to_string()).size(TextSize::Small)).child(Input::new().placeholder("Your message...").value(message_display.get()))).child(Button::new("Send Message".to_string()).variant(ButtonVariant::Primary).on_click(move || {
        let has_name = !name_submit.get().is_empty();
        let has_email = email_submit.get().contains("@");
        let has_message = !message_submit.get().is_empty();
        if has_name && has_email && has_message {
            is_submitting.set(true);
            println!("Form submitted successfully!");
            submit_success.set(true);
            name_submit.set("".to_string());
            email_submit.set("".to_string());
            message_submit.set("".to_string())
        } else {
            println!("Validation failed")
        }
    }))));
    App::new("Contact Form".to_string(), ui.to_vnode()).run()
}

fn main() {
    start()
}

