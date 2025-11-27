# Windjammer UI v0.2.0 - Major Feature Release 🎉

## Overview

This release adds **10 new production-ready UI components** to `windjammer-ui`, bringing the total to **55 components**, all written in pure Windjammer (`.wj` files). This major dogfooding milestone validates that Windjammer is ready for real-world library development and identified critical compiler bugs (fixed in companion Windjammer v0.37.2 release).

## 🆕 New Components

### Layout & Structure
1. **Stack** - Simplified vertical/horizontal layout with gap, alignment, and justification
2. **Form** - Form wrapper with action, method, and onsubmit handlers
3. **FormField** - Form field with label, help text, error messages, and required indicator

### Data Display
4. **Table** - Data table with striped rows, borders, hover effects, and customizable columns
5. **Timeline** - Chronological event display with timestamps, icons, and descriptions
6. **Stepper** - Multi-step progress indicator for wizards and forms

### Feedback & Interaction
7. **Loading** - Loading states with spinner, message, and optional overlay
8. **Modal** - Overlay dialog with backdrop, customizable size, and close button

### Tags & Labels
9. **Chip** - Tag/pill component with 6 variants, removable option, and icons
10. **Rating** - Star rating display with half-star support and customizable colors

## 🔧 Build System Improvements

### Automated Build Pipeline
- `build.rs` now automatically transpiles `.wj` files to Rust during compilation
- Added `--no-cargo` flag support for library builds
- Integrated `rustfmt` for generated code formatting
- Seamless integration with Cargo build process

### CI/CD Enhancements
- Platform-specific feature testing (web on Linux/Windows, all features on macOS)
- System dependency installation for GTK/GLib on Linux
- Improved pre-commit hooks with `cargo publish --dry-run` validation
- Fixed formatting check order (build → format → clippy)

## 🐛 Code Quality Fixes

### Clippy Warnings Resolved
- **needless_borrow**: Removed redundant `&` in `push_str()` calls
- **single_char_add_str**: Use `push()` instead of `push_str()` for single characters
- **explicit_counter_loop**: Replaced manual counters with `.enumerate()`
- **unused_variables**: Prefixed unused variables with `_`
- **noop_method_call**: Fixed via Windjammer compiler improvements (see below)

### Generated Code Quality
- All 55 components now generate clean, idiomatic Rust code
- Zero clippy warnings in generated code
- Proper formatting applied automatically

## 🔗 Companion Windjammer v0.37.2 Release

This release required fixes in the Windjammer compiler (v0.37.2):

### Compiler Bug Fixes
1. **String Literal `.to_string()` Bug**: Fixed incorrect `.to_string()` on string literals passed to `push_str()`
2. **Tuple Pattern Matching**: Fixed `.as_str()` being incorrectly added to tuple match values
3. **AUTO-CLONE String Literals**: Compiler now tracks string literal variables and skips unnecessary `.clone()` calls
4. **External Crate Naming**: Fixed underscore → hyphen conversion for crate names in generated `Cargo.toml`

## 📊 Statistics

- **Total Components**: 55 (up from 49)
- **Lines of Windjammer Code**: ~5,500 lines of `.wj` source
- **Generated Rust Code**: ~15,000 lines (auto-generated and formatted)
- **Test Coverage**: 112 tests passing
- **Clippy Warnings**: 0

## 🎨 Gallery Updates

The interactive component gallery (`examples/gallery.html`) has been updated with:
- All 55 components showcased with live demos
- Dark mode support
- Interactive examples (modals, toasts, pagination, navigation)
- Responsive design
- Zero JavaScript required for most components

## 📝 Documentation

- Updated README with 55 component count
- Added Windjammer v0.37.2+ requirement badge
- Inline documentation in all component source files
- Example apps demonstrating component usage

## 🚀 Dogfooding Impact

This release proves that:
- ✅ Windjammer can build sophisticated UI libraries
- ✅ The builder pattern works seamlessly in Windjammer
- ✅ Generated Rust code is production-quality
- ✅ The compiler correctly infers ownership and mutability
- ✅ The development workflow is smooth and efficient

## ⚠️ Breaking Changes

None. This is a purely additive release.

## 📦 Installation

```toml
[dependencies]
windjammer-ui = "0.2.0"
```

Requires Windjammer v0.37.2 or later.

## 🔜 What's Next

- Continue dogfooding with `windjammer-game` framework
- Add more interactive components based on user feedback
- Improve desktop platform support
- Expand mobile platform support

## 🙏 Acknowledgments

This release represents a major milestone in the Windjammer project. The dogfooding process identified and fixed critical compiler bugs, making Windjammer more robust for all users.

---

**Full Changelog**: https://github.com/jeffreyfriedman/windjammer-ui/compare/v0.1.1...v0.2.0

