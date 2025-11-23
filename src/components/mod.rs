//! Windjammer UI Component Library
//!
//! Production-ready components for building web, desktop, and mobile applications.

// ===========================================================================
// GENERATED WINDJAMMER COMPONENTS (from .wj source)
// ===========================================================================
pub mod generated;

// ===========================================================================
// LEGACY RUST COMPONENTS (to be migrated to .wj)
// ===========================================================================
pub mod advanced_code_editor;
// pub mod alert; // MIGRATED to generated/alert.wj
// pub mod badge; // MIGRATED to generated/badge.wj
// pub mod button; // MIGRATED to generated/button.wj
pub mod card;
// pub mod checkbox; // MIGRATED to generated/checkbox.wj
pub mod code_editor;
pub mod collapsible_section;
pub mod color_picker;
// pub mod container; // MIGRATED to generated/container.wj
pub mod dialog;
// pub mod divider; // MIGRATED to generated/divider.wj
pub mod file_tree;
// pub mod flex; // MIGRATED to generated/flex.wj
pub mod grid;
// pub mod input; // MIGRATED to generated/input.wj
pub mod panel;
pub mod progress;
pub mod radio;
pub mod scroll_area;
pub mod select;
// pub mod slider; // MIGRATED to generated/slider.wj
// pub mod spacer; // MIGRATED to generated/spacer.wj
// pub mod spinner; // MIGRATED to generated/spinner.wj
pub mod split_panel;
pub mod switch;
pub mod tab_panel;
pub mod tabs;
// pub mod text; // MIGRATED to generated/text.wj
pub mod toolbar;
pub mod tooltip;
pub mod tree_view;

// ===========================================================================
// RE-EXPORTS - Generated Windjammer Components
// ===========================================================================
pub use generated::{
    Alert, AlertVariant, Badge, BadgeSize, BadgeVariant, Button, ButtonSize, ButtonVariant,
    Checkbox, CheckboxSize, Container, Divider, DividerOrientation, Flex, FlexDirection, Input,
    Slider, Spacer, Spinner, SpinnerSize, Text, TextSize, TextWeight,
};

// ===========================================================================
// RE-EXPORTS - Legacy Rust Components
// ===========================================================================
pub use advanced_code_editor::AdvancedCodeEditor;
pub use card::Card;
pub use code_editor::CodeEditor;
pub use collapsible_section::CollapsibleSection;
pub use color_picker::ColorPicker;
pub use dialog::Dialog;
pub use file_tree::{FileNode, FileTree};
pub use grid::Grid;
pub use panel::Panel;
pub use progress::{Progress, ProgressVariant};
pub use radio::{RadioGroup, RadioOption};
pub use scroll_area::{ScrollArea, ScrollDirection};
pub use select::{Select, SelectOption};
pub use split_panel::{SplitDirection, SplitPanel};
pub use switch::{Switch, SwitchSize};
pub use tab_panel::{Tab as TabPanelTab, TabPanel};
pub use tabs::{Tab, Tabs};
pub use toolbar::Toolbar;
pub use tooltip::{Tooltip, TooltipPosition};
pub use tree_view::{TreeItem, TreeView};
