//! Windjammer UI Component Library
//!
//! Production-ready components for building web, desktop, and mobile applications.

// ===========================================================================
// GENERATED WINDJAMMER COMPONENTS (from .wj source)
// ===========================================================================
pub mod generated;

// ===========================================================================
// LEGACY RUST COMPONENTS (ALL MIGRATED TO WINDJAMMER! 🎉)
// ===========================================================================
// pub mod advanced_code_editor; // MIGRATED to generated/advancedcodeeditor.wj
// pub mod alert; // MIGRATED to generated/alert.wj
// pub mod badge; // MIGRATED to generated/badge.wj
// pub mod button; // MIGRATED to generated/button.wj
// pub mod card; // MIGRATED to generated/card.wj
// pub mod checkbox; // MIGRATED to generated/checkbox.wj
// pub mod code_editor; // MIGRATED to generated/codeeditor.wj
// pub mod collapsible_section; // MIGRATED to generated/collapsible.wj
// pub mod color_picker; // MIGRATED to generated/colorpicker.wj
// pub mod container; // MIGRATED to generated/container.wj
// pub mod dialog; // MIGRATED to generated/dialog.wj
// pub mod divider; // MIGRATED to generated/divider.wj
// pub mod file_tree; // MIGRATED to generated/filetree.wj
// pub mod flex; // MIGRATED to generated/flex.wj
// pub mod grid; // MIGRATED to generated/grid.wj
// pub mod input; // MIGRATED to generated/input.wj
// pub mod panel; // MIGRATED to generated/panel.wj
// pub mod progress; // MIGRATED to generated/progress.wj
// pub mod radio; // MIGRATED to generated/radio.wj
// pub mod scroll_area; // MIGRATED to generated/scrollarea.wj
// pub mod select; // MIGRATED to generated/select.wj
// pub mod slider; // MIGRATED to generated/slider.wj
// pub mod spacer; // MIGRATED to generated/spacer.wj
// pub mod spinner; // MIGRATED to generated/spinner.wj
// pub mod split_panel; // MIGRATED to generated/splitpanel.wj
// pub mod switch; // MIGRATED to generated/switch.wj
// pub mod tab_panel; // MIGRATED to generated/tabpanel.wj
// pub mod tabs; // MIGRATED to generated/tabs.wj
// pub mod text; // MIGRATED to generated/text.wj
// pub mod toolbar; // MIGRATED to generated/toolbar.wj
// pub mod tooltip; // MIGRATED to generated/tooltip.wj
// pub mod tree_view; // MIGRATED to generated/treeview.wj

// 🎉 100% WINDJAMMER! All 32+ UI components now written in pure Windjammer!

// ===========================================================================
// RE-EXPORTS - ALL Windjammer Components (32 total!)
// ===========================================================================
// Re-export Renderable trait for convenience
pub use generated::Renderable;

pub use generated::{
    // Advanced Components
    Accordion,
    AccordionItem,
    AdvancedCodeEditor,
    // Basic Components
    Alert,
    AlertVariant,
    Avatar,
    AvatarSize,
    Badge,
    BadgeSize,
    BadgeVariant,
    // Navigation Components
    Breadcrumb,
    BreadcrumbItem,
    Button,
    ButtonSize,
    ButtonVariant,
    // Data Display
    Card,
    // Chat Components
    ChatInput,
    ChatMessage,
    Checkbox,
    CheckboxSize,
    CodeBlock,
    CodeEditor,
    CollapsibleSection,
    // Form Components
    ColorPicker,
    // Layout Components
    Container,
    // Interaction Components
    ContextMenu,
    ContextMenuItem,
    Dialog,
    Divider,
    DividerOrientation,
    Dropdown,
    DropdownItem,
    // Tree Components
    FileNode,
    FileTree,
    Flex,
    FlexDirection,
    Grid,
    HamburgerMenu,
    HamburgerMenuItem,
    Input,
    Menu,
    MenuItem,
    MessageList,
    MessageRole,
    Navbar,
    NavbarItem,
    NavbarPosition,
    Pagination,
    Panel,
    Progress,
    ProgressVariant,
    RadioGroup,
    RadioOption,
    ScrollArea,
    ScrollDirection,
    Select,
    SelectOption,
    Sidebar,
    SidebarItem,
    SidebarPosition,
    Skeleton,
    SkeletonVariant,

    Slider,
    Spacer,
    Spinner,
    SpinnerSize,
    SplitDirection,
    SplitPanel,

    Switch,
    SwitchSize,

    Tab,
    TabPanel,
    TabPanelTab,
    Tabs,
    Text,
    TextSize,
    TextWeight,

    Toast,
    ToastPosition,
    ToastVariant,

    Toolbar,
    Tooltip,
    TooltipPosition,

    TreeItem,
    TreeView,

    TypingIndicator,
};
