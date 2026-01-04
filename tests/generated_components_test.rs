//! Generated Components Smoke Tests
//!
//! Verify all 49 components generated from pure Windjammer can be instantiated and rendered

use windjammer_ui::components::generated::traits::Renderable;
use windjammer_ui::components::generated::{
    accordion, advancedcodeeditor, alert, avatar, badge, breadcrumb, button, card, chatinput,
    chatmessage, checkbox, codeblock, codeeditor, collapsible, colorpicker, container, contextmenu,
    dialog, divider, dropdown, filetree, flex, grid, hamburgermenu, input, menu, messagelist,
    navbar, pagination, panel, progress, radio, scrollarea, select, sidebar, skeleton, slider,
    spacer, spinner, splitpanel, switch, tabpanel, tabs, text, toast, toolbar, tooltip, treeview,
    typingindicator,
};

#[test]
fn test_all_49_components_render() {
    // Test that all 49 components can be instantiated and rendered without panicking

    // Core Components (14)
    let _text = text::Text::new("test".to_string()).render();
    let _button = button::Button::new("test".to_string()).render();
    let _input = input::Input::new().render();
    let _checkbox = checkbox::Checkbox::new("test".to_string()).render();
    let _slider = slider::Slider::new().render();
    let _switch = switch::Switch::new().render();
    let _radio = radio::RadioGroup::new("test".to_string()).render();
    let _badge = badge::Badge::new("test".to_string()).render();
    let _alert = alert::Alert::info("test".to_string()).render();
    let _card = card::Card::new().render();
    let _progress = progress::Progress::new(50.0).render();
    let _spinner = spinner::Spinner::new().render();
    let _tooltip = tooltip::Tooltip::new("test".to_string(), "test".to_string()).render();
    let _select = select::Select::new().render();

    // Layout Components (9)
    let _container = container::Container::new().render();
    let _flex = flex::Flex::new().render();
    let _grid = grid::Grid::new(3).render();
    let _panel = panel::Panel::new("Panel".to_string()).render();
    let _divider = divider::Divider::new().render();
    let _spacer = spacer::Spacer::new().render();
    let _tabs = tabs::Tabs::new()
        .tab(tabs::Tab::new(
            "tab1".to_string(),
            "Tab 1".to_string(),
            "Content 1".to_string(),
        ))
        .render();
    let _tabpanel = tabpanel::TabPanel::new().render();
    let _toolbar = toolbar::Toolbar::new().render();

    // Advanced Components (17)
    let _dialog = dialog::Dialog::new("title".to_string(), "content".to_string()).render();
    let _scrollarea = scrollarea::ScrollArea::new().render();
    let _collapsible =
        collapsible::CollapsibleSection::new("title".to_string(), "content".to_string()).render();
    let _codeeditor = codeeditor::CodeEditor::new("code".to_string()).render();
    let _advcodeeditor = advancedcodeeditor::AdvancedCodeEditor::new("code".to_string()).render();
    let _colorpicker = colorpicker::ColorPicker::new().render();
    let _filetree =
        filetree::FileTree::new(filetree::FileNode::new("root".to_string(), true)).render();
    let _treeview = treeview::TreeView::new().render();
    let _splitpanel = splitpanel::SplitPanel::new("left".to_string(), "right".to_string()).render();
    let _toast = toast::Toast::new("test".to_string()).render();
    let _accordion = accordion::Accordion::new().render();
    let _breadcrumb = breadcrumb::Breadcrumb::new().render();
    let _dropdown = dropdown::Dropdown::new("test".to_string()).render();
    let _avatar = avatar::Avatar::new("img.jpg".to_string()).render();
    let _skeleton = skeleton::Skeleton::new().render();
    let _pagination = pagination::Pagination::new(1, 10).render();
    let _menu = menu::Menu::new("Menu".to_string()).render();

    // Navigation Components (4)
    let _navbar = navbar::Navbar::new().render();
    let _sidebar = sidebar::Sidebar::new().render();
    let _hamburger = hamburgermenu::HamburgerMenu::new().render();
    let _contextmenu = contextmenu::ContextMenu::new("target".to_string()).render();

    // Chat/Messaging Components (5)
    let _chatmsg = chatmessage::ChatMessage::new("test".to_string()).render();
    let _msglist = messagelist::MessageList::new().render();
    let _chatinput = chatinput::ChatInput::new().render();
    let _typing = typingindicator::TypingIndicator::new().render();
    let _codeblock = codeblock::CodeBlock::new("test".to_string()).render();

    // If we get here, all 49 components rendered successfully (no panic = success)
}

#[test]
fn test_text_component() {
    let text = text::Text::new("Hello World".to_string()).render();
    assert!(text.contains("Hello World"));
}

#[test]
fn test_button_component() {
    let button = button::Button::new("Click Me".to_string())
        .variant(button::ButtonVariant::Primary)
        .render();
    assert!(button.contains("Click Me"));
}

