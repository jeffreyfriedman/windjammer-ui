//! Dashboard and Data Display Tests
//! 
//! Test common patterns for business dashboards and data visualization

use windjammer_ui::components::*;
use windjammer_ui::simple_vnode::VNode;
use windjammer_ui::to_vnode::ToVNode;

#[test]
fn test_stat_card() {
    // Test a statistics card component
    let stat_card = Card::new()
        .child(Text::new("Total Users").render())
        .child(Text::new("1,234").render())
        .child(Badge::new("+12%").variant(BadgeVariant::Success).render())
        .render();
    
    assert!(matches!(stat_card, VNode::Element { .. }));
}

#[test]
fn test_grid_layout_for_stats() {
    // Test grid layout for multiple stat cards
    let stats_grid = Grid::new()
        .columns(4)
        .children(vec![
            Card::new().render(),
            Card::new().render(),
            Card::new().render(),
            Card::new().render(),
        ])
        .render();
    
    match stats_grid {
        VNode::Element { children, .. } => {
            assert_eq!(children.len(), 4);
        }
        _ => panic!("Expected element node"),
    }
}

#[test]
fn test_progress_indicator() {
    // Test progress bar for goals/metrics
    let progress = Progress::new(75.0).render();
    assert!(matches!(progress, VNode::Element { .. }));
    
    let circular = Progress::new(60.0)
        .variant(ProgressVariant::Circular)
        .render();
    assert!(matches!(circular, VNode::Element { .. }));
}

#[test]
fn test_badge_variants_for_status() {
    // Test different badge types for status indicators
    let success = Badge::new("Active")
        .variant(BadgeVariant::Success)
        .render();
    
    let warning = Badge::new("Pending")
        .variant(BadgeVariant::Warning)
        .render();
    
    let error = Badge::new("Error")
        .variant(BadgeVariant::Error)
        .render();
    
    let info = Badge::new("Info")
        .variant(BadgeVariant::Info)
        .render();
    
    assert!(matches!(success, VNode::Element { .. }));
    assert!(matches!(warning, VNode::Element { .. }));
    assert!(matches!(error, VNode::Element { .. }));
    assert!(matches!(info, VNode::Element { .. }));
}

#[test]
fn test_activity_feed_layout() {
    // Test activity/notification feed
    let activity_item = Flex::new()
        .direction(FlexDirection::Row)
        .children(vec![
            Container::new()
                .child(Text::new("JD").render())
                .render(), // Avatar
            Flex::new()
                .direction(FlexDirection::Column)
                .children(vec![
                    Text::new("John Doe updated the report").render(),
                    Text::new("2 hours ago").render(),
                ])
                .render(),
        ])
        .render();
    
    match activity_item {
        VNode::Element { children, .. } => {
            assert_eq!(children.len(), 2);
        }
        _ => panic!("Expected element node"),
    }
}

#[test]
fn test_tabs_for_different_views() {
    // Test tabs for switching between dashboard views
    let tabs = Tabs::new()
        .tab(Tab::new("tab1", "Overview"))
        .tab(Tab::new("tab2", "Analytics"))
        .tab(Tab::new("tab3", "Reports"))
        .render();
    
    assert!(matches!(tabs, VNode::Element { .. }));
}

#[test]
fn test_split_panel_layout() {
    // Test split panel for dashboard with sidebar
    let split = SplitPanel::new()
        .direction(SplitDirection::Horizontal)
        .left(Text::new("Sidebar").render())
        .right(Text::new("Main Content").render())
        .render();
    
    assert!(matches!(split, VNode::Element { .. }));
}

#[test]
fn test_toolbar_with_actions() {
    // Test toolbar for dashboard actions
    let toolbar = Toolbar::new()
        .child(Button::new("Refresh").render())
        .child(Button::new("Export").render())
        .child(Button::new("Settings").render())
        .render();
    
    assert!(matches!(toolbar, VNode::Element { .. }));
}

#[test]
fn test_card_with_header_and_content() {
    // Test card component structure
    let card = Card::new()
        .child(Text::new("Card Title").render())
        .child(Divider::new().render())
        .child(Text::new("Card content goes here").render())
        .render();
    
    assert!(matches!(card, VNode::Element { .. }));
}

#[test]
fn test_metric_comparison() {
    // Test side-by-side metric comparison
    let comparison = Flex::new()
        .direction(FlexDirection::Row)
        .children(vec![
            Flex::new()
                .direction(FlexDirection::Column)
                .children(vec![
                    Text::new("This Month").render(),
                    Text::new("$45,678").render(),
                ])
                .render(),
            Divider::new().render(),
            Flex::new()
                .direction(FlexDirection::Column)
                .children(vec![
                    Text::new("Last Month").render(),
                    Text::new("$42,123").render(),
                ])
                .render(),
        ])
        .render();
    
    match comparison {
        VNode::Element { children, .. } => {
            assert_eq!(children.len(), 3);
        }
        _ => panic!("Expected element node"),
    }
}

#[test]
fn test_time_range_selector() {
    // Test time range filter buttons
    let time_selector = Flex::new()
        .direction(FlexDirection::Row)
        .children(vec![
            Button::new("Day").variant(ButtonVariant::Ghost).render(),
            Button::new("Week").variant(ButtonVariant::Primary).render(),
            Button::new("Month").variant(ButtonVariant::Ghost).render(),
        ])
        .render();
    
    match time_selector {
        VNode::Element { children, .. } => {
            assert_eq!(children.len(), 3);
        }
        _ => panic!("Expected element node"),
    }
}

#[test]
fn test_responsive_dashboard_layout() {
    // Test responsive grid that adjusts to screen size
    let dashboard = Flex::new()
        .direction(FlexDirection::Column)
        .children(vec![
            // Header
            Flex::new()
                .direction(FlexDirection::Row)
                .children(vec![
                    Text::new("Dashboard").render(),
                    Button::new("Refresh").render(),
                ])
                .render(),
            // Stats grid
            Grid::new()
                .columns(4)
                .children(vec![
                    Card::new().render(),
                    Card::new().render(),
                    Card::new().render(),
                    Card::new().render(),
                ])
                .render(),
            // Main content
            Flex::new()
                .direction(FlexDirection::Row)
                .children(vec![
                    Card::new().render(), // Chart
                    Card::new().render(), // Activity feed
                ])
                .render(),
        ])
        .render();
    
    match dashboard {
        VNode::Element { children, .. } => {
            assert_eq!(children.len(), 3);
        }
        _ => panic!("Expected element node"),
    }
}

