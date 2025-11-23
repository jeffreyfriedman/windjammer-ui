//! Form Validation Tests
//!
//! Test form input, validation, and error handling patterns

use std::cell::RefCell;
use std::rc::Rc;
use windjammer_ui::components::*;
use windjammer_ui::reactivity::Signal;
use windjammer_ui::simple_vnode::VNode;
use windjammer_ui::to_vnode::ToVNode;

#[test]
fn test_input_component() {
    let mut input = Input::new();
    input.placeholder = "Enter text".to_string();
    let vnode = input.render();

    assert!(matches!(vnode, VNode::Element { .. }));
}

#[test]
fn test_input_with_value() {
    let mut input = Input::new();
    input.value = "test value".to_string();
    let vnode = input.render();

    match vnode {
        VNode::Element { attrs, .. } => {
            assert!(attrs.iter().any(|(key, _)| key == "value"));
        }
        _ => panic!("Expected element node"),
    }
}

#[test]
fn test_checkbox_component() {
    let checkbox = Checkbox::new("Unchecked").to_vnode();
    assert!(matches!(checkbox, VNode::Element { .. }));

    let checked = Checkbox::new("Checked").checked(true).to_vnode();
    assert!(matches!(checked, VNode::Element { .. }));
}

#[test]
fn test_select_with_options() {
    let select = Select::new()
        .option("opt1", "Option 1")
        .option("opt2", "Option 2")
        .option("opt3", "Option 3");
    let vnode = select.render();

    match vnode {
        VNode::Element { children, .. } => {
            // Should have 3 options
            assert!(children.len() >= 3);
        }
        _ => panic!("Expected element node"),
    }
}

#[test]
fn test_switch_component() {
    let switch_off = Switch::new().to_vnode();
    assert!(matches!(switch_off, VNode::Element { .. }));

    let switch_on = Switch::new().checked(true).to_vnode();
    assert!(matches!(switch_on, VNode::Element { .. }));
}

#[test]
fn test_alert_variants() {
    let success = Alert::success("Success!").render();
    let error = Alert::error("Error!").render();
    let warning = Alert::warning("Warning!").render();
    let info = Alert::info("Info").render();

    assert!(matches!(success, VNode::Element { .. }));
    assert!(matches!(error, VNode::Element { .. }));
    assert!(matches!(warning, VNode::Element { .. }));
    assert!(matches!(info, VNode::Element { .. }));
}

#[test]
fn test_form_layout_structure() {
    // Test that form components can be composed
    let mut name_input = Input::new();
    name_input.placeholder = "Name".to_string();

    let mut email_input = Input::new();
    email_input.placeholder = "Email".to_string();

    let form = Flex::new()
        .direction(FlexDirection::Column)
        .children(vec![
            Text::new("Contact Form").render(),
            name_input.render(),
            email_input.render(),
            Button::new("Submit")
                .variant(ButtonVariant::Primary)
                .render(),
        ])
        .render();

    match form {
        VNode::Element { children, .. } => {
            assert_eq!(children.len(), 4);
        }
        _ => panic!("Expected element node"),
    }
}

#[test]
fn test_validation_error_display() {
    // Test error message display pattern
    let error_message = Alert::error("Email is required").render();

    match error_message {
        VNode::Element { children, .. } => {
            assert!(!children.is_empty());
        }
        _ => panic!("Expected element node"),
    }
}

#[test]
fn test_required_field_pattern() {
    // Test the pattern of marking required fields
    let label_with_asterisk = Text::new("Email *").render();

    // Text component creates an Element, not a Text VNode
    assert!(matches!(label_with_asterisk, VNode::Element { .. }));
}

#[test]
fn test_form_with_multiple_input_types() {
    // Test a form with various input types
    let mut text_input = Input::new();
    text_input.placeholder = "Text input".to_string();

    let components = vec![
        text_input.render(),
        Checkbox::new("Option").to_vnode(),
        Select::new().render(),
        Switch::new().to_vnode(),
        Slider::new().to_vnode(),
    ];

    assert_eq!(components.len(), 5);
    for component in components {
        assert!(matches!(component, VNode::Element { .. }));
    }
}
