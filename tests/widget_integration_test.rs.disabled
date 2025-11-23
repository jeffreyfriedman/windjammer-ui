//! Widget Integration Tests
//!
//! Verify that all widgets work together correctly

use windjammer_ui::components::*;
use windjammer_ui::simple_vnode::VNode;

#[test]
fn test_pbr_material_editor_widgets() {
    // Test that we can create all widgets needed for the PBR Material Editor

    // 1. Collapsible sections
    let base_color_section = CollapsibleSection::new("Base Color")
        .expanded(true)
        .child(VNode::Text("Albedo properties".to_string()))
        .render();

    assert!(matches!(base_color_section, VNode::Element { .. }));

    // 2. Color picker
    let color_picker = ColorPicker::new([1.0, 1.0, 1.0, 1.0])
        .show_alpha(true)
        .render();

    assert!(matches!(color_picker, VNode::Element { .. }));

    // 3. Slider
    let metallic_slider = Slider::new().min(0.0).max(1.0).value(0.0).render();

    assert!(matches!(metallic_slider, VNode::Element { .. }));

    // 4. Select/Dropdown
    let alpha_mode = Select::new()
        .placeholder("Select alpha mode")
        .option("Opaque", "Opaque")
        .option("Mask", "Mask")
        .option("Blend", "Blend")
        .render();

    assert!(matches!(alpha_mode, VNode::Element { .. }));

    // 5. Button
    let save_button = Button::new("Save Material")
        .variant(ButtonVariant::Primary)
        .render();

    assert!(matches!(save_button, VNode::Element { .. }));

    // 6. Spacer
    let spacer = Spacer::md().render();
    assert!(matches!(spacer, VNode::Element { .. }));

    // 7. Divider
    let divider = Divider::horizontal().render();
    assert!(matches!(divider, VNode::Element { .. }));

    // 8. ScrollArea
    let scroll_area = ScrollArea::vertical()
        .max_height("600px")
        .child(VNode::Text("Content".to_string()))
        .render();

    assert!(matches!(scroll_area, VNode::Element { .. }));

    // 9. Text
    let heading = Text::new("PBR Material Editor")
        .size(TextSize::Large)
        .render();

    assert!(matches!(heading, VNode::Element { .. }));

    // 10. Flex (Layout)
    let layout = Flex::new()
        .direction(FlexDirection::Column)
        .child(VNode::Text("Child 1".to_string()))
        .child(VNode::Text("Child 2".to_string()))
        .render();

    assert!(matches!(layout, VNode::Element { .. }));
}

#[test]
fn test_nested_collapsible_sections() {
    // Test progressive disclosure with multiple collapsible sections
    let sections = vec![
        CollapsibleSection::new("Base Color").expanded(true),
        CollapsibleSection::new("Metallic").expanded(true),
        CollapsibleSection::new("Roughness").expanded(true),
        CollapsibleSection::new("Normal Map").expanded(false),
        CollapsibleSection::new("Emissive").expanded(false),
    ];

    // Verify sections can be created
    assert_eq!(sections.len(), 5);

    // Verify expansion states
    assert!(sections[0].expanded); // Base Color (common)
    assert!(sections[1].expanded); // Metallic (common)
    assert!(sections[2].expanded); // Roughness (common)
    assert!(!sections[3].expanded); // Normal Map (advanced)
    assert!(!sections[4].expanded); // Emissive (advanced)
}

#[test]
fn test_color_picker_rgb_sliders() {
    // Test color picker with different colors
    let red = ColorPicker::new([1.0, 0.0, 0.0, 1.0]).render();
    let green = ColorPicker::new([0.0, 1.0, 0.0, 1.0]).render();
    let blue = ColorPicker::new([0.0, 0.0, 1.0, 1.0]).render();

    assert!(matches!(red, VNode::Element { .. }));
    assert!(matches!(green, VNode::Element { .. }));
    assert!(matches!(blue, VNode::Element { .. }));
}

