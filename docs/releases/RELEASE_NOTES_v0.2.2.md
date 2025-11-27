# Windjammer UI v0.2.2 - Release Workflow Fix

## Overview

This is a patch release to fix the publishing workflow. No code changes to the library itself.

## 🐛 Fixes

### CARGO_TOKEN Workflow Issue
- **Problem**: v0.2.1 release failed with "a value is required for '--token <TOKEN>'" error
- **Root Cause**: Using `--token ${{ secrets.CARGO_TOKEN }}` flag didn't pass the token correctly
- **Fix**: Changed to use `CARGO_REGISTRY_TOKEN` environment variable (standard approach)
- **Impact**: Release workflow can now successfully publish to crates.io

### Workflow Changes
```yaml
# Before
run: cargo publish --token ${{ secrets.CARGO_TOKEN }}

# After
env:
  CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_TOKEN }}
run: cargo publish
```

This matches the successful pattern used in the `windjammer` repository.

## 📦 What's Included

This release includes all features from v0.2.0:
- **55 components** (all written in pure Windjammer)
- Interactive gallery with dark mode
- Comprehensive test suite (112 tests)
- Production-ready build system
- Full documentation

## 🔗 Changes Since v0.2.1

- `.github/workflows/release.yml` - Fixed CARGO_TOKEN usage

## 📝 Upgrade Notes

No code changes. If you're already using v0.2.0 or v0.2.1, you can continue using them. This release only fixes the publishing workflow.

```toml
[dependencies]
windjammer-ui = "0.2.2"
```

Requires Windjammer v0.37.2 or later.

---

**Full Changelog**: https://github.com/jeffreyfriedman/windjammer-ui/compare/v0.2.1...v0.2.2

