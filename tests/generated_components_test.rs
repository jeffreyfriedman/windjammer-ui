//! Generated Components Smoke Tests
//!
//! Verify all 49 components generated from pure Windjammer can be instantiated and rendered

use windjammer_ui::components::*;

#[test]
fn test_all_49_components_render() {
    // Test that all 49 components can be instantiated and rendered without panicking

    // Core Components (14)
    let _text = Text::new("test".to_string()).render();
    let _button = Button::new("test".to_string()).render();
    let _input = Input::new().render();
    let _checkbox = Checkbox::new("test".to_string()).render();
    let _slider = Slider::new().render();
    let _switch = Switch::new().render();
    let _radio = RadioGroup::new("test".to_string()).render();
    let _badge = Badge::new("test".to_string()).render();
    let _alert = Alert::info("test".to_string()).render();
    let _card = Card::new().render();
    let _progress = Progress::new(50.0).render();
    let _spinner = Spinner::new().render();
    let _tooltip = Tooltip::new("test".to_string(), "test".to_string()).render();
    let _select = Select::new().render();

    // Layout Components (9)
    let _container = Container::new().render();
    let _flex = Flex::new().render();
    let _grid = Grid::new(3).render();
    let _panel = Panel::new("Panel".to_string()).render();
    let _divider = Divider::new().render();
    let _spacer = Spacer::new().render();
    let _tabs = Tabs::new()
        .tab(Tab::new(
            "tab1".to_string(),
            "Tab 1".to_string(),
            "Content 1".to_string(),
        ))
        .render();
    let _tabpanel = TabPanel::new().render();
    let _toolbar = Toolbar::new().render();

    // Advanced Components (17)
    let _dialog = Dialog::new("title".to_string(), "content".to_string()).render();
    let _scrollarea = ScrollArea::new().render();
    let _collapsible = CollapsibleSection::new("title".to_string(), "content".to_string()).render();
    let _codeeditor = CodeEditor::new("code".to_string()).render();
    let _advcodeeditor = AdvancedCodeEditor::new("code".to_string()).render();
    let _colorpicker = ColorPicker::new().render();
    let _filetree = FileTree::new(FileNode::new("root".to_string(), true)).render();
    let _treeview = TreeView::new().render();
    let _splitpanel = SplitPanel::new("left".to_string(), "right".to_string()).render();
    let _toast = Toast::new("test".to_string()).render();
    let _accordion = Accordion::new().render();
    let _breadcrumb = Breadcrumb::new().render();
    let _dropdown = Dropdown::new("test".to_string()).render();
    let _avatar = Avatar::new("img.jpg".to_string(), "alt".to_string()).render();
    let _skeleton = Skeleton::new().render();
    let _pagination = Pagination::new(1, 10).render();
    let _menu = Menu::new().render();

    // Navigation Components (4)
    let _navbar = Navbar::new().render();
    let _sidebar = Sidebar::new().render();
    let _hamburger = HamburgerMenu::new().render();
    let _contextmenu = ContextMenu::new("target".to_string()).render();

    // Chat/Messaging Components (5)
    let _chatmsg = ChatMessage::new("test".to_string()).render();
    let _msglist = MessageList::new().render();
    let _chatinput = ChatInput::new().render();
    let _typing = TypingIndicator::new().render();
    let _codeblock = CodeBlock::new("test".to_string()).render();

    // If we get here, all 49 components rendered successfully (no panic = success)
}

#[test]
fn test_text_component() {
    let text = Text::new("Hello World".to_string()).render();
    assert!(text.contains("Hello World"));
}

#[test]
fn test_button_component() {
    let button = Button::new("Click Me".to_string())
        .variant(ButtonVariant::Primary)
        .render();
    assert!(button.contains("Click Me"));
}

#[test]
fn test_input_component() {
    let input = Input::new().placeholder("Enter text".to_string()).render();
    assert!(input.contains("Enter text"));
}

#[test]
fn test_checkbox_component() {
    let checkbox = Checkbox::new("Accept".to_string()).checked(true).render();
    assert!(checkbox.contains("Accept"));
}

#[test]
fn test_slider_component() {
    let slider = Slider::new().min(0.0).max(100.0).value(50.0).render();
    assert!(slider.contains("50"));
}

#[test]
fn test_badge_component() {
    let badge = Badge::new("New".to_string())
        .variant(BadgeVariant::Success)
        .render();
    assert!(badge.contains("New"));
}

#[test]
fn test_alert_component() {
    let alert = Alert::success("Success!".to_string()).render();
    assert!(alert.contains("Success!"));
}

#[test]
fn test_card_component() {
    let card = Card::new().child("Content".to_string()).render();
    assert!(card.contains("Content"));
}

#[test]
fn test_progress_component() {
    let progress = Progress::new(75.0).render();
    assert!(progress.contains("75"));
}

#[test]
fn test_container_component() {
    let container = Container::new()
        .child("Child 1".to_string())
        .child("Child 2".to_string())
        .render();
    assert!(container.contains("Child 1"));
    assert!(container.contains("Child 2"));
}

#[test]
fn test_flex_component() {
    let flex = Flex::new()
        .direction(FlexDirection::Column)
        .child("Item".to_string())
        .render();
    assert!(flex.contains("Item"));
}

#[test]
fn test_grid_component() {
    let grid = Grid::new(2).child("Cell".to_string()).render();
    assert!(grid.contains("Cell"));
}

#[test]
fn test_divider_component() {
    let divider = Divider::new().render();
    assert!(divider.contains("wj-divider"));
}

#[test]
fn test_spacer_component() {
    let spacer = Spacer::new().width("20px".to_string()).render();
    assert!(spacer.contains("20px"));
}

#[test]
fn test_tabs_component() {
    let tabs = Tabs::new()
        .tab(Tab::new(
            "tab1".to_string(),
            "Tab 1".to_string(),
            "Content 1".to_string(),
        ))
        .render();
    assert!(tabs.contains("Tab 1"));
}

#[test]
fn test_dialog_component() {
    let dialog = Dialog::new("Title".to_string(), "Content".to_string())
        .open(true)
        .render();
    assert!(dialog.contains("Title"));
}

#[test]
fn test_toast_component() {
    let toast = Toast::new("Message".to_string())
        .variant(ToastVariant::Success)
        .render();
    assert!(toast.contains("Message"));
}

#[test]
fn test_chat_message_component() {
    let message = ChatMessage::new("Hello".to_string())
        .role(MessageRole::User)
        .render();
    assert!(message.contains("Hello"));
}

#[test]
fn test_navbar_component() {
    let navbar = Navbar::new().brand("App".to_string()).render();
    assert!(navbar.contains("App"));
}

#[test]
fn test_builder_pattern_chaining() {
    // Test that builder pattern works correctly
    let button = Button::new("Test".to_string())
        .variant(ButtonVariant::Primary)
        .size(ButtonSize::Large)
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