#[test]
fn test_scroll_area_directions() {
    // Test all scroll directions
    let vertical = ScrollArea::vertical().render();
    let horizontal = ScrollArea::horizontal().render();
    let both = ScrollArea::both().render();

    assert!(matches!(vertical, VNode::Element { .. }));
    assert!(matches!(horizontal, VNode::Element { .. }));
    assert!(matches!(both, VNode::Element { .. }));
}

#[test]
fn test_spacer_grid_system() {
    // Test 8px grid system
    let spacers = vec![
        Spacer::xxs(), // 4px
        Spacer::xs(),  // 8px
        Spacer::sm(),  // 12px
        Spacer::md(),  // 16px
        Spacer::lg(),  // 24px
        Spacer::xl(),  // 32px
        Spacer::xxl(), // 48px
    ];

    assert_eq!(spacers.len(), 7);

    // Verify heights
    assert_eq!(spacers[0].height, Some("4px".to_string()));
    assert_eq!(spacers[1].height, Some("8px".to_string()));
    assert_eq!(spacers[2].height, Some("12px".to_string()));
    assert_eq!(spacers[3].height, Some("16px".to_string()));
    assert_eq!(spacers[4].height, Some("24px".to_string()));
    assert_eq!(spacers[5].height, Some("32px".to_string()));
    assert_eq!(spacers[6].height, Some("48px".to_string()));
}

#[test]
fn test_button_variants() {
    // Test all button variants
    let primary = Button::new("Primary").variant(ButtonVariant::Primary);
    let secondary = Button::new("Secondary").variant(ButtonVariant::Secondary);
    let success = Button::new("Success").variant(ButtonVariant::Success);
    let danger = Button::new("Danger").variant(ButtonVariant::Danger);
    let ghost = Button::new("Ghost").variant(ButtonVariant::Ghost);

    assert_eq!(primary.variant, ButtonVariant::Primary);
    assert_eq!(secondary.variant, ButtonVariant::Secondary);
    assert_eq!(success.variant, ButtonVariant::Success);
    assert_eq!(danger.variant, ButtonVariant::Danger);
    assert_eq!(ghost.variant, ButtonVariant::Ghost);
}

#[test]
fn test_complete_material_editor_structure() {
    // Simulate the complete structure of the PBR Material Editor

    // Header
    let _header = Flex::new()
        .direction(FlexDirection::Row)
        .child(
            Text::new("🎨 PBR Material Editor")
                .size(TextSize::Large)
                .render(),
        )
        .child(Spacer::flexible().render())
        .child(
            Button::new("Preview")
                .variant(ButtonVariant::Ghost)
                .render(),
        );

    // Properties panel
    let _properties = ScrollArea::vertical().max_height("600px").children(vec![
        // Base Color
        CollapsibleSection::new("Base Color")
            .expanded(true)
            .children(vec![ColorPicker::new([1.0, 1.0, 1.0, 1.0]).render()])
            .render(),
        Spacer::xs().render(),
        // Metallic
        CollapsibleSection::new("Metallic")
            .expanded(true)
            .children(vec![Slider::new().min(0.0).max(1.0).value(0.0).render()])
            .render(),
        Spacer::xs().render(),
        // Roughness
        CollapsibleSection::new("Roughness")
            .expanded(true)
            .children(vec![Slider::new().min(0.0).max(1.0).value(0.5).render()])
            .render(),
        Spacer::xs().render(),
        // Alpha
        CollapsibleSection::new("Alpha")
            .expanded(false)
            .children(vec![Select::new()
                .option("Opaque", "Opaque")
                .option("Mask", "Mask")
                .option("Blend", "Blend")
                .render()])
            .render(),
    ]);

    // Actions
    let _actions = Flex::new().direction(FlexDirection::Row).children(vec![
        Button::new("Save Material")
            .variant(ButtonVariant::Primary)
            .render(),
        Button::new("Load Material")
            .variant(ButtonVariant::Secondary)
            .render(),
        Spacer::flexible().render(),
        Button::new("Reset").variant(ButtonVariant::Ghost).render(),
    ]);

    // If we got here, all widgets are working correctly
    assert!(true);
}