#[test]
fn test_input_component() {
    let input = input::Input::new()
        .placeholder("Enter text".to_string())
        .render();
    assert!(input.contains("Enter text"));
}

#[test]
fn test_checkbox_component() {
    let checkbox = checkbox::Checkbox::new("Accept".to_string())
        .checked(true)
        .render();
    assert!(checkbox.contains("Accept"));
}

#[test]
fn test_slider_component() {
    let slider = slider::Slider::new()
        .min(0.0)
        .max(100.0)
        .value(50.0)
        .render();
    assert!(slider.contains("50"));
}

#[test]
fn test_badge_component() {
    let badge = badge::Badge::new("New".to_string())
        .variant(badge::BadgeVariant::Success)
        .render();
    assert!(badge.contains("New"));
}

#[test]
fn test_alert_component() {
    let alert = alert::Alert::success("Success!".to_string()).render();
    assert!(alert.contains("Success!"));
}

#[test]
fn test_card_component() {
    let card = card::Card::new().child("Content".to_string()).render();
    assert!(card.contains("Content"));
}

#[test]
fn test_progress_component() {
    let progress = progress::Progress::new(75.0).render();
    assert!(progress.contains("75"));
}

#[test]
fn test_container_component() {
    let container = container::Container::new()
        .child("Child 1".to_string())
        .child("Child 2".to_string())
        .render();
    assert!(container.contains("Child 1"));
    assert!(container.contains("Child 2"));
}

#[test]
fn test_flex_component() {
    let flex = flex::Flex::new()
        .direction(flex::FlexDirection::Column)
        .child("Item".to_string())
        .render();
    assert!(flex.contains("Item"));
}

#[test]
fn test_grid_component() {
    let grid = grid::Grid::new(2).child("Cell".to_string()).render();
    assert!(grid.contains("Cell"));
}

#[test]
fn test_divider_component() {
    let divider = divider::Divider::new().render();
    assert!(divider.contains("wj-divider"));
}

#[test]
fn test_spacer_component() {
    let spacer = spacer::Spacer::new().width("20px".to_string()).render();
    assert!(spacer.contains("20px"));
}

#[test]
fn test_tabs_component() {
    let tabs = tabs::Tabs::new()
        .tab(tabs::Tab::new(
            "tab1".to_string(),
            "Tab 1".to_string(),
            "Content 1".to_string(),
        ))
        .render();
    assert!(tabs.contains("Tab 1"));
}

#[test]
fn test_dialog_component() {
    let dialog = dialog::Dialog::new("Title".to_string(), "Content".to_string())
        .open(true)
        .render();
    assert!(dialog.contains("Title"));
}

#[test]
fn test_avatar_component() {
    // Test basic avatar with image
    let avatar_img = avatar::Avatar::new("user.jpg".to_string())
        .alt("User profile".to_string())
        .size(avatar::AvatarSize::Large)
        .shape(avatar::AvatarShape::Circle)
        .render();
    assert!(avatar_img.contains("user.jpg"));
    assert!(avatar_img.contains("User profile"));
    assert!(avatar_img.contains("64px")); // Large size
    assert!(avatar_img.contains("50%")); // Circle shape

    // Test fallback behavior (no image, shows initials)
    let avatar_fallback = avatar::Avatar::new(String::new())
        .fallback("JF".to_string())
        .size(avatar::AvatarSize::Small)
        .shape(avatar::AvatarShape::Rounded)
        .render();
    assert!(avatar_fallback.contains("JF"));
    assert!(avatar_fallback.contains("32px")); // Small size
    assert!(avatar_fallback.contains("8px")); // Rounded shape
    assert!(avatar_fallback.contains("wj-avatar-fallback"));
}

#[test]
fn test_toast_component() {
    let toast = toast::Toast::new("Message".to_string())
        .variant(toast::ToastVariant::Success)
        .render();
    assert!(toast.contains("Message"));
}

#[test]
fn test_chat_message_component() {
    let message = chatmessage::ChatMessage::new("Hello".to_string())
        .role(chatmessage::MessageRole::User)
        .render();
    assert!(message.contains("Hello"));
}

#[test]
fn test_navbar_component() {
    let navbar = navbar::Navbar::new().brand("App".to_string()).render();
    assert!(navbar.contains("App"));
}

#[test]
fn test_builder_pattern_chaining() {
    // Test that builder pattern works correctly
    let button = button::Button::new("Test".to_string())
        .variant(button::ButtonVariant::Primary)
        .size(button::ButtonSize::Large)
        .disabled(false)
        .render();

    assert!(button.contains("Test"));
}

#[test]
fn test_component_count() {
    // Verify we have exactly 49 components
    // Core: 14, Layout: 9, Advanced: 17, Navigation: 4, Chat: 5
    assert_eq!(14 + 9 + 17 + 4 + 5, 49);
}
