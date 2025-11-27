# Windjammer UI v0.2.3 - Secret Name Fix

## Overview

This is a patch release to fix the publishing workflow with the correct secret name. No code changes to the library itself.

## 🐛 Fixes

### CARGO_REGISTRY_TOKEN Secret Name
- **Problem**: Workflow was using `secrets.CARGO_TOKEN` but the repository secret is named `CARGO_REGISTRY_TOKEN`
- **Root Cause**: Secret name mismatch between workflow and repository settings
- **Fix**: Updated workflow to use the correct secret name
- **Impact**: Release workflow can now successfully publish to crates.io

### Workflow Changes
```yaml
# Before
run: cargo publish --token ${{ secrets.CARGO_TOKEN }}

# After
run: cargo publish --token ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

### Manual Workflow Trigger
Added `workflow_dispatch` to allow manual triggering of releases from GitHub Actions UI:
```yaml
on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:
    inputs:
      version:
        description: 'Version to publish (e.g., 0.2.3)'
        required: true
        type: string
```

## 📦 What's Included

This release includes all features from v0.2.0:
- **55 components** (all written in pure Windjammer)
- Interactive gallery with dark mode
- Comprehensive test suite (112 tests)
- Production-ready build system
- Full documentation

## 🔗 Changes Since v0.2.2

- `.github/workflows/release.yml` - Fixed secret name and added manual trigger

## 📝 Upgrade Notes

No code changes. If you're already using v0.2.0, v0.2.1, or v0.2.2, you can continue using them. This release only fixes the publishing workflow.

```toml
[dependencies]
windjammer-ui = "0.2.3"
```

Requires Windjammer v0.37.2 or later.

---

**Full Changelog**: https://github.com/jeffreyfriedman/windjammer-ui/compare/v0.2.2...v0.2.3

