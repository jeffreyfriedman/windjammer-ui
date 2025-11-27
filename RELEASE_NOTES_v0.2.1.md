# Windjammer UI v0.2.1 - Release Workflow Hotfix

## Overview

This is a patch release to fix the release workflow that failed during v0.2.0 publishing. No code changes to the library itself.

## 🐛 Fixes

### Release Workflow System Dependencies
- **Problem**: The publish job failed due to missing `libsoup-3.0` system dependency
- **Fix**: Added complete GTK/WebKit system dependency list to release workflow
- **Impact**: Release workflow can now successfully build and publish to crates.io

### System Dependencies Added
```bash
pkg-config
libglib2.0-dev
libgtk-3-dev
libjavascriptcoregtk-4.1-dev
libwebkit2gtk-4.1-dev
libsoup-3.0-dev
libxdo-dev
libssl-dev
libayatana-appindicator3-dev
librsvg2-dev
```

### Platform-Specific Testing
- Changed release workflow tests from `--all-features` to `--features web`
- Avoids winit platform issues on Linux (same as test workflow)

## 📦 What's Included

This release includes all features from v0.2.0:
- **55 components** (10 new in v0.2.0)
- All written in pure Windjammer
- Interactive gallery with dark mode
- Comprehensive test suite
- Production-ready build system

## 🔗 Changes Since v0.2.0

- `.github/workflows/release.yml` - Updated system dependencies and test configuration
- `Cargo.toml` - Version bump to 0.2.1

## 📝 Upgrade Notes

No code changes. If you're already using v0.2.0, you can continue using it. This release only fixes the publishing workflow.

```toml
[dependencies]
windjammer-ui = "0.2.1"
```

Requires Windjammer v0.37.2 or later.

---

**Full Changelog**: https://github.com/jeffreyfriedman/windjammer-ui/compare/v0.2.0...v0.2.1

