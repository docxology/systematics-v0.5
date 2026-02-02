# GitHub Configuration Guidelines

## Overview

This document provides guidance for maintaining GitHub Actions and templates.

## Workflow Modification

### Adding a New Workflow

1. Create file in `.github/workflows/`
2. Use descriptive name (e.g., `security-audit.yml`)
3. Document purpose in header comment
4. Add to this README

### Modifying CI Pipeline

When updating `ci.yml`:

1. Test locally first when possible
2. Keep job names descriptive
3. Use caching for dependencies
4. Fail fast on errors

### Key CI Patterns

```yaml
# Cache Cargo dependencies
- name: Cache Cargo dependencies
  uses: actions/cache@v4
  with:
    path: |
      ~/.cargo/bin/
      ~/.cargo/registry/index/
      ~/.cargo/registry/cache/
      ~/.cargo/git/db/
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```

```yaml
# WASM target
- name: Install wasm32 target
  run: rustup target add wasm32-unknown-unknown
```

## Deployment Workflow

### Fly.io Deployment

The `deploy.yml` uses:

- `superfly/flyctl-actions/setup-flyctl@master`
- `FLY_API_TOKEN` secret

To deploy manually:

```bash
flyctl deploy --remote-only
```

### Shuttle Deployment (Alternative)

For Shuttle deployment:

```bash
cargo shuttle deploy --package systematics-backend
```

## Issue Templates

### Template Structure

```markdown
---
name: Template Name
about: Brief description
title: ""
labels: label1, label2
assignees: ""
---

Template body...
```

### Adding New Template

1. Create `.md` file in `ISSUE_TEMPLATE/`
2. Add YAML front matter
3. Include clear sections
4. Update `config.yml` if needed

## Dependabot Configuration

Current settings in `dependabot.yml`:

- Package ecosystem: `cargo`
- Update schedule: weekly
- Target branch: `main`

## Secret Management

### Required Secrets

| Secret | How to Obtain |
|--------|--------------|
| `FLY_API_TOKEN` | `flyctl tokens create deploy` |

### Adding Secrets

1. Go to Settings → Secrets and variables → Actions
2. Click "New repository secret"
3. Add name and value
4. Reference in workflow: `${{ secrets.SECRET_NAME }}`

## Troubleshooting

### CI Failures

1. Check Actions tab for error details
2. Run failed command locally
3. Verify Rust toolchain version
4. Check for dependency conflicts

### wasm-opt Issues

CI includes a dummy wasm-opt workaround:

```yaml
- name: Create dummy wasm-opt
  run: |
    mkdir -p ~/.cache/trunk/wasm-opt-version_116/bin
    # ... creates passthrough script
```

This bypasses incompatibility between recent Rust and wasm-opt v116.

### Deployment Failures

1. Verify `FLY_API_TOKEN` is set
2. Check Fly.io dashboard for app status
3. Review Docker build logs
4. Ensure `fly.toml` is valid
